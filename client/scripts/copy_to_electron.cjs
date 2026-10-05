#! /usr/bin/env node
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const fs = require('fs');
const path = require('path');
const zlib = require('zlib');

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

  console.log(`>>> compress with brotli ${path.relative(WORKDIR, EDITICS_DEST)}`);
  compressWithBrotli(EDITICS_DEST);
}
console.log('Done!');

// OnlyOffice is huge (hundreds of MB, mostly text), and the asar archive is not
// compressed. So each file `X` worth compressing is replaced by `X.br`, which is
// decompressed on the fly by Electron (see `electron/src/serveDirectory.ts`).
// Note OnlyOffice already ships `.br` files for most of its assets.
function compressWithBrotli(dir) {
  const COMPRESSIBLE = /\.(js|css|html|svg|json|wasm|ttf|dic|aff|dat|bin|idx|txt)$/;
  const MIN_SIZE = 64 * 1024; // files of at least 64KB

  const files = fs
    .readdirSync(dir, { recursive: true })
    .map((file) => path.join(dir, file))
    .filter((file) => fs.statSync(file).isFile());

  for (const file of files) {
    const size = fs.statSync(file).size;
    const compressedFile = `${file}.br`;

    if (COMPRESSIBLE.test(file) && size >= MIN_SIZE) {
      const compressed = zlib.brotliCompressSync(fs.readFileSync(file), {
        params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 9, [zlib.constants.BROTLI_PARAM_SIZE_HINT]: size },
      });
      fs.writeFileSync(compressedFile, compressed);
      fs.rmSync(file);
    }
  }
}
