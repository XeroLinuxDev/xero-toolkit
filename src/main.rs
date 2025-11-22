use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::Application;
use log::info;
use std::sync::OnceLock;

mod config;
mod core;
mod ui;
mod utils;

static AUR_HELPER: OnceLock<String> = OnceLock::new();

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    info!(
        "Starting {} v{}",
        config::app_info::NAME,
        config::app_info::VERSION
    );
    info!("Application ID: {}", config::app_info::ID);

    let app = Application::builder()
        .application_id(config::app_info::ID)
        .build();

    app.connect_activate(ui::setup_application_ui);

    app.run();
}

pub fn set_aur_helper(helper: &str) {
    let _ = AUR_HELPER.set(helper.to_string());
}

pub fn aur_helper() -> Option<&'static str> {
    AUR_HELPER.get().map(String::as_str)
}
