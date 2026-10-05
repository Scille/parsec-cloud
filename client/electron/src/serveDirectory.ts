// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Serve directories (i.e. `app` & `app-editics`) through custom schemes.
//
// This mirrors what `electron-serve` does, with one addition: a file `X` can be
// shipped as `X.br` only (see `scripts/copy_to_electron.cjs`), in which case it is
// decompressed on the fly.
// Note Chromium does not decode `Content-Encoding: br` on custom schemes, hence
// the decompression must be done here.

import { BrowserWindow, app, net, protocol, session } from 'electron';
import log from 'electron-log/main.js';
import mime from 'mime-types';
import fs from 'node:fs';
import path from 'node:path';
import { Readable } from 'node:stream';
import { pathToFileURL } from 'node:url';
import zlib from 'node:zlib';

// mime-types doesn't know OnlyOffice data files: `.dic`, `.aff`, `.dat`, `.idx`.
for (const extension of ['dic', 'aff', 'dat', 'idx']) {
  mime.types[extension] = 'application/octet-stream';
}

interface ServedDirectory {
  scheme: string;
  directory: string;
}

async function isFile(filePath: string): Promise<boolean> {
  try {
    return (await fs.promises.stat(filePath)).isFile();
  } catch {
    return false;
  }
}

function createHandler(directory: string): (request: Request) => Promise<Response> {
  const indexPath = path.join(directory, 'index.html');

  return async (request: Request): Promise<Response> => {
    let filePath = path.join(directory, decodeURIComponent(new URL(request.url).pathname));

    const relativePath = path.relative(directory, filePath);
    if (relativePath.startsWith('..') || path.isAbsolute(relativePath)) {
      return new Response(null, { status: 404 });
    }

    if (await isFile(`${filePath}.br`)) {
      const stream = fs.createReadStream(`${filePath}.br`).pipe(zlib.createBrotliDecompress());
      let contentType = mime.lookup(filePath);
      if (!contentType) {
        log.warn(`Unknown MIME type for brotli-compressed file \`${filePath}\``);
        contentType = 'application/octet-stream';
      }
      return new Response(Readable.toWeb(stream) as ReadableStream, { headers: { 'Content-Type': contentType } });
    }

    if (!(await isFile(filePath))) {
      const extension = path.extname(filePath);
      if (extension && extension !== '.html') {
        return new Response(null, { status: 404 });
      }
      // Directory or extension-less route (e.g. `/loading`): fallback on the
      // directory's `index.html`, or the root one for a single page app.
      const directoryIndex = path.join(filePath, 'index.html');
      filePath = (await isFile(directoryIndex)) ? directoryIndex : indexPath;
    }

    return net.fetch(pathToFileURL(filePath).toString());
  };
}

// Must be called (once!) before the app is ready, since all custom schemes must be
// registered by a single `protocol.registerSchemesAsPrivileged` call.
// Returns a function to load each scheme's root page in a window.
export function serveDirectories(served: Array<ServedDirectory>): Array<(window: BrowserWindow) => Promise<void>> {
  protocol.registerSchemesAsPrivileged(
    served.map(({ scheme }) => ({
      scheme,
      privileges: {
        standard: true,
        secure: true,
        allowServiceWorkers: true,
        supportFetchAPI: true,
        corsEnabled: true,
        stream: true,
        codeCache: true,
      },
    })),
  );

  app.on('ready', () => {
    for (const { scheme, directory } of served) {
      session.defaultSession.protocol.handle(scheme, createHandler(path.resolve(app.getAppPath(), directory)));
    }
  });

  return served.map(({ scheme }) => async (window: BrowserWindow): Promise<void> => {
    await window.loadURL(`${scheme}://-`);
  });
}
