{ sources ? import ./nix/sources.nix }:
let
  nixpkgs = import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  rust = (nixpkgs.rustChannelOf { channel = "nightly"; }).rust;
  rustPlatform = nixpkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
in
rustPlatform.buildRustPackage rec {
  name = "lines-are-rusty";
  src = builtins.path { inherit name; path = ./.; };
  cargoSha256 = "0b3a8q497bjv2rrb83mxf59c00mgd89wy4g10yabibbdlnsq5gqg";
}
