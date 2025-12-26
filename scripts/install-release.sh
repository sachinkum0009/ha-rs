#!/usr/bin/env bash
# Build the project in release mode and install the binary into a location on PATH
# Usage: ./scripts/install-release.sh

set -euo pipefail
IFS=$'\n\t'

# Resolve project root (one level above this script)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/.."
PROJECT_ROOT="$(cd "$PROJECT_ROOT" && pwd)"

BINARY_NAME="ha_cli"
RELEASE_PATH="$PROJECT_ROOT/target/release/$BINARY_NAME"

echo "Project root: $PROJECT_ROOT"

echo "Building release..."
cd "$PROJECT_ROOT"
cargo build --release

if [[ ! -f "$RELEASE_PATH" ]]; then
  echo "Error: release binary not found at $RELEASE_PATH"
  exit 1
fi

# Candidate install locations in order of preference.
# Prefer a user-local bin to avoid sudo when possible.
CANDIDATES=("$HOME/.local/bin" "/opt/homebrew/bin" "/usr/local/bin")
INSTALL_DIR=""

for d in "${CANDIDATES[@]}"; do
  # create user-local bin if it doesn't exist
  if [[ "$d" == "$HOME/.local/bin" && ! -d "$d" ]]; then
    echo "Creating $d"
    mkdir -p "$d"
  fi

  # check if directory exists and is writable (for copy)
  if [[ -d "$d" && -w "$d" ]]; then
    INSTALL_DIR="$d"
    break
  fi

done

# If none writable, try with sudo on opt/homebrew or /usr/local
if [[ -z "$INSTALL_DIR" ]]; then
  for d in "/opt/homebrew/bin" "/usr/local/bin"; do
    if [[ -d "$d" ]]; then
      echo "No writable user install dir found; will attempt to use sudo to copy to $d"
      INSTALL_DIR="$d"
      SUDO_REQUIRED=1
      break
    fi
  done
fi

# If still empty, fallback to $HOME/.local/bin (create it)
if [[ -z "$INSTALL_DIR" ]]; then
  INSTALL_DIR="$HOME/.local/bin"
  echo "Falling back to $INSTALL_DIR (will create it)"
  mkdir -p "$INSTALL_DIR"
fi

DESTPATH="$INSTALL_DIR/$BINARY_NAME"

if [[ "${SUDO_REQUIRED:-0}" -eq 1 ]]; then
  echo "Copying $RELEASE_PATH -> $DESTPATH using sudo"
  sudo cp "$RELEASE_PATH" "$DESTPATH"
  sudo chmod +x "$DESTPATH"
else
  echo "Copying $RELEASE_PATH -> $DESTPATH"
  cp "$RELEASE_PATH" "$DESTPATH"
  chmod +x "$DESTPATH"
fi

echo "Installed $BINARY_NAME to $DESTPATH"

# If installed to $HOME/.local/bin, warn if not on PATH
if [[ "$INSTALL_DIR" == "$HOME/.local/bin" ]]; then
  if ! echo ":$PATH:" | grep -q ":$HOME/.local/bin:"; then
    cat <<EOF
Warning: $HOME/.local/bin is not in your PATH.
You can add it by adding this to your shell rc (for zsh):

  echo 'export PATH="\$HOME/.local/bin:\$PATH"' >> ~/.zprofile
  source ~/.zprofile

After that you will be able to run '$BINARY_NAME' from any terminal.
EOF
  fi
fi

echo "Done. You can run '$BINARY_NAME' from the terminal now (may need a new shell)."
