// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec } from '@/plugins/libparsec';
import * as pdfjs from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?worker&url';

const BASE = import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;

async function _initPdf(): Promise<void> {
  pdfjs.GlobalWorkerOptions.workerSrc = pdfjsWorker;
}

// The streaming worker is stateless, for every read, it sends us a `READ` request along with a dedicated port to answer on.
interface WorkerReadRequest {
  type: 'READ';
  workspaceHandle: number;
  filePath: string;
  offset: number;
  size: number;
  historyHandle: number | null;
}

type ReadOutcome = { data: Uint8Array } | { error: unknown };

async function _readWorkspaceHistoryFile(historyHandle: number, filePath: string, offset: number, size: number): Promise<ReadOutcome> {
  const openResult = await libparsec.workspaceHistoryOpenFile(historyHandle, filePath);
  if (!openResult.ok) {
    return { error: openResult.error };
  }
  const fd = openResult.value;
  try {
    const readResult = await libparsec.workspaceHistoryFdRead(historyHandle, fd, BigInt(offset), BigInt(size));
    return readResult.ok ? { data: readResult.value } : { error: readResult.error };
  } finally {
    await libparsec.workspaceHistoryFdClose(historyHandle, fd);
  }
}

async function _readWorkspaceFile(workspaceHandle: number, filePath: string, offset: number, size: number): Promise<ReadOutcome> {
  const openResult = await libparsec.workspaceOpenFile(workspaceHandle, filePath, {
    read: true,
    write: false,
    truncate: false,
    create: false,
    createNew: false,
  });
  if (!openResult.ok) {
    return { error: openResult.error };
  }
  const fd = openResult.value;
  try {
    const readResult = await libparsec.workspaceFdRead(workspaceHandle, fd, BigInt(offset), BigInt(size));
    return readResult.ok ? { data: readResult.value } : { error: readResult.error };
  } finally {
    await libparsec.workspaceFdClose(workspaceHandle, fd);
  }
}

async function _readFile({ workspaceHandle, filePath, offset, size, historyHandle }: WorkerReadRequest): Promise<ReadOutcome> {
  try {
    return historyHandle !== null
      ? await _readWorkspaceHistoryFile(historyHandle, filePath, offset, size)
      : await _readWorkspaceFile(workspaceHandle, filePath, offset, size);
  } catch (err) {
    return { error: err instanceof Error ? err.message : 'Unknown error' };
  }
}

async function _onWorkerMessage(event: MessageEvent): Promise<void> {
  const request = event.data as WorkerReadRequest | undefined;
  const port = event.ports[0];
  if (request?.type !== 'READ' || !port) {
    return;
  }
  const outcome = await _readFile(request);
  if ('data' in outcome) {
    // Transfer the buffer to avoid a copy across the message channel.
    port.postMessage({ type: 'READ_REPLY', data: outcome.data, error: null }, [outcome.data.buffer]);
  } else {
    port.postMessage({ type: 'READ_REPLY', data: null, error: outcome.error });
  }
  port.close();
}

async function _initStreamingWorker(): Promise<void> {
  if (!('serviceWorker' in navigator)) {
    console.warn('Streaming worker: service workers not supported, video/audio streaming unavailable');
    return;
  }

  // Assigning `onmessage` (unlike `addEventListener`) also starts the delivery of the messages
  // sent by the worker, this must be done before it can send us anything.
  navigator.serviceWorker.onmessage = _onWorkerMessage;
  await navigator.serviceWorker.register(`${BASE}streaming-worker.js`, { scope: BASE });
  // Requests can only be served once the worker is active (and controls this page).
  await navigator.serviceWorker.ready;
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
