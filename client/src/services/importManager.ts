// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createFile, FsPath, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { wait } from '@/parsec/internals';
import { Mutex } from 'async-mutex';
import { v4 as uuid4 } from 'uuid';

export const ImportManagerKey = 'importManager';

enum ImportState {
  FileProgress,
  FileImported,
  FileAdded,
  CreateFailed,
  WriteError,
  Cancelled,
}

interface FileProgressStateData {
  importData: ImportData;
  progress: number;
}

interface FileImportedStateData {
  importData: ImportData;
}

interface FileAddedStateData {
  importData: ImportData;
}

interface CreateFailedStateData {
  importData: ImportData;
  error: string;
}

interface WriteErrorStateData {
  importData: ImportData;
  error: string;
}

export type StateData = FileProgressStateData | FileImportedStateData | FileAddedStateData | CreateFailedStateData | WriteErrorStateData;

type FileImportCallback = (state: ImportState, data: StateData) => Promise<void>;
type ImportID = string;

class ImportData {
  id: ImportID;
  file: File;
  path: FsPath;
  workspaceHandle: WorkspaceHandle;
  workspaceId: WorkspaceID;

  constructor(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, file: File, path: FsPath) {
    this.id = uuid4();
    this.file = file;
    this.path = path;
    this.workspaceHandle = workspaceHandle;
    this.workspaceId = workspaceId;
  }
}

class ImportManager {
  private importData: Array<ImportData>;
  private callbacks: Array<[string, FileImportCallback]>;
  private mockFD: number;
  private running: boolean;
  private fileMutex: Mutex;
  private cbMutex: Mutex;

  constructor() {
    this.importData = [];
    this.callbacks = [];
    this.mockFD = 0;
    this.running = false;
    this.fileMutex = new Mutex();
    this.cbMutex = new Mutex();
    this.start();
  }

  get isRunning(): boolean {
    return this.running;
  }

  async importFile(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, file: File, path: FsPath): Promise<void> {
    const newData = new ImportData(workspaceHandle, workspaceId, file, path);
    await this.fileMutex
      .runExclusive(async () => {
        console.log(`Adding file ${file.name} to import to ${path}`);
        this.importData.unshift(newData);
      })
      .then(() => {
        this.sendState(ImportState.FileAdded, { importData: newData });
      });
  }

  async cancelImport(id: ImportID): Promise<void> {
    await this.fileMutex
      .runExclusive(async () => {
        const index = this.importData.findIndex((item) => item.id === id);
        if (index !== -1) {
          console.log(`Canceling import of ${this.importData[index].file.name}`);
          return this.importData.splice(index, 1);
        }
      })
      .then((result) => {
        if (result && result.length > 0) {
          this.sendState(ImportState.Cancelled, { importData: result[0] });
        }
      });
  }

  async registerCallback(cb: FileImportCallback): Promise<string> {
    const id = uuid4();
    const release = await this.cbMutex.acquire();
    try {
      this.callbacks.push([id, cb]);
    } finally {
      release();
    }
    return id;
  }

  async removeCallback(id: string): Promise<void> {
    await this.cbMutex.runExclusive(async () => {
      this.callbacks = this.callbacks.filter((elem) => elem[0] !== id);
    });
  }

  private async sendState(state: ImportState, data: StateData): Promise<void> {
    await this.cbMutex.runExclusive(async () => {
      for (const elem of this.callbacks) {
        elem[1](state, data);
      }
    });
  }

  private async mockOpen(_path: string, _mode: string): Promise<number> {
    this.mockFD += 1;
    return this.mockFD;
  }

  // eslint-disable-next-line @typescript-eslint/no-empty-function
  private async mockClose(_fd: number): Promise<void> {}

  private async mockWrite(fd: number, chunk: Uint8Array): Promise<number> {
    // Simulate 500ms to write
    await wait(500);
    return chunk.length;
  }

  private async doImport(data: ImportData): Promise<void> {
    const reader = data.file.stream().getReader();

    const result = await createFile(data.path);
    if (!result.ok) {
      console.log(`Failed to create file ${data.path} (reason: ${result.error.tag}), cancelling...`);
      this.sendState(ImportState.CreateFailed, {
        importData: data,
        error: result.error.tag,
      });
      return;
    }
    const fd = await this.mockOpen(data.path, 'w+');
    // Would prefer to use
    // for await (const chunk of data.file.stream()) {}
    // instead but it's not available on streams.

    let writtenData = 0;
    // eslint-disable-next-line no-constant-condition
    while (true) {
      const buffer = await reader.read();
      if (buffer.done) {
        break;
      }
      await this.mockWrite(fd, buffer.value);
      writtenData += buffer.value.byteLength;
      this.sendState(ImportState.FileProgress, {
        importData: data,
        progress: (writtenData / (data.file.size || 1)) * 100,
      });
    }
    await this.mockClose(fd);
    this.sendState(ImportState.FileProgress, {
      importData: data,
      progress: 100,
    });
    this.sendState(ImportState.FileImported, { importData: data });
  }

  async stop(): Promise<void> {
    this.running = false;
  }

  async start(): Promise<void> {
    if (this.running) {
      return;
    }
    this.running = true;

    // eslint-disable-next-line no-constant-condition
    while (true) {
      if (!this.running) {
        break;
      }
      const release = await this.fileMutex.acquire();
      let elem: ImportData | undefined = undefined;
      try {
        elem = this.importData.pop();
      } finally {
        release();
      }

      if (elem) {
        console.log(`Importing file ${elem.file.name} to ${elem.path}`);
        this.sendState(ImportState.FileProgress, {
          importData: elem,
          progress: 0,
        });
        await this.doImport(elem);
      } else {
        await wait(500);
      }
    }
  }
}

export { ImportData, ImportManager, ImportState };

/*

This big comment is just here to get an idea of how this manager
is meant to be used (it helped design the API for it).

This can be removed when it's fully implemented.

<template>
  <p
    v-for="import in imports"
  >
    {{ import[0].file.name }}

    <span
      v-if="import[1] >= 0 && import[1] < 100"
    >
      {{ import[1] }}%
    </span>
    <ion-icon
      v-if="import[1] === 100"
      :icon="checkmark"
    />
    <ion-icon
      v-if="import[1[ === -1"
      :icon="close-circle"
    />

    <ion-button
      @click="cancelImport(import[0].id)"
    >
      {{ 'Cancel' }}
    </ion-button>
  </p>
</template>

<script setup lang="ts">
const importManager = inject();
const dbId = importManager.registerCallback(onImportEvent);
const imports: Ref<Array<[ImportData, number]>> = ref([]);

onUnmounted(async () => {
  importManager.removeCallback(dbId);
});

function updateProgress(id: string, progress: number): void {
  const index = imports.findIndex((item) => item[0].id === id);
  if (index !== -1) {
    let item = imports[index];
    item[1] = progress;
    imports[index] = item;
  }
}

async function cancelImport(importId: string): Promise<void> {
  importManager.cancelImport(importId);
}

async function onImportEvent(state: ImportState, data: StateData): Promise<void> {
  if (state === ImportState.FileAdded) {
    imports.push([data.importData, 0]);
  } else if (state === ImportState.FileProgress) {
    updateProgress(data.importData.id, data.progress);
  } else if (state === ImportState.FileImported) {
    updateProgress(data.importData.id, 100);
  } else if (state === ImportState.CreateFailed) {
    updateProgress(data.importData.id, -1);
  } else if (state === ImportState.Cancelled) {
    const index = imports.findIndex((item) => item[0].id === data.importData.id);
    if (!index !== -1) {
      imports.splice(index, 1);
    }
  }
}
</script>
*/
