#!/bin/bash
# LinuxTeasing Installer (v1.0 Pixel Art Edition)

set -e

echo -e "\033[0;36m🐧 Installing LinuxTeasing v1.0...\033[0m"

# 1. Build Release
echo "Building release binary..."
cargo build --release

# 2. Install to /usr/local/bin
TARGET_DIR="/usr/local/bin"
BINARY_NAME="linux-teasing"

if [ -w "$TARGET_DIR" ]; then
    cp "target/release/linux-teasing" "$TARGET_DIR/$BINARY_NAME"
else
    echo "Requesting sudo permission to copy binary to $TARGET_DIR..."
    sudo cp "target/release/linux-teasing" "$TARGET_DIR/$BINARY_NAME"
fi

echo -e "\033[0;32mBinary installed to $TARGET_DIR/$BINARY_NAME\033[0m"

# 3. Add to Shell config
SHELL_CONFIG=""
case "$SHELL" in
    */bash) SHELL_CONFIG="$HOME/.bashrc" ;;
    */zsh) SHELL_CONFIG="$HOME/.zshrc" ;;
    */fish) SHELL_CONFIG="$HOME/.config/fish/config.fish" ;;
    *) echo -e "\033[0;33mCould not detect shell. Please add '$BINARY_NAME' manually.\033[0m"; exit 0 ;;
esac

if [ -f "$SHELL_CONFIG" ]; then
    if grep -q "$BINARY_NAME" "$SHELL_CONFIG"; then
        echo -e "\033[0;37mStartup command already in $SHELL_CONFIG.\033[0m"
    else
        echo "" >> "$SHELL_CONFIG"
        echo "# LinuxTeasing Judgment" >> "$SHELL_CONFIG"
        echo "$BINARY_NAME" >> "$SHELL_CONFIG"
        echo -e "\033[0;32mAdded startup command to $SHELL_CONFIG\033[0m"
    fi
else
    echo -e "\033[0;33mConfig file $SHELL_CONFIG not found.\033[0m"
fi

echo -e "\033[0;36m🐧 Installation Complete. Stay strictly on UTC.\033[0m"
