#! /usr/bin/env node

const { exit, platform } = require('node:process');
const { spawnSync } = require('node:child_process');
const path = require('path');

const WORKDIR = path.join(__dirname, '..');

let cmdPrefix = 'npm';
if (platform === 'win32') {
  cmdPrefix = 'npm.cmd';
}
const cmdArgs = ['exec', '--', 'vite', 'build', ...process.argv.slice(2)];

console.log('>>> ', cmdPrefix, cmdArgs.join(' '));
const ret = spawnSync(cmdPrefix, cmdArgs, {
  stdio: ['inherit', 'inherit', 'inherit'],
  cwd: WORKDIR,
  env: {
    ...process.env,
    // Here is the secret sauce !
    PLATFORM: 'native',
  },
  // Recent versions of node (since >= 20) now require to set `shell: true` when executing batch script.
  // eslint-disable-next-line max-len
  // https://nodejs.org/en/blog/vulnerability/april-2024-security-releases-2#command-injection-via-args-parameter-of-child_processspawn-without-shell-option-enabled-on-windows-cve-2024-27980---high
  shell: platform === 'win32',
});
if (ret.status !== 0) {
  exit(ret.status);
}
