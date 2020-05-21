with import <nixpkgs> {};

runCommand "casync-webdav" {

  buildInputs = [
    openssl
    pkgconfig
    rustup
  ];
} ""
