#!/usr/bin/env bash
set -euo pipefail

REPO="subhradeepsarkae-ai/noir"
BIN_DIR="${HOME}/.local/bin"
EXE_PATH="${BIN_DIR}/nr"

# Detect platform
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
esac

case "${OS}" in
    linux)  ASSET="nr-${ARCH}-unknown-linux-gnu" ;;
    darwin) ASSET="nr-${ARCH}-apple-darwin" ;;
    *)      echo "Unsupported OS: ${OS}"; exit 1 ;;
esac

mkdir -p "${BIN_DIR}"

# Check if running from repo (local build) vs remote install
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LOCAL_EXE="$(dirname "${SCRIPT_DIR}")/target/release/nr"
if [ -f "${LOCAL_EXE}" ]; then
    echo "Installing from local build..."
    cp "${LOCAL_EXE}" "${EXE_PATH}"
else
    echo "Downloading nr latest release..."
    TAG=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | cut -d'"' -f4)
    URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}.tar.gz"
    curl -sL "${URL}" | tar xz -C /tmp
    cp "/tmp/${ASSET}/nr" "${EXE_PATH}"
    rm -rf "/tmp/${ASSET}"
fi

chmod +x "${EXE_PATH}"

# Add to PATH if not already there
case ":${PATH}:" in
    *":${BIN_DIR}:"*) ;;
    *) echo "export PATH=\"\${PATH}:${BIN_DIR}\"" >> "${HOME}/.bashrc"
       echo "export PATH=\"\${PATH}:${BIN_DIR}\"" >> "${HOME}/.zshrc"
       echo "Added ${BIN_DIR} to PATH in ~/.bashrc and ~/.zshrc" ;;
esac

echo "✓ nr installed to ${EXE_PATH}"
echo "  Run 'nr --help' to get started."
