#! /usr/bin/env node
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const { argv, exit, platform } = require('node:process');
const { spawn, spawnSync } = require('node:child_process');
const fs = require("node:fs");
const fsPromise = require("node:fs/promises");
const path = require("path");

const DEFAULT_PROFILE = "release";
const WORKDIR = path.join(__dirname, "..");
const OUTPUT_DIR = 'dist'
const BUILD_LOG = OUTPUT_DIR + '/cargo-build-log.json';

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

/**
  * @param {string[]} args
  * @param {SpawnSyncOptionsWithStringEncoding} options
  * @returns {SpawnSyncReturns<Buffer>}
  */
function exec_cmd(args, options) {
  const command = args.join(" ");
  console.log(">>> ", command);

  ret = spawnSync(args[0], args.slice(1), { stdio: 'inherit', cwd: WORKDIR, ...options });

  handle_cmd_status(command, ret)

  return ret;
}

/**
  * @param {string} name
  * @param {SpawnSyncReturns<Buffer>} ret
  */
function handle_cmd_status(name, ret) {
  if (ret.status !== 0) {
    console.error(`The following command as fail: ${name}`);
    console.error(`  status: ${ret.status}`);
    console.error(`  signal: ${ret.signal}`);
    console.error(`  error: ${ret.error}`);
    exit(1)
  }
}

// Fetch Cargo compile flags
function fetch_cargo_flags() {
  const ARGS = [
    'python',
    path.join(__dirname, "../../../make.py"),
    `electron-${profile}-libparsec-cargo-flags`,
    "--quiet",
  ];

  let ret = exec_cmd(ARGS, {
    // ignore stdin, stdout in pipe, stderr to actual stderr
    stdio: ['ignore', 'pipe', 'inherit'],
  });

  return ret.stdout.toString(encoding = "ascii").trim().split(" ");
}

async function build_electron_bindings(cargo_flags) {
  const CARGO_ARGS = ['cargo', 'build', '--locked', '--verbose', '--message-format=json-render-diagnostics', '--package', 'libparsec_bindings_electron', ...cargo_flags];
  const log = await new Promise((resolve) => {
    const stream = fs.createWriteStream(BUILD_LOG);
    stream.on('ready', () => resolve(stream));
  });

  exec_cmd(
    CARGO_ARGS,
    {
      stdio: ['inherit', log, 'inherit'],
    }
  );
}

function process_rust_lib() {
  const NEON_ARGS = [
    // On Windows only .exe/.bat can be directly executed, `npx.cmd` is the bat version of `npx`
    on_windows() ? "npx.cmd" : "npx",
    'neon', 'dist', '--verbose', '--log', BUILD_LOG
  ];

  exec_cmd(
    NEON_ARGS,
    {
      env: {
        ...process.env,
        NEON_DIST_OUTPUT: 'dist/libparsec.node',
      },
    }
  );
}

async function main() {
  const cargo_flags = fetch_cargo_flags();

  await fsPromise.mkdir(OUTPUT_DIR, { recursive: true });
  // Actually do the compilation
  await build_electron_bindings(cargo_flags);
  process_rust_lib();
  await fsPromise.cp('src/index.d.ts', OUTPUT_DIR + '/libparsec.d.ts');
}

main()
