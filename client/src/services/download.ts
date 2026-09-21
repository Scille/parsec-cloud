// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BASE } from '@/services/viewers';

export interface DownloadEntry {
  path: string;
  isFile: boolean;
  size?: number;
}

export interface DownloadRequest {
  workspaceHandle: number;
  name: string;
  archive: boolean;
  root: string;
  entries: Array<DownloadEntry>;
}

const DOWNLOAD_FRAME_NAME = 'parsec-download';

// The worker needs to know which tab to use to access libparsec, and only it can tell us who we are.
// The id doesn't change as long as the page is not reloaded.
let _clientId: string | undefined;

async function _getClientId(worker: ServiceWorker): Promise<string> {
  if (_clientId === undefined) {
    _clientId = await new Promise<string>((resolve, reject) => {
      const { port1, port2 } = new MessageChannel();
      const timer = setTimeout(() => reject(new Error('The streaming worker did not answer')), 5000);
      port1.onmessage = (event: MessageEvent): void => {
        clearTimeout(timer);
        port1.close();
        resolve(event.data.clientId);
      };
      worker.postMessage({ type: 'WHOAMI' }, [port2]);
    });
  }
  return _clientId;
}

function _getDownloadFrame(): HTMLIFrameElement {
  let frame = document.querySelector<HTMLIFrameElement>(`iframe[name="${DOWNLOAD_FRAME_NAME}"]`);
  if (!frame) {
    frame = document.createElement('iframe');
    frame.name = DOWNLOAD_FRAME_NAME;
    frame.hidden = true;
    frame.tabIndex = -1;
    document.body.appendChild(frame);
  }
  return frame;
}

// We need to use a POST to be able to give the worker a list of files, so we're tricking the browser
// by submitting in a hidden iframe to avoid navigation.
export async function startDownload(request: DownloadRequest): Promise<void> {
  const worker = 'serviceWorker' in navigator ? navigator.serviceWorker.controller : null;
  if (!worker) {
    throw new Error('The streaming worker is not available');
  }
  const clientId = await _getClientId(worker);

  const input = document.createElement('input');
  input.type = 'hidden';
  input.name = 'request';
  input.value = JSON.stringify({ ...request, clientId });

  const form = document.createElement('form');
  form.method = 'POST';
  form.action = `${BASE}parsec-download`;
  form.target = _getDownloadFrame().name;
  form.hidden = true;
  form.appendChild(input);
  document.body.appendChild(form);
  form.submit();
  form.remove();
}
