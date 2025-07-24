# Configuration Guide

WIMM supports extensive customization through its configuration system. Configuration is stored in a TOML file and can be managed through the CLI or by editing the file directly.

## Configuration File Location

The configuration file is stored in the standard platform-specific configuration directory:

- **Linux**: `~/.config/io.wimm.wimm/config.toml`
- **macOS**: `~/Library/Application Support/io.wimm.wimm/config.toml`
- **Windows**: `%APPDATA%\io.wimm.wimm\config.toml`

You can find the exact path on your system with:

```bash
wimm config path
```

## CLI Configuration Commands

### Show Current Configuration

```bash
wimm config show
```

### List Available Options

```bash
# List color schemes
wimm config list-colors

# List keymaps
wimm config list-keymaps
```

### Set Configuration Values

```bash
# Set color scheme (key-value format)
wimm config set color-scheme dark

# Set keymap (key-value format)
wimm config set keymap vi

# Set default defer hour (key-value format)
wimm config set defer-hour 8

# Set default due hour (key-value format)
wimm config set due-hour 18

# Set timezone (key-value format)
wimm config set timezone "America/New_York"

# Or use flag-style format
wimm config set --color-scheme dark --defer-hour 8 --due-hour 18

# Mix both formats
wimm config set timezone UTC --color-scheme light
```

### Reset to Defaults

```bash
wimm config reset
```

### Edit Configuration File

```bash
wimm config edit
```

## Bulk Configuration Changes

You can set multiple configuration values at once using the flag-style format:

```bash
# Set multiple values at once
wimm config set --color-scheme dark --defer-hour 8 --due-hour 18 --keymap vi

# Mix flag-style and key-value formats
wimm config set timezone UTC --color-scheme light
```

## Configuration Structure

The configuration file uses TOML format and has the following sections:

### Colors Section

Defines the active color scheme:

```toml
[colors]
name = "dark"
fg = "#e0e0e0"
bg = "#1a1a1a"
accent = "#4a90e2"
completed = "#666666"
overdue = "#d32f2f"
deferred = "#ffa726"
border = "#333333"
help = "#b0b0b0"
```

### Keymap Section

Defines the active keymap:

```toml
[keymap]
name = "default"

[keymap.normal]
q = "quit"
"?" = "help"
n = "new_task"
e = "edit_task"
# ... more keybindings

[keymap.insert]
Esc = "escape"
Enter = "confirm"
Tab = "next_field"
# ... more keybindings
```

### Time Defaults Section

Defines default times for task scheduling:

```toml
[time]
defer_hour = 9    # 9 AM for defer dates
due_hour = 17     # 5 PM for due dates
timezone = "America/New_York"  # Optional timezone
```

## Built-in Color Schemes

### Default

A high-contrast black and white theme:

- Background: Black (`#000000`)
- Foreground: White (`#ffffff`)
- Accent: Bright green (`#00ff00`)

### Dark

A modern dark theme with blue accents:

- Background: Dark gray (`#1a1a1a`)
- Foreground: Light gray (`#e0e0e0`)
- Accent: Blue (`#4a90e2`)

### Light

A clean light theme:

- Background: White (`#ffffff`)
- Foreground: Dark gray (`#333333`)
- Accent: Blue (`#1976d2`)

## Built-in Keymaps

### Default

Standard keybindings inspired by common TUI applications:

**Normal Mode:**

- `q` - Quit
- `?` - Help
- `n` - New task
- `e` - Edit task
- `d` - Delete task
- `c` - Complete/uncomplete task
- `j/k` - Move down/up
- `Enter` - Edit selected task

Available as flag options:

```bash
wimm config set --color-scheme default --keymap default
```

**Insert Mode:**

- `Esc` - Return to normal mode
- `Enter` - Confirm input
- `Tab` - Next field
- `Shift+Tab` - Previous field

### Vi

Vim-inspired keybindings:

**Normal Mode:**

- `q` - Quit
- `?` - Help
- `i` - New task (insert)
- `e` - Edit task
- `dd` - Delete task
- `x` - Complete/uncomplete task
- `j/k` - Move down/up
- `gg` - Go to top
- `G` - Go to bottom

Available as flag options:

```bash
wimm config set --keymap vi
```

**Insert Mode:**

- `Esc` or `Ctrl+[` - Return to normal mode
- `Enter` - Confirm input
- `Tab` - Next field
- `Shift+Tab` - Previous field

## Creating Custom Color Schemes

You can add custom color schemes by adding them to the `[[color_schemes]]` array:

```toml
[[color_schemes]]
name = "my-theme"
fg = "#f0f0f0"
bg = "#2d2d2d"
accent = "#ff6b6b"
completed = "#999999"
overdue = "#ff4757"
deferred = "#ffa502"
border = "#555555"
help = "#cccccc"
```

Then activate it:

```bash
# Key-value format
wimm config set color-scheme my-theme

# Or flag format
wimm config set --color-scheme my-theme
```

## Creating Custom Keymaps

You can add custom keymaps by adding them to the `[[keymaps]]` array:

```toml
[[keymaps]]
name = "my-keys"

[keymaps.normal]
# Your custom normal mode bindings
"Ctrl+q" = "quit"
"Ctrl+n" = "new_task"
# ... more bindings

[keymaps.insert]
# Your custom insert mode bindings
"Ctrl+c" = "escape"
# ... more bindings
```

## Available Actions

### Normal Mode Actions

- `quit` - Exit the application
- `help` - Toggle help panel
- `new_task` - Create a new task
- `edit_task` - Edit the selected task
- `delete_task` - Delete the selected task
- `complete_task` - Toggle task completion status
- `move_up` - Move selection up
- `move_down` - Move selection down
- `move_top` - Move to first task
- `move_bottom` - Move to last task
- `escape` - Cancel current operation

### Insert Mode Actions

- `escape` - Return to normal mode
- `confirm` - Confirm input and save
- `next_field` - Move to next input field
- `prev_field` - Move to previous input field

## Time Configuration

### Default Hours

- `defer_hour`: Default hour (0-23) for defer dates when only a date is specified
- `due_hour`: Default hour (0-23) for due dates when only a date is specified

These are used when you enter dates like "tomorrow" or "friday" without specifying a time.

### Timezone

The `timezone` setting accepts:

- IANA timezone names (e.g., "America/New_York", "Europe/London", "Asia/Tokyo")
- `null` or omitted for system timezone
- "system" for explicit system timezone

## Examples

### Minimal Configuration

```toml
[time]
defer_hour = 8
due_hour = 18
```

### Developer-Friendly Configuration

```toml
[colors]
name = "dark"

[keymap]
name = "vi"

[time]
defer_hour = 9
due_hour = 17
timezone = "UTC"
```

Set up quickly with:

```bash
wimm config set --color-scheme dark --keymap vi --defer-hour 9 --due-hour 17
wimm config set timezone UTC
```

### Night Owl Configuration

```toml
[colors]
name = "dark"

[time]
defer_hour = 10  # Start later
due_hour = 22    # Work late
```
