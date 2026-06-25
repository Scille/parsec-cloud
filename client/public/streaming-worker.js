// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// MessagePort handed to us by trampoline-web.ts at startup, used to forward
// file read requests to libparsec.
let libparsecPort = null;
let nextRequestId = 0;
const pendingRequests = new Map();

const MAX_CHUNK_SIZE = 2 * 1024 * 1024; // 2MB
// Derive base path from the SW's own URL so fetch interception works at any deployment sub-path.
const BASE_PATH = new URL('./', self.location.href).pathname;

self.addEventListener('install', () => {
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener('message', (event) => {
  if (event.data.type === 'INIT') {
    libparsecPort = event.ports[0];
    libparsecPort.onmessage = (e) => {
      const { id, data, error } = e.data;
      const pending = pendingRequests.get(id);
      if (!pending) return;
      pendingRequests.delete(id);
      error ? pending.reject(new Error(JSON.stringify(error))) : pending.resolve(data);
    };
    console.log('Streaming worker initialized');
  }
});

self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);
  if (url.pathname === `${BASE_PATH}parsec-stream` && event.request.method === 'GET') {
    event.respondWith(handleStreamRequest(event.request, url));
  }
});

async function handleStreamRequest(request, url) {
  if (!libparsecPort) {
    return new Response('Service worker not initialized', { status: 503 });
  }

  const workspaceHandle = Number(url.searchParams.get('handle'));
  const filePath = url.searchParams.get('path');
  const totalSize = Number(url.searchParams.get('size'));
  const historyParam = url.searchParams.get('history');
  const historyHandle = historyParam !== null ? Number(historyParam) : null;

  if (!filePath || isNaN(workspaceHandle) || isNaN(totalSize) || isNaN(historyHandle)) {
    return new Response('Invalid parsec-stream parameters', { status: 400 });
  }

  let start = 0;
  let end = totalSize - 1;
  const rangeHeader = request.headers.get('Range');
  if (rangeHeader) {
    const match = rangeHeader.match(/bytes=(\d+)-(\d*)/);
    if (match) {
      start = Number(match[1]);
      end = match[2] ? Number(match[2]) : totalSize - 1;
    }
  }
  end = Math.min(end, start + MAX_CHUNK_SIZE - 1);

  try {
    const data = await readFromLibparsec(workspaceHandle, filePath, start, end - start + 1, historyHandle);
    return new Response(data, {
      status: rangeHeader ? 206 : 200,
      headers: {
        'Content-Type': 'application/octet-stream',
        'Content-Length': String(end - start + 1),
        'Content-Range': `bytes ${start}-${end}/${totalSize}`,
        'Accept-Ranges': 'bytes',
        // Prevent the browser from persisting decrypted data to disk cache
        'Cache-Control': 'no-store',
      },
    });
  } catch (err) {
    return new Response(err instanceof Error ? err.message : 'Unknown error', { status: 500 });
  }
}

function readFromLibparsec(workspaceHandle, filePath, offset, size, historyHandle) {
  return new Promise((resolve, reject) => {
    const id = nextRequestId++;
    pendingRequests.set(id, { resolve, reject });
    libparsecPort.postMessage({ id, workspaceHandle, filePath, offset, size, historyHandle });
  });
}

console.log('Streaming worker started');
