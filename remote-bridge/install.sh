#!/bin/bash
# Install remote-bridge binary from GitHub releases
set -e

REPO="${REMOTE_BRIDGE_REPO:-oclb/oconnorlab-agent-config}"
BINARY="remote-bridge"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    darwin)
        case "$ARCH" in
            x86_64) PLATFORM="darwin-x86_64" ;;
            arm64)  PLATFORM="darwin-arm64" ;;
            *)      echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    linux)
        case "$ARCH" in
            x86_64) PLATFORM="linux-x86_64" ;;
            *)      echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

ASSET_NAME="$BINARY-$PLATFORM"

# Get latest release or use specified version
VERSION="${VERSION:-latest}"
if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$ASSET_NAME"
else
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET_NAME"
fi

echo "Installing $BINARY ($PLATFORM)..."
echo "Download URL: $DOWNLOAD_URL"

# Create install directory if needed
mkdir -p "$INSTALL_DIR"

# Download and install
curl -fsSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/$BINARY"
chmod +x "$INSTALL_DIR/$BINARY"

echo "Installed to $INSTALL_DIR/$BINARY"

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "Note: $INSTALL_DIR is not in your PATH."
    echo "Add it with: export PATH=\"\$PATH:$INSTALL_DIR\""
fi
