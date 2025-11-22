//! Command execution pipeline with progress UI.
//!
//! Presents step-by-step execution status, live logs, and cancellation support.
//! Replaces the terminal UI with a clean progress dialog showing a progress bar,
//! friendly status messages, and collapsible output details.

use crate::{aur_helper, utils};
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Button, Expander, Label, ProgressBar, TextBuffer, TextTag, TextView, Window};
use log::{error, info, warn};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Command execution context (privilege, helpers, etc.)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandType {
    Normal,
    Privileged,
    Aur,
}

/// Global flag to track if an action is currently running
static ACTION_RUNNING: AtomicBool = AtomicBool::new(false);

/// Check if an action is currently running
pub fn is_action_running() -> bool {
    ACTION_RUNNING.load(Ordering::SeqCst)
}

/// Command to execute
#[derive(Clone, Debug)]
pub struct CommandStep {
    pub command_type: CommandType,
    pub command: String,
    pub args: Vec<String>,
    pub friendly_name: String,
}

impl CommandStep {
    /// Create a new command with an explicit command type
    pub fn new(
        command_type: CommandType,
        command: &str,
        args: &[&str],
        friendly_name: &str,
    ) -> Self {
        Self {
            command_type,
            command: command.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            friendly_name: friendly_name.to_string(),
        }
    }

    /// Convenience helper for normal commands
    pub fn normal(command: &str, args: &[&str], friendly_name: &str) -> Self {
        Self::new(CommandType::Normal, command, args, friendly_name)
    }

    /// Convenience helper for privileged commands (runs through pkexec)
    pub fn privileged(command: &str, args: &[&str], friendly_name: &str) -> Self {
        Self::new(CommandType::Privileged, command, args, friendly_name)
    }

    /// Convenience helper for AUR helper commands (paru/yay)
    pub fn aur(args: &[&str], friendly_name: &str) -> Self {
        Self::new(CommandType::Aur, "aur", args, friendly_name)
    }
}

struct CommandExecutionWidgets {
    window: Window,
    title_label: Label,
    progress_bar: ProgressBar,
    output_view: TextView,
    output_buffer: TextBuffer,
    cancel_button: Button,
    close_button: Button,
    expander: Expander,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CommandResult {
    Success,
    Failure { exit_code: Option<i32> },
}

struct RunningCommandContext {
    widgets: Rc<CommandExecutionWidgets>,
    commands: Rc<Vec<CommandStep>>,
    index: usize,
    cancelled: Rc<RefCell<bool>>,
    on_complete: Option<Rc<dyn Fn(bool) + 'static>>,
    current_process: Rc<RefCell<Option<gio::Subprocess>>>,
    stdout_done: Cell<bool>,
    stderr_done: Cell<bool>,
    exit_result: RefCell<Option<CommandResult>>,
}

impl RunningCommandContext {
    fn new(
        widgets: Rc<CommandExecutionWidgets>,
        commands: Rc<Vec<CommandStep>>,
        index: usize,
        cancelled: Rc<RefCell<bool>>,
        on_complete: Option<Rc<dyn Fn(bool) + 'static>>,
        current_process: Rc<RefCell<Option<gio::Subprocess>>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            widgets,
            commands,
            index,
            cancelled,
            on_complete,
            current_process,
            stdout_done: Cell::new(false),
            stderr_done: Cell::new(false),
            exit_result: RefCell::new(None),
        })
    }

    fn mark_stream_done(self: &Rc<Self>, is_error_stream: bool) {
        if is_error_stream {
            self.stderr_done.set(true);
        } else {
            self.stdout_done.set(true);
        }
        self.try_finalize();
    }

    fn set_exit_result(self: &Rc<Self>, result: CommandResult) {
        *self.exit_result.borrow_mut() = Some(result);
        self.try_finalize();
    }

    fn try_finalize(self: &Rc<Self>) {
        if !(self.stdout_done.get() && self.stderr_done.get()) {
            return;
        }

        let result = {
            let mut exit_result = self.exit_result.borrow_mut();
            exit_result.take()
        };

        let Some(result) = result else {
            return;
        };

        self.current_process.borrow_mut().take();

        if *self.cancelled.borrow() {
            finalize_dialog(&self.widgets, false, "Operation cancelled");
            if let Some(callback) = &self.on_complete {
                callback(false);
            }
            return;
        }

        match result {
            CommandResult::Success => {
                append_output(&self.widgets, "✓ Step completed successfully\n", false);
                execute_commands_sequence(
                    self.widgets.clone(),
                    self.commands.clone(),
                    self.index + 1,
                    self.cancelled.clone(),
                    self.on_complete.clone(),
                    self.current_process.clone(),
                );
            }
            CommandResult::Failure { exit_code } => {
                if let Some(code) = exit_code {
                    append_output(
                        &self.widgets,
                        &format!("✗ Command failed with exit code: {}\n", code),
                        true,
                    );
                }
                finalize_dialog(
                    &self.widgets,
                    false,
                    &format!(
                        "Operation failed at step {} of {}",
                        self.index + 1,
                        self.commands.len()
                    ),
                );
                if let Some(callback) = &self.on_complete {
                    callback(false);
                }
            }
        }
    }
}

// Remove DataStreamReader struct and add helper functions
fn attach_stream_reader(
    subprocess: &gio::Subprocess,
    context: Rc<RunningCommandContext>,
    is_error_stream: bool,
) {
    let stream = if is_error_stream {
        subprocess.stderr_pipe()
    } else {
        subprocess.stdout_pipe()
    };

    if let Some(stream) = stream {
        let data_stream = gio::DataInputStream::new(&stream);
        read_stream(data_stream, context, is_error_stream);
    } else {
        context.mark_stream_done(is_error_stream);
    }
}

fn read_stream(
    data_stream: gio::DataInputStream,
    context: Rc<RunningCommandContext>,
    is_error_stream: bool,
) {
    let stream_clone = data_stream.clone();
    data_stream.clone().read_line_utf8_async(
        glib::Priority::default(),
        None::<&gio::Cancellable>,
        move |res| match res {
            Ok(Some(line)) => {
                let mut text = line.to_string();
                text.push('\n');
                append_output(&context.widgets, &text, is_error_stream);
                read_stream(stream_clone.clone(), context.clone(), is_error_stream);
            }
            Ok(None) => {
                context.mark_stream_done(is_error_stream);
            }
            Err(err) => {
                append_output(
                    &context.widgets,
                    &format!("✗ Failed to read command output: {}\n", err),
                    true,
                );
                context.mark_stream_done(is_error_stream);
            }
        },
    );
}

/// Show progress dialog and run commands
pub fn run_commands_with_progress(
    parent: &Window,
    commands: Vec<CommandStep>,
    title: &str,
    on_complete: Option<Box<dyn Fn(bool) + 'static>>,
) {
    if commands.is_empty() {
        error!("No commands provided");
        return;
    }

    if is_action_running() {
        warn!("Action already running - ignoring request");
        return;
    }

    ACTION_RUNNING.store(true, Ordering::SeqCst);

    // Convert callback to Rc for use across non-Send contexts
    let on_complete = on_complete.map(|cb| Rc::new(cb) as Rc<dyn Fn(bool) + 'static>);

    let builder = gtk4::Builder::from_resource("/xyz/xerolinux/xero-toolkit/ui/progress_dialog.ui");

    let window: Window = builder
        .object("progress_window")
        .expect("Failed to get progress_window");
    let title_label: Label = builder
        .object("progress_title")
        .expect("Failed to get progress_title");
    let progress_bar: ProgressBar = builder
        .object("progress_bar")
        .expect("Failed to get progress_bar");
    let output_view: TextView = builder
        .object("output_view")
        .expect("Failed to get output_view");
    let cancel_button: Button = builder
        .object("cancel_button")
        .expect("Failed to get cancel_button");
    let close_button: Button = builder
        .object("close_button")
        .expect("Failed to get close_button");
    let expander: Expander = builder
        .object("output_expander")
        .expect("Failed to get output_expander");

    window.set_transient_for(Some(parent));
    window.set_title(Some(title));

    let output_buffer = output_view.buffer();

    // Create a tag for error text
    let error_tag = TextTag::new(Some("error"));
    error_tag.set_foreground(Some("red"));
    error_tag.set_weight(700); // bold
    output_buffer.tag_table().add(&error_tag);

    let widgets = Rc::new(CommandExecutionWidgets {
        window: window.clone(),
        title_label,
        progress_bar,
        output_view,
        output_buffer,
        cancel_button: cancel_button.clone(),
        close_button: close_button.clone(),
        expander,
    });

    let cancelled = Rc::new(RefCell::new(false));
    let current_process = Rc::new(RefCell::new(None::<gio::Subprocess>));
    let commands = Rc::new(commands);

    // Cancel button handler
    let widgets_clone = widgets.clone();
    let cancelled_clone = cancelled.clone();
    let running_process = current_process.clone();
    cancel_button.connect_clicked(move |_| {
        *cancelled_clone.borrow_mut() = true;
        append_output(&widgets_clone, "\n[Cancelled by user]\n", true);
        widgets_clone.cancel_button.set_sensitive(false);
        if let Some(process) = running_process.borrow().as_ref() {
            process.force_exit();
        }
    });

    // Close button handler
    let widgets_clone = widgets.clone();
    let on_complete_clone = on_complete.clone();
    close_button.connect_clicked(move |_| {
        widgets_clone.window.close();
        if let Some(ref callback) = on_complete_clone {
            callback(true);
        }
    });

    // Window close handler
    let on_complete_clone = on_complete.clone();
    let current_process_clone = current_process.clone();
    window.connect_close_request(move |_| {
        ACTION_RUNNING.store(false, Ordering::SeqCst);
        if let Some(process) = current_process_clone.borrow().as_ref() {
            process.force_exit();
        }
        if let Some(ref callback) = on_complete_clone {
            callback(false);
        }
        glib::Propagation::Proceed
    });

    window.present();

    // Start executing commands
    execute_commands_sequence(
        widgets,
        commands,
        0,
        cancelled,
        on_complete,
        current_process,
    );
}

fn execute_commands_sequence(
    widgets: Rc<CommandExecutionWidgets>,
    commands: Rc<Vec<CommandStep>>,
    index: usize,
    cancelled: Rc<RefCell<bool>>,
    on_complete: Option<Rc<dyn Fn(bool) + 'static>>,
    current_process: Rc<RefCell<Option<gio::Subprocess>>>,
) {
    if *cancelled.borrow() {
        finalize_dialog(&widgets, false, "Operation cancelled");
        if let Some(callback) = on_complete {
            callback(false);
        }
        return;
    }

    if index >= commands.len() {
        finalize_dialog(&widgets, true, "All operations completed successfully!");
        if let Some(callback) = on_complete {
            callback(true);
        }
        return;
    }

    let cmd = &commands[index];
    let total = commands.len();
    let progress = (index as f64) / (total as f64);

    widgets.progress_bar.set_fraction(progress);
    widgets
        .progress_bar
        .set_text(Some(&format!("Step {} of {}", index + 1, total)));
    widgets.title_label.set_label(&cmd.friendly_name);

    append_output(
        &widgets,
        &format!(
            "\n=== Step {}/{}: {} ===\n",
            index + 1,
            total,
            cmd.friendly_name
        ),
        false,
    );

    let (full_command, full_args) = match resolve_command(cmd) {
        Ok(result) => result,
        Err(err) => {
            append_output(&widgets, &format!("✗ {}\n", err), true);
            finalize_dialog(&widgets, false, "Failed to prepare command");
            if let Some(callback) = on_complete {
                callback(false);
            }
            return;
        }
    };

    info!("Executing: {} {:?}", full_command, full_args);

    let mut argv: Vec<OsString> = Vec::with_capacity(1 + full_args.len());
    argv.push(OsString::from(full_command.clone()));
    for arg in &full_args {
        argv.push(OsString::from(arg));
    }
    let argv_refs: Vec<&std::ffi::OsStr> = argv.iter().map(|s| s.as_os_str()).collect();

    let flags = gio::SubprocessFlags::STDOUT_PIPE | gio::SubprocessFlags::STDERR_PIPE;
    let subprocess = match gio::Subprocess::newv(&argv_refs, flags) {
        Ok(proc) => proc,
        Err(err) => {
            append_output(
                &widgets,
                &format!("✗ Failed to start command: {}\n", err),
                true,
            );
            finalize_dialog(&widgets, false, "Failed to start operation");
            if let Some(callback) = on_complete {
                callback(false);
            }
            return;
        }
    };

    *current_process.borrow_mut() = Some(subprocess.clone());

    let context = RunningCommandContext::new(
        widgets.clone(),
        commands.clone(),
        index,
        cancelled.clone(),
        on_complete.clone(),
        current_process.clone(),
    );

    attach_stream_reader(&subprocess, context.clone(), false);
    attach_stream_reader(&subprocess, context.clone(), true);

    let wait_context = context.clone();
    let wait_subprocess = subprocess.clone();
    wait_subprocess
        .clone()
        .wait_async(None::<&gio::Cancellable>, move |result| match result {
            Ok(_) => {
                if wait_subprocess.is_successful() {
                    wait_context.set_exit_result(CommandResult::Success);
                } else {
                    wait_context.set_exit_result(CommandResult::Failure {
                        exit_code: Some(wait_subprocess.exit_status()),
                    });
                }
            }
            Err(err) => {
                append_output(
                    &wait_context.widgets,
                    &format!("✗ Failed to wait for command: {}\n", err),
                    true,
                );
                wait_context.set_exit_result(CommandResult::Failure { exit_code: None });
            }
        });
}

fn resolve_command(command: &CommandStep) -> Result<(String, Vec<String>), String> {
    match command.command_type {
        CommandType::Normal => Ok((command.command.clone(), command.args.clone())),
        CommandType::Privileged => {
            let mut args = Vec::with_capacity(command.args.len() + 1);
            args.push(command.command.clone());
            args.extend(command.args.clone());
            Ok(("pkexec".to_string(), args))
        }
        CommandType::Aur => {
            let helper = aur_helper()
                .map(|h| h.to_string())
                .or_else(|| utils::detect_aur_helper().map(|h| h.to_string()))
                .ok_or_else(|| "AUR helper not initialized (paru or yay required).".to_string())?;
            let mut args = Vec::with_capacity(command.args.len() + 2);
            args.push("--sudo".to_string());
            args.push("pkexec".to_string());
            args.extend(command.args.clone());
            Ok((helper, args))
        }
    }
}

fn append_output(widgets: &CommandExecutionWidgets, text: &str, is_error: bool) {
    let buffer = &widgets.output_buffer;
    let mut end_iter = buffer.end_iter();

    if is_error {
        if let Some(error_tag) = buffer.tag_table().lookup("error") {
            buffer.insert_with_tags(&mut end_iter, text, &[&error_tag]);
        } else {
            buffer.insert(&mut end_iter, text);
        }
    } else {
        buffer.insert(&mut end_iter, text);
    }

    // Auto-scroll to bottom
    let mark = buffer.create_mark(None, &buffer.end_iter(), false);
    widgets
        .output_view
        .scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
}

fn finalize_dialog(widgets: &CommandExecutionWidgets, success: bool, message: &str) {
    ACTION_RUNNING.store(false, Ordering::SeqCst);

    widgets.title_label.set_label(message);
    widgets.cancel_button.set_visible(false);
    widgets.close_button.set_visible(true);
    widgets.close_button.set_sensitive(true);

    if success {
        widgets.progress_bar.set_fraction(1.0);
        widgets.progress_bar.set_text(Some("Completed"));
        append_output(widgets, &format!("\n✓ {}\n", message), false);
    } else {
        append_output(widgets, &format!("\n✗ {}\n", message), true);
        // Expand output on error
        widgets.expander.set_expanded(true);
    }
}