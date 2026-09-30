#! /usr/bin/env node
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const fs = require('fs');
const path = require('path');

const WORKDIR = path.join(__dirname, '..');
const SRC = path.join(WORKDIR, 'dist');
const DEST = path.join(WORKDIR, 'electron', 'app');
const EDITICS_DEST = path.join(WORKDIR, 'electron', 'app-editics');
// Served by Electron from their own custom scheme (i.e. a different origin than
// the app, see `electron/src/setup.ts`), so they are kept out of `app`.
// In the dist folder, everything editics-related lives in `editics/` (including
// the OnlyOffice & x2t).
const EDITICS_SRC_DIR = path.join(SRC, 'editics');

if (!fs.existsSync(SRC)) {
  console.error(`Source directory not found: ${SRC}`);
  console.error('Run `npm run native:build` first');
  process.exit(1);
}

for (const dest of [DEST, EDITICS_DEST]) {
  if (fs.existsSync(dest)) {
    console.log(`>>> rm -rf ${path.relative(WORKDIR, dest)}`);
    fs.rmSync(dest, { recursive: true });
  }
}

console.log(`>>> cp -r ${path.relative(WORKDIR, SRC)}/!(editics) ${path.relative(WORKDIR, DEST)}`);
fs.cpSync(SRC, DEST, {
  recursive: true,
  filter: (src) => path.relative(SRC, src).split(path.sep)[0] !== 'editics',
});

// Not built when editics is disabled (e.g. hardened builds)
if (fs.existsSync(EDITICS_SRC_DIR)) {
  console.log(`>>> cp -r ${path.relative(WORKDIR, EDITICS_SRC_DIR)} ${path.relative(WORKDIR, EDITICS_DEST)}`);
  fs.cpSync(EDITICS_SRC_DIR, path.join(EDITICS_DEST, 'editics'), { recursive: true });
}
console.log('Done!');
