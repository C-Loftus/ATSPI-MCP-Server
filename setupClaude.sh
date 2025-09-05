#!/bin/bash
set -e

# Claude must be restarted to reload the mcp server
# and it appears the process is just called electron
killall -SIGINT electron || true

# Get the current directory dynamically
PROJECT_DIR="$(pwd)"

# Build the project in release mode
echo "Building project in $PROJECT_DIR..."
cargo build --release

# Path to the release binary
BINARY_PATH="$PROJECT_DIR/target/release/atspi_mcp"

# Create the JSON content
CONFIG_JSON=$(cat <<EOF
{
  "mcpServers": {
    "Demo": {
      "command": "$BINARY_PATH",
      "args": []
    }
  }
}
EOF
)

# Destination config file
CONFIG_DIR="$HOME/.config/Claude"
CONFIG_FILE="$CONFIG_DIR/claude_desktop_config.json"

# Ensure the destination directory exists
mkdir -p "$CONFIG_DIR"

# Write the JSON to the config file
echo "$CONFIG_JSON" > "$CONFIG_FILE"

echo "Config written to $CONFIG_FILE"

claude-desktop