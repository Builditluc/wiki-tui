<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-4-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
[![Stargazers](https://img.shields.io/github/stars/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/stargazers)
[![Issues](https://img.shields.io/github/issues/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/issues)
[![MIT license](https://img.shields.io/github/license/Builditluc/wiki-tui?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/blob/stable/LICENSE.txt)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Builditluc/wiki-tui/Rust?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/Builditluc/wiki-tui?style=for-the-badge)

<br />
<p align="center">
  <a href="https://github.com/Builditluc/wiki-tui">
    <img src= "logo.png" alt="Logo" width="234" height="167">
  </a>

  <h3 align="center">WIKI-TUI</h3>

  <p align="center">
    A simple and easy to use Wikipedia Text User Interface
  </p>
</p>

> Note: wiki-tui is still under active development and breaking changes can occur. Please always check the release notes before upgrading

## Preview
### How it looks

![image](https://user-images.githubusercontent.com/37375448/139769364-46a69dce-f386-4369-a82e-4a45adac3b52.png)
![image](https://user-images.githubusercontent.com/37375448/139769469-0b2e9f01-f758-4bb2-8227-4186f658cfcc.png) <br>
> Note: These Screenshots were taken on [iTerm2](https://iterm2.com) (MacOS) with the [Fira Code](https://github.com/tonsky/FiraCode) font

### Usage

Please refer to `wiki-tui --help` for more information on cli arguments.
> Note: when searching for an article or opening one, please wait while the program fetches the results

#### Controls
<table>
  <tr><th>Key</th><th>Function</th></tr>
  <tr><td>q</td><td>Quit the program (except when inside the search bar)</td></tr>
  <tr><td>Up and Down</td><td>Scrolling</td></tr>
  <tr><td>Left and Right</td><td>Select the next link (only in the article view)</td></tr>
</table>

## Installation
The binary executable is `wiki-tui`

### Cargo
```
cargo install wiki-tui
```

When installing wiki-tui via cargo, you can choose what backend to use (The default one is crossterm). More information on the
available backends can be found [here](https://github.com/gyscos/cursive/wiki/Backends#available-backends)

To use a different backend, you will have to disable the default features and enable the desired backend feature.
```
cargo install wiki-tui --no-default-features --features termion-backend
```

### NetBSD
Using the package manager
```
pkgin install wiki-tui
```
Building from source
```
cd /usr/pkgsrc/www/wiki-tui
make install
```

### Nix
The wiki-tui package is in the [nixpkgs](https://search.nixos.org/packages?channel=unstable&show=wiki-tui&from=0&size=50&sort=relevance&type=packages&query=wiki-tui) package repository.

Either install it with `nix-env -iA wiki-tui`, get it temporarily with `nix-shell -p wiki-tui` or add it to your configuration.

### AUR
The wiki-tui package is available in the [AUR](https://aur.archlinux.org/packages/wiki-tui).

Either install it with `makepkg` manually, or using the preferred AUR helper.

## Configuration

### Location of the config file
#### MacOS, Linux and NetBSD
```
$HOME/.config/wiki-tui/config.toml
```
#### Windows
```
C:\Users\{USERNAME}\wiki-tui\config.toml
```

### Settings
Default configuration
```toml
[api]
base_url = "https://en.wikipedia.org/"  # this is the url of wikipedia, it can be changed to change the language of wikipedia 

# The settings here are all colors and can be set by either the name of the color or a hex string (valid formats are: #ffffff, #fff)
# The actual colors displayed in your terminal can change depending on your terminal settings

[theme]
background = "white"                    # color used for View backgrounds
title = "red"                           # color used for the title text
highlight = "red"                       # color used for highlighting text
highlight_inactive = "blue"             # color used for highlighting inactive text
highlight_text = "white"                # color used for highlighted text
text = "black"                          # color used for the text
search_match = "red"                    # color used for a search match in the results view

# You can also change the theme of specific views
# These settings are the same for every supported view
[theme.search_bar]
background = "white"
title = "red"
highlight = "red"
highlight_inactive = "blue"
highlight_text = "white"
text = "black"

# Currently, these views are supported:
# - search_bar
# - search_results
# - search_preview
# - article_view
# - toc_view

[logging]
enabled = true                          # can be either true or false. enables/disables logging
log_dir = "wiki_tui.log"                # location of the file where the log will be written to
log_level = "Info"                      # log level to be used, can be Debug, Info, Warn, Error

# wiki-tui allows you to disable certain features like links completely.
# All features are enabled by default
[features]
links = true                            # enables/disables links (link selection, link opening, etc)
toc = true                          # enables/disables the table of contents

# You can change the keybindings of certain actions
# These are the default values
[keybindings]
down.key = ""
# Here you can define the key, it can be a simple character or a non-character key
# Supported non-character Keys (lower-/uppercase do not matter):
# - insert
# - delete
# - home
# - end
# - pageup
# - pagedown
# - pausebreak
# - numpadcenter
# - f0 - f12

down.mode = "normal"
# Here you can change the mode of the keybinding. The standard mode is normal and doesn't need to be set.
# This value is optional and the following modes are available:
# Characters:
# - normal
# - ctrl
# Non-Character Keys:
# - normal
# - shift
# - alt
# - altshift
# - ctrl
# - ctrlshift
# - ctrlalt


# Currently, these actions can be changed with a keybinding:
# - down
# - up
# - left
# - right

# Note: the default keys (Up, Down, Left, Right) will still work even after changing the keybinding.

# [PRE-RELEASE] These options haven't been released yet
# You can change different settings here
[settings.toc]
# Here you can change the position of the toc view. Available options are "left" and "right" (default).
position = "right"
# You can change the title of the table of contents, the available options are
# - default (uses the toc title given by the article)
# - article (uses the title of the article you are viewing)
# - custom  (uses the title configured in the option "title_custom", if "title_custom" is empty, it will display "NONE")
title = "default"
title_custom = "My Custom Title"
# You can also change the min and max width of the toc view. The defaults are 20 for the min width and 60 for the max width
min_width = 20
max_width = 60

# By default, there are horizontal and vertical scrollbars that appear when there isn't enough space for the toc. 
#If you don't want these scrollbars, you can disable them.
scroll_x = true
scroll_y = true
# You can also change how the items are generated. Available values are
# - {NUMBER} : current number of the header (1, 1.1, 1.2, 2, ...)
# - {TEXT}   : text of the header
item_format = "{NUMBER} {TEXT}"
```

## Contributing
See [contributing guidelines](/CONTRIBUTING.md) for contributing conventions.

## Similar Projects

* [hexrcs/wiki-cli](https://github.com/hexrcs/wiki-cli)
* [yashinghcodes/wik](https://github.com/yashsinghcodes/wik)

## Acknowledgements

* [cursive](https://github.com/gyscos/cursive)
* [Best-README-Template](https://github.com/0fakhri/Best-README-Template)

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/Builditluc"><img src="https://avatars.githubusercontent.com/u/37375448?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Builditluc</b></sub></a><br /><a href="#ideas-Builditluc" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/Builditluc/wiki-tui/commits?author=Builditluc" title="Code">💻</a> <a href="https://github.com/Builditluc/wiki-tui/commits?author=Builditluc" title="Documentation">📖</a> <a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3ABuilditluc" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/0323pin"><img src="https://avatars.githubusercontent.com/u/90570748?v=4?s=100" width="100px;" alt=""/><br /><sub><b>0323pin</b></sub></a><br /><a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3A0323pin" title="Bug reports">🐛</a> <a href="#platform-0323pin" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://github.com/legendofmiracles"><img src="https://avatars.githubusercontent.com/u/30902201?v=4?s=100" width="100px;" alt=""/><br /><sub><b>legendofmiracles</b></sub></a><br /><a href="#platform-legendofmiracles" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://github.com/ThomasFrans"><img src="https://avatars.githubusercontent.com/u/48214567?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Thomas</b></sub></a><br /><a href="#platform-ThomasFrans" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3AThomasFrans" title="Bug reports">🐛</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
