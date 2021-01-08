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
  cargoSha256 = "1yhqq9n0qi0d5mj7d701qag1i95vpyy46g5av2v1ail36pqas2gf";
}
