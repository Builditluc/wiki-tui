{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ libiconv rustup ];

    buildInputs = with pkgs; [ openssl ncurses pkg-config ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
