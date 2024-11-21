# Unreleased v0.9.0-pre

## Additions

- Add toggleable logger view
- Add async event system with centralized catching and message passing
- Add processing pane when loading search results or fetching pages
- Add `theme.toml` file for configuring the theme
    - Add `--theme-config-path` cli flag to get the theme config location
    - Add options for changing the border colors
    - Add options for changing the scrollbar colors
    - Add options for changing the statusbar colors
- Add zen-mode to hide ui elements
- Add multiple new configurable keybindings

## Changes

- Rewrite the UI in `ratatui` instead of `cursive`
- Rewrite the parser to create a tree of nodes for a page
    - Use the MediaWiki HTML DOM Spec for pages and parsing
- Rewrite the renderer using `textwrap` to wrap lines of text
- Rewrite the theme configuration structure
- Rewrite the configuration structure
- Change the application pattern to a component-based architecture
- Change the logging library used to `tracing`
- Change the cli library from `structopt` to `clap`
- Change the configuration layout of the keybindings
- Improve debug panic messages using `better-panic`
- Improve release panic messages using `human-panic`
- Improve the cli interface

## Fixes

- Fix multiple issues with line-wrapping and rendering of different languages
- Fix indentation of lists and nested lists
- Fix scrollover from happening in some edge-cases
- Fix language links from not parsing correctly
- Fix cli errors not appearing when selecting an invalid language

# v0.8.2 (Fri Aug 11 2023)

:tada: This release contains work from a new contributor! :tada:

Thank you, null[@ethamck](https://github.com/ethamck), for all your work!

#### Bug Fixes 🐛

- Fix notice for unsupported elements [#215](https://github.com/Builditluc/wiki-tui/pull/215) ([@Builditluc](https://github.com/Builditluc))
- Fix translations not appearing [#214](https://github.com/Builditluc/wiki-tui/pull/214) ([@Builditluc](https://github.com/Builditluc))
- Fix word cut off [#213](https://github.com/Builditluc/wiki-tui/pull/213) ([@Builditluc](https://github.com/Builditluc))
- Fix invalid search links after language change [#209](https://github.com/Builditluc/wiki-tui/pull/209) ([@Builditluc](https://github.com/Builditluc))

#### Structure and Style Changes

- Replace raster logo with vectors [#208](https://github.com/Builditluc/wiki-tui/pull/208) ([@ethamck](https://github.com/ethamck))

#### Authors: 2

- [@Builditluc](https://github.com/Builditluc)
- [@ethamck](https://github.com/ethamck)

---

# v0.8.1 (Sat Aug 05 2023)

### Release Notes

#### Add language selection ([#190](https://github.com/Builditluc/wiki-tui/pull/190))

You can now change the language of the current article by pressing `F3` (requires an article to be open and the key can be configured). Within the popup, you can select a new language from the available ones (not all articles are available in all languages). This feature also include several configuration options, so be sure to check them out in the docs

<img width="937" alt="image" src="https://github.com/Builditluc/wiki-tui/assets/37375448/bc425809-c353-4c97-8441-ec25eb895e10">

---

#### Exciting New Features 🎉

- Add language selection [#190](https://github.com/Builditluc/wiki-tui/pull/190) ([@Builditluc](https://github.com/Builditluc))

#### Bug Fixes 🐛

- Fix whitespace caused by quoteboxes [#205](https://github.com/Builditluc/wiki-tui/pull/205) ([@Builditluc](https://github.com/Builditluc))
- Fix navigation links appearing in articles [#204](https://github.com/Builditluc/wiki-tui/pull/204) ([@Builditluc](https://github.com/Builditluc))
- Fix missing elements because of nested divs [#203](https://github.com/Builditluc/wiki-tui/pull/203) ([@Builditluc](https://github.com/Builditluc))
- Fix long gaps after lists [#202](https://github.com/Builditluc/wiki-tui/pull/202) ([@Builditluc](https://github.com/Builditluc))
- Fix ghost lists from Portalbox [#201](https://github.com/Builditluc/wiki-tui/pull/201) ([@Builditluc](https://github.com/Builditluc))
- Fix nested indentation [#200](https://github.com/Builditluc/wiki-tui/pull/200) ([@Builditluc](https://github.com/Builditluc))
- Fix description list parsing [#199](https://github.com/Builditluc/wiki-tui/pull/199) ([@Builditluc](https://github.com/Builditluc))
- Fix stack overflow crash when displaying a Namespace [#198](https://github.com/Builditluc/wiki-tui/pull/198) ([@Builditluc](https://github.com/Builditluc))
- Fix padding for list items [#196](https://github.com/Builditluc/wiki-tui/pull/196) ([@Builditluc](https://github.com/Builditluc))
- Fix disambiguation padding and prefix [#195](https://github.com/Builditluc/wiki-tui/pull/195) ([@Builditluc](https://github.com/Builditluc))
- Fix viewport reset after link selection [#194](https://github.com/Builditluc/wiki-tui/pull/194) ([@Builditluc](https://github.com/Builditluc))
- Notify on links with an Anchor [#192](https://github.com/Builditluc/wiki-tui/pull/192) ([@Builditluc](https://github.com/Builditluc))

#### Structure and Style Changes

- Rework Links to be more stable [#191](https://github.com/Builditluc/wiki-tui/pull/191) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.8.0 (Fri Jul 07 2023)

:tada: This release contains work from a new contributor! :tada:

Thank you, Enoumy ([@Enoumy](https://github.com/Enoumy)), for all your work!

### Release Notes

#### Correctly handle non-existent pages ([#186](https://github.com/Builditluc/wiki-tui/pull/186))

When you now try to open a link leading to a page that doesn't exist yet, a warning will now pop up informing you about the missing page.

#### Vim movements (ctrl+d, ctrl+u, gg, G) ([#180](https://github.com/Builditluc/wiki-tui/pull/180))

The Vim keybindings `ctrl+d`, `ctrl+u`, `gg`, and `G` have been implemented!

---

#### Exciting New Features 🎉

- Correctly handle non-existent pages [#186](https://github.com/Builditluc/wiki-tui/pull/186) ([@Builditluc](https://github.com/Builditluc))
- Vim movements (ctrl+d, ctrl+u, gg, G) [#180](https://github.com/Builditluc/wiki-tui/pull/180) ([@Enoumy](https://github.com/Enoumy))

#### Bug Fixes 🐛

- Fix url encoded links [#181](https://github.com/Builditluc/wiki-tui/pull/181) ([@Builditluc](https://github.com/Builditluc))

#### CI Pipeline and Dependency Updates

- Bump select to v0.6 [#189](https://github.com/Builditluc/wiki-tui/pull/189) ([@Builditluc](https://github.com/Builditluc))

#### Documentation Changes

- Update and Improve the Documentation [#188](https://github.com/Builditluc/wiki-tui/pull/188) ([@Builditluc](https://github.com/Builditluc))

#### Structure and Style Changes

- Change default keybindings to vim [#185](https://github.com/Builditluc/wiki-tui/pull/185) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 3

- [@Builditluc](https://github.com/Builditluc)
- Enoumy ([@Enoumy](https://github.com/Enoumy))

---

# v0.7.0 (Thu May 18 2023)

:tada: This release contains work from new contributors! :tada:

Thanks for all your work!

:heart: null[@cshjsc](https://github.com/cshjsc)

:heart: krixcrox ([@falkwitte](https://github.com/falkwitte))

### Release Notes

*Breaking Changes occurred, please check the [docs](https://builditluc.github.io/wiki-tui/0.7/changelog/upgrade/#upgrade-from-v06x-to-v07x)for instructions on how to upgrade*

#### Change log directory ([#175](https://github.com/Builditluc/wiki-tui/pull/175))

The path of the logfile and the crash report have changed, check the upgrading page to learn more

#### Add Language Selection ([#168](https://github.com/Builditluc/wiki-tui/pull/168))

You can now change the language on the fly either by using the new `-l` and `--language` cli arguments, or by pressing F2 in the tui and selecting a new language from the menu. This feature also includes several new configuration options, so be sure to check out the docs about them

Please also check the docs to learn about what breaking changes have occurred since 0.6.x

![image](https://user-images.githubusercontent.com/37375448/234084862-f317b52c-6031-4383-94f9-a87bfbe2cea4.png)
> Language Switching Dialog

---

#### Breaking Changes 🛠

- Change log directory [#175](https://github.com/Builditluc/wiki-tui/pull/175) ([@Builditluc](https://github.com/Builditluc))
- Add Language Selection [#168](https://github.com/Builditluc/wiki-tui/pull/168) ([@Builditluc](https://github.com/Builditluc) [@cshjsc](https://github.com/cshjsc))

#### Exciting New Features 🎉

- Display header numbers in the article [#173](https://github.com/Builditluc/wiki-tui/pull/173) ([@Builditluc](https://github.com/Builditluc))
- Added macro to create enum with all languages at compile time [#169](https://github.com/Builditluc/wiki-tui/pull/169) ([@cshjsc](https://github.com/cshjsc) [@Builditluc](https://github.com/Builditluc))
- Exit when no layers are present anymore [#162](https://github.com/Builditluc/wiki-tui/pull/162) ([@cshjsc](https://github.com/cshjsc))

#### Bug Fixes 🐛

- Fix cargo publish [#172](https://github.com/Builditluc/wiki-tui/pull/172) ([@Builditluc](https://github.com/Builditluc))

#### CI Pipeline and Dependency Updates

- Improve speed of ci workflow [#171](https://github.com/Builditluc/wiki-tui/pull/171) ([@Builditluc](https://github.com/Builditluc))

#### Documentation Changes

- Update documentation [#174](https://github.com/Builditluc/wiki-tui/pull/174) ([@Builditluc](https://github.com/Builditluc))
- Fix typo [#170](https://github.com/Builditluc/wiki-tui/pull/170) ([@falkwitte](https://github.com/falkwitte))
- Update Documentation [#167](https://github.com/Builditluc/wiki-tui/pull/167) ([@Builditluc](https://github.com/Builditluc))
- Add versioning to the documentation [#166](https://github.com/Builditluc/wiki-tui/pull/166) ([@Builditluc](https://github.com/Builditluc))

#### Structure and Style Changes

- Rework the Scrolling System [#164](https://github.com/Builditluc/wiki-tui/pull/164) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 3

- [@Builditluc](https://github.com/Builditluc)
- [@cshjsc](https://github.com/cshjsc)
- krixcrox ([@falkwitte](https://github.com/falkwitte))

---

# v0.6.4 (Fri Feb 17 2023)

#### Bug Fixes 🐛

- Fix disambiguations not being shown [#160](https://github.com/Builditluc/wiki-tui/pull/160) ([@Builditluc](https://github.com/Builditluc))
- Fix whitespace at the top of some articles [#159](https://github.com/Builditluc/wiki-tui/pull/159) ([@Builditluc](https://github.com/Builditluc))
- Fix no redirect messages being shown [#158](https://github.com/Builditluc/wiki-tui/pull/158) ([@Builditluc](https://github.com/Builditluc))

#### CI Pipeline and Dependency Updates

- Add changelog and use auto for creating releases [#156](https://github.com/Builditluc/wiki-tui/pull/156) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.6.3 (Thu Feb 16 2023)

#### Bug Fixes 🐛

- Fix wrong error origin in dialogs [#155](https://github.com/Builditluc/wiki-tui/pull/155) ([@Builditluc](https://github.com/Builditluc))
- Fix incorrect error message formatting when opening links [#154](https://github.com/Builditluc/wiki-tui/pull/154) ([@Builditluc](https://github.com/Builditluc))
- Fix missing whitespace on list items starting with a special character [#153](https://github.com/Builditluc/wiki-tui/pull/153) ([@Builditluc](https://github.com/Builditluc))
- Fix html tags in toc [#152](https://github.com/Builditluc/wiki-tui/pull/152) ([@Builditluc](https://github.com/Builditluc))
- Fix large gaps before and after lists [#151](https://github.com/Builditluc/wiki-tui/pull/151) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.6.2 (Sun Feb 12 2023)

#### Bug Fixes 🐛

- Fix link selection reset on layout change [#142](https://github.com/Builditluc/wiki-tui/pull/142) ([@Builditluc](https://github.com/Builditluc))
- Fix link selection of mulit-line links [#141](https://github.com/Builditluc/wiki-tui/pull/141) ([@Builditluc](https://github.com/Builditluc))

#### CI Pipeline and Dependency Updates

- Add keywords to the cargo manifest [#145](https://github.com/Builditluc/wiki-tui/pull/145) ([@Builditluc](https://github.com/Builditluc))

#### Documentation Changes

- Fix broken link on documentation site [#146](https://github.com/Builditluc/wiki-tui/pull/146) ([@Builditluc](https://github.com/Builditluc))
- Add important notice to readme [#144](https://github.com/Builditluc/wiki-tui/pull/144) ([@Builditluc](https://github.com/Builditluc))
- Improve Contribution Documentation [#139](https://github.com/Builditluc/wiki-tui/pull/139) ([@Builditluc](https://github.com/Builditluc))
- Simplify Readme [#138](https://github.com/Builditluc/wiki-tui/pull/138) ([@Builditluc](https://github.com/Builditluc))

#### Structure and Style Changes

- Rework backend [#150](https://github.com/Builditluc/wiki-tui/pull/150) ([@Builditluc](https://github.com/Builditluc))
- Create new parser system [#149](https://github.com/Builditluc/wiki-tui/pull/149) ([@Builditluc](https://github.com/Builditluc))
- Create Api Handler [#147](https://github.com/Builditluc/wiki-tui/pull/147) ([@Builditluc](https://github.com/Builditluc))
- Update label configurations in workflows [#128](https://github.com/Builditluc/wiki-tui/pull/128) ([@Builditluc](https://github.com/Builditluc))
- Update default labels in the issue templates [#127](https://github.com/Builditluc/wiki-tui/pull/127) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.6.1 (Sat Dec 31 2022)

:tada: This release contains work from a new contributor! :tada:

Thank you, Imgbot ([@ImgBotApp](https://github.com/ImgBotApp)), for all your work!

#### Bug Fixes 🐛

- Fix external links not being recognized [#121](https://github.com/Builditluc/wiki-tui/pull/121) ([@Builditluc](https://github.com/Builditluc))
- Fix short articles having a limited size [#120](https://github.com/Builditluc/wiki-tui/pull/120) ([@Builditluc](https://github.com/Builditluc))
- Fix multiple views not displaying the correct border [#118](https://github.com/Builditluc/wiki-tui/pull/118) ([@Builditluc](https://github.com/Builditluc))
- Fix error when no results are available [#105](https://github.com/Builditluc/wiki-tui/pull/105) ([@Builditluc](https://github.com/Builditluc))

#### CI Pipeline and Dependency Updates

- Update lockfile dependencies [#122](https://github.com/Builditluc/wiki-tui/pull/122) ([@Builditluc](https://github.com/Builditluc))
- Fix large release size [#119](https://github.com/Builditluc/wiki-tui/pull/119) ([@Builditluc](https://github.com/Builditluc))
- Bump cursive to 0.20 [#106](https://github.com/Builditluc/wiki-tui/pull/106) ([@Builditluc](https://github.com/Builditluc))

#### Structure and Style Changes

- Fix spelling error [#123](https://github.com/Builditluc/wiki-tui/pull/123) ([@Builditluc](https://github.com/Builditluc))

#### Other Changes

- Update contributors badge style [#108](https://github.com/Builditluc/wiki-tui/pull/108) ([@Builditluc](https://github.com/Builditluc))
- Change GitHub workflow badge routes [#107](https://github.com/Builditluc/wiki-tui/pull/107) ([@Builditluc](https://github.com/Builditluc))
- [ImgBot] Optimize images [#103](https://github.com/Builditluc/wiki-tui/pull/103) ([@ImgBotApp](https://github.com/ImgBotApp) [@imgbot[bot]](https://github.com/imgbot[bot]))

#### Authors: 3

- [@Builditluc](https://github.com/Builditluc)
- [@imgbot[bot]](https://github.com/imgbot[bot])
- Imgbot ([@ImgBotApp](https://github.com/ImgBotApp))

---

# v0.6.0 (Mon Nov 14 2022)

:tada: This release contains work from a new contributor! :tada:

Thank you, Nuno Teixeira ([@nunotexbsd](https://github.com/nunotexbsd)), for all your work!

#### Exciting New Features 🎉

- Overhaul the UI [#81](https://github.com/Builditluc/wiki-tui/pull/81) ([@Builditluc](https://github.com/Builditluc))
- Create config file if not existent [#88](https://github.com/Builditluc/wiki-tui/pull/88) ([@Builditluc](https://github.com/Builditluc))
- Better logging messages [#83](https://github.com/Builditluc/wiki-tui/pull/83) ([@Builditluc](https://github.com/Builditluc))
- Add focus keybindings to the config [#79](https://github.com/Builditluc/wiki-tui/pull/79) ([@Builditluc](https://github.com/Builditluc))

#### Bug Fixes 🐛

- Fix crash on non writable directory [#99](https://github.com/Builditluc/wiki-tui/pull/99) ([@Builditluc](https://github.com/Builditluc))
- Disable logging if not enabled in the config [#91](https://github.com/Builditluc/wiki-tui/pull/91) ([@Builditluc](https://github.com/Builditluc))
- Fix no article being displayed [#73](https://github.com/Builditluc/wiki-tui/pull/73) ([@Builditluc](https://github.com/Builditluc))

#### CI Pipeline and Dependency Updates

- Fix documentation action [#101](https://github.com/Builditluc/wiki-tui/pull/101) ([@Builditluc](https://github.com/Builditluc))
- Bump toml dependency to 0.5.9 [#85](https://github.com/Builditluc/wiki-tui/pull/85) ([@Builditluc](https://github.com/Builditluc))

#### Documentation Changes

- Add FreeBSD install instructions [#86](https://github.com/Builditluc/wiki-tui/pull/86) ([@nunotexbsd](https://github.com/nunotexbsd))
- Change Branch naming [#76](https://github.com/Builditluc/wiki-tui/pull/76) ([@Builditluc](https://github.com/Builditluc))
- Add toc configuration documentation [#75](https://github.com/Builditluc/wiki-tui/pull/75) ([@Builditluc](https://github.com/Builditluc))
- Add mkdocs documentation website [#54](https://github.com/Builditluc/wiki-tui/pull/54) ([@Builditluc](https://github.com/Builditluc))
- Change config usage [#77](https://github.com/Builditluc/wiki-tui/pull/77) ([@Builditluc](https://github.com/Builditluc))

#### Other Changes

- Add label to exclude PRs from release notes [#72](https://github.com/Builditluc/wiki-tui/pull/72) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 2

- [@Builditluc](https://github.com/Builditluc)
- Nuno Teixeira ([@nunotexbsd](https://github.com/nunotexbsd))

---

# v0.5.1 (Mon Aug 1 2022)

#### Exciting New Features 🎉

- Add more toc settings [#66](https://github.com/Builditluc/wiki-tui/pull/66) ([@Builditluc](https://github.com/Builditluc))
- Add toc position setting [#65](https://github.com/Builditluc/wiki-tui/pull/65) ([@Builditluc](https://github.com/Builditluc))

#### Bug Fixes 🐛

- Fix the crash that occurs when no links or headers exist [#70](https://github.com/Builditluc/wiki-tui/pull/70) ([@Builditluc](https://github.com/Builditluc))
- Fix Word Spacing [#68](https://github.com/Builditluc/wiki-tui/pull/68) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.5.0 (Thu Jul 21 2022)

:tada: This release contains work from new contributors! :tada:

Thanks for all your work!

:heart: Ikko Eltociear Ashimine ([@eltociear](https://github.com/eltociear))

:heart: Thomas Frans ([@ThomasFrans](https://github.com/ThomasFrans))

#### Breaking Changes 🛠

- Move Parser Configuration into a Feature Option [#63](https://github.com/Builditluc/wiki-tui/pull/63) ([@Builditluc](https://github.com/Builditluc))

#### Exciting New Features 🎉

- Add option to change the keybindings via configuration [#38](https://github.com/Builditluc/wiki-tui/pull/38) ([@Builditluc](https://github.com/Builditluc))

#### Bug Fixes 🐛

- Fix toc not having scrollbars [#64](https://github.com/Builditluc/wiki-tui/pull/64) ([@Builditluc](https://github.com/Builditluc))
- Add Codespaces Configuration [#62](https://github.com/Builditluc/wiki-tui/pull/62) ([@Builditluc](https://github.com/Builditluc))
- Disable stdout logging for release builds [#47](https://github.com/Builditluc/wiki-tui/pull/47) ([@ThomasFrans](https://github.com/ThomasFrans))
- Add missing element after line split [#48](https://github.com/Builditluc/wiki-tui/pull/48) ([@ThomasFrans](https://github.com/ThomasFrans))
- Fix Keybinding Crash [#46](https://github.com/Builditluc/wiki-tui/pull/46) ([@Builditluc](https://github.com/Builditluc))
- Fix Header Selection [#41](https://github.com/Builditluc/wiki-tui/pull/41) ([@Builditluc](https://github.com/Builditluc))

#### CI Pipeline and Dependency Updates

- Increase Stale Days to 365 [#61](https://github.com/Builditluc/wiki-tui/pull/61) ([@Builditluc](https://github.com/Builditluc))
- Fix publish action in the cd workflow [#60](https://github.com/Builditluc/wiki-tui/pull/60) ([@Builditluc](https://github.com/Builditluc))
- Add checksums for the release assets [#59](https://github.com/Builditluc/wiki-tui/pull/59) ([@Builditluc](https://github.com/Builditluc))
- Add documentation job to ci workflow [#55](https://github.com/Builditluc/wiki-tui/pull/55) ([@Builditluc](https://github.com/Builditluc))
- Add label requirement to ci workflow [#52](https://github.com/Builditluc/wiki-tui/pull/52) ([@Builditluc](https://github.com/Builditluc))
- Add a desktop-entry for packaging [#44](https://github.com/Builditluc/wiki-tui/pull/44) ([@ThomasFrans](https://github.com/ThomasFrans))

#### Documentation Changes

- Add yashinghcodes/wik project to similar projects [#58](https://github.com/Builditluc/wiki-tui/pull/58) ([@Builditluc](https://github.com/Builditluc))
- Add AUR install option [#42](https://github.com/Builditluc/wiki-tui/pull/42) ([@ThomasFrans](https://github.com/ThomasFrans))

#### Structure and Style Changes

- Fix typo in search.rs [#56](https://github.com/Builditluc/wiki-tui/pull/56) ([@eltociear](https://github.com/eltociear))
- Update Bug Report Issue Template [#40](https://github.com/Builditluc/wiki-tui/pull/40) ([@Builditluc](https://github.com/Builditluc))

#### Other Changes

- Add development notice in the README ([@Builditluc](https://github.com/Builditluc))

#### Authors: 3

- [@Builditluc](https://github.com/Builditluc)
- Ikko Eltociear Ashimine ([@eltociear](https://github.com/eltociear))
- Thomas Frans ([@ThomasFrans](https://github.com/ThomasFrans))

---

# v0.4.8 (Tue May 17 2022)

#### Exciting New Features 🎉

- Add an option for selecting a different backend [#30](https://github.com/Builditluc/wiki-tui/pull/30) ([@Builditluc](https://github.com/Builditluc))

#### Bug Fixes 🐛

- Fix the crash that occurred when scrolling during the article loading [#33](https://github.com/Builditluc/wiki-tui/pull/33) ([@Builditluc](https://github.com/Builditluc))
- Fix flickering issue with the crossterm and termion backend [#32](https://github.com/Builditluc/wiki-tui/pull/32) ([@Builditluc](https://github.com/Builditluc))

#### Structure and Style Changes

- Add issue templates [#36](https://github.com/Builditluc/wiki-tui/pull/36) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.4.7 (Fri Apr 15 2022)

#### CI Pipeline and Dependency Updates

- Fix build of published Versions on crates.io [#29](https://github.com/Builditluc/wiki-tui/pull/29) ([@Builditluc](https://github.com/Builditluc))
- Fix CD-Workflow [#28](https://github.com/Builditluc/wiki-tui/pull/28) ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.4.6 (Fri Apr 15 2022)

#### Breaking Changes 🛠

- Rework Branching and CI/CD Pipeline [#24](https://github.com/Builditluc/wiki-tui/pull/24) ([@Builditluc](https://github.com/Builditluc))

#### Changes

- Refactor with clippy and fix Tests [#25](https://github.com/Builditluc/wiki-tui/pull/25) ([@Builditluc](https://github.com/Builditluc))
- Rework the whole codebase ([@Builditluc](https://github.com/Builditluc))
- fix(ui): fix true color support ([@Builditluc](https://github.com/Builditluc))
- fix(ui): fix toc rendering ([@Builditluc](https://github.com/Builditluc))
- refactor(*): refactor wiki-tui completely ([@Builditluc](https://github.com/Builditluc))
- fix(ui): fix header selection ([@Builditluc](https://github.com/Builditluc))
- fix(*): fix link selection ([@Builditluc](https://github.com/Builditluc))
- refactor(*): finish link selecting and opening ([@Builditluc](https://github.com/Builditluc))
- refactor(*): finish text rendering and wrapping ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add new article content ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add new lines wrapper ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): remove old article structs ([@Builditluc](https://github.com/Builditluc))
- docs(wiki): further document the wiki module ([@Builditluc](https://github.com/Builditluc))
- docs(wiki): document the search result struct ([@Builditluc](https://github.com/Builditluc))
- docs(wiki): add documentation for the wiki ([@Builditluc](https://github.com/Builditluc))
- docs(ui): add documentation for the ui ([@Builditluc](https://github.com/Builditluc))
- refactor(ui): implement new search and remove articles temporarily ([@Builditluc](https://github.com/Builditluc))
- feat(cargo): change cursive backend to crossterm ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add article builder struct ([@Builditluc](https://github.com/Builditluc))
- chore(*): remove dotenv dependency ([@Builditluc](https://github.com/Builditluc))
- chore(*): add deadpendency configuration ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add parser struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): implement partialeq and debug for the article struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): remove old article file ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add article struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add tableofcontents struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add article element struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add search builder struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add search struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add search result and search info structs ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add search sort-order enum ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): use macros in the search metadata struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add search properties struct ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): add search metadata struct ([@Builditluc](https://github.com/Builditluc))
- chore(*): change name of github actions ([@Builditluc](https://github.com/Builditluc))
- style(*): refactor with clippy ([@Builditluc](https://github.com/Builditluc))
- refactor(error): change error message and crash-report location ([@Builditluc](https://github.com/Builditluc))
- chore(*): update changelog github action ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.4.5 (Wed Jan 26 2022)

:tada: This release contains work from a new contributor! :tada:

Thank you, python128 ([@python128](https://github.com/python128)), for all your work!

#### Changes

- docs(readme): Add blockquote for note [#17](https://github.com/Builditluc/wiki-tui/pull/17) (nexinov5vatr@gmail.com [@python128](https://github.com/python128))
- docs(changelog): add release checklist ([@Builditluc](https://github.com/Builditluc))
- chore(*): change branch and run condition of the github changelog action ([@Builditluc](https://github.com/Builditluc))
- docs(*): add class diagram of the code ([@Builditluc](https://github.com/Builditluc))
- chore(*): move github actions into seperate files ([@Builditluc](https://github.com/Builditluc))
- chore(*): add github action publishing wiki-tui to crates.io ([@Builditluc](https://github.com/Builditluc))
- chore(*): add github action building and testing the code ([@Builditluc](https://github.com/Builditluc))
- feat(wiki): add response error code checking ([@Builditluc](https://github.com/Builditluc))
- feat(*): make --search flag default ([@Builditluc](https://github.com/Builditluc))

#### Authors: 3

- [@Builditluc](https://github.com/Builditluc)
- NexiNov (nexinov5vatr@gmail.com)
- python128 ([@python128](https://github.com/python128))

---

# v0.4.4 (Wed Jan 12 2022)

#### Changes

- fix(ui): fix horizontal link focus ([@Builditluc](https://github.com/Builditluc))
- fix(ui): fix article view not taking focus ([@Builditluc](https://github.com/Builditluc))
- style(ui): refactor with cargo fmt ([@Builditluc](https://github.com/Builditluc))
- fix(ui): fix link focus when selecting an header ([@Builditluc](https://github.com/Builditluc))
- feat(*): add cli support ([@Builditluc](https://github.com/Builditluc))
- feat(*): add cli 'level' argument ([@Builditluc](https://github.com/Builditluc))
- docs(readme): change usage information ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.4.3 (Wed Dec 22 2021)

#### Changes

- chore(*): change generate Changelog action ([@Builditluc](https://github.com/Builditluc))
- docs(contributing): change project setup instructions ([@Builditluc](https://github.com/Builditluc))
- style(ui): remove several redundant closures ([@Builditluc](https://github.com/Builditluc))
- feat(config): add parser configuration options ([@Builditluc](https://github.com/Builditluc))
- fix(ui): open the correct url when selecting a link ([@Builditluc](https://github.com/Builditluc))
- refactor(wiki): remove reflist parser ([@Builditluc](https://github.com/Builditluc))
- feat(wiki): add code blocks to the article ([@Builditluc](https://github.com/Builditluc))
- chore(*): update nix shell ([@Builditluc](https://github.com/Builditluc))
- fix(ui): fix selecting a header in the toc view ([@Builditluc](https://github.com/Builditluc))
- fix(ui): remove the url prefix of the article name in the open dialog ([@Builditluc](https://github.com/Builditluc))
- fix(ui): fix selecting the first link in an article ([@Builditluc](https://github.com/Builditluc))
- chore(*): update rust action ([@Builditluc](https://github.com/Builditluc))

#### Authors: 1

- [@Builditluc](https://github.com/Builditluc)

---

# v0.4.2 (Sun Nov 7 2021)

### Release Notes

Added:

* nix installation (#9)

Fixes:

* garbled ascii on specific terminal geometries (#10)

#### Changes

- Add nix installation instructions [#9](https://github.com/Builditluc/wiki-tui/pull/9) ([@Builditluc](https://github.com/Builditluc) [@legendofmiracles](https://github.com/legendofmiracles))
- docs(readme): remove contributors badge ([@Builditluc](https://github.com/Builditluc))
- chore(*): add generate Changelog action ([@Builditluc](https://github.com/Builditluc))
- docs(CHANGELOG): add release notes ([@Builditluc](https://github.com/Builditluc))
- chore(cargo): update version to 0.4.2 ([@Builditluc](https://github.com/Builditluc))
- docs(contributing): change code contribution guide ([@Builditluc](https://github.com/Builditluc))
- docs(readme): update contributing and preview ([@Builditluc](https://github.com/Builditluc))
- docs(contributing): add CONTRIBUTING.md file ([@Builditluc](https://github.com/Builditluc))
- refactored the code ([@Builditluc](https://github.com/Builditluc))
- added similar projects to the readme ([@Builditluc](https://github.com/Builditluc))
- fixes #10 ([@Builditluc](https://github.com/Builditluc))
- moved the initialization stuff into a function ([@Builditluc](https://github.com/Builditluc))
- fixed opening links ([@Builditluc](https://github.com/Builditluc))

#### Authors: 2

- [@Builditluc](https://github.com/Builditluc)
- [@legendofmiracles](https://github.com/legendofmiracles)

---

# v0.4.1 (Fri Nov 5 2021)

#### Release Notes

Fixes:

* Removed a warning
* Fixed a bug where wiki-tui would crash when trying to access an invalid link
* Fixed a bug where wiki-tui wouldn't change the background of the search bar
* Fixed the color of headers
* Fixed scrolling inside of articles with no links

#### Changes

- Add nix installation instructions [#9](https://github.com/Builditluc/wiki-tui/pull/9) ([@Builditluc](https://github.com/Builditluc) [@legendofmiracles](https://github.com/legendofmiracles))
- fixed scrolling ([@Builditluc](https://github.com/Builditluc))
- fixed the color of headers ([@Builditluc](https://github.com/Builditluc))
- fixed setting the background of the search_bar ([@Builditluc](https://github.com/Builditluc))
- fixed a crash when trying to access an invalid link ([@Builditluc](https://github.com/Builditluc))
- fixed a warning ([@Builditluc](https://github.com/Builditluc))

#### Authors: 2

- [@Builditluc](https://github.com/Builditluc)
- [@legendofmiracles](https://github.com/legendofmiracles)

---

# v0.4.0 (Tue Nov 2 2021)

#### Release Notes

Added:

* You can now go to the headers by pressing ENTER on the item in the table of contents
* When wiki-tui crashes, it generates a crash report
* You can now change the color of specific views

Changes:

* The logging is now more informative
* Added cargo & NetBSD installation method
* The configuration format is now toml

Fixes:

* Words are no longer overlapping
* Fixed a bug where wiki-tui would crash when pressing keys while searching (#6)

#### Changes

- Add nix installation instructions [#9](https://github.com/Builditluc/wiki-tui/pull/9) ([@Builditluc](https://github.com/Builditluc) [@legendofmiracles](https://github.com/legendofmiracles))
- changed preview pictures ([@Builditluc](https://github.com/Builditluc))
- added usage to readme ([@Builditluc](https://github.com/Builditluc))
- config::Config now uses logging ([@Builditluc](https://github.com/Builditluc))
- fixed the view background when changing themes ([@Builditluc](https://github.com/Builditluc))
- Added a way to change the color of specific views (WIP) ([@Builditluc](https://github.com/Builditluc))
- Added CHANGELOG.md ([@Builditluc](https://github.com/Builditluc))
- bump to 0.3.5 ([@Builditluc](https://github.com/Builditluc))
- Added .envrc ([@Builditluc](https://github.com/Builditluc))
- Fixes #6 ([@Builditluc](https://github.com/Builditluc))
- Added Cargo.lock ([@Builditluc](https://github.com/Builditluc))
- bump to 0.3.4 ([@Builditluc](https://github.com/Builditluc))
- Changed the configuration format from ini to toml ([@Builditluc](https://github.com/Builditluc))
- Fixed the header selection ([@Builditluc](https://github.com/Builditluc))
- Added cargo installation method ([@Builditluc](https://github.com/Builditluc))
- Preperation for publish to crates.io ([@Builditluc](https://github.com/Builditluc))
- Added support for cli arguments ([@Builditluc](https://github.com/Builditluc))
- bump to 0.3.3 ([@Builditluc](https://github.com/Builditluc))
- Removed human-panic and replaced it with a custom panic handler ([@Builditluc](https://github.com/Builditluc))
- Added ArticleView::move_focus, ArticleView::on_event uses it now ([@Builditluc](https://github.com/Builditluc))
- Added human-panic and panic-message ([@Builditluc](https://github.com/Builditluc))
- Program won't crash if an invalid header is selected ([@Builditluc](https://github.com/Builditluc))
- Refactoring ([@Builditluc](https://github.com/Builditluc))
- Created a LinesIterator and moved ArticleView::calculate_lines() to it ([@Builditluc](https://github.com/Builditluc))
- Modified .gitignore ([@Builditluc](https://github.com/Builditluc))
- Better and more informative logging ([@Builditluc](https://github.com/Builditluc))
- You can now go to the headers by pressing ENTER on the item in the toc ([@Builditluc](https://github.com/Builditluc))
- Fixed the scrolling ([@Builditluc](https://github.com/Builditluc))
- Added some settings to the config and changed the README ([@Builditluc](https://github.com/Builditluc))
- Added a initialization thread ([@Builditluc](https://github.com/Builditluc))
- Fixed the calculate_lines function. Words will no longer be overlapping ([@Builditluc](https://github.com/Builditluc))
- Moved some code around ([@Builditluc](https://github.com/Builditluc))
- Refactored parser.rs ([@Builditluc](https://github.com/Builditluc))
- bump to 0.3.1 ([@Builditluc](https://github.com/Builditluc))
- added Contributing and Acknowledgements to the README ([@Builditluc](https://github.com/Builditluc))
- Removed tests.rs ([@Builditluc](https://github.com/Builditluc))

#### Authors: 3

- [@Builditluc](https://github.com/Builditluc)
- [@legendofmiracles](https://github.com/legendofmiracles)
- pin ([@0323pin](https://github.com/0323pin))

---

# v0.3.0 (Tue Jul 29 2021)

:tada: This release contains work from a new contributor! :tada:

Thank you, pin ([@0323pin](https://github.com/0323pin)), for all your work!

#### Release Notes

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

#### Changes

- Add NetBSD install to readme [#8](https://github.com/Builditluc/wiki-tui/pull/8) ([@Builditluc](https://github.com/Builditluc) [@0323pin](https://github.com/0323pin))
- bump to 0.3.0, refactored with clippy ([@Builditluc](https://github.com/Builditluc))
- Update README.md ([@Builditluc](https://github.com/Builditluc))
- Changed the user agent for the requests ([@Builditluc](https://github.com/Builditluc))
- The wiki-tui LOGO is now centered ([@Builditluc](https://github.com/Builditluc))
- bump to 0.2.9 ([@Builditluc](https://github.com/Builditluc))
- Even more refactoring ([@Builditluc](https://github.com/Builditluc))
- Refactoring ([@Builditluc](https://github.com/Builditluc))
- The logger can now be configured via logging.yml ([@Builditluc](https://github.com/Builditluc))
- Moved the location of the config file ([@Builditluc](https://github.com/Builditluc))
- New lines won't have whitespaces at their start in the article view ([@Builditluc](https://github.com/Builditluc))
- Changed some log messages ([@Builditluc](https://github.com/Builditluc))
- The width of the article_view is now calculated correctely ([@Builditluc](https://github.com/Builditluc))
- The article_view and the toc_view now use the whole height ([@Builditluc](https://github.com/Builditluc))
- Fixed the line splitting ([@Builditluc](https://github.com/Builditluc))
- Added a toc_view ([@Builditluc](https://github.com/Builditluc))
- Began adding the TableOfContents ([@Builditluc](https://github.com/Builditluc))
- Changed the configuration of the logger ([@Builditluc](https://github.com/Builditluc))
- The dialog is now working ([@Builditluc](https://github.com/Builditluc))
- Added dialog when trying to open a link ([@Builditluc](https://github.com/Builditluc))
- Improved the line splitting ([@Builditluc](https://github.com/Builditluc))
- Created a function to render a single element ([@Builditluc](https://github.com/Builditluc))
- ArticleContent is now displaying the content given as a StyledString ([@Builditluc](https://github.com/Builditluc))
- Changed on_link_submit_callback() to on_link_submit() ([@Builditluc](https://github.com/Builditluc))
- Added the function on_link_submit_callback() to the ArticleView ([@Builditluc](https://github.com/Builditluc))
- Added the on_link_submit_callback ([@Builditluc](https://github.com/Builditluc))
- Moved some code around ([@Builditluc](https://github.com/Builditluc))
- Added UP and DOWN link selecting ([@Builditluc](https://github.com/Builditluc))
- Implemented basic link selecting ([@Builditluc](https://github.com/Builditluc))
- Links are now added to the LinkHandler ([@Builditluc](https://github.com/Builditluc))
- Removed error.txt ([@Builditluc](https://github.com/Builditluc))
- Implemented the new Element System ([@Builditluc](https://github.com/Builditluc))
- Moved some files around ([@Builditluc](https://github.com/Builditluc))
- Removed the ArticleContentInner struct ([@Builditluc](https://github.com/Builditluc))
- Added parser::Default::parse_child and added Italic/Bold text ([@Builditluc](https://github.com/Builditluc))
- wiki-tui now also shows lists ([@Builditluc](https://github.com/Builditluc))
- Removed the article categories view ([@Builditluc](https://github.com/Builditluc))
- wiki-tui does no longer crash when no search results were found ([@Builditluc](https://github.com/Builditluc))
- Removes todo.md ([@Builditluc](https://github.com/Builditluc))
- Bug fixes ([@Builditluc](https://github.com/Builditluc))
- Merge branch 'article_view' into experimental ([@Builditluc](https://github.com/Builditluc))
- Added more caches to increase performance ([@Builditluc](https://github.com/Builditluc))
- Added the TODO file ([@Builditluc](https://github.com/Builditluc))
- Removed the article_categories_view ([@Builditluc](https://github.com/Builditluc))
- Added a log message ([@Builditluc](https://github.com/Builditluc))
- Merge branch 'stable' into experimental ([@Builditluc](https://github.com/Builditluc))
- The article preview is now also shown for the first search result ([@Builditluc](https://github.com/Builditluc))
- Clears the search bar after a search ([@Builditluc](https://github.com/Builditluc))
- v0.2.4 ([@Builditluc](https://github.com/Builditluc))
- Performance improvements ([@Builditluc](https://github.com/Builditluc))
- More Performance improvements ([@Builditluc](https://github.com/Builditluc))
- Minor Performance improvements ([@Builditluc](https://github.com/Builditluc))
- Merge branch 'experimental' into article_view ([@Builditluc](https://github.com/Builditluc))
- Bug Fixing ([@Builditluc](https://github.com/Builditluc))
- Added the article_categories_view which show the articles categories ([@Builditluc](https://github.com/Builditluc))
- Fixed a bug where the program would crash when no search results are found ([@Builditluc](https://github.com/Builditluc))
- Changed the base url and fixed a bug ([@Builditluc](https://github.com/Builditluc))
- v0.2.3 ([@Builditluc](https://github.com/Builditluc))
- Added Configuration ([@Builditluc](https://github.com/Builditluc))
- ui::article::ArticleView now takes focus ([@Builditluc](https://github.com/Builditluc))
- Added ArticleContent::render and several struct inside of the module ([@Builditluc](https://github.com/Builditluc))
- Added ui::article::ArticleContent ([@Builditluc](https://github.com/Builditluc))
- Row-Elements are now selectable ([@Builditluc](https://github.com/Builditluc))
- Replaced the TextView with the custom ArticleView ([@Builditluc](https://github.com/Builditluc))
- Made the ArticleView scrollable ([@Builditluc](https://github.com/Builditluc))
- Added basic printing of text to the ArticleView ([@Builditluc](https://github.com/Builditluc))
- v0.2.2 ([@Builditluc](https://github.com/Builditluc))
- Added the title to the article ([@Builditluc](https://github.com/Builditluc))
- Fixed a Theme bug ([@Builditluc](https://github.com/Builditluc))
- Changed the search_match color ([@Builditluc](https://github.com/Builditluc))
- Removed the borders color ([@Builditluc](https://github.com/Builditluc))
- The Custom Theme is now applied to the program ([@Builditluc](https://github.com/Builditluc))
- Added Config::load_theme and Config::parse_color ([@Builditluc](https://github.com/Builditluc))
- Added a loading message box when loading the article ([@Builditluc](https://github.com/Builditluc))
- The log messages now contain the function which called them ([@Builditluc](https://github.com/Builditluc))
- Added more Detailed logs to config::Config ([@Builditluc](https://github.com/Builditluc))
- Added log statements to config::Config ([@Builditluc](https://github.com/Builditluc))
- wiki::WikiApi now uses config::CONFIG ([@Builditluc](https://github.com/Builditluc))
- Finished writing the new Config system ([@Builditluc](https://github.com/Builditluc))
- Began working on the new config system ([@Builditluc](https://github.com/Builditluc))
- Fixed a typo ([@Builditluc](https://github.com/Builditluc))
- v0.2.1 ([@Builditluc](https://github.com/Builditluc))
- Added the lazy_static crate and created the static variable CONFIG ([@Builditluc](https://github.com/Builditluc))
- Created the two modules wiki and ui and moved some code around ([@Builditluc](https://github.com/Builditluc))
- the log file will be overwritten everytime the program starts ([@Builditluc](https://github.com/Builditluc))
- Replaced simple_logging with log4rs ([@Builditluc](https://github.com/Builditluc))
- Refactored wiki.rs ([@Builditluc](https://github.com/Builditluc))
- Finished refactoring main.rs ([@Builditluc](https://github.com/Builditluc))
- Refactored the on_search function ([@Builditluc](https://github.com/Builditluc))
- Updated the logo ([@Builditluc](https://github.com/Builditluc))
- Added an ASCII Art logo ([@Builditluc](https://github.com/Builditluc))

#### Authors: 2

- [@Builditluc](https://github.com/Builditluc)
- pin ([@0323pin](https://github.com/0323pin))

---

# v0.2.0 (Wed Apr 28 2021)

#### Release Notes

Changes:
* Moved the location of the configuration file
* Displays now a preview of the selected search result and highlights the search term inside of it
* wiki-tui now uses web scraping to format and display the article in a nice way
* Fixed the bug, where wiki-tui would crash if the search term is empty
* Performance improvements
* A lot under the hood was changed, like the complete rework of the way wiki-tui uses the configuration file</>

#### Changes

- compiles [#5](https://github.com/Builditluc/wiki-tui/pull/5) ([@legendofmiracles](https://github.com/legendofmiracles))
- Delete 3 [#4](https://github.com/Builditluc/wiki-tui/pull/4) ([@Builditluc](https://github.com/Builditluc))
- Update README.md ([@Builditluc](https://github.com/Builditluc))
- Bug fixing ([@Builditluc](https://github.com/Builditluc))
- Changed the article view back to a single TextView instead of a ([@Builditluc](https://github.com/Builditluc))
- Fixed a bug ([@Builditluc](https://github.com/Builditluc))
- Added the Default parser ([@Builditluc](https://github.com/Builditluc))
- Made the article view a Linear Layout. The on_article_submit function ([@Builditluc](https://github.com/Builditluc))
- Fixed a bug in the impl of Parser for wiki::articles::Article ([@Builditluc](https://github.com/Builditluc))
- Added the Parser trait and modified the wiki::Wiki::get_article() function. It now returns the structs::wiki::article::Article struct ([@Builditluc](https://github.com/Builditluc))
- Added the structs::article::Article struct ([@Builditluc](https://github.com/Builditluc))
- v0.1.5 ([@Builditluc](https://github.com/Builditluc))
- v0.1.4 ([@Builditluc](https://github.com/Builditluc))
- Fully implemented anyhow ([@Builditluc](https://github.com/Builditluc))
- Began to implement anyhow and use the unused results ([@Builditluc](https://github.com/Builditluc))
- Removed the scraper crate ([@Builditluc](https://github.com/Builditluc))
- Added the anyhow crate ([@Builditluc](https://github.com/Builditluc))
- v0.1.3.1 ([@Builditluc](https://github.com/Builditluc))
- fixed Wiki::search_articles() and continue_search in main.rs ([@Builditluc](https://github.com/Builditluc))
- v0.1.3 ([@Builditluc](https://github.com/Builditluc))
- Removes .main.rs.swp ([@Builditluc](https://github.com/Builditluc))
- Made the results_view scrollable and set a fixed height for the results layer ([@Builditluc](https://github.com/Builditluc))
- Added functionality to the continue button ([@Builditluc](https://github.com/Builditluc))
- Compressed code inside of on_search() ([@Builditluc](https://github.com/Builditluc))
- Added some more things to main.rs() ([@Builditluc](https://github.com/Builditluc))
- on_article_submit() and on_search() are now using a mutable reference of wiki::Wiki ([@Builditluc](https://github.com/Builditluc))
- v0.1.2 ([@Builditluc](https://github.com/Builditluc))
- Fixed the lifetime bug with the help of Alexandre Bury (Author of the cursive library ([@Builditluc](https://github.com/Builditluc))
- Removes 3 ([@Builditluc](https://github.com/Builditluc))
- The UI now uses one instance of wiki::Wiki, program crashes because of lifetime problems ([@Builditluc](https://github.com/Builditluc))
- wiki::Wiki now uses ApiConfig, errors are appearing in main.rs ([@Builditluc](https://github.com/Builditluc))
- added config::ApiConfig and some functions in config::Config to load the Api Config ([@Builditluc](https://github.com/Builditluc))
- logging.rs now uses config::LoggingConfig ([@Builditluc](https://github.com/Builditluc))
- Added Config::load_logging() and Config::get_logging_config() which returns a reference to the LoggingConfig ([@Builditluc](https://github.com/Builditluc))
- Removed something ([@Builditluc](https://github.com/Builditluc))
- Added Config::load() and implemented Default for config::Config. Also added Config::new() ([@Builditluc](https://github.com/Builditluc))
- Added Config::is_config_valid(), this is a prviate function used only inside of config::Config ([@Builditluc](https://github.com/Builditluc))
- Added Config::create_config_file(), this is a private function only used inside of config::Config ([@Builditluc](https://github.com/Builditluc))
- Added Config::get_config_file(), this is a private function only used inside config::Config ([@Builditluc](https://github.com/Builditluc))
- Removed some of the Config code ([@Builditluc](https://github.com/Builditluc))
- Continued working on config::Config ([@Builditluc](https://github.com/Builditluc))
- Added config::Config and config::Theme. Also added the get_or_create_config_paths() function ([@Builditluc](https://github.com/Builditluc))
- Added the config.rs file and imported the dirs crate ([@Builditluc](https://github.com/Builditluc))
- Create README.md ([@Builditluc](https://github.com/Builditluc))
- v0.1.1 ([@Builditluc](https://github.com/Builditluc))
- Changed build branch into stable ([@Builditluc](https://github.com/Builditluc))
- Update rust.yml ([@Builditluc](https://github.com/Builditluc))
- Added some callbacks for the paging ([@Builditluc](https://github.com/Builditluc))
- Rearranged some Views in the Results View and added the Search Info ([@Builditluc](https://github.com/Builditluc))
- The Results Preview has now a fixed width ([@Builditluc](https://github.com/Builditluc))
- Forgot to display the title as well ([@Builditluc](https://github.com/Builditluc))
- These stupid searchmatches are finally highlighted ([@Builditluc](https://github.com/Builditluc))
- Removed the <span> html tags inside of the Article snippet ([@Builditluc](https://github.com/Builditluc))
- Added the scraper crate ([@Builditluc](https://github.com/Builditluc))
- Added a Preview for the Results ([@Builditluc](https://github.com/Builditluc))

#### Authors: 2

- [@Builditluc](https://github.com/Builditluc)
- [@legendofmiracles](https://github.com/legendofmiracles)

---

# v0.1.0 (Mon Apr 5 2021)

:tada: This release contains work from new contributors! :tada:

Thanks for all your work!

:heart: null[@Builditluc](https://github.com/Builditluc)

:heart: null[@legendofmiracles](https://github.com/legendofmiracles)

#### Release Notes

With wiki-tui, you can browse on Wikipedia within your Terminal.
To run it, clone this repository and then build/run it using cargo.
In future Releases, there will be precompiled binaries.

#### Changes 

- adds shell.nix [#3](https://github.com/Builditluc/wiki-tui/pull/3) ([@legendofmiracles](https://github.com/legendofmiracles))
- Removed everything (almost) [#2](https://github.com/Builditluc/wiki-tui/pull/2) ([@Builditluc](https://github.com/Builditluc))
- Create LICENSE [#1](https://github.com/Builditluc/wiki-tui/pull/1) ([@Builditluc](https://github.com/Builditluc))
- Moved the widgets around and added a Search Results popup ([@Builditluc](https://github.com/Builditluc))
- The Title of the Searchbar is now left-aligned ([@Builditluc](https://github.com/Builditluc))
- The Results View is now automatically focussed after searching ([@Builditluc](https://github.com/Builditluc))
- Removed the html2text crate ([@Builditluc](https://github.com/Builditluc))
- Added searching via pressing the Enter key inside of the search bar ([@Builditluc](https://github.com/Builditluc))
- Implemented the Article View functionality ([@Builditluc](https://github.com/Builditluc))
- The Article is now displayed correctly ([@Builditluc](https://github.com/Builditluc))
- The Tui is now showing the parsed Content of the Article (Not quite well) ([@Builditluc](https://github.com/Builditluc))
- The Results View now calls the on_article_submit function ([@Builditluc](https://github.com/Builditluc))
- removes shell.nix ([@Builditluc](https://github.com/Builditluc))
- Implemented the Search function ([@Builditluc](https://github.com/Builditluc))
- Fully implemented the on_search function ([@Builditluc](https://github.com/Builditluc))
- Fixed the crashing bug ([@Builditluc](https://github.com/Builditluc))
- Merge branch 'experimental' into stable ([@Builditluc](https://github.com/Builditluc))
- Update README.md ([@Builditluc](https://github.com/Builditluc))
- Added some more things to on_search(). Program crashes now when clicking on the search button ([@Builditluc](https://github.com/Builditluc))
- Implemented the From trait for wiki::ArticleResultPreview ([@Builditluc](https://github.com/Builditluc))
- Created a on_search function which is called when the user presses the Search button ([@Builditluc](https://github.com/Builditluc))
- Created a Demo struct for the ArticlePreview in the results view ([@Builditluc](https://github.com/Builditluc))
- Moved all of the structs for the web requests into the structs::wiki mod ([@Builditluc](https://github.com/Builditluc))
- Moved the UI stuff into main.rs and fixed some bugs ([@Builditluc](https://github.com/Builditluc))
- Changed the README ([@Builditluc](https://github.com/Builditluc))
- Added more of the UI ([@Builditluc](https://github.com/Builditluc))
- Added the tui struct and created the search bar (non functional) ([@Builditluc](https://github.com/Builditluc))
- Added the continue_search function. Moved the normal search in to a search_articles function ([@Builditluc](https://github.com/Builditluc))
- Bug fixing ([@Builditluc](https://github.com/Builditluc))
- Merge branch 'stable' of https://github.com/Builditluc/wiki-tui into stable ([@Builditluc](https://github.com/Builditluc))
- Added the ArticleResponse structs for the get_article function. Added the get_article function to the Wiki struct ([@Builditluc](https://github.com/Builditluc))
- Created the Response structs for the Search request. Added the wiki_tui.log file to .gitignore ([@Builditluc](https://github.com/Builditluc))
- Added the .gradle/ directory to the gitignore file ([@Builditluc](https://github.com/Builditluc))
- Fixed a bug where cargo would not compile ([@Builditluc](https://github.com/Builditluc))
- Added and implemented the rust-ini crate ([@Builditluc](https://github.com/Builditluc))
- Added the wiki::Wikipedia trait ([@Builditluc](https://github.com/Builditluc))
- Changed some things in the README ([@Builditluc](https://github.com/Builditluc))
- Add files via upload ([@Builditluc](https://github.com/Builditluc))
- Rename LICENSE to LICENSE.txt ([@Builditluc](https://github.com/Builditluc))
- Create README.md ([@Builditluc](https://github.com/Builditluc))
- Create rust.yml ([@Builditluc](https://github.com/Builditluc))
- Added the search_articles function ([@Builditluc](https://github.com/Builditluc))
- Implemented the update_article function of the WikiSql struct ([@Builditluc](https://github.com/Builditluc))
- Implemented the fetch_article function for the WikiSql struct ([@Builditluc](https://github.com/Builditluc))
- Added the Article and the NewArticle model ([@Builditluc](https://github.com/Builditluc))
- Created the table articles ([@Builditluc](https://github.com/Builditluc))
- Added the fetch_article function in the Api struct ([@Builditluc](https://github.com/Builditluc))
- Fixed a bug ([@Builditluc](https://github.com/Builditluc))
- Implemented the Removable trait for WikiSql ([@Builditluc](https://github.com/Builditluc))
- Implemented the Updatable Trait for WikiSql ([@Builditluc](https://github.com/Builditluc))
- Removed an unused import ([@Builditluc](https://github.com/Builditluc))
- Added the trait functions to the WikiSql struct ([@Builditluc](https://github.com/Builditluc))
- Added some functions for the ArticleIndex model ([@Builditluc](https://github.com/Builditluc))
- Implemented the wiki::Fetchable trait for WikiSql ([@Builditluc](https://github.com/Builditluc))
- Added TODO ([@Builditluc](https://github.com/Builditluc))
- Added the traits wiki::Fetchable, wiki::Updatable, wiki::Removable ([@Builditluc](https://github.com/Builditluc))
- The wiki struct now establishes a new connection when created ([@Builditluc](https://github.com/Builditluc))
- Added the traits file ([@Builditluc](https://github.com/Builditluc))
- Added logging for the Api struct ([@Builditluc](https://github.com/Builditluc))
- Prepared everything for the database integration ([@Builditluc](https://github.com/Builditluc))
- Fixed the env bug ([@Builditluc](https://github.com/Builditluc))
- Added the Logging module ([@Builditluc](https://github.com/Builditluc))
- Added the Api Struct and the ArticlesResultCallback Trait ([@Builditluc](https://github.com/Builditluc))
- Modified .gitignore ([@Builditluc](https://github.com/Builditluc))
- Initial commit ([@Builditluc](https://github.com/Builditluc))

#### Authors: 2

- [@Builditluc](https://github.com/Builditluc)
- [@legendofmiracles](https://github.com/legendofmiracles)
