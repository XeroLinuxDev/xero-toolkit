#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOU'
Usage: tools/bump-version.sh <major|minor|subminor|sync>

Actions:
  major     Bump X in X.Y.Z, reset Y and Z to 0
  minor     Bump Y in X.Y.Z, reset Z to 0
  subminor  Bump Z in X.Y.Z
  sync      Keep version as-is from PKGBUILD and sync Cargo.toml files to it

Notes:
  - Version format is strictly one digit per component: X.Y.Z
  - Each component must be in range 0..9
  - Bumps that would exceed 9 are rejected
  - Cargo manifests to update are defined in CARGO_MANIFEST_DIRS
EOU
}

if [[ $# -ne 1 ]]; then
  usage
  exit 1
fi

action="$1"

case "$action" in
major|minor|subminor|sync) ;;
*)
  usage
  exit 1
  ;;
esac

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." >/dev/null 2>&1 && pwd)"
PKGBUILD_FILE="$REPO_ROOT/packaging/PKGBUILD"
ROOT_CARGO_FILE="$REPO_ROOT/Cargo.toml"

# Relative directories (from repo root) whose Cargo.toml should be version-synced.
CARGO_MANIFEST_DIRS=(
  "."
  "gui"
  "xero-auth"
)

extract_cargo_version() {
  sed -nE 's/^version = "([0-9]+\.[0-9]+\.[0-9]+)"/\1/p' "$ROOT_CARGO_FILE" | head -n1
}

extract_pkgbuild_version() {
  sed -nE 's/^pkgver=([0-9]+\.[0-9]+\.[0-9]+)$/\1/p' "$PKGBUILD_FILE" | head -n1
}

is_single_digit_triplet() {
  [[ "$1" =~ ^[0-9]\.[0-9]\.[0-9]$ ]]
}

set_cargo_manifest_version() {
  local cargo_file="$1"
  local new_version="$2"
  local tmp_file

  if grep -Eq '^version\.workspace[[:space:]]*=[[:space:]]*true$' "$cargo_file"; then
    return 3
  fi

  tmp_file="$(mktemp)"

  if awk -v v="$new_version" '
    BEGIN { in_workspace_package = 0; in_package = 0; changed = 0 }
    {
      if ($0 ~ /^\[workspace\.package\]$/) {
        in_workspace_package = 1
        in_package = 0
        print
        next
      }

      if ($0 ~ /^\[package\]$/) {
        in_package = 1
        in_workspace_package = 0
        print
        next
      }

      if ($0 ~ /^\[[^]]+\]$/) {
        in_workspace_package = 0
        in_package = 0
        print
        next
      }

      if ((in_workspace_package || in_package) && $0 ~ /^version = "/) {
        print "version = \"" v "\""
        changed = 1
        next
      }

      print
    }
    END {
      if (!changed) {
        exit 3
      }
    }
  ' "$cargo_file" > "$tmp_file"; then
    cat "$tmp_file" > "$cargo_file"
    rm -f "$tmp_file"
    return 0
  fi

  local rc=$?
  rm -f "$tmp_file"
  return "$rc"
}

set_versions() {
  local new_version="$1"
  local tmp_file
  local cargo_file
  local dir
  local rc

  local -a updated_cargo_files=()
  local -a skipped_cargo_files=()

  for dir in "${CARGO_MANIFEST_DIRS[@]}"; do
    cargo_file="$REPO_ROOT/$dir/Cargo.toml"

    if [[ ! -f "$cargo_file" ]]; then
      echo "Error: missing Cargo.toml at $cargo_file" >&2
      exit 1
    fi
    if set_cargo_manifest_version "$cargo_file" "$new_version"; then
      rc=0
    else
      rc=$?
    fi

    if [[ "$rc" -eq 0 ]]; then
      updated_cargo_files+=("$cargo_file")
      continue
    fi

    if [[ "$rc" -eq 3 ]]; then
      skipped_cargo_files+=("$cargo_file")
      continue
    fi

    echo "Error: failed to update version in $cargo_file" >&2
    exit 1
  done

  tmp_file="$(mktemp)"
  awk -v v="$new_version" '
    BEGIN { done = 0 }
    {
      if (!done && $0 ~ /^pkgver=/) {
        print "pkgver=" v
        done = 1
        next
      }
      print
    }
    END {
      if (!done) {
        exit 2
      }
    }
  ' "$PKGBUILD_FILE" > "$tmp_file"
  cat "$tmp_file" > "$PKGBUILD_FILE"
  rm -f "$tmp_file"

  echo "Updated Cargo.toml files:"
  if [[ ${#updated_cargo_files[@]} -eq 0 ]]; then
    echo "  (none)"
  else
    printf '  %s\n' "${updated_cargo_files[@]}"
  fi

  if [[ ${#skipped_cargo_files[@]} -gt 0 ]]; then
    echo "Skipped Cargo.toml files (no explicit version field):"
    printf '  %s\n' "${skipped_cargo_files[@]}"
  fi
}

cargo_version="$(extract_cargo_version)"
pkgbuild_version="$(extract_pkgbuild_version)"

if [[ -z "$cargo_version" ]]; then
  echo "Error: unable to read workspace version from $ROOT_CARGO_FILE" >&2
  exit 1
fi

if [[ -z "$pkgbuild_version" ]]; then
  echo "Error: unable to read pkgver from $PKGBUILD_FILE" >&2
  exit 1
fi

if ! is_single_digit_triplet "$cargo_version"; then
  echo "Error: Cargo version '$cargo_version' must be one-digit triplet (X.Y.Z with 0..9)." >&2
  exit 1
fi

if ! is_single_digit_triplet "$pkgbuild_version"; then
  echo "Error: PKGBUILD version '$pkgbuild_version' must be one-digit triplet (X.Y.Z with 0..9)." >&2
  exit 1
fi

if [[ "$cargo_version" != "$pkgbuild_version" ]]; then
  echo "Notice: Cargo version ($cargo_version) differs from PKGBUILD ($pkgbuild_version); using PKGBUILD as source." >&2
fi

new_version="$pkgbuild_version"

if [[ "$action" != "sync" ]]; then
  IFS='.' read -r major minor subminor <<< "$pkgbuild_version"
  case "$action" in
    major)
      if (( major >= 9 )); then
        echo "Error: cannot bump major beyond 9." >&2
        exit 1
      fi
      new_version="$((major + 1)).0.0"
      ;;
    minor)
      if (( minor >= 9 )); then
        echo "Error: cannot bump minor beyond 9." >&2
        exit 1
      fi
      new_version="${major}.$((minor + 1)).0"
      ;;
    subminor)
      if (( subminor >= 9 )); then
        echo "Error: cannot bump subminor beyond 9." >&2
        exit 1
      fi
      new_version="${major}.${minor}.$((subminor + 1))"
      ;;
  esac
fi

set_versions "$new_version"

echo "Updated versions:"
echo "  PKGBUILD: $pkgbuild_version -> $new_version"
echo "  Workspace Cargo.toml: $cargo_version -> $new_version"
