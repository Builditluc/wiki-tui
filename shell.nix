{ 
  pkgs ? import <nixpkgs> { },
}:
let
  overrides = builtins.readFile ./rust-toolchain;
in
pkgs.callPackage (
  { stdenv, mkShell, rustup, rustPlatform, }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      rustup
      rustPlatform.bindgenHook
    ];
    buildInputs = with pkgs; [
      openssl
      pkg-config
    ] ++ lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.Security
    ];
    packages = with pkgs; [ gdb ];
    RUSTC_VERSION = overrides;

    # https://github.com/rust-lang/rust-bindgen#environment-variables
    shellHook = ''
      export PATH="''${CARGO_HOME:-~/.cargo}/bin}":"$PATH"
      export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
    '';
  }
) { }
