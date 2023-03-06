#! /usr/bin/env node

const { argv, exit, platform } = require('node:process');
const { spawnSync } = require('node:child_process');
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

// Fetch Cargo compile flags

const cmd1_cmd = "python";
const cmd1_args = [
    path.join(__dirname, "../../../../make.py"),
    `web-${profile}-libparsec-cargo-flags`,
    "--quiet",
]
console.log(">>> ", cmd1_cmd, cmd1_args.join(" "));
const ret1 = spawnSync(
    cmd1_cmd,
    cmd1_args,
    {
        // ignore stdin, stdout in pipe, stderr to actual stderr
        stdio: ['ignore', 'pipe', 'inherit'],
        cwd: WORKDIR,
    }
);
if (ret1.status != 0) {
    console.log("stdout:", ret1.stdout.toString());
    exit(ret1.status);
}
const cargo_flags = ret1.stdout.toString(encoding="ascii").trim().split(" ");

// Actually do the compilation

// On Windows only .exe/.bat can be directly executed, `npx.cmd` is the bat version of `npx`
const cmd2_cmd = platform == "win32" ? "npx.cmd": "npx";
const cmd2_args = [
    "wasm-pack",
    "build",
    "--target",
    "web",
    // Small hack here: we always pass the `--dev` flag to wasm-pack given it
    // considers there is no need to pass extra args to cargo (i.e. cargo build
    // for dev by default). This way we can pass our own `--profile=foo` extra args
    // without cargo complaining it is clashing with `--release` (provided by wasm-pack, and
    // equivalent to `--profile=realese`).
    // This should be safe given if anything change, cargo won't allow multiple profiles
    // to be passed (hence this script will simply fail).
    // cf. https://github.com/rustwasm/wasm-pack/blob/b4e619c8a13a8441b804895348afbfd4fb1a68a3/src/build/mod.rs#L91-L106
    // and https://github.com/rustwasm/wasm-pack/blob/b4e619c8a13a8441b804895348afbfd4fb1a68a3/src/command/build.rs#L220
    "--dev",
    "--",
].concat(cargo_flags);
console.log(">>> ", cmd2_cmd, cmd2_args.join(" "));
const ret2 = spawnSync(
    cmd2_cmd,
    cmd2_args,
    {
        stdio: 'inherit',
        cwd: WORKDIR,
    },
);
if (ret2.status != 0) {
    exit(ret2.status);
}
