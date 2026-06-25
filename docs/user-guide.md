# wiki-tui User Guide

## Introduction

wiki-tui is a simple and easy-to-use Wikipedia Text User Interface. Browse Wikipedia articles directly from your terminal with a rich, interactive experience.

## Features

- **Rich Search Results** - Preview articles before opening
- **Layered Navigation** - Open multiple articles in layers (ESC to close)
- **Table of Contents** - Jump to sections quickly
- **Rich Article View** - Functioning links, lists, headers
- **Multi-language Support** - Switch Wikipedia languages on the fly
- **VIM-like Keybindings** - Efficient keyboard navigation
- **Customizable Theme** - Change colors and appearance
- **Logging** - Debug and track your browsing

## Installation

### From Crates.io (Recommended)

```bash
cargo install wiki-tui
```

### From Source

```bash
git clone https://github.com/Builditluc/wiki-tui
cd wiki-tui
cargo build --release

# Binary location
./target/release/wiki-tui
```

### Package Managers

**Arch Linux**:
```bash
yay -S wiki-tui
# or
paru -S wiki-tui
```

**Homebrew** (macOS):
```bash
brew install wiki-tui
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/Builditluc/wiki-tui/releases).

## Quick Start

### Launch wiki-tui

```bash
wiki-tui
```

### Basic Navigation

| Key | Action |
|-----|--------|
| `/` | Search Wikipedia |
| `Enter` | Open selected article |
| `ESC` | Close current layer/go back |
| `j`/`k` | Scroll down/up |
| `g`/`G` | Go to top/bottom |
| `Tab` | Switch between sections |
| `q` | Quit |

### Search for Articles

1. Press `/` to open search
2. Type your query (e.g., "Rust programming")
3. Use `↑`/`↓` to navigate results
4. Press `Enter` to open article

## Configuration

### Config File Location

- **Linux**: `~/.config/wiki-tui/config.toml`
- **macOS**: `~/Library/Application Support/wiki-tui/config.toml`
- **Windows**: `%APPDATA%\wiki-tui\config.toml`

### Basic Configuration

```toml
# ~/.config/wiki-tui/config.toml

# Wikipedia language
language = "en"

# Theme
theme = "default"

# Logging
logging = false
log_file = "~/.cache/wiki-tui/wiki-tui.log"

# UI Options
show_table_of_contents = true
show_links = true
toc_position = "right"
```

### Custom Theme

```toml
[theme]
# Background and foreground colors
background = "black"
foreground = "white"

# Highlight colors
highlight = "blue"
selected = "yellow"

# Article styling
title_color = "cyan"
header_color = "green"
link_color = "blue"
```

### Keybindings

```toml
[keybindings]
# Navigation
scroll_down = "j"
scroll_up = "k"
goto_top = "g"
goto_bottom = "G"

# Actions
search = "/"
open = "enter"
back = "esc"
quit = "q"

# Table of Contents
toggle_toc = "t"
```

## Advanced Usage

### Batch Article Download

```bash
# Download multiple articles for offline reading
wiki-tui --download "Rust" "Python" "Linux" --output ~/wiki-offline/
```

### Custom Search Engines

```toml
# ~/.config/wiki-tui/config.toml
[search]
# Use specific Wikipedia language
language = "en"

# Custom API endpoint (for private wikis)
api_url = "https://your-wiki.com/api.php"

# Search result limit
limit = 20
```

### Scripting Integration

```bash
#!/bin/bash
# Daily random article learning
topics=("Physics" "History" "Philosophy" "Art" "Technology")
random=${topics[$RANDOM % ${#topics[@]}]}
echo "Today's learning topic: $random"
wiki-tui --search "$random"
```

### API Integration

```bash
# Use wiki-tui with other tools
wiki-tui --search "Rust" --export markdown > rust-article.md

# Pipe to other processors
wiki-tui --search "Linux" --raw | grep -i "kernel"
```

### Search Tips

**Specific searches**:
- `"exact phrase"` - Search for exact phrase
- `prefix:*` - Search by prefix
- `category:Science` - Search in category

**Search result navigation**:
- `↑`/`↓` - Navigate results
- `Enter` - Open selected
- `Esc` - Cancel search

### Layer Management

wiki-tui uses a layer system for articles:

- Each opened article creates a new layer
- Press `ESC` to go back to previous layer
- Multiple layers allow deep exploration

**Example workflow**:
1. Search "Rust"
2. Open "Rust (programming language)"
3. Click link to "Mozilla"
4. New layer opens for Mozilla
5. Press `ESC` to return to Rust article

### Article View

```
┌────────────────────────────────────────────────────────────┐
│  < Rust (programming language)               [TOC: t]     │
├──────────────────┬─────────────────────────────────────────┤
│                  │                                         │
│  Contents        │  Rust is a multi-paradigm, general-     │
│  ────────────────│  purpose programming language.          │
│                  │                                         │
│  • History       │  Design                                 │
│  • Features      │  ─────────                              │
│  • Syntax        │  Rust has been used for systems         │
│  • Performance   │  programming, web assembly, and         │
│  • Comparisons   │  embedded devices.                      │
│                  │                                         │
│                  │  Memory Safety                          │
│                  │  ───────────────                        │
│                  │  Rust's ownership system ensures        │
│                  │  memory safety without garbage          │
│                  │  collection.                            │
│                  │                                         │
└──────────────────┴─────────────────────────────────────────┘
```

### Article View

```
┌────────────────────────────────────────────────────────────┐
│  < Rust (programming language)               [TOC: t]     │
├──────────────────┬─────────────────────────────────────────┤
│                  │                                         │
│  Contents        │  Rust is a multi-paradigm, general-     │
│  ────────────────│  purpose programming language.          │
│                  │                                         │
│  • History       │  Design                                 │
│  • Features      │  ─────────                              │
│  • Syntax        │  Rust has been used for systems         │
│  • Performance   │  programming, web assembly, and         │
│  • Comparisons   │  embedded devices.                      │
│                  │                                         │
│                  │  Memory Safety                          │
│                  │  ───────────────                        │
│                  │  Rust's ownership system ensures        │
│                  │  memory safety without garbage          │
│                  │  collection.                            │
│                  │                                         │
└──────────────────┴─────────────────────────────────────────┘
```

### Table of Contents

Access article sections quickly:

- Toggle with `t`
- Navigate with `↑`/`↓`
- Jump to section with `Enter`

**Customize TOC**:
```toml
[table_of_contents]
position = "right"  # or "left"
width = 30
show_section_numbers = true
```

## Tips and Tricks

### Efficient Browsing

1. **Use search wisely** - Be specific to find articles faster
2. **Layer navigation** - Don't be afraid to go deep, ESC brings you back
3. **TOC shortcuts** - Jump to sections instead of scrolling
4. **Language switching** - Compare articles across languages

### Performance

- **Disable features** for faster rendering:
  ```toml
  show_links = false
  show_table_of_contents = false
  ```

- **Reduce log verbosity**:
  ```toml
  logging = false
  ```

### Accessibility

- **High contrast theme**:
  ```toml
  [theme]
  background = "black"
  foreground = "white"
  highlight = "yellow"
  ```

- **Larger text** (if terminal supports):
  ```bash
  # Increase terminal font size
  ```

## Troubleshooting

### Search Not Working

1. Check internet connection
2. Verify Wikipedia API is accessible
3. Try different language setting

### Rendering Issues

```bash
# Clear terminal and restart
reset
wiki-tui
```

**Common fixes**:
- Ensure terminal supports UTF-8
- Try different theme
- Disable TOC if rendering is slow

### Configuration Issues

```bash
# Reset to default config
rm ~/.config/wiki-tui/config.toml
wiki-tui  # Generates new default config
```

### Performance Slow

1. Disable unnecessary features
2. Reduce TOC width
3. Close unused layers
4. Check log file for errors

## Comparison with Alternatives

| Feature | wiki-tui | wiki-cli | wik |
|---------|----------|----------|-----|
| **TUI Interface** | ✅ Full | ❌ CLI only | ✅ Basic |
| **Layer Navigation** | ✅ Yes | ❌ No | ⚠️ Limited |
| **Table of Contents** | ✅ Yes | ❌ No | ❌ No |
| **Multi-language** | ✅ Yes | ✅ Yes | ⚠️ Limited |
| **Customization** | ✅ High | ⚠️ Medium | ❌ Low |
| **Active Development** | ✅ Yes | ⚠️ Slow | ⚠️ Slow |

## Keyboard Shortcuts Reference

### Navigation

| Key | Action |
|-----|--------|
| `j`/`↓` | Scroll down |
| `k`/`↑` | Scroll up |
| `g` | Go to top |
| `G` | Go to bottom |
| `Ctrl+d` | Half page down |
| `Ctrl+u` | Half page up |

### Actions

| Key | Action |
|-----|--------|
| `/` | Search |
| `Enter` | Open/Select |
| `ESC` | Back/Close layer |
| `q` | Quit |
| `?` | Show help (if available) |

### UI Toggles

| Key | Action |
|-----|--------|
| `t` | Toggle TOC |
| `l` | Toggle links |
| `Tab` | Switch focus |

## Integration with Other Tools

### Terminal Multiplexers

**tmux**:
```bash
# Add to .tmux.conf
bind w run-shell "wiki-tui"
```

**screen**:
```bash
# Add to .screenrc
bind w wiki-tui
```

### Shell Aliases

```bash
# ~/.bashrc or ~/.zshrc
alias wiki='wiki-tui'
alias wikien='wiki-tui --lang en'
alias wikide='wiki-tui --lang de'
```

### Scripting

```bash
#!/bin/bash
# Open random article
TOPICS=("Rust" "Python" "Linux" "Physics" "History")
RANDOM_TOPIC=${TOPICS[$RANDOM % ${#TOPICS[@]}]}
echo "Today's topic: $RANDOM_TOPIC"
wiki-tui --search "$RANDOM_TOPIC"
```

## Community

- **Repository**: https://github.com/Builditluc/wiki-tui
- **Issues**: https://github.com/Builditluc/wiki-tui/issues
- **Discussions**: https://github.com/Builditluc/wiki-tui/discussions
- **Website**: https://wiki-tui.net

## Contributing

Contributions welcome! Areas needing help:
- Bug reports and fixes
- Feature requests
- Documentation improvements
- Theme contributions
- Translation support

---

*Community contribution - unofficial user guide*
