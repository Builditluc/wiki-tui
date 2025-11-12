[![Contributors](https://img.shields.io/github/all-contributors/builditluc/wiki-tui/main?color=orange&label=Contributors&style=for-the-badge)](#contributors-)
[![Stargazers](https://img.shields.io/github/stars/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/stargazers)
[![Issues](https://img.shields.io/github/issues/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/issues)
[![MIT license](https://img.shields.io/github/license/Builditluc/wiki-tui?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/blob/stable/LICENSE.txt)
[![Continuous Integration](https://img.shields.io/github/actions/workflow/status/builditluc/wiki-tui/ci.yml?label=Continuous%20Integration&style=for-the-badge)](https://github.com/Builditluc/wiki-tui/actions/workflows/ci.yml)
[![Continuous Deployment](https://img.shields.io/github/actions/workflow/status/builditluc/wiki-tui/cd.yml?label=Continuous%20Deployment&style=for-the-badge)](https://github.com/Builditluc/wiki-tui/actions/workflows/cd.yml)

<br />
<p align="center">
  <a href="https://github.com/Builditluc/wiki-tui">
    <img src="https://raw.githubusercontent.com/Builditluc/wiki-tui/main/icons/hicolor/scalable/apps/wiki-tui.svg" width="50%" alt="Logo; wiki-tui entered into a command line prompt"/>
  </a>

  <h3 align="center">WIKI-TUI</h3>

  <p align="center">
    A simple and easy to use Wikipedia Text User Interface
  </p>
</p>

> Note: wiki-tui is still under active development and breaking changes can occur. Please always check the release notes before upgrading

<br>

## Installation

### Cargo
You can install this forked version with:

```sh
cargo install wiki-tui
```

# What can it do?

- Browse wikipedia in a TUI. Features of the TUI include:

    - Rich search results (result previews, more information on results,
      continue the current search)
    - Open articles in layers (press ESC to close a layer)
    - A Table of Contents with support for jumping to the section in the
      article
    - Rich article view (functioning links, lists, headers)
    - Switch the language of an article

- VIM-like Keybindings:
    
    - Basic movement
    - Goto Top / Bottom
    - Half up / down

- It's customizable (we plan on having more features in the future):

    - Change the global and local theme of individual UI components
    - Change some of the keybindings
    - Disable features (Table of Contents, Links)
    - Customize the Table of Contents (position, title format, sizes, etc.)
    - Change wikipedias language (even on the fly)
    - Logging

## How it looks

[![preview-3](https://raw.githubusercontent.com/Builditluc/wiki-tui/main/docs/docs/assets/images/preview-3.png)](docs/docs/assets/images/preview-3.png)

## Similar Projects

* [hexrcs/wiki-cli](https://github.com/hexrcs/wiki-cli)
* [yashinghcodes/wik](https://github.com/yashsinghcodes/wik)

## Acknowledgements

* [ratatui](https://ratatui.rs/)
* [rust](https://www.rust-lang.org/)
* [mkdocs](https://www.mkdocs.org/)
* [mkdocs-material](https://github.com/squidfunk/mkdocs-material)

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Builditluc"><img src="https://avatars.githubusercontent.com/u/37375448?v=4?s=100" width="100px;" alt="Builditluc"/><br /><sub><b>Builditluc</b></sub></a><br /><a href="#ideas-Builditluc" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/Builditluc/wiki-tui/commits?author=Builditluc" title="Code">💻</a> <a href="https://github.com/Builditluc/wiki-tui/commits?author=Builditluc" title="Documentation">📖</a> <a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3ABuilditluc" title="Bug reports">🐛</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/0323pin"><img src="https://avatars.githubusercontent.com/u/90570748?v=4?s=100" width="100px;" alt="0323pin"/><br /><sub><b>0323pin</b></sub></a><br /><a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3A0323pin" title="Bug reports">🐛</a> <a href="#platform-0323pin" title="Packaging/porting to new platform">📦</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/legendofmiracles"><img src="https://avatars.githubusercontent.com/u/30902201?v=4?s=100" width="100px;" alt="legendofmiracles"/><br /><sub><b>legendofmiracles</b></sub></a><br /><a href="#platform-legendofmiracles" title="Packaging/porting to new platform">📦</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ThomasFrans"><img src="https://avatars.githubusercontent.com/u/48214567?v=4?s=100" width="100px;" alt="Thomas"/><br /><sub><b>Thomas</b></sub></a><br /><a href="#platform-ThomasFrans" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3AThomasFrans" title="Bug reports">🐛</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/nunotexbsd"><img src="https://avatars.githubusercontent.com/u/43720668?v=4?s=100" width="100px;" alt="Nuno Teixeira"/><br /><sub><b>Nuno Teixeira</b></sub></a><br /><a href="#platform-nunotexbsd" title="Packaging/porting to new platform">📦</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://juans.dev"><img src="https://avatars.githubusercontent.com/u/47149574?v=4?s=100" width="100px;" alt="cshjsc"/><br /><sub><b>cshjsc</b></sub></a><br /><a href="https://github.com/Builditluc/wiki-tui/commits?author=cshjsc" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/non-descriptive"><img src="https://avatars.githubusercontent.com/u/5757533?v=4?s=100" width="100px;" alt="Dmitry Kozlovtsev"/><br /><sub><b>Dmitry Kozlovtsev</b></sub></a><br /><a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3Anon-descriptive" title="Bug reports">🐛</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Enoumy"><img src="https://avatars.githubusercontent.com/u/45022810?v=4?s=100" width="100px;" alt="Enoumy"/><br /><sub><b>Enoumy</b></sub></a><br /><a href="https://github.com/Builditluc/wiki-tui/commits?author=Enoumy" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ethamck"><img src="https://avatars.githubusercontent.com/u/111709736?v=4?s=100" width="100px;" alt="ethamck"/><br /><sub><b>ethamck</b></sub></a><br /><a href="#design-ethamck" title="Design">🎨</a> <a href="#platform-ethamck" title="Packaging/porting to new platform">📦</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://pwned.place"><img src="https://avatars.githubusercontent.com/u/52998857?v=4?s=100" width="100px;" alt="lstuma"/><br /><sub><b>lstuma</b></sub></a><br /><a href="https://github.com/Builditluc/wiki-tui/issues?q=author%3Alstuma" title="Bug reports">🐛</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
