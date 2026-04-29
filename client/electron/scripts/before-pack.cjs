// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const builder = require('electron-builder');

const WINFSP_VERSION = '2.1.25156';
const WINFSP_RELEASE_BRANCH = 'v2.1';
const WINFSP_URL = `https://github.com/winfsp/winfsp/releases/download/${WINFSP_RELEASE_BRANCH}/winfsp-${WINFSP_VERSION}.msi`;
// `wget -O /dev/stdout ${WINFSP_URL} | openssl dgst -sha256 -binary | openssl base64 -A`
const WINFSP_SHA256SUM = 'Bzpw4A93Qj40vtmLhuYA3vkzk7pYIiBPrFeikyTbn3o=';
const WINFSP_DEST_FILE = `build/winfsp-${WINFSP_VERSION}.msi`;

/**
 *
 * @param {string} url
 * @param {string} sha256sum The expected sha256sum in base64
 * @param {path} dest
 */
async function downloadFile(url, sha256sum, dest) {
  const { createHash } = require('node:crypto');
  const fs = require('node:fs');
  const { pipeline } = require('node:stream/promises');
  const { Readable, Transform } = require('node:stream');

  const hash = createHash('sha256');
  const destStream = fs.createWriteStream(dest, {
    encoding: 'binary',
  });

  const response = await fetch(url);

  if (!response.ok || response.status !== 200) {
    throw new Error(`Failed to download ${url}: invalid response: ${response.status}`);
  }

  const httpStream = Readable.fromWeb(response.body);
  const hashStream = new Transform({
    transform(chunk, _enc, cb) {
      hash.update(chunk);
      cb(null, chunk);
    },
  });
  await pipeline(httpStream, hashStream, destStream);

  const fileDigest = hash.digest('base64');
  if (fileDigest !== sha256sum) {
    throw new Error(`Failed to download ${url}: sha256sum mismatch: got ${fileDigest} expected ${sha256sum}`);
  }
}

/**
 * @param {import('app-builder-lib/out/configuration').BeforePackContext} context
 */
exports.default = async function beforePack(context) {
  switch (context.electronPlatformName) {
    case builder.Platform.WINDOWS.nodeName:
      console.log('Downloading WinFSP');
      await downloadFile(WINFSP_URL, WINFSP_SHA256SUM, WINFSP_DEST_FILE).then(() =>
        console.log(`WinFSP downloaded to ${WINFSP_DEST_FILE}`),
      );
      break;
    default:
      console.log(`No hook for platform ${context.electronPlatformName}`);
      break;
  }
};
