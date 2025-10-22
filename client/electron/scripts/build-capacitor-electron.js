// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */
const esbuild = require('esbuild');
const { readdirSync } = require('fs');
const { join } = require('path');
const tar = require('tar');

async function packTemplate() {
  const templateSrc = join('./', 'capacitor-electron', 'electron-platform-template');
  const destTemplateFilePath = join('./', 'template.tar.gz');
  const files = [];
  readdirSync(templateSrc).forEach((file) => {
    files.push(file);
  });
  await tar.create({ gzip: true, file: destTemplateFilePath, cwd: templateSrc }, files);
  console.log(`Packed ${destTemplateFilePath}!`);
}

async function buildCliScripts() {
  console.log('Building capacitor-electron CLI scripts');
  await esbuild.build({
    entryPoints: ['capacitor-electron/cli-scripts/index.ts'],
    bundle: true,
    outfile: 'dist/cli-scripts/cap-scripts.js',
    platform: 'node',
    target: 'node16',
    minify: true,
    external: ['child_process', 'fs', 'path', 'fs-extra', 'crypto', 'chalk', 'ora'],
  });
}

async function buildPlatformCore() {
  console.log('Building capacitor-electron Platform Code');
  await esbuild.build({
    entryPoints: ['capacitor-electron/electron-platform/index.ts'],
    bundle: true,
    outfile: 'src/capacitor-electron/index.js',
    platform: 'node',
    target: 'node16',
    minify: true,
    external: ['electron', 'fs', 'path', 'mime-types', 'events'],
  });
}

(async () => {
  try {
    await buildPlatformCore();
    await buildCliScripts();
    // await packTemplate();
    console.log('\nPlatform Build Complete.\n');
  } catch (e) {
    console.error(e);
    process.exit(1);
  }
})();
