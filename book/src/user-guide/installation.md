# Installation

wiki-tui already is in the following package repositories:

- [nixpkgs](#nix)
- [void](#void)
- [NetBSD](#netbsd)
- [FreeBSD](#freebsd)

If not, you can always [build from source](#building-from-source)

## Pre-compiled binaries

Executable binaries are available for download on the [GitHub Releases
page](https://github.com/builditluc/wiki-tui/releases). After downloading the binary for your
platform and extracting the archive, you can run the contained executable `wiki-tui`.

For your convenience, put it into your `PATH`.

## Nix

You can find the package in the [nixpkgs](https://search.nixos.org/packages?channel=unstable&show=wiki-tui&from=0&size=50&sort=relevance&type=packages&query=wiki-tui) repository. Either install it with

```sh
nix-env -iA wiki-tui
```

Get it temporarily to try it out

```sh
nix-shell -p wiki-tui
```

Or add it to your configuration.

## Void

As root, run

```
xbps-install -S wiki-tui
```

with the main repos installed. Works on glibc and musl installations.

## NetBSD

Using the package manager

```sh
pkgin install wiki-tui
```

Building from source

```sh
cd /usr/pkgsrc/www/wiki-tui
make install
```

## FreeBSD

Using the package manager

```sh
pkg install wiki-tui
```

Building from source

```sh
cd /usr/ports/www/wiki-tui/
make install clean
```

## Building from source

To build the `wiki-tui` executable from source, you will first need to install Rust and Cargo.
wiki-tui currently requires at lest Rust version 1.71.1

```sh
cargo install wiki-tui
```

If you want, you can also build the latest development version from GitHub (can be unstable)

```sh
cargo install --git https://github.com/builditluc/wiki-tui
```

To uninstall, run the command `cargo uninstall wiki-tui`.
