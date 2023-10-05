{
  pkgs ?
    import <nixpkgs> {
      overlays = [(import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))];
    },
}: let
  rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  # bazel = pkgs.bazel.overrideAttrs (_: rec {
  #   version = "6.3.0";
  #   src = builtins.fetchurl {
  #     url = "https://github.com/bazelbuild/bazel/releases/download/6.3.0/bazel-6.3.0-dist.zip";
  #     sha256 = "sha256:1q7d0sx43l0ravwcm52mz71p55ry3fgyf4srq0pi29hx3fc9h8ch";
  #   };
  # });
in
  (pkgs.buildFHSUserEnv {
    name = "charted-server";
    targetPkgs = pkgs:
      with pkgs; [
        # ~ node ~
        nodePackages.pnpm
        nodejs_20

        cargo-expand
        pkg-config
        clang_16
        openssl
        bazel_6
        lld_16
        glibc
        zlib
        rust
        mold
        gcc
      ];
  })
  .env
