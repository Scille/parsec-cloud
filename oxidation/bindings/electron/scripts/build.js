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
    `electron-${profile}-libparsec-cargo-flags`,
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
const cmd2_cmd = "npx";
const cmd2_args = [
    "cargo-cp-artifact",
    "--npm",
    "cdylib",
    "dist/libparsec/index.node",
    "--",
    "cargo",
    "build",
    "--message-format=json-render-diagnostics"
].concat(cargo_flags);
console.log(">>> ", cmd2_cmd, cmd2_args.join(" "));
const ret2 = spawnSync(
    cmd2_cmd,
    cmd2_args,
    {
        stdio: 'inherit',
        cwd: WORKDIR,
        // Unlike python, npx command is a bash script. On Windows only .exe/.bat
        // can be directly executed so to pass it to a shell instead.
        shell: platform == "win32",
    },
);
if (ret2.status != 0) {
    exit(ret2.status);
}
