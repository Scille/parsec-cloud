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

function exec_cmd(args, options) {
  console.log(">>> ", args.join(" "));

  ret = spawnSync(args[0], args.slice(1), options);

  if (ret.signal != null) {
    console.log(`The command ${args.join(" ")} receive the signal ${ret.signal}`);
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

  const ret = exec_cmd(cargo_flag_args, {
    // ignore stdin, stdout in pipe, stderr to actual stderr
    stdio: ['ignore', 'pipe', 'inherit'],
    cwd: WORKDIR,
    env: process.env
  });

  if (ret.status != 0) {
    console.log("stdout:", ret.stdout.toString());
    exit(ret.status);
  }
  return ret.stdout.toString(encoding = "ascii").trim().split(" ");
}

function ensure_rust_target_installed() {

  const rustup_target_args = [
    on_windows() ? "rustup.exe" : "rustup",
    "target",
    "add",
    TARGET
  ];

  const ret = exec_cmd(rustup_target_args, { stdio: 'inherit', cwd: WORKDIR, env: process.env });

  if (ret.status != 0) {
    console.log("Failed too add rust wasm target :", ret.stdout());
    exit(ret.status);
  }
}

function hack_wasm_pack_folder(source, dest) {
  source = path.resolve(ROOT_DIR, "target", TARGET, source);
  dest = path.resolve(ROOT_DIR, "target", TARGET, dest);

  function create_link() {
    console.log(`Create a link between ${source} => ${dest}`);
    fs.symlinkSync(source, dest, 'dir');
  }

  function remove_file(file) {
    console.log(`Removing ${dest}`);
    fs.rmSync(dest, { force: true, recursive: true });
  }

  try {
    fs.accessSync(dest, fs.constants.F_OK);
  } catch {
    console.log("Link is missing, will create it...");
    create_link();
  } finally {
    let stat = fs.lstatSync(dest);
    if (!stat.isSymbolicLink()) {
      console.log("Dest isn't a symlink, removing it...");
      remove_file(dest);
    }
    else {
      let link_source = fs.readlinkSync(dest);
      if (link_source != source) {
        console.log("Dest is a symlink but with the wrong source, removing it...");
        remove_file(dest);
      } else {
        console.log(`Symlink ${dest} already point to ${link_source}`);
        return;
      }
      create_link();
    }
  }
}

function build_wasm(cargo_flags) {
  // On Windows only .exe/.bat can be directly execute
  const wasm_build_args = [
    on_windows() ? "wasm-pack.exe" : "wasm-pack",
    "build",
    "--target=web",
    "--locked",
    ...cargo_flags
  ];

  const ret = exec_cmd(
    wasm_build_args,
    {
      stdio: 'inherit',
      cwd: WORKDIR,
      env: process.env
    },
  );
  return ret;
}


const cargo_flags = get_cargo_flag(profile);

ensure_rust_target_installed();

switch (profile) {
  case "ci":
    hack_wasm_pack_folder("ci-rust", "debug");
}

// Actually do the compilation
const build_res = build_wasm(cargo_flags);
if (build_res.status != 0) {
  exit(build_res.status);
}
