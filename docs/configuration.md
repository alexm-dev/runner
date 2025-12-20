# Runner Configuration Guide

runner is under active development and options may change over time.

## Config File

`runner` is configured via a TOML file located at:

`~/.config/runner/runner.toml` (Linux/macOS)

`C:\Users\<UserName>\.config\runner\runner.toml` (Windows)

**Override**: You can specify a custom path by setting the `RUNNER_CONFIG` environment variable.

## Quick Start

If you don't have a config file yet, you can generate one automatically:

- `rn --init`: Generates the full configuration with all default options.
- `rn --init-minimal`: Generates a minimal, clean config file, with some common overrides
- `rn --config-help`: Displays all configuration options.

## General Settings
```toml

# --- General Settings ---

# Sort directories before files
dirs_first = true

# Shows hidden files (dotfiles)
show_hidden = false

# Shows hidden system files (Mostly useful for Windows)
show_system = false

# Ignore case sensitivity when searching or sorting
case_insensitive = true

[display]
# Shows the selection icon next to the file/directory name
selection_marker = true

# Shows the default '/' symbol next to the directory name
dir_marker = true

# Border style for the application: "none", "unified", or "split"
borders = "unified"

# Show pane titles at the top (e.g., "Main", "Preview")
titles = false

# Draw vertical lines between panes
separators = false

# Show the parent directory pane (left)
origin = false

# Show the file preview pane (right)
preview = false

# Pane width ratios (relative to each other)
origin_ratio = 30
main_ratio = 40
preview_ratio = 30

# Scroll paddding of the main pane
scroll_padding = 5

[theme]
# Background color (terminal color name or Hex "#RRGGBB")
background = "default"

# Text color for the selection highlight
selection_fg = "default"

# Color for markers, icons, and highlights
accent_fg = "default"

# Default text color for file/folder entries
entry_fg = "default"

# Color of the vertical split lines
separator_fg = "default"

# Text color for the parent (origin) pane
origin_fg = "default"

# Text color for the preview pane
preview_fg = "default"

# The symbol used to indicate the current selection
selection_icon = "> "

[editor]
# Change the default editor to open files with
cmd = "nvim"

[keys]
# List of keys mapped to specific actions
open_file   = ["Enter"]
go_up       = ["k", "Up Arrow"]
go_down     = ["j", "Down Arrow"]
go_origin   = ["h", "Left Arrow", "Backspace"]
go_into_dir = ["l", "Right Arrow"]
quit        = ["q", "Esc"]
```
