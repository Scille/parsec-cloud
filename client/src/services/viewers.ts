// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec } from '@/plugins/libparsec';
import * as pdfjs from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?worker&url';

const BASE = import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;

async function _initPdf(): Promise<void> {
  pdfjs.GlobalWorkerOptions.workerSrc = pdfjsWorker;
}

async function _initStreamingWorker(): Promise<void> {
  if (!('serviceWorker' in navigator)) {
    console.warn('Streaming worker: service workers not supported, video/audio streaming unavailable');
    return;
  }

  const registration = await navigator.serviceWorker.register(`${BASE}streaming-worker.js`, { scope: BASE });

  // On first install the worker goes through installing → activated.
  // On subsequent page loads registration.active is already set.
  let sw: ServiceWorker;
  if (registration.active) {
    sw = registration.active;
  } else {
    const pending = registration.installing ?? registration.waiting;
    if (!pending) {
      console.warn('Streaming worker: unexpected registration state');
      return;
    }
    sw = await new Promise<ServiceWorker>((resolve) => {
      pending.addEventListener('statechange', function onStateChange() {
        if (registration.active) {
          pending.removeEventListener('statechange', onStateChange);
          resolve(registration.active!);
        }
      });
    });
  }

  // port1 lives in this tab and proxies read requests to libparsec.
  // port2 is transferred to the service worker so it can send read requests here.
  const { port1, port2 } = new MessageChannel();

  port1.onmessage = async (event: MessageEvent): Promise<void> => {
    const { id, workspaceHandle, filePath, offset, size, historyHandle } = event.data as {
      id: number;
      workspaceHandle: number;
      filePath: string;
      offset: number;
      size: number;
      historyHandle: number | null;
    };
    let fd: number | undefined;
    try {
      if (historyHandle !== null) {
        const openResult = await libparsec.workspaceHistoryOpenFile(historyHandle, filePath);
        if (!openResult.ok) {
          port1.postMessage({ type: 'READ_REPLY', id, data: null, error: openResult.error });
          return;
        }
        fd = openResult.value;
        const readResult = await libparsec.workspaceHistoryFdRead(historyHandle, fd, BigInt(offset), BigInt(size));
        if (!readResult.ok) {
          port1.postMessage({ type: 'READ_REPLY', id, data: null, error: readResult.error });
        } else {
          port1.postMessage({ type: 'READ_REPLY', id, data: readResult.value, error: null }, [readResult.value.buffer]);
        }
      } else {
        const openResult = await libparsec.workspaceOpenFile(workspaceHandle, filePath, {
          read: true,
          write: false,
          truncate: false,
          create: false,
          createNew: false,
        });
        if (!openResult.ok) {
          port1.postMessage({ type: 'READ_REPLY', id, data: null, error: openResult.error });
          return;
        }
        fd = openResult.value;
        const readResult = await libparsec.workspaceFdRead(workspaceHandle, fd, BigInt(offset), BigInt(size));
        if (!readResult.ok) {
          port1.postMessage({ type: 'READ_REPLY', id, data: null, error: readResult.error });
        } else {
          // Transfer the buffer to avoid a copy across the message channel.
          port1.postMessage({ type: 'READ_REPLY', id, data: readResult.value, error: null }, [readResult.value.buffer]);
        }
      }
    } catch (err) {
      port1.postMessage({ type: 'READ_REPLY', id, data: null, error: err instanceof Error ? err.message : 'Unknown error' });
    } finally {
      if (fd !== undefined) {
        if (historyHandle !== null) {
          await libparsec.workspaceHistoryFdClose(historyHandle, fd);
        } else {
          await libparsec.workspaceFdClose(workspaceHandle, fd);
        }
      }
    }
  };

  sw.postMessage({ type: 'INIT' }, [port2]);
}

export function getStreamUrl(workspaceHandle: number, path: string, size: number, historyHandle?: number): string {
  const params = new URLSearchParams({ handle: String(workspaceHandle), path, size: String(size) });
  if (historyHandle !== undefined) {
    params.set('history', String(historyHandle));
  }
  return `${BASE}parsec-stream?${params}`;
}

export async function initViewers(): Promise<void> {
  await _initPdf();
  try {
    await _initStreamingWorker();
  } catch (err: unknown) {
    console.error('Failed to initialize streaming service', err);
  }
}
