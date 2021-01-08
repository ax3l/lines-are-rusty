let
  base-nixpkgs = import <nixpkgs> {};
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust = (nixpkgs.rustChannelOf { channel = "nightly"; }).rust;
  rustPlatform = nixpkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
in

rustPlatform.buildRustPackage {
  name = "lines-are-rusty";
  src = ./.;
  cargoSha256 = "0bnj1id4vbb7rhsrhnalnvh35kf04bxvhwdn051xnzp6kdi57imm";
}
