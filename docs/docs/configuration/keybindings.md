# Keybindings

There are certain actions that you can change the Keybindings for. The configuration for each action is the same.

## Keybinding Configuration

!!! tip "Multiple Bindings per Action"
    You can also define multiple bindings for one action by putting them into an array (`action = [bindning1, binding2, etc.]`).

A bindig can be either a keycode or a keycode combined with one or more modifiers for that key. All
of the following are a valid way of configuring a binding (where `action` is one of the configurable
actions)

```toml
action = "esc"
```
> A keycode without any modifiers

```toml
action = { code = "l", modifiers = "CONTROL | SHIFT" } # or with just a single modifier
action = { code = "l", modifiers = "CONTROL" }
```
> A keycode with modifiers

These can be mixed together when defining multiple bindings for one action

```toml
action = [
    { code = "l", modifiers = "CONTROL" },
    "esc",
]
```

### Keycodes

A keycode can be a simple character or a non-character key
These are the supported non-character keys (lower-/uppercase doesn't matter);

| Key            | Config Name |
|----------------|-------------|
| ++backspace++  | backspace   |
| ++enter++      | enter       |
| ++left++       | left        |
| ++right++      | right       |
| ++up++         | up          |
| ++down++       | down        |
| ++home++       | home        |
| ++end++        | end         |
| ++page-up++    | pageup      |
| ++page-down++  | pagedown    |
| ++tab++        | tab         |
| ++backtab++    | backtab     |
| ++delete++     | delete      |
| ++insert++     | insert      |
| ++esc++        | esc         |
| ++f1++-++f12++ | f1-f12      |

### Modifiers

The following modifiers are available. You can also combine them using `|` as a separator. Please
note that these modifiers are case-sensitive

| Key            | Config Name |
|----------------|-------------|
| ++shift++      | `SHIFT`     |
| ++ctrl++       | `CONTROL`   |
| ++alt++        | `ALT`       |

## Default Keybindings

Below are the default keybindings for all of the configurable actions

### Global Keybindings

| Action                             | Description                                          | Default Binding            |
|------------------------------------|------------------------------------------------------|----------------------------|
| `scroll_down`                      | Scroll down                                          | ++j++ / ++down++           |
| `scroll_up`                        | Scroll down                                          | ++k++ / ++up++             |
| `scroll_to_top`                    | Scroll to the top                                    | ++'g'++ / ++home++         |
| `scroll_to_bottom`                 | Scroll to the bottom                                 | ++'G'++ / ++end++          |
| `pop_popup`                        | Remove the displayed popup                           | ++esc++                    |
| `half_down`                        | Scroll half a page down                              | ++ctrl+d++ / ++page-down++ |
| `half-up`                          | Scroll half a page up                                | ++ctrl+u++ / ++page-up++   |
| `unselect_scroll`                  | Unselect the current selection                       | ++h++                      |
| `submit`                           | Submit the selected form or open the selection       | ++enter++                  |
| `quit`                             | Quit the program                                     | ++q++                      |
| `enter_search_bar`                 | Focus the searchbar                                  | ++i++                      |
| `exit_search_bar`                  | Defocus the searchbar (return to the previous focus) | ++esc++                    |
| `switch_context_search`            | Switch to the search pane                            | ++s++                      |
| `switch_context_page`              | Switch to the page pane                              | ++p++                      |
| `toggle_search_language_selection` | Toggle the search language selection popup           | ++f2++                     |
| `toggle_logger`                    | Toggle the logger view                               | ++l++                      |

The default configuration file for the global keybindings

```toml
[bindings.global]
scroll_down = "j"
scroll_up = "k"

scroll_to_top = [ "g", "home" ]
scroll_to_bottom = [
    { code = "G", modifiers = "SHIFT" }, 
    "end",
]

pop_popup = "esc"

half_down = [
    { code = "d", modifiers = "CONTROL" },
    "pagedown",
]
half_up = [
    { code = "u", modifiers = "CONTROL" },
    "pageup",
]

unselect_scroll = "h"

submit = "enter"
quit = "q"

enter_search_bar = "i"
exit_search_bar = "esc"

switch_context_search = "s"
switch_context_page = "p"

toggle_search_language_selection = "f2"
toggle_logger = "l"
```

### Search Keybindings

| Action            | Description         | Default Binding |
|-------------------|---------------------|-----------------|
| `continue_search` | Continue the search | ++c++           | 

The default configuration for the search keybindings

```toml
[bindings.search]
continue_search = "c"
```

### Page Keybindings

| Action                           | Description                                                      | Default Binding       |
|----------------------------------|------------------------------------------------------------------|-----------------------|
| `pop_page`                       | Remove the current page from the stack                           | ++esc++               |
| `jump_to_header`                 | Jump to the selected header in the toc (only if toc is focussed) | ++enter++             |
| `select_first_link`              | Select the first link in the page                                | ++shift+left++        |
| `select_last_link`               | Select the last link in the page                                 | ++shift+right++       |
| `select_prev_link`               | Select the previous link in the page                             | ++left++              |
| `select_next_link`               | Select the next link in the page                                 | ++right++             |
| `open_link`                      | Open the currently selected link                                 | ++enter++             |
| `toggle_page_language_selection` | Toggle the popup for changing the page language                  | ++f3++                |
| `toggle_zen_mode`                | Toggle the zen-mode for the page                                 | ++f4++                |
| `toggle_toc`                     | Switch the focus to the toc (or page)                            | ++tab++ / ++backtab++ |

The default configuration for the page keybindings

```toml
[bindings.page]
pop_page = "esc"
jump_to_header = "enter"

select_first_link = { code = "left", modifiers = "SHIFT" }
select_last_link = { code = "right", modifiers = "SHIFT" }

select_prev_link = "left"
select_next_link = "right"

open_link = "enter"

toggle_page_language_selection = "f3"
toggle_zen_mode = "f4"
toggle_toc = [ "tab", "backtab" ]
```
