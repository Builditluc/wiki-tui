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

| Action                  | Config Name  | Default Keybinding |
|-------------------------|--------------|--------------------|
| Scroll Down             | `down`       | ++down++           |
| Scroll Up               | `up`         | ++up++             |
| Scroll / Select Left    | `left`       | ++left++           |
| Scroll / Select Right   | `right`      | ++right++          |
| Focus the next view     | `focus_next` | ++tab++            |
| Focus the previous view | `focus_prev` | ++shift+tab++      |
| Go to Top               | N/a          | ++gg++             |
| Go to Bottom            | N/a          | ++G++              |
| Half page down          | N/a          | ++ctrl+d++         |
| Half page up            | N/a          | ++ctrl+u++         |
| Focus the previous view | `focus_prev` | ++shift+tab++      |
| Toggle the language selection | `toggle_language_selection` | ++f2++ |

> When updating the language via the selection popup, existing search results and links in articles
> won't work until you've changed the language back to what it was then opening the article /
> starting the search

## Sample Remap

```toml
[keybindings]
down.key = "j"
down.mode = "shift"
```

[release-0.5.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.5.0
[release-0.6.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.6.0
