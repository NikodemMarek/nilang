# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "errors/default"
    "nilang-types/default"
    "nilang-lexer/default"
    "nilang-parser/default"
    "nilang-generator/default"
    "nilang-runner/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  cargoUnstableFlags ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
  ignoreLockHash,
}:
let
  nixifiedLockHash = "7e6b2e8727645f947caa23b0742facda9ce5307c8bb00c346b623bd9b29b6f45";
  workspaceSrc = if args.workspaceSrc == null then ./. else args.workspaceSrc;
  currentLockHash = builtins.hashFile "sha256" (workspaceSrc + /Cargo.lock);
  lockHashIgnored = if ignoreLockHash
                  then builtins.trace "Ignoring lock hash" ignoreLockHash
                  else ignoreLockHash;
in if !lockHashIgnored && (nixifiedLockHash != currentLockHash) then
  throw ("Cargo.nix ${nixifiedLockHash} is out of sync with Cargo.lock ${currentLockHash}")
else let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts cargoUnstableFlags rustcLinkFlags rustcBuildFlags; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.11.0";
  workspace = {
    errors = rustPackages.unknown.errors."0.1.0";
    nilang-types = rustPackages.unknown.nilang-types."0.1.0";
    nilang-lexer = rustPackages.unknown.nilang-lexer."0.1.0";
    nilang-parser = rustPackages.unknown.nilang-parser."0.1.0";
    nilang-generator = rustPackages.unknown.nilang-generator."0.1.0";
    nilang-runner = rustPackages.unknown.nilang-runner."0.1.0";
  };
  "registry+https://github.com/rust-lang/crates.io-index".colored."2.1.0" = overridableMkRustCrate (profileName: rec {
    name = "colored";
    version = "2.1.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "cbf2150cce219b664a8a70df7a1f933836724b503f8a413af9365b4dcc4d90b8"; };
    dependencies = {
      lazy_static = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.5.0" { inherit profileName; }).out;
      ${ if hostPlatform.isWindows then "windows_sys" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" { inherit profileName; }).out;
    };
  });
  
  "unknown".errors."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "errors";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      colored = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".colored."2.1.0" { inherit profileName; }).out;
      nilang_types = (rustPackages."unknown".nilang-types."0.1.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".eyre."0.6.12" = overridableMkRustCrate (profileName: rec {
    name = "eyre";
    version = "0.6.12";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "7cd915d99f24784cdc19fd37ef22b97e3ff0ae756c7e492e9fbfe897d61e2aec"; };
    features = builtins.concatLists [
      [ "auto-install" ]
      [ "default" ]
      [ "track-caller" ]
    ];
    dependencies = {
      indenter = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".indenter."0.3.3" { inherit profileName; }).out;
      once_cell = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".once_cell."1.20.2" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".indenter."0.3.3" = overridableMkRustCrate (profileName: rec {
    name = "indenter";
    version = "0.3.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ce23b50ad8242c51a442f3ff322d56b02f08852c77e4c0b4d3fd684abc89c683"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.5.0" = overridableMkRustCrate (profileName: rec {
    name = "lazy_static";
    version = "1.5.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bbd2bcb4c963f2ddae06a2efc7e9f3591312473c50c6685e1f298068316e66fe"; };
  });
  
  "unknown".nilang-generator."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-generator";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      errors = (rustPackages."unknown".errors."0.1.0" { inherit profileName; }).out;
      eyre = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".eyre."0.6.12" { inherit profileName; }).out;
      nilang_types = (rustPackages."unknown".nilang-types."0.1.0" { inherit profileName; }).out;
    };
  });
  
  "unknown".nilang-lexer."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-lexer";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      errors = (rustPackages."unknown".errors."0.1.0" { inherit profileName; }).out;
      nilang_types = (rustPackages."unknown".nilang-types."0.1.0" { inherit profileName; }).out;
    };
  });
  
  "unknown".nilang-parser."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-parser";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      errors = (rustPackages."unknown".errors."0.1.0" { inherit profileName; }).out;
      nilang_types = (rustPackages."unknown".nilang-types."0.1.0" { inherit profileName; }).out;
    };
  });
  
  "unknown".nilang-runner."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-runner";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      colored = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".colored."2.1.0" { inherit profileName; }).out;
      errors = (rustPackages."unknown".errors."0.1.0" { inherit profileName; }).out;
      eyre = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".eyre."0.6.12" { inherit profileName; }).out;
      nilang_generator = (rustPackages."unknown".nilang-generator."0.1.0" { inherit profileName; }).out;
      nilang_lexer = (rustPackages."unknown".nilang-lexer."0.1.0" { inherit profileName; }).out;
      nilang_parser = (rustPackages."unknown".nilang-parser."0.1.0" { inherit profileName; }).out;
    };
  });
  
  "unknown".nilang-types."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-types";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".once_cell."1.20.2" = overridableMkRustCrate (profileName: rec {
    name = "once_cell";
    version = "1.20.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "1261fe7e33c73b354eab43b1273a57c8f967d0391e80353e51f764ac02cf6775"; };
    features = builtins.concatLists [
      [ "alloc" ]
      [ "default" ]
      [ "race" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows-sys";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "677d2418bec65e3338edb076e806bc1ec15693c5d0104683f2efe857f61056a9"; };
    features = builtins.concatLists [
      [ "Win32" ]
      [ "Win32_Foundation" ]
      [ "Win32_System" ]
      [ "Win32_System_Console" ]
      [ "default" ]
    ];
    dependencies = {
      windows_targets = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.48.5" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows-targets";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9a2fa6e2155d7247be68c096456083145c183cbbbc2764150dda45a87197940c"; };
    dependencies = {
      ${ if hostPlatform.config == "aarch64-pc-windows-gnullvm" then "windows_aarch64_gnullvm" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.48.5" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.abi.name == "msvc" then "windows_aarch64_msvc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.48.5" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "gnu" then "windows_i686_gnu" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.48.5" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "msvc" then "windows_i686_msvc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.48.5" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.abi.name == "gnu" then "windows_x86_64_gnu" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.48.5" { inherit profileName; }).out;
      ${ if hostPlatform.config == "x86_64-pc-windows-gnullvm" then "windows_x86_64_gnullvm" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.48.5" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.abi.name == "msvc" then "windows_x86_64_msvc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.48.5" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows_aarch64_gnullvm";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "2b38e32f0abccf9987a4e3079dfb67dcd799fb61361e53e2882c3cbaf0d905d8"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows_aarch64_msvc";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "dc35310971f3b2dbbf3f0690a219f40e2d9afcf64f9ab7cc1be722937c26b4bc"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows_i686_gnu";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a75915e7def60c94dcef72200b9a8e58e5091744960da64ec734a6c6e9b3743e"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows_i686_msvc";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8f55c233f70c4b27f66c523580f78f1004e8b5a8b659e05a4eb49d4166cca406"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_gnu";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "53d40abd2583d23e4718fddf1ebec84dbff8381c07cae67ff7768bbf19c6718e"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_gnullvm";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "0b7b52767868a23d5bab768e390dc5f5c55825b6d30b86c844ff2dc7414044cc"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.48.5" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_msvc";
    version = "0.48.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ed94fce61571a4006852b7389a063ab983c02eb1bb37b47f8272ce92d06d9538"; };
  });
  
}
