let
   rustOverlay = import ("${builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz}/rust-overlay.nix");

   pkgs = import <nixpkgs> { overlays =  [rustOverlay]; };

   channel = pkgs.rustChannelOf {
     date = "2019-09-30";
     channel = "nightly";
   };

   rust = (channel.rust.override {
     targets = [ "thumbv7em-none-eabihf" ];
   });
in
with pkgs;

  stdenv.mkDerivation {
    name = "rust-env";

    buildInputs = [
      rust 
      gcc-arm-embedded 
      openocd
      ];

   OCD_SCRIPT = "${openocd}/share/openocd/scripts/board/ek-lm4f120xl.cfg";
   RUST_BACKTRACE = 1;
}

