# Keybindings

There are certain actions that you can change the Keybindings for. The configuration for each action is the same.

## Action Configuration

### Key

The key setting can be a simple character or a non-character key
These are the supported non-character keys (lower-/uppercase doesn't matter);

| Key              | Config Name    |
|------------------|----------------|
| ++insert++       | `insert`       |
| ++delete++       | `delete`       |
| ++home++         | `home`         |
| ++end++          | `end`          |
| ++page-up++      | `pageup`       |
| ++page-down++    | `pagedown`     |
| ++break++        | `pausebreak`   |
| ++num-enter++    | `numpadcenter` |
| ++f1++ - ++f12++ | `f1` - `f12`   |

### Mode

The following modes are supported

| Key            | Config Name |
|----------------|-------------|
 |                | `normal`    |
| ++shift++      | `shift`      |
| ++alt++        | `alt`       |
| ++alt+shift++  | `altshift`  |
| ++ctrl++       | `ctrl`      |
| ++ctrl+shift++ | `ctrlshift` |
| ++ctrl+alt++   | `ctrlalt`   |

## Supported Actions

| Action                  | Config Name  | Default Keybinding | Changeable Since                          |
|-------------------------|--------------|--------------------|-------------------------------------------|
| Scroll Down             | `down`       | ++down++           | [:octicons-tag-24: 0.5.0][release-0.5.0]  |
| Scroll Up               | `up`         | ++up++             | [:octicons-tag-24: 0.5.0][release-0.5.0]  |
| Scroll / Select Left    | `left`       | ++left++           | [:octicons-tag-24: 0.5.0][release-0.5.0]  |
| Scroll / Select Right   | `right`      | ++right++          | [:octicons-tag-24: 0.5.0][release-0.5.0]  |
| Focus the next view     | `focus_next` | ++tab++            | :fontawesome-solid-microchip: pre-release |
| Focus the previous view | `focus_prev` | ++shift+tab++      | :fontawesome-solid-microchip: pre-release |

## Sample Remap

```toml
[keybindings]
down.key = "j"
down.mode = "shift"
```

## Vim Bindings Config

This is a simple keybinding configuration that lets you use the vim Keybindings (++h++/++j++/++k++/++l++) to navigate around wiki-tui.

```toml
[keybindings]
down.key = "j"
up.key = "k"
left.key = "h"
right.key = "l"
```

[release-0.5.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.5.0
