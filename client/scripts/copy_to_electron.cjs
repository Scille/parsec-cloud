#! /usr/bin/env node
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const fs = require('fs');
const path = require('path');

const WORKDIR = path.join(__dirname, '..');
const SRC = path.join(WORKDIR, 'dist');
const DEST = path.join(WORKDIR, 'electron', 'app');

if (!fs.existsSync(SRC)) {
  console.error(`Source directory not found: ${SRC}`);
  console.error('Run `npm run native:build` first');
  process.exit(1);
}

if (fs.existsSync(DEST)) {
  console.log(`>>> rm -rf ${path.relative(WORKDIR, DEST)}`);
  fs.rmSync(DEST, { recursive: true });
}

console.log(`>>> cp -r ${path.relative(WORKDIR, SRC)} ${path.relative(WORKDIR, DEST)}`);
fs.cpSync(SRC, DEST, { recursive: true });
console.log('Done!');
