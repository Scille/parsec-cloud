// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// This worker serves decrypted file content to <video>, <audio> and <img> elements.
//
// It is stateless on purpose: browsers terminate idle service workers (after ~30s
// without events) and wipe their global state, so nothing can be kept in memory
// between two requests. Instead, every read is delegated to the tab that made the
// request (the only place with access to libparsec) through a dedicated one-shot
// MessageChannel:
//
//   worker -> tab (client.postMessage):  { type: 'READ', workspaceHandle, filePath, offset, size, historyHandle }
//                                        + port2 of a new MessageChannel
//   tab -> worker (on that port):        { type: 'READ_REPLY', data, error }
//
// There is no fallback on another tab: the requesting tab is the one consuming the
// response, if it is gone the browser has already aborted the request.

// Size of the reads made on libparsec, which is also the granularity of the backpressure
const READ_CHUNK_SIZE = 2 * 1024 * 1024; // 2MB
// Maximum time given to the tab to read the data
const READ_TIMEOUT_MS = 60 * 1000;

// Derive base path from the SW's own URL so fetch interception works at any deployment sub-path.
const BASE_PATH = new URL('./', self.location.href).pathname;

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
// The tab was reached, but the read itself failed
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
  try {
    // The first chunk is read before answering, so that a failure results in a proper
    // HTTP error instead of a broken stream.
    const firstChunk = await read(start, Math.min(READ_CHUNK_SIZE, length));
    if (firstChunk.byteLength === 0) {
      throw new ReadError('Unexpected end of file');
    }
    return new Response(createBodyStream(read, firstChunk, start, end), { status, headers });
  } catch (err) {
    return createErrorResponse(err);
  }
}

// Body of the response, produced chunk by chunk as the browser consumes it, so that we
// never hold more than a couple of chunks in memory whatever the size of the file is.
function createBodyStream(read, firstChunk, start, end) {
  let offset = start + firstChunk.byteLength;
  return new ReadableStream({
    start(controller) {
      controller.enqueue(firstChunk);
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

async function readFromClient(client, target, offset, size) {
  const data = await requestRead(client, { ...target, offset, size });
  return data.byteLength > size ? data.subarray(0, size) : data;
}

function requestRead(client, params) {
  return new Promise((resolve, reject) => {
    const { port1, port2 } = new MessageChannel();
    let settled = false;
    const timer = setTimeout(() => settle(new ReadError('Timed out while reading the file')), READ_TIMEOUT_MS);

    function settle(error, data) {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      port1.onmessage = null;
      port1.close();
      error ? reject(error) : resolve(data);
    }

    port1.onmessage = (event) => {
      const message = event.data;
      if (message.type === 'READ_REPLY') {
        message.error ? settle(new ReadError(describeError(message.error))) : settle(null, message.data);
      }
    };

    try {
      client.postMessage({ type: 'READ', ...params }, [port2]);
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
