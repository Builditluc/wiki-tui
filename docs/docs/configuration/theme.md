# Theme Settings

The configuration file of the theme can be found in `WIKI_TUI_CONFIG/theme.toml`

Usually it's this path:

```
$HOME/.config/wiki-tui/theme.toml
```

!!! warning
    The actual colors displayed in your terminal can change depending on your terminal settings

## About Colors

All colors from the [ANSI color table](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) are 
supported (though some names are not exactly the same).

| Color Name     | Foreground | Background |
|----------------|------------|------------|
| `black`        | 30         | 40         |
| `red`          | 31         | 41         |
| `green`        | 32         | 42         |
| `yellow`       | 33         | 43         |
| `blue`         | 34         | 44         |
| `magenta`      | 35         | 45         |
| `cyan`         | 36         | 46         |
| `gray`         | 37         | 47         |
| `darkgray`     | 90         | 100        |
| `lightred`     | 91         | 101        |
| `lightgreen`   | 92         | 102        |
| `lightyellow`  | 93         | 103        |
| `lightblue`    | 94         | 104        |
| `lightmagenta` | 95         | 105        |
| `lightcyan`    | 96         | 106        |
| `white`        | 97         | 107        |

### RGB Color

We also support RGB colors, but note that only terminals that support 24-bit true color will 
display this correctly. Notably versions of Windows Terminal prior to Windows 10 and macOS 
Terminal.app do not support this.

### 8-Bit 256 Indexed

Indexed 8-bit colors are also supported, see
[https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit)
for more information about them.

??? note "Further Information about Color parsing"
    This section of the documentation was taken mostly from the [ratatui::style::Color](https://docs.rs/ratatui/0.26.3/ratatui/style/enum.Color.html#)
    documentation. If you want to see exactly how the colors are being parsed, you can check
    ratatui's implementation [here](https://docs.rs/ratatui/0.26.3/src/ratatui/style/color.rs.html#278-334)

## Theme

### Background
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `reset`

The color used for the view backgrounds
```toml
bg = "reset"
```

### Foreground
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `reset`

The color used for the foreground, mainly the color of the text
```toml
fg = "reset"
```

### Title
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `white`

The color is used for the title of the borders
```toml
title = "white"
```

### Search Title
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `red`

The color for the titles in the search results
```toml
search_title_fg = "red"
```

### Selected Items
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `darkgray` | `reset`

The colors used for the foreground and background of selected items (eg. in lists)
```toml
selected_bg = "darkgray"
selected_fg = "reset"
```

### Inactive
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `blue`

The color is used for inactive items (eg. inactive text)
```toml
inactive_fg = "blue"
```

### Highlight
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `white`

The color is used for the highlighted items (eg. text)
```toml
highlight_fg = "white"
```

### Scrollbar
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `black` | `blue`

The colors used for the thumb and track of scrollbars
```toml
scrollbar_track_fg = "black"
scrollbar_thumb_fg = "blue"
```

## Configure the borders

### Colors
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `white` | `reset`

The colors used for the foreground and background of the borders
```toml
border_fg = "white"
border_bg = "reset"
```

### Border Type
[:octicons-tag-24: 0.9.0][release-0.9.0] :octicons-milestone-16: Default: `Rounded`

!!! warning
    Depending on your font, some border styles may differ from the examples below

You can change the look of the borders in wiki-tui. Available styles are:

* `Plain`
* `Rounded`
* `Double`
* `Thick`
* `QuadrantInside`
* `QuadrantOutside`

```toml
border_type = "Rounded"
```

> More information about the characters for the borders can be found on ratatui's documentation on
> the `BorderType` [here](https://docs.rs/ratatui/0.26.3/ratatui/widgets/block/enum.BorderType.html)

[release-0.9.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.9
