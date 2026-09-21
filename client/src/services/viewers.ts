// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Path } from '@/parsec';
import { EntryStatTag, libparsec } from '@/plugins/libparsec';
import * as pdfjs from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?worker&url';

export const BASE = import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;

async function _initPdf(): Promise<void> {
  pdfjs.GlobalWorkerOptions.workerSrc = pdfjsWorker;
}

// The streaming worker is stateless, for every request it needs libparsec for, it sends us a message
// along with a dedicated port to answer on.
interface WorkerReadRequest {
  type: 'READ';
  workspaceHandle: number;
  filePath: string;
  offset: number;
  size: number;
  historyHandle: number | null;
}

interface WorkerListRequest {
  type: 'LIST';
  workspaceHandle: number;
  path: string;
}

interface ListedEntry {
  name: string;
  path: string;
  isFile: boolean;
  size?: number;
}

type Outcome<T> = { data: T } | { error: unknown };

async function _readWorkspaceHistoryFile(
  historyHandle: number,
  filePath: string,
  offset: number,
  size: number,
): Promise<Outcome<Uint8Array>> {
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

async function _readWorkspaceFile(workspaceHandle: number, filePath: string, offset: number, size: number): Promise<Outcome<Uint8Array>> {
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

async function _readFile({ workspaceHandle, filePath, offset, size, historyHandle }: WorkerReadRequest): Promise<Outcome<Uint8Array>> {
  try {
    return historyHandle !== null
      ? await _readWorkspaceHistoryFile(historyHandle, filePath, offset, size)
      : await _readWorkspaceFile(workspaceHandle, filePath, offset, size);
  } catch (err) {
    return { error: err instanceof Error ? err.message : 'Unknown error' };
  }
}

async function _listFolder(workspaceHandle: number, path: string): Promise<Outcome<Array<ListedEntry>>> {
  try {
    const result = await libparsec.workspaceStatFolderChildren(workspaceHandle, path);
    if (!result.ok) {
      return { error: result.error };
    }
    const entries: Array<ListedEntry> = [];
    for (const [name, stat] of result.value) {
      // Confined entries are not visible in the workspace, they are not part of what's downloaded either
      if (stat.confinementPoint) {
        continue;
      }
      entries.push({
        name: name,
        path: Path.quickJoin(path, name),
        isFile: stat.tag === EntryStatTag.File,
        size: stat.tag === EntryStatTag.File ? Number(stat.size) : undefined,
      });
    }
    return { data: entries };
  } catch (err) {
    return { error: err instanceof Error ? err.message : 'Unknown error' };
  }
}

async function _onWorkerMessage(event: MessageEvent): Promise<void> {
  const request = event.data as WorkerReadRequest | WorkerListRequest | undefined;
  const port = event.ports[0];
  if (!request || !port) {
    return;
  }

  let outcome: Outcome<Uint8Array | Array<ListedEntry>>;
  switch (request.type) {
    case 'READ':
      outcome = await _readFile(request);
      break;
    case 'LIST':
      outcome = await _listFolder(request.workspaceHandle, request.path);
      break;
    default:
      return;
  }
  if ('data' in outcome) {
    // Transfer the buffer of a read to avoid a copy across the message channel.
    port.postMessage({ data: outcome.data, error: null }, outcome.data instanceof Uint8Array ? [outcome.data.buffer] : []);
  } else {
    port.postMessage({ data: null, error: outcome.error });
  }
  port.close();
}

const HEARTBEAT_INTERVAL_MS = 10 * 1000;

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
  setInterval(() => navigator.serviceWorker.controller?.postMessage({ type: 'PING' }), HEARTBEAT_INTERVAL_MS);
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
