#! /usr/bin/env node
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const { argv, exit, platform } = require('node:process');
const { spawnSync } = require('node:child_process');
const fs = require("fs");
const path = require("path");

const DEFAULT_PROFILE = "release";
const WORKDIR = path.join(__dirname, "..");

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
  return platform == 'win32';
}

function exec_cmd(args, options) {
  console.log(">>> ", args.join(" "));

  ret = spawnSync(args[0], args.slice(1), options);

  if (ret.signal != null) {
    console.error(`The command ${args.join(" ")} received the signal ${ret.signal}`);
    exit(1);
  }

  return ret;
}

// This function is here to detect if `python3` is present or if we have to fallback to `python`.
// We prefer to use `python3` since we're sure it's `python-3.x` where `python` can be `python-2.x`.
function get_python() {
  ret = exec_cmd(["python3", "--version"]);

  if (ret.status == 0) {
    return "python3";
  } else {
    console.log("`python3` wasn't found, fallback to `python`");
    return "python";
  }
}

// Fetch Cargo compile flags
function fetch_cargo_flags() {
  const PYTHON = get_python();
  const ARGS = [
    PYTHON,
    path.join(__dirname, "../../../make.py"),
    `electron-${profile}-libparsec-cargo-flags`,
    "--quiet",
  ];

  ret = exec_cmd(ARGS, {
    // ignore stdin, stdout in pipe, stderr to actual stderr
    stdio: ['ignore', 'pipe', 'inherit'],
    cwd: WORKDIR,
  });

  if (ret.status != 0) {
    console.log("stdout:", ret.stdout.toString());
    exit(ret.status);
  }

  return ret.stdout.toString(encoding = "ascii").trim().split(" ");
}

function build_electron_bindings(cargo_flags) {
  // On Windows only .exe/.bat can be directly executed, `npx.cmd` is the bat version of `npx`
  const ARGS = [
    on_windows() ? "npx.cmd" : "npx",
    "cargo-cp-artifact",
    "--npm",
    "cdylib",
    "dist/libparsec/index.node",
    "--",
    "cargo",
    "build",
    "--locked",
    "--message-format=json-render-diagnostics",
    ...cargo_flags
  ];

  ret2 = exec_cmd(ARGS,
    {
      stdio: 'inherit',
      cwd: WORKDIR,
    },
  );

  if (ret2.status != 0) {
    exit(ret2.status);
  }
}

const cargo_flags = fetch_cargo_flags();

// Actually do the compilation
build_electron_bindings(cargo_flags);


// Finally, copy the typing info
fs.copyFileSync('src/index.d.ts', 'dist/libparsec/index.d.ts');
