# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "nilang-generator/default"
    "nilang-parser/default"
    "nilang-lexer/default"
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
  nixifiedLockHash = "ce7806ed93dec02d422f995ba3c129135a69fc8f23cdf0929725eec7d4090254";
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
    nilang-generator = rustPackages.unknown.nilang-generator."0.1.0";
    nilang-parser = rustPackages.unknown.nilang-parser."0.1.0";
    nilang-lexer = rustPackages.unknown.nilang-lexer."0.1.0";
    nilang-runner = rustPackages.unknown.nilang-runner."0.1.0";
  };
  "unknown".nilang-generator."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-generator";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      nilang_parser = (rustPackages."unknown".nilang-parser."0.1.0" { inherit profileName; }).out;
    };
  });
  
  "unknown".nilang-lexer."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-lexer";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
  });
  
  "unknown".nilang-parser."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-parser";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      nilang_lexer = (rustPackages."unknown".nilang-lexer."0.1.0" { inherit profileName; }).out;
    };
  });
  
  "unknown".nilang-runner."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "nilang-runner";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      nilang_generator = (rustPackages."unknown".nilang-generator."0.1.0" { inherit profileName; }).out;
      nilang_lexer = (rustPackages."unknown".nilang-lexer."0.1.0" { inherit profileName; }).out;
      nilang_parser = (rustPackages."unknown".nilang-parser."0.1.0" { inherit profileName; }).out;
    };
  });
  
}
