#! /usr/bin/env node

const fs = require('fs');
const path = require('path');

const WORKDIR = path.join(__dirname, '..');
const SRCDIR = path.join(WORKDIR, '../../bindings/electron/dist/libparsec');
const DSTDIR = path.join(WORKDIR, 'build');

function copy(src, dst) {
  if (!fs.existsSync(src)) {
    throw new Error(`*** Can't find \`${src}\`, run \`./make.py electron-dev-install\` first ***`);
  }

  const dstParent = path.dirname(dst);
  console.log(`>>> mkdir -p ${path.relative(WORKDIR, dstParent)}`);
  fs.mkdirSync(dstParent, {recursive: true});
  console.log(`>>> cp ${path.relative(WORKDIR, src)} ${path.relative(WORKDIR, dst)}`);
  fs.copyFileSync(src, dst);
}

copy(
  path.join(SRCDIR, 'index.node'),
  path.join(DSTDIR, 'src/libparsec/index.node')
);
copy(
  path.join(SRCDIR, 'index.d.ts'),
  path.join(DSTDIR, 'generated-ts/src/libparsec.d.ts')
);
