# CHANGELOG

## [v0.4.4](https://github.com/Builditluc/wiki-tui/releases/tag/v0.4.4) - 2022-01-11 12:19:39

*No description*

### Feature

- *:
  - add cli support ([8ed14a2](https://github.com/Builditluc/wiki-tui/commit/8ed14a21c0ec5fd56434aaebcf3d6d35331df17e))
  - add cli 'level' argument
- parse cli arguments in the config
- override log level ([4964e0f](https://github.com/Builditluc/wiki-tui/commit/4964e0fc63d9c67f34fe7974e3b59b47573981a6))
  - add cli support
- add structopt crate for parsing cli arguments
- add cli struct
- implement search_query argument
- implement article_id argument ([61f6923](https://github.com/Builditluc/wiki-tui/commit/61f6923956f30ffcb756f8b15cc83403eaab0915))

### Bug Fixes

- ui:
  - fix horizontal link focus ([1b1e60d](https://github.com/Builditluc/wiki-tui/commit/1b1e60d1cd0431184ae2f3f1381cce1a174f91dc))
  - fix article view not taking focus ([7fd75a9](https://github.com/Builditluc/wiki-tui/commit/7fd75a95e9166c61093e59e5493e9efaab88e8e8))
  - fix link focus when selecting an header ([eab4958](https://github.com/Builditluc/wiki-tui/commit/eab495812432c91b4fa8c32a7bedbe38d9f0411b))

### Documentation

- readme:
  - change usage information ([4689bdb](https://github.com/Builditluc/wiki-tui/commit/4689bdb40a2fcfd35e3753923cdb8f55598bfdfe))

## [v0.4.3](https://github.com/Builditluc/wiki-tui/releases/tag/v0.4.3) - 2021-12-22 15:04:33

*No description*

### Feature

- config:
  - add parser configuration options
- add UserParserConfig and ParserConfig
- implement the options in the Default parser
- add documentation in the readme ([04885c7](https://github.com/Builditluc/wiki-tui/commit/04885c702021afb1a19ede320e8d2b9c49d0a41b))

- wiki:
  - add code blocks to the article ([c58242a](https://github.com/Builditluc/wiki-tui/commit/c58242abfbe2d8d935d7928c7a045356eb35a526))

### Bug Fixes

- ui:
  - open the correct url when selecting a link ([bf14f09](https://github.com/Builditluc/wiki-tui/commit/bf14f09b6ed78942f5ef1e94110487eb336c8984))
  - fix selecting a header in the toc view
- add simulation of keypresses in the select function of the toc view
Closes #14 ([511a925](https://github.com/Builditluc/wiki-tui/commit/511a925bf87b2ecb192cf0dae0d1ea6487908aae))
  - remove the url prefix of the article name in the open dialog ([72d799f](https://github.com/Builditluc/wiki-tui/commit/72d799fc67fd5cbbb391a42d3307c3fcb406eb95))
  - fix selecting the first link in an article ([39c087e](https://github.com/Builditluc/wiki-tui/commit/39c087ef55bf27d9eccf218c2390668889659380))

### Documentation

- cargo:
  - bump to 0.4.3 ([7df3bb4](https://github.com/Builditluc/wiki-tui/commit/7df3bb44280996de7f50776fb14c1842ec85b1f6))

- contributing:
  - change project setup instructions ([0d72936](https://github.com/Builditluc/wiki-tui/commit/0d729362f61f2ec7e6dab4be05aeed175254f7a8))

### Refactor

- wiki:
  - remove reflist parser
- remove empty reflist parser ([b29e37b](https://github.com/Builditluc/wiki-tui/commit/b29e37b132af1a5f9c8b1c348430213f8869905c))

## [v0.4.2](https://github.com/Builditluc/wiki-tui/releases/tag/v0.4.2) - 2021-11-06 23:37:15

Added:

- nix installation (#9)

Fixes:

- garbled ascii on specific terminal geometries (#10)

**Full Changelog**: https://github.com/Builditluc/wiki-tui/compare/v0.4.1...v0.4.2

### Bug Fixes

- general:
  - fixes #10 ([ca22b09](https://github.com/Builditluc/wiki-tui/commit/ca22b095b229d4b2fd91b9b5e50d586db5804593))
  - fixed opening links ([181fc87](https://github.com/Builditluc/wiki-tui/commit/181fc87123041e9e53e70851657e920e52a9d5d9))

### Documentation

- general:
  - update .all-contributorsrc [skip ci] ([ed6abb4](https://github.com/Builditluc/wiki-tui/commit/ed6abb43d2a28b5aef9df748d71e42fd64121e8f)) ([#13](https://github.com/Builditluc/wiki-tui/pull/13))
  - update README.md [skip ci] ([0cb472b](https://github.com/Builditluc/wiki-tui/commit/0cb472bc899d7dbff8178aaec2004b5a7366b314)) ([#13](https://github.com/Builditluc/wiki-tui/pull/13))
  - update .all-contributorsrc [skip ci] ([e5b84a0](https://github.com/Builditluc/wiki-tui/commit/e5b84a0183815e0a84a13f83ea14b4e42de908a3)) ([#12](https://github.com/Builditluc/wiki-tui/pull/12))
  - update README.md [skip ci] ([95e3983](https://github.com/Builditluc/wiki-tui/commit/95e39832014e59b9b4e7ab5c71d1d786d5dea939)) ([#12](https://github.com/Builditluc/wiki-tui/pull/12))
  - create .all-contributorsrc [skip ci] ([179f78d](https://github.com/Builditluc/wiki-tui/commit/179f78d7302a542044a2de00bece360c4fe4a33b)) ([#11](https://github.com/Builditluc/wiki-tui/pull/11))
  - update README.md [skip ci] ([fd1e982](https://github.com/Builditluc/wiki-tui/commit/fd1e982a56636bca26eb45fdad0c7de36361c7b4)) ([#11](https://github.com/Builditluc/wiki-tui/pull/11))

- readme:
  - remove contributors badge ([94b5f4c](https://github.com/Builditluc/wiki-tui/commit/94b5f4cb7a0c4730f5af231c39eb2eaa82ad6cde))
  - update contributing and preview ([e537992](https://github.com/Builditluc/wiki-tui/commit/e5379927db690b914b9b92abe581121187c24789))

- contributing:
  - change code contribution guide ([731b1d7](https://github.com/Builditluc/wiki-tui/commit/731b1d7990f15d6c01d0cf0a11e8c07399da1990))
  - change code contribution guide ([3461f86](https://github.com/Builditluc/wiki-tui/commit/3461f8649b7490fe1e079f71d6794ede1dd4114c))
  - add CONTRIBUTING.md file ([5c77085](https://github.com/Builditluc/wiki-tui/commit/5c7708548c02fb959aafd781a293869f650ff9f3))

### Refactor

- general:
  - refactored the code ([b50913b](https://github.com/Builditluc/wiki-tui/commit/b50913b712c25dd9c00bc8f820fc80a10d9aa67b))

## [v0.4.1](https://github.com/Builditluc/wiki-tui/releases/tag/v0.4.1) - 2021-11-05 06:25:22

Fixes:

- Removed a warning
- Fixed a bug where wiki-tui would crash when trying to access an invalid link
- Fixed a bug where wiki-tui wouldn't change the background of the search bar
- Fixed the color of headers 
- Fixed scrolling inside of articles with no links

### Bug Fixes

- general:
  - fixed scrolling ([3ce6fda](https://github.com/Builditluc/wiki-tui/commit/3ce6fda6a73fcb39b653f8d1d022f02d4e933925))
  - fixed the color of headers ([07ebaa7](https://github.com/Builditluc/wiki-tui/commit/07ebaa74c642f9fbbe2d9f4a75e740e2310f255a))
  - fixed setting the background of the search_bar ([da318b5](https://github.com/Builditluc/wiki-tui/commit/da318b5295f50d139e2ed968897d445f6c9bd150))
  - fixed a crash when trying to access an invalid link ([28e178e](https://github.com/Builditluc/wiki-tui/commit/28e178efb6f0a0bd920f6fa3ea17d03bb513777b))
  - fixed a warning ([6fd9ad6](https://github.com/Builditluc/wiki-tui/commit/6fd9ad6ed558c46e5024a72c1eaec4cac14030d5))

## [v0.4](https://github.com/Builditluc/wiki-tui/releases/tag/v0.4) - 2021-11-02 01:10:34

Added:

- You can now go to the headers by pressing ENTER on the item in the table of contents
- When wiki-tui crashes, it generates a crash report
- You can now change the color of specific views

Changes:

- The logging is now more informative
- Added cargo & NetBSD installation method
- The configuration format is now toml

Fixes:

- Words are no longer overlapping
- Fixed a bug where wiki-tui would crash when pressing keys while searching (#6)

### Bug Fixes

- general:
  - fixed the view background when changing themes ([efad088](https://github.com/Builditluc/wiki-tui/commit/efad088a2f34eb60054a7a11e8f726c6dbdd1ad7))

## [v0.3](https://github.com/Builditluc/wiki-tui/releases/tag/v0.3) - 2021-07-29 19:44:52

Added:

- Added a table of contents on the right side of the Wikipedia article
- Links can now be opened by pressing ENTER on them
- Created a custom article view improving performance
- The logger can now be configured with a config file

Changes:

- Improved the welcome screen
- Moved the config file
- Changed the logging library to [log4rs](https://github.com/estk/log4rs)

Fixes:

- Fixed a bug where wiki-tui would crash when the search query is empty
- Fixed a bug where wiki-tui wouldn't show the first search result
- Fixed a bug where wiki-tui wouldn't start when the config file doesn't exist

## [v0.2](https://github.com/Builditluc/wiki-tui/releases/tag/v0.2) - 2021-04-28 19:32:18

Changes:
<ul>
<li>Moved the location of the configuration file</li>
<li>Displays now a preview of the selected search result and highlights the search term inside of it</li>
<li>wiki-tui now uses web scraping to format and display the article in a nice way</li>
<li>Fixed the bug, where wiki-tui would crash if the search term is empty</li>
<li>Performance improvements</li>
<li>A lot under the hood was changed, like the complete rework of the way wiki-tui uses the configuration file</>
</ul>


### Bug Fixes

- general:
  - fixed Wiki::search_articles() and continue_search in main.rs ([419be8f](https://github.com/Builditluc/wiki-tui/commit/419be8f9510bc4492ca916dff1b9398b73d49f89))

## [v0.1](https://github.com/Builditluc/wiki-tui/releases/tag/v0.1) - 2021-04-05 17:43:17

With wiki-tui, you can browse on Wikipedia within your Terminal.
To run it, clone this repository and then build/run it using cargo.
In future Releases, there will be precompiled binaries.

\* *This CHANGELOG was automatically generated by [auto-generate-changelog](https://github.com/BobAnkh/auto-generate-changelog)*
