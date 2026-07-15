#! /usr/bin/env node
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// See `//megasharkLibSource` in `package.json` for why this script exists.
//
// `megashark-lib` is only ever cloned & packed by npm (git dependency), it's never
// re-built in place: `node_modules/megashark-lib` only contains what its `files` field
// allows (`dist/`, `package.json`, ...), not the source needed to run `vite build` again.
// So to get a fresh `dist/`, we have to make npm redo the clone+prepare+pack itself,
// this time with scripts allowed.

const { exit, platform } = require('node:process');
const { spawnSync } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');

const WORKDIR = path.join(__dirname, '..');
const packageJson = require(path.join(WORKDIR, 'package.json'));

const source = packageJson.megasharkLibSource;
if (!source) {
  console.error('Could not find `megasharkLibSource` in `package.json`');
  exit(1);
}

const distDir = path.join(WORKDIR, 'node_modules', 'megashark-lib', 'dist');
const markerFile = path.join(WORKDIR, 'node_modules', 'megashark-lib', '.built-from-source');

if (fs.existsSync(distDir) && fs.existsSync(markerFile) && fs.readFileSync(markerFile, 'utf-8').trim() === source) {
  console.log(`megashark-lib is already built from ${source}, skipping`);
  exit(0);
}

let cmdPrefix = 'npm';
if (platform === 'win32') {
  cmdPrefix = 'npm.cmd';
}
const cmdArgs = ['install', `megashark-lib@${source}`, '--no-ignore-scripts', '--no-save'];

console.log('>>> ', cmdPrefix, cmdArgs.join(' '));
const ret = spawnSync(cmdPrefix, cmdArgs, {
  stdio: ['inherit', 'inherit', 'inherit'],
  cwd: WORKDIR,
  // Recent versions of node (since >= 20) now require to set `shell: true` when executing batch script.
  // eslint-disable-next-line max-len
  // https://nodejs.org/en/blog/vulnerability/april-2024-security-releases-2#command-injection-via-args-parameter-of-child_processspawn-without-shell-option-enabled-on-windows-cve-2024-27980---high
  shell: platform === 'win32',
});
if (ret.status !== 0) {
  exit(ret.status);
}

fs.writeFileSync(markerFile, source);
