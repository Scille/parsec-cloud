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
// The folder names must be preserved: `editics/offline.html` loads the others
// with relative URLs (e.g. `../onlyoffice/...`).
const EDITICS_DIRS = ['editics', 'onlyoffice', 'onlyoffice-x2t', 'onlyoffice-templates'];

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

console.log(`>>> cp -r ${path.relative(WORKDIR, SRC)} ${path.relative(WORKDIR, DEST)} (without editics)`);
fs.cpSync(SRC, DEST, {
  recursive: true,
  filter: (src) => !EDITICS_DIRS.includes(path.relative(SRC, src).split(path.sep)[0]),
});

for (const dir of EDITICS_DIRS) {
  const src = path.join(SRC, dir);
  // Not built when editics is disabled (e.g. hardened builds)
  if (fs.existsSync(src)) {
    const dest = path.join(EDITICS_DEST, dir);
    console.log(`>>> cp -r ${path.relative(WORKDIR, src)} ${path.relative(WORKDIR, dest)}`);
    fs.cpSync(src, dest, { recursive: true });
  }
}
console.log('Done!');
