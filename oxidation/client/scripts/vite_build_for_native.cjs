#! /usr/bin/env node

const { exit, platform } = require('node:process');
const { spawnSync } = require('node:child_process');
const path = require('path');

const WORKDIR = path.join(__dirname, '..');

let cmdPrefix = 'npm';
if (platform === 'win32') {
  cmdPrefix = 'npm.cmd';
}
const cmdArgs = ['exec', '--', 'vite', 'build'];

console.log('>>> ', cmdPrefix, cmdArgs.join(' '));
const ret = spawnSync(
  cmdPrefix,
  cmdArgs,
  {
    stdio: ['inherit', 'inherit', 'inherit'],
    cwd: WORKDIR,
    env: {
      ...process.env,
      // Here is the secret sauce !
      PLATFORM: 'native'
    }
  }
);
if (ret.status !== 0) {
  exit(ret.status);
}
