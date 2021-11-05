[![Contributors](https://img.shields.io/github/contributors/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/graphs/contributors)
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

## Preview

### Features
`wiki-tui` currently has these features:
- Browse through Wikipedia (Set the language by changing the url in the config)
- Uses webscraping and a custom view to display wikipedia articles in the terminal
- Use links to open other articles


These features are planned:
- View and interact with more elements of wikipedia articles (like tables, images, etc.)
- Better error handling

### How it looks

![image](https://user-images.githubusercontent.com/37375448/139769364-46a69dce-f386-4369-a82e-4a45adac3b52.png)
![image](https://user-images.githubusercontent.com/37375448/139769469-0b2e9f01-f758-4bb2-8227-4186f658cfcc.png) <br>
Note: These Screenshots were taken on [iTerm2](https://iterm2.com) (MacOS) with the [Fira Code](https://github.com/tonsky/FiraCode) font

### Usage

If you want to search for an article quickly, you can pass the search term as an argument.
```
wiki-tui <ARTICLE>
```
wiki-tui then searches for this article automatically. 

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
```

## Contributing

Any contributions you make are greatly appreciated.

1. Fork the Project
2. Switch to the experimental branch
3. Commit your Changes
4. Push to the branch
5. Open a Pull Request

## Acknowledgements

* [cursive](https://github.com/gyscos/cursive)
* Some parts of [Best-README-Template](https://github.com/0fakhri/Best-README-Template)
