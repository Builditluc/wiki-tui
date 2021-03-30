{ pkgs ? import <nixpkgs> {}  }:
pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo gcc pkg-config ];
          
          buildInputs = with pkgs; [ openssl ncurses ];
   }
