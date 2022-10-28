# Getting started

wiki-tui is a simple and easy-to-use Wikipedia text user interface written in rust. The package already is in a few package repositories with more coming but you can also compile it from source if you want.

## Installation

After installing wiki-tui with your preferred method you can start it with `wiki-tui`.

### Nix

You can find the package in the [nixpkgs](https://search.nixos.org/packages?channel=unstable&show=wiki-tui&from=0&size=50&sort=relevance&type=packages&query=wiki-tui) repository. Either install it with

```sh
nix-env -iA wiki-tui
```

Get it temporarily to try it out

```sh
nix-shell -p wiki-tui
```

Or add it to your configuration.

### AUR

The package is available in the [AUR](https://aur.archlinux.org/packages/wiki-tui). Either install it with `makepkg` manually or use the preferred AUR helper.

### NetBSD

Using the package manager

```sh
pkgin install wiki-tui
```

Building from source

```sh
cd /usr/pkgsrc/www/wiki-tui
make install
```

### FreeBSD

Using the package manager

```sh
pkg install wiki-tui
```

Building from source

```sh
cd /usr/ports/www/wiki-tui/
make install clean
```

### Cargo

If wiki-tui cannot be installed with your package manager, you can also install it with cargo. There are no extra dependencies (except rust of course).

```sh
cargo install wiki-tui
```

If you want, you can also use the latest development version from GitHub (can be unstable)

```sh
cargo install --git https://github.com/builditluc/wiki-tui
```

When you install wiki-tui with cargo, you can choose what kind of backend (this is what the UI library [cursive](https://github.com/gyscos/cursive) uses to draw to the screen and interface with the terminal) is being used. You can select a different backend (the default one is `crossterm`) by modifying 
the installation command 

The following backends are available:

=== "crossterm (default)"
    Uses the pure-rust [crossterm](https://github.com/TimonPost/crossterm) library. Works cross-platform, even for windows systems down to version 7

    Install command
    ```sh
    cargo install wiki-tui --no-default-features --features crossterm-backend
    ```

=== "termion"
    Uses the pure-rust [termion](https://github.com/ticki/termion) library. Works on Linux, macOS, and Redox

    Install command
    ```sh
    cargo install wiki-tui --no-default-features --features termion-backend
    ```

=== "ncurses"
    Uses the [ncurses-rs](https://github.com/jeaye/ncurses-rs) library directly. Currently only compatible on Linux and macOS. Requires [ncurses](https://github.com/gyscos/Cursive/wiki/Install-ncurses) to be installed on the system

    !!! warning
        The ncurses backend doesn't support true colors (which is the full RGB color spectrum with 255 values per channel). What that means is the configured colors won't be displayed correctly and depend on your terminal theme

    Install command
    ```sh
    cargo install wiki-tui --no-default-features --features ncurses-backend
    ```

=== "pancurses"
    Uses the [pancurses](https://github.com/ihalila/pancurses) library, which forwards calls to [ncurses-rs](https://github.com/jeaye/ncurses-rs) on Linux/macOS or [pdcurses-sys](https://github.com/ihalila/pdcurses-sys) on Windows

    !!! warning
        The pancurses backend doesn't support true colors (which is the full RGB color spectrum with 255 values per channel). What that means is the configured colors won't be displayed correctly and depend on your terminal theme

    ```sh
    cargo install wiki-tui --no-default-features --features pancurses-backend
    ```

=== "blt"
    Uses the cross-platform [BearLibTerminal.rs](https://github.com/nabijaczleweli/BearLibTerminal.rs) binding. Works on Linux and Windows.

    !!! note
        BearLibTerminal is a graphical application emulating a terminal. There is an [archlinux package](https://aur.archlinux.org/packages/bearlibterminal-hg/), or you can 
        [download a release](https://github.com/nabijaczleweli/BearLibTerminal.rs/releases)

    ```sh
    cargo install wiki-tui --no-default-features --features blt-backend
    ```

!!! info
    The description of the available backends was taken from the corresponding [wiki](https://github.com/gyscos/cursive/wiki/Backends) page of the 
    [cursive library](https://github.com/gyscos/cursive) (wiki-tui uses cursive for the ui).

###  Git

wiki-tui can be directly used from the GitHub [repository](https://github.com/builditluc/wiki-tui) by cloning it into a folder on your system and compiling it from source. This can be useful if you want to try out the latest version
without installing it directly on your system.

=== "HTTPS"
    ```
    git clone https://github.com/Builditluc/wiki-tui.git
    ```

=== "SSH"
    ```
    git clone git@github.com:Builditluc/wiki-tui.git
    ```

=== "GitHub CLI"
    ```
    gh repo clone Builditluc/wiki-tui
    ```

??? note "External Dependencies of different backends"
    When trying out a different backend, check if the backend requires any external dependencies
