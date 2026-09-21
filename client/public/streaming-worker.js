// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// This worker serves decrypted file content:
// - `GET parsec-stream` streams a file to <video>, <audio> and <img> elements (Range requests are supported),
// - `POST parsec-download` makes the browser download a file, or a zip archive of files and folders.
//
// It is stateless on purpose: browsers terminate idle service workers (after ~30s
// without events) and wipe their global state, so nothing can be kept in memory
// between two requests. Instead, everything that requires libparsec is delegated to
// a tab (the only place with access to it) through a dedicated one-shot MessageChannel:
//
//   worker -> tab (client.postMessage):  { type: 'READ', workspaceHandle, filePath, offset, size, historyHandle }
//                                        { type: 'LIST', workspaceHandle, path }
//                                        + port2 of a new MessageChannel
//   tab -> worker (on that port):        { data, error }
//                                        (data is the bytes read, or the children of the folder as
//                                        an array of { name, path, isFile, size })
//
// Other messages sent by the tabs to the worker:
//
//   { type: 'WHOAMI' } + a port:         the worker answers with { clientId } on that port, a tab needs its id
//                                        to make a download request (see below)
//   { type: 'PING' }:                    no answer, see below
//
// The tab used for a request is the one that made it (for `parsec-stream`, the id is provided by
// the browser, for `parsec-download`, it is part of the request). There is no fallback on another
// tab: if it is gone, the browser has already aborted the request (or the download is aborted).
//

// Size of the reads made on libparsec, which is also the granularity of the backpressure
const READ_CHUNK_SIZE = 2 * 1024 * 1024; // 2MB
// Maximum time given to the tab to answer a request (reading data, listing a folder)
const READ_TIMEOUT_MS = 60 * 1000;

// Derive base path from the SW's own URL so fetch interception works at any deployment sub-path.
const BASE_PATH = new URL('./', self.location.href).pathname;

// zip.js, used to build archives. It is a third-party file vendored as is, see its header.
// `importScripts()` can only be used while the worker starts, that's why it's not loaded on demand.
try {
  importScripts('vendor/zip-native.min.js');
  self.zip.configure({ useWebWorkers: false });
} catch (err) {
  // Better a worker that can only stream than no worker at all
  console.error('Failed to load zip.js, downloading archives is not possible', err);
}

const MIME_TYPES = {
  // Images
  png: 'image/png',
  jpg: 'image/jpeg',
  jpeg: 'image/jpeg',
  gif: 'image/gif',
  webp: 'image/webp',
  avif: 'image/avif',
  bmp: 'image/bmp',
  svg: 'image/svg+xml',
  // Audios
  mp3: 'audio/mpeg',
  wav: 'audio/wav',
  ogg: 'audio/ogg',
  oga: 'audio/ogg',
  opus: 'audio/ogg',
  m4a: 'audio/mp4',
  aac: 'audio/aac',
  flac: 'audio/flac',
  // Videos
  mp4: 'video/mp4',
  m4v: 'video/mp4',
  mov: 'video/quicktime',
  mpeg: 'video/mpeg',
  mpg: 'video/mpeg',
  webm: 'video/webm',
  ogv: 'video/ogg',
};

// The tab that made the request can't be reached
class ClientUnavailableError extends Error {}
// The tab was reached, but what was asked of it (reading, listing) failed
class ReadError extends Error {}

self.addEventListener('install', () => {
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);
  if (url.pathname === `${BASE_PATH}parsec-stream` && event.request.method === 'GET') {
    event.respondWith(handleStreamRequest(event, url));
  } else if (url.pathname === `${BASE_PATH}parsec-download` && event.request.method === 'POST') {
    event.respondWith(handleDownloadRequest(event));
  }
});

self.addEventListener('message', (event) => {
  if (event.data?.type === 'WHOAMI' && event.source && event.ports[0]) {
    event.ports[0].postMessage({ clientId: event.source.id });
  }
});

async function handleStreamRequest(event, url) {
  const workspaceHandle = Number(url.searchParams.get('handle'));
  const filePath = url.searchParams.get('path');
  const totalSize = Number(url.searchParams.get('size'));
  const historyParam = url.searchParams.get('history');
  const historyHandle = historyParam !== null ? Number(historyParam) : null;

  if (
    !filePath ||
    !url.searchParams.has('size') ||
    !Number.isInteger(workspaceHandle) ||
    !Number.isInteger(totalSize) ||
    totalSize < 0 ||
    Number.isNaN(historyHandle)
  ) {
    return new Response('Invalid parsec-stream parameters', { status: 400 });
  }

  const client = event.clientId ? await self.clients.get(event.clientId) : undefined;
  if (!client) {
    return new Response('The tab that made the request is not available', { status: 503 });
  }

  const range = parseRange(event.request.headers.get('Range'), totalSize);
  if (range.kind === 'unsatisfiable') {
    return new Response(null, { status: 416, headers: { 'Content-Range': `bytes */${totalSize}` } });
  }

  const headers = {
    'Content-Type': getMimeType(filePath),
    'Accept-Ranges': 'bytes',
    // The content is served as-is, never to be interpreted as something else, nor to run scripts
    // if the URL happens to be navigated to (e.g. an svg)
    'X-Content-Type-Options': 'nosniff',
    'Content-Security-Policy': 'sandbox',
    // Prevent the browser from persisting decrypted data to disk cache
    'Cache-Control': 'no-store',
  };
  let status = 200;
  let start = 0;
  let end = totalSize - 1;
  if (range.kind === 'partial') {
    status = 206;
    start = range.start;
    // Not truncating the request range, and it breaks on non-HTTPS URL (so our parsec-desktop://).
    end = range.end;
    headers['Content-Range'] = `bytes ${start}-${end}/${totalSize}`;
  }
  const length = end - start + 1;
  headers['Content-Length'] = String(length);

  if (length === 0) {
    return new Response(null, { status, headers });
  }

  const read = (offset, size) => readFromClient(client, { workspaceHandle, filePath, historyHandle }, offset, size);
  return createFileResponse(read, start, end, status, headers);
}

// Response with the content of a file from `start` to `end` (both inclusive, at least one byte).
async function createFileResponse(read, start, end, status, headers) {
  try {
    // The first chunk is read before answering, so that a failure results in a proper
    // HTTP error instead of a broken stream.
    const firstChunk = await read(start, Math.min(READ_CHUNK_SIZE, end - start + 1));
    if (firstChunk.byteLength === 0) {
      throw new ReadError('Unexpected end of file');
    }
    return new Response(createBodyStream(read, start, end, firstChunk), { status, headers });
  } catch (err) {
    return createErrorResponse(err);
  }
}

// Stream of the content of a file from `start` to `end` (both inclusive, `end` being lower than `start`
// for an empty file), produced chunk by chunk as it is consumed, so that we never hold more than a couple
// of chunks in memory whatever the size of the file is. `firstChunk` is the data already read at `start`, if any.
function createBodyStream(read, start, end, firstChunk) {
  let offset = firstChunk ? start + firstChunk.byteLength : start;
  return new ReadableStream({
    start(controller) {
      if (firstChunk) {
        controller.enqueue(firstChunk);
      }
      if (offset > end) {
        controller.close();
      }
    },
    async pull(controller) {
      try {
        const chunk = await read(offset, Math.min(READ_CHUNK_SIZE, end - offset + 1));
        if (chunk.byteLength === 0) {
          // The file is shorter than announced. Erroring the stream makes the browser fail
          // the request, rather than considering the truncated content as complete.
          throw new ReadError('Unexpected end of file');
        }
        controller.enqueue(chunk);
        offset += chunk.byteLength;
        if (offset > end) {
          controller.close();
        }
      } catch (err) {
        // No-op if the browser already cancelled the request
        controller.error(err);
      }
    },
  });
}

function createErrorResponse(err) {
  if (err instanceof ClientUnavailableError) {
    return new Response('No client available to read the file', { status: 503 });
  }
  return new Response(err instanceof Error ? err.message : 'Unknown error', { status: 500 });
}

async function handleDownloadRequest(event) {
  let request;
  try {
    request = parseDownloadRequest(await event.request.formData());
  } catch (err) {
    return new Response(`Invalid parsec-download request: ${err.message}`, { status: 400 });
  }

  // The id of the tab is only known by the tab, and it cannot be guessed: it prevents any other
  // page from making a download request.
  const client = await self.clients.get(request.clientId);
  if (!client) {
    return new Response('The tab that made the request is not available', { status: 503 });
  }

  return request.archive ? createArchiveResponse(event, client, request) : createSingleFileDownloadResponse(client, request);
}

function parseDownloadRequest(formData) {
  const raw = formData.get('request');
  if (typeof raw !== 'string') {
    throw new Error('missing request');
  }
  const request = JSON.parse(raw);
  const isValidEntry = (entry) =>
    typeof entry.path === 'string' &&
    entry.path.length > 0 &&
    typeof entry.isFile === 'boolean' &&
    (!entry.isFile || (Number.isInteger(entry.size) && entry.size >= 0));
  if (
    typeof request.clientId !== 'string' ||
    !Number.isInteger(request.workspaceHandle) ||
    typeof request.name !== 'string' ||
    request.name.length === 0 ||
    typeof request.archive !== 'boolean' ||
    typeof request.root !== 'string' ||
    !Array.isArray(request.entries) ||
    request.entries.length === 0 ||
    !request.entries.every(isValidEntry)
  ) {
    throw new Error('invalid parameters');
  }
  if (!request.archive && (request.entries.length !== 1 || !request.entries[0].isFile)) {
    throw new Error('only a single file can be downloaded without an archive');
  }
  return request;
}

function createDownloadHeaders(name, contentType) {
  return {
    'Content-Type': contentType,
    'Content-Disposition': createContentDisposition(name),
    // The content is saved as is, never to be interpreted as something else
    'X-Content-Type-Options': 'nosniff',
    'Cache-Control': 'no-store',
  };
}

function createContentDisposition(name) {
  // encodeURIComponent() keeps `'()*`, which `filename*` doesn't allow
  const encoded = encodeURIComponent(name).replace(/['()*]/g, (char) => `%${char.charCodeAt(0).toString(16).toUpperCase()}`);
  return `attachment; filename*=UTF-8''${encoded}`;
}

async function createSingleFileDownloadResponse(client, request) {
  const [entry] = request.entries;
  const headers = { ...createDownloadHeaders(request.name, 'application/octet-stream'), 'Content-Length': String(entry.size) };
  if (entry.size === 0) {
    return new Response(null, { status: 200, headers });
  }
  const read = (offset, size) =>
    readFromClient(client, { workspaceHandle: request.workspaceHandle, filePath: entry.path, historyHandle: null }, offset, size);
  return createFileResponse(read, 0, entry.size - 1, 200, headers);
}

function createArchiveResponse(event, client, request) {
  if (!self.zip) {
    return new Response('Archives are not available', { status: 500 });
  }

  // The archive is written in `writable` as it is built, and read from `readable` as it is downloaded.
  let transformController;
  const { readable, writable } = new TransformStream({
    start(controller) {
      transformController = controller;
    },
  });
  const archiving = writeArchive(writable, client, request).catch((err) => {
    console.warn('The archive could not be completed', err);
    transformController.error(err);
  });
  event.waitUntil(archiving);

  return new Response(readable, { status: 200, headers: createDownloadHeaders(request.name, 'application/zip') });
}

async function writeArchive(writable, client, request) {
  const { workspaceHandle, root } = request;
  const zipWriter = new self.zip.ZipWriter(writable, { zip64: true });

  for await (const entry of walkEntries(client, workspaceHandle, request.entries)) {
    const name = getArchiveEntryName(entry.path, root);
    if (entry.isFile) {
      const read = (offset, size) => readFromClient(client, { workspaceHandle, filePath: entry.path, historyHandle: null }, offset, size);
      // Files are stored as is, not compressed: it's a lot faster, and most of the content (videos,
      // images, documents...) is already compressed anyway
      await zipWriter.add(name, createBodyStream(read, 0, entry.size - 1), { level: 0 });
    } else {
      // An empty folder, otherwise it wouldn't be in the archive
      await zipWriter.add(`${name}/`, undefined, { directory: true });
    }
  }
  await zipWriter.close();
}

// Files to put in the archive, depth-first. Folders are explored as we go, not in advance: there can
// be a lot of them, and the list of everything would take a long time to get and a lot of memory to hold.
// An empty folder is returned as is (`isFile` is false).
async function* walkEntries(client, workspaceHandle, entries) {
  for (const entry of entries) {
    if (entry.isFile) {
      yield entry;
      continue;
    }
    const children = await listFromClient(client, workspaceHandle, entry.path);
    if (children.length === 0) {
      yield entry;
    } else {
      yield* walkEntries(client, workspaceHandle, children);
    }
  }
}

function getArchiveEntryName(path, root) {
  const prefix = root.endsWith('/') ? root : `${root}/`;
  return path.startsWith(prefix) ? path.slice(prefix.length) : path.slice(1);
}

// Parse a Range header. Returns:
// - { kind: 'full' } if the whole content must be sent (no header, or a header we must ignore),
// - { kind: 'partial', start, end } (both inclusive, `end` clamped to the size),
// - { kind: 'unsatisfiable' }.
// Only the first range is honored if several are requested.
function parseRange(header, totalSize) {
  // Range: bytes=<first>-<last>`
  const match = header ? /^\s*bytes=\s*(\d*)\s*-\s*(\d*)/.exec(header) : null;
  if (!match || (match[1] === '' && match[2] === '')) {
    return { kind: 'full' };
  }

  const [, first, last] = match;
  if (first === '') {
    // Range: bytes=-500
    const suffixLength = Number(last);
    if (suffixLength === 0 || totalSize === 0) {
      return { kind: 'unsatisfiable' };
    }
    // A suffix longer than the file gets the whole file
    return { kind: 'partial', start: Math.max(0, totalSize - suffixLength), end: totalSize - 1 };
  }

  const start = Number(first);
  // Inverted range, e.g. Range: bytes=10-5
  if (last !== '' && Number(last) < start) {
    return { kind: 'full' };
  }
  // Starts past the end of the file
  if (start >= totalSize) {
    return { kind: 'unsatisfiable' };
  }
  // Open-ended range, up to the end of the file (Range: bytes=0-) or closed range (Range: bytes=65536-462515)
  return { kind: 'partial', start, end: last === '' ? totalSize - 1 : Math.min(Number(last), totalSize - 1) };
}

function getMimeType(filePath) {
  const dot = filePath.lastIndexOf('.');
  const extension = dot === -1 ? '' : filePath.slice(dot + 1).toLowerCase();
  return MIME_TYPES[extension] ?? 'application/octet-stream';
}

// Reads `size` bytes at `offset` from a file, using the given tab.
async function readFromClient(client, target, offset, size) {
  const data = await requestClient(client, { type: 'READ', ...target, offset, size });
  // Never send more than announced, whatever libparsec returned
  return data.byteLength > size ? data.subarray(0, size) : data;
}

// Lists the children of a folder ({ name, path, isFile, size }), using the given tab.
function listFromClient(client, workspaceHandle, path) {
  return requestClient(client, { type: 'LIST', workspaceHandle, path });
}

// Sends a request to a tab and waits for its answer.
function requestClient(client, request) {
  return new Promise((resolve, reject) => {
    const { port1, port2 } = new MessageChannel();
    let settled = false;
    const timer = setTimeout(() => settle(new ReadError('Timed out waiting for the tab')), READ_TIMEOUT_MS);

    function settle(error, data) {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      port1.onmessage = null;
      port1.close();
      error ? reject(error) : resolve(data);
    }

    port1.onmessage = (event) => {
      const reply = event.data;
      reply.error ? settle(new ReadError(describeError(reply.error))) : settle(null, reply.data);
    };

    try {
      client.postMessage(request, [port2]);
    } catch {
      settle(new ClientUnavailableError('Could not reach the tab that made the request'));
    }
  });
}

function describeError(error) {
  if (typeof error === 'string') {
    return error;
  }
  try {
    return JSON.stringify(error);
  } catch {
    return 'Unknown error';
  }
}
