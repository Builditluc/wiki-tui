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

### Void

As root, run

```
xbps-install -S wiki-tui
```

with the main repos installed. Works on glibc and musl installations.

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

If you want, you can also use the latest development version from the GitHub (can be unstable)

```sh
cargo install --git https://github.com/builditluc/wiki-tui
```

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
