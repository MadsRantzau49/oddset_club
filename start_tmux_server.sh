#!/bin/bash

# Define session name and commands
SESSION_NAME="oddset"
SERVER_CMD="cargo run"
TUNNEL_CMD="cloudflared tunnel --no-autoupdate run oddsetklubben"

# Kill existing tmux session if it exists
if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
    echo "Killing existing tmux session: $SESSION_NAME"
    tmux kill-session -t "$SESSION_NAME"
fi

# Create new tmux session and run the server in the first pane
tmux new-session -d -s "$SESSION_NAME" "$SERVER_CMD"

# Split the first window vertically and run the tunnel
tmux split-window -v -t "$SESSION_NAME" "$TUNNEL_CMD"

# Optional: adjust pane sizes (first pane bigger)
tmux select-pane -t 0
tmux resize-pane -D 10  # increase server pane height

# Attach to the session
tmux attach -t "$SESSION_NAME"

