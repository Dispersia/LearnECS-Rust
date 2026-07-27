{
  description = "Bevy development environment (Rust + Android SDK/NDK)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            allowUnfree = true;
            android_sdk.accept_license = true;
          };
        };

        buildToolsVersion = "34.0.0";
        platformVersion = "34";      # SDK platform to compile against
        ndkVersion = "26.3.11579264";

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [
            "aarch64-linux-android"
            "armv7-linux-androideabi"
            "x86_64-linux-android"
            "i686-linux-android"
            "wasm32-unknown-unknown"
          ];
        };

        androidComposition = pkgs.androidenv.composeAndroidPackages {
          platformToolsVersion = "35.0.2";
          buildToolsVersions = [ buildToolsVersion ];
          platformVersions = [ platformVersion "33" ];
          includeNDK = true;
          ndkVersions = [ ndkVersion ];
          includeEmulator = false;
          includeSystemImages = false;
        };
        androidSdk = androidComposition.androidsdk;
        androidSdkRoot = "${androidSdk}/libexec/android-sdk";
        androidNdkRoot = "${androidSdkRoot}/ndk/${ndkVersion}";

        buildInputs = with pkgs; [
          alsa-lib
          udev
          libx11
          libxcursor
          libxi
          libxrandr
          wayland
          libxkbcommon
          vulkan-loader
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          clang
          mold
        ];

        runtimeLibs = with pkgs; [
          vulkan-loader
          wayland
          libxkbcommon
          libx11
          libxcursor
          libxi
          libxrandr
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          nativeBuildInputs = nativeBuildInputs ++ [
            rustToolchain
            androidSdk
            pkgs.jdk21
            pkgs.android-tools
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;

          ANDROID_HOME = androidSdkRoot;
          ANDROID_SDK_ROOT = androidSdkRoot;
          ANDROID_NDK_ROOT = androidNdkRoot;
          ANDROID_NDK_HOME = androidNdkRoot;

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
          '';
        };
      });
}
