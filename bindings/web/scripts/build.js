#! /usr/bin/env node

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
  const ret = spawnSync(
    on_windows() ? "wasm-pack.exe" : "wasm-pack",
    ["--version"],
    {
      // ignore stdin & stdout, stderr to actual stderr
      stdio: ['ignore', 'ignore', 'inherit'],
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
    on_windows() ? "rustup.exe" : "rustup",
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
    install_rust_target(TARGET)
  }
}

function install_rust_target(target) {
  const rustup_install_target_args = [
    on_windows() ? "rustup.exe" : "rustup",
    "target",
    "install",
    target
  ];

  console.log(`Installing rust target ${target}`);
  const ret = exec_cmd(rustup_install_target_args, {
    // ignore stdin, stdout in pipe, stderr to actual stderr
    stdio: ['ignore', 'pipe', 'inherit'],
    cwd: WORKDIR,
    env: process.env
  });
}

function build_wasm(cargo_flags) {
  // On Windows only .exe/.bat can be directly execute
  const wasm_build_args = [
    on_windows() ? "wasm-pack.exe" : "wasm-pack",
    "build",
    "--target=web",
    "--locked",
    ...cargo_flags,
    // If we don't force the target dir, the default one is used (i.e. `target/<profile>`)
    // However this clashes with native platform builds (i.e. `cargo build`): both
    // builds uses the same target dir and overwrite each other work !
    "--target-dir",
    path.resolve(ROOT_DIR, "target", TARGET),
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

// Client GUI only separates between `debug` and `release`, hence it won't
// find the `ci-rust` build.
// So the solution is to symlink `ci-rust` as `debug`.
switch (profile) {
  case "ci":
    const source = path.resolve(ROOT_DIR, "target", TARGET, "ci-rust");
    const dest = path.resolve(ROOT_DIR, "target", TARGET, "debug");
    console.log(`Create symlink ${source} => ${dest}`);
    fs.symlinkSync(source, dest, 'dir');
}

// Actually do the compilation
build_wasm(cargo_flags);
