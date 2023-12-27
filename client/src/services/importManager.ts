// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createFile, FsPath, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { wait } from '@/parsec/internals';
import { Mutex } from 'async-mutex';
import { v4 as uuid4 } from 'uuid';

const MAX_SIMULTANEOUS_IMPORT_JOBS = 2;

export const ImportManagerKey = 'importManager';

enum ImportState {
  ImportAllStarted,
  ImportAllFinished,
  FileProgress,
  FileImported,
  FileAdded,
  ImportStarted,
  CreateFailed,
  WriteError,
  Cancelled,
}

export interface FileProgressStateData {
  progress: number;
}

interface CreateFailedStateData {
  error: string;
}

interface WriteErrorStateData {
  error: string;
}

export type StateData = FileProgressStateData | CreateFailedStateData | WriteErrorStateData;

type FileImportCallback = (state: ImportState, importData?: ImportData, stateData?: StateData) => Promise<void>;
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
  private cancelList: Array<ImportID>;
  private mockFD: number;
  private running: boolean;
  private fileMutex: Mutex;
  private cbMutex: Mutex;
  private cancelMutex: Mutex;
  private importJobs: Array<Promise<void>>;

  constructor() {
    this.importData = [];
    this.callbacks = [];
    this.cancelList = [];
    this.importJobs = [];
    this.mockFD = 0;
    this.running = false;
    this.fileMutex = new Mutex();
    this.cbMutex = new Mutex();
    this.cancelMutex = new Mutex();
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
        this.sendState(ImportState.FileAdded, newData);
      });
  }

  async cancelImport(id: ImportID): Promise<void> {
    // Lock the mutex, if id is not in the cancel list, add it
    await this.cancelMutex.runExclusive(async () => {
      if (!this.cancelList.find((item) => item === id)) {
        this.cancelList.push(id);
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

  private async sendState(state: ImportState, importData?: ImportData, stateData?: StateData): Promise<void> {
    await this.cbMutex.runExclusive(async () => {
      for (const elem of this.callbacks) {
        elem[1](state, importData, stateData);
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

  private async mockRemove(path: string): Promise<void> {
    console.log('Deleting', path);
  }

  private async doImport(data: ImportData): Promise<void> {
    this.sendState(ImportState.ImportStarted, data);
    const reader = data.file.stream().getReader();

    const result = await createFile(data.path);
    if (!result.ok) {
      console.log(`Failed to create file ${data.path} (reason: ${result.error.tag}), cancelling...`);
      this.sendState(ImportState.CreateFailed, data, { error: result.error.tag });
      return;
    }
    const fd = await this.mockOpen(data.path, 'w+');
    // Would prefer to use
    // for await (const chunk of data.file.stream()) {}
    // instead but it's not available on streams.

    this.sendState(ImportState.FileProgress, data, { progress: 0 });
    let writtenData = 0;
    // eslint-disable-next-line no-constant-condition
    while (true) {
      // Check if the import has been cancelled
      const shouldCancel = await this.cancelMutex.runExclusive(async () => {
        const index = this.cancelList.findIndex((item) => item === data.id);

        if (index !== -1) {
          // Remove from cancel list
          this.cancelList.splice(index, 1);
          return true;
        }
        return false;
      });

      if (shouldCancel) {
        // Delete the file
        await this.mockRemove(data.path);
        // Inform about the cancellation
        this.sendState(ImportState.Cancelled, data);
        return;
      }

      const buffer = await reader.read();
      if (buffer.done) {
        break;
      }
      await this.mockWrite(fd, buffer.value);
      writtenData += buffer.value.byteLength;
      this.sendState(ImportState.FileProgress, data, { progress: (writtenData / (data.file.size || 1)) * 100 });
    }
    await this.mockClose(fd);
    this.sendState(ImportState.FileProgress, data, { progress: 100 });
    this.sendState(ImportState.FileImported, data);
  }

  async stop(): Promise<void> {
    this.running = false;
  }

  async start(): Promise<void> {
    if (this.running) {
      return;
    }
    this.running = true;
    let importStarted = false;

    // eslint-disable-next-line no-constant-condition
    while (true) {
      if (!this.running) {
        break;
      }

      if (this.importJobs.length >= MAX_SIMULTANEOUS_IMPORT_JOBS) {
        // Remove the files that have been cancelled but have not yet
        // started their import

        await this.cancelMutex.runExclusive(async () => {
          const release = await this.fileMutex.acquire();

          try {
            for (const cancelId of this.cancelList.slice()) {
              const index = this.importData.findIndex((item) => item.id === cancelId);
              if (index !== -1) {
                // Remove from cancelList
                this.cancelList.slice(
                  this.cancelList.findIndex((item) => cancelId === item),
                  1,
                );
                // Inform of the cancel
                const elem = this.importData[index];
                this.sendState(ImportState.Cancelled, elem);
                // Remove from file list
                this.importData.slice(index, 1);
              }
            }
          } finally {
            release();
          }
        });
        await wait(500);
        continue;
      }
      const release = await this.fileMutex.acquire();
      let elem: ImportData | undefined = undefined;
      try {
        elem = this.importData.pop();
      } finally {
        release();
      }

      if (elem) {
        if (!importStarted) {
          this.sendState(ImportState.ImportAllStarted);
          importStarted = true;
        }
        const elemId = elem.id;
        // check if elem is in cancel list
        await this.cancelMutex.runExclusive(async () => {
          const index = this.cancelList.findIndex((item) => item === elemId);
          if (index !== -1) {
            this.cancelList.splice(index, 1);
            this.sendState(ImportState.Cancelled, elem as ImportData);
          }
        });
        console.log(`Importing file ${elem.file.name} to ${elem.path}`);
        const job = this.doImport(elem);
        this.importJobs.push(job);
        job.then(() => {
          const index = this.importJobs.findIndex((item) => item === job);
          if (index !== -1) {
            this.importJobs.splice(index, 1);
          }
          if (this.importJobs.length === 0 && this.importData.length === 0) {
            this.sendState(ImportState.ImportAllFinished);
            importStarted = false;
          }
        });
      } else {
        await wait(500);
      }
    }
  }
}

export { ImportData, ImportManager, ImportState };
