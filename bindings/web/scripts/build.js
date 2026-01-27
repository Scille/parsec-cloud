#! /usr/bin/env node
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


const { argv, exit, platform } = require('node:process');
const { spawnSync } = require('node:child_process');
const path = require("path");
const fs = require('node:fs');

const DEFAULT_PROFILE = "release";
const ROOT_DIR = path.resolve(__dirname, "../../..");
const WORKDIR = path.resolve(__dirname, '..');
const TARGET = "wasm32-unknown-unknown";

switch (argv.length) {
  // argv[0] is node, argv[1] is build.js
  case 2:
    profile = DEFAULT_PROFILE;
    break;
  case 3:
    profile = argv[2];
    break;

  default:
    console.log("Usage: build.js [profile]");
    exit(1);
}

console.log(`Current working dir ${WORKDIR}`);

function on_windows() {
  return platform == "win32";
}
const WASM_PACK_CMD = on_windows() ? "wasm-pack.exe" : "wasm-pack";
const RUSTUP_CMD = on_windows() ? "rustup.exe" : "rustup";

function exec_cmd(args, options) {
  console.log(">>> ", args.join(" "));

  ret = spawnSync(args[0], args.slice(1), options);

  if (ret.error) {
    console.log(`The command ${args.join(" ")} couldn't run: ${ret.error}`);
    exit(1);
  }
  if (ret.status) {
    console.log(`The command ${args.join(" ")} failed with status ${ret.status}`);
    exit(1);
  }

  return ret;
}


// Fetch Cargo compile flags
function get_cargo_flag(profile) {
  const cargo_flag_args = [
    "python",
    path.resolve(ROOT_DIR, "make.py"),
    `web-${profile}-libparsec-cargo-flags`,
    "--quiet",
  ];

  let ret = exec_cmd(cargo_flag_args, {
    // ignore stdin, stdout in pipe, stderr to actual stderr
    stdio: ['ignore', 'pipe', 'inherit'],
    cwd: WORKDIR,
    env: process.env
  });

  return ret.stdout.toString(encoding = "ascii").trim().split(" ");
}

function ensure_wasm_pack_installed() {
  const ret = exec_cmd(
    [WASM_PACK_CMD, "--version"],
    {
      // ignore stdin, stdout and stderr to their respective outputs
      stdio: ['ignore', 'inherit', 'inherit'],
      cwd: WORKDIR,
      env: process.env
    }
  );

  if (ret.error !== undefined) {
    console.log("Command `wasm-pack` is not installed !")
    console.log("Install it with:")
    console.log("  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh")
    exit(1);
  }
}

function ensure_rust_target_installed() {
  const rustup_installed_targets_args = [
    RUSTUP_CMD,
    "target",
    "list",
    "--installed",
  ];

  const ret = exec_cmd(rustup_installed_targets_args, {
    // ignore stdin, stdout in pipe, stderr to actual stderr
    stdio: ['ignore', 'pipe', 'inherit'],
    cwd: WORKDIR,
    env: process.env
  });

  const targets = ret.stdout.toString(encoding = "ascii").trim().split("\n");
  if (targets.find((e) => e === TARGET) === undefined) {
    console.log(`Rust target \`${TARGET}\` is not installed !`)
    console.log("Install it with:")
    console.log(`  rustup target add ${TARGET}`)
    exit(1);
  }
}

// Note the build output will be split into two parts:
// - `target/wasm32-unknown-unknown/<profile>`: actual wasm32 code
// - `target/<profile>`: native build of the proc macros used during compilation
function build_wasm(cargo_flags) {
  // On Windows only .exe/.bat can be directly execute
  const wasm_build_args = [
    WASM_PACK_CMD,
    "build",
    "--target=web",
    "--locked",
    ...cargo_flags,
  ];

  exec_cmd(
    wasm_build_args,
    {
      stdio: 'inherit',
      cwd: WORKDIR,
      env: process.env
    },
  );
}

ensure_rust_target_installed();
ensure_wasm_pack_installed();

const cargo_flags = get_cargo_flag(profile);

// wasm-pack only separates between `debug` and `release` target folder (depending
// of the presence of `--dev` parameter), hence it won't find the target folder
// for build using the CI profile.
// So the solution is to symlink the CI profile folder as the `debug` one.
// Note this means both CI and regular dev profile will build in the `debug`
// folder. This should be okay since 1) on the CI only the CI profile is used and
// 2) worst case is more code is going to be recompiled when switching between
// the two profiles which is no big deal.
switch (profile) {
  case "ci":
    const ci_profile = "ci-bindings"

    // Sanity check on the expected name of the profile
    if (cargo_flags.find((e) => e == `--profile=${ci_profile}`) == undefined) {
      console.log(`Expected to find argument \`--profile=${ci_profile}\` in the cargo flags (i.e. \`${cargo_flags.join(" ")}\`)`)
      exit(1);
    }

    const ci_target_dir = path.resolve(ROOT_DIR, "target", TARGET)
    const ci_source_dir = path.resolve(ci_target_dir, ci_profile);
    const ci_dest_dir_name = "debug"
    const ci_dest_dir = path.resolve(ci_target_dir, ci_dest_dir_name);

    let ci_source_dir_resolved = undefined
    try {
      ci_source_dir_resolved = fs.realpathSync(ci_source_dir);
    } catch {
      // Source doesn't exist yet
    }

    if (ci_source_dir_resolved != ci_dest_dir) {
      if (ci_source_dir_resolved != undefined) {
        console.log(`Remove invalid entry ${ci_source_dir}`);
        fs.rmSync(ci_source_dir, { force: true, recursive: true });
      }
      console.log(`Create symlink ${ci_source_dir} => ${ci_dest_dir}`);
      fs.mkdirSync(ci_dest_dir, { recursive: true });
      fs.symlinkSync(ci_dest_dir_name, ci_source_dir);
    }

    break;
}

// Actually do the compilation
build_wasm(cargo_flags);
