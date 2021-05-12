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
<ul>
  <li>Search through Wikipedia (currently english only!) and read articles in the terminal</li>
  <li>Load more search results</li>
  <li>Preview the search results with highlighting of the search query</li>
</ul>

### How it looks

![image](https://user-images.githubusercontent.com/37375448/116461454-8ec0d780-a868-11eb-8725-a503bce4828c.png)
![image](https://user-images.githubusercontent.com/37375448/116461510-a0a27a80-a868-11eb-950b-f804ffa4ad3b.png)
![image](https://user-images.githubusercontent.com/37375448/116461593-bb74ef00-a868-11eb-9280-cf64eaa4e11f.png) <br>
Note: These Screenshots were taken on iTerm2 (MacOS) with the [spaceduck](https://github.com/pineapplegiant/spaceduck-terminal) theme and the SF Mono Font

## Installation

Currently, you can install `wiki-tui` only by compiling it manually.
Just clone the repository and compile the stable branch.
## Configuration

### Location of the config file
#### MacOS
```
$HOME/Library/Application Support/wiki-tui/config.ini
```
#### Linux
```
$HOME/.config/wiki-tui/config.ini
```
#### Windows
```
C:\Users\{USERNAME}\wiki-tui\config.ini
```

### Settings
#### Api
```ini
; this is the url of the Api, wiki-tui uses to interact with Wikipedia
BASE_URL = "https://en.wikipedia.org/w/api.php"
```
#### Theme
The settings here are all colors and can be set by either the name of the color or a hex string (valid formats are: `#ffffff`, `#fff`). If your color wasn't applied, check the logs to find out why.
```ini
; color used for View backgrounds
background = white
; color used for the title text
title = red
; color used for highlighting text
highlight = red
; color used for highlighting inactive text
highlight_inactive = blue
; color used for highlighted text
highlight_text = white
; color used for the text
text = black
; color used for a search match in the results view
search_match = red
```
