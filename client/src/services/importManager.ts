// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  FsPath,
  Path,
  WorkspaceCreateFolderErrorTag,
  WorkspaceFdWriteErrorTag,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceOpenFileErrorTag,
  closeFile,
  createFolder,
  deleteFile,
  openFile,
  resizeFile,
  writeFile,
} from '@/parsec';
import { wait } from '@/parsec/internals';
import { v4 as uuid4 } from 'uuid';

const MAX_SIMULTANEOUS_IMPORT_JOBS = 5;

export const ImportManagerKey = 'importManager';

enum ImportState {
  ImportAllStarted = 1,
  ImportAllFinished,
  FileProgress,
  FileImported,
  FileAdded,
  ImportStarted,
  CreateFailed,
  WriteError,
  Cancelled,
  FolderCreated,
}

export interface FileProgressStateData {
  progress: number;
}

export interface CreateFailedStateData {
  error: WorkspaceOpenFileErrorTag | WorkspaceCreateFolderErrorTag;
}

export interface WriteErrorStateData {
  error: WorkspaceFdWriteErrorTag;
}

export interface FolderCreatedStateData {
  path: FsPath;
  workspaceHandle: WorkspaceHandle;
}

export type StateData = FileProgressStateData | CreateFailedStateData | WriteErrorStateData | FolderCreatedStateData;

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
  private running: boolean;
  private importJobs: Array<[ImportID, Promise<void>]>;

  constructor() {
    this.importData = [];
    this.callbacks = [];
    this.cancelList = [];
    this.importJobs = [];
    this.running = false;
    this.start();
  }

  get isRunning(): boolean {
    return this.running;
  }

  isImporting(): boolean {
    return this.importJobs.length + this.importData.length > 0;
  }

  async importFile(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, file: File, path: FsPath): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new ImportData(workspaceHandle, workspaceId, file, path);
    // Sending the information before adding to the list, else the file may get imported before
    // we've even informed that it was added
    await this.sendState(ImportState.FileAdded, newData);
    this.importData.unshift(newData);
  }

  async cancelImport(id: ImportID): Promise<void> {
    if (!this.cancelList.find((item) => item === id)) {
      this.cancelList.push(id);
    }
  }

  async cancelAll(): Promise<void> {
    for (const elem of this.importJobs) {
      await this.cancelImport(elem[0]);
    }
    for (const elem of this.importData) {
      await this.cancelImport(elem.id);
    }
  }

  async registerCallback(cb: FileImportCallback): Promise<string> {
    const id = uuid4();
    this.callbacks.push([id, cb]);
    return id;
  }

  async removeCallback(id: string): Promise<void> {
    this.callbacks = this.callbacks.filter((elem) => elem[0] !== id);
  }

  private async sendState(state: ImportState, importData?: ImportData, stateData?: StateData): Promise<void> {
    for (const elem of this.callbacks) {
      await elem[1](state, importData, stateData);
    }
  }

  private async doImport(data: ImportData): Promise<void> {
    await this.sendState(ImportState.ImportStarted, data);
    const reader = data.file.stream().getReader();
    const filePath = await Path.join(data.path, data.file.name);

    if (data.path !== '/') {
      const result = await createFolder(data.workspaceHandle, data.path);
      if (result.ok) {
        await this.sendState(ImportState.FolderCreated, undefined, { path: data.path, workspaceHandle: data.workspaceHandle });
      } else if (!result.ok && result.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
        console.log(`Failed to create folder ${data.path} (reason: ${result.error.tag}), cancelling...`);
        await this.sendState(ImportState.CreateFailed, data, { error: result.error.tag });
        // No need to go further if the folder creation failed
        return;
      }
    }

    const openResult = await openFile(data.workspaceHandle, filePath, { write: true, truncate: true, create: true });

    if (!openResult.ok) {
      await this.sendState(ImportState.CreateFailed, data, { error: openResult.error.tag });
      return;
    }

    const fd = openResult.value;

    const resizeResult = await resizeFile(data.workspaceHandle, fd, data.file.size);
    if (!resizeResult.ok) {
      await closeFile(data.workspaceHandle, fd);
      await deleteFile(data.workspaceHandle, filePath);
      await this.sendState(ImportState.WriteError, data, { error: resizeResult.error.tag as unknown as WorkspaceFdWriteErrorTag });
      return;
    }

    await this.sendState(ImportState.FileProgress, data, { progress: 0 });
    let writtenData = 0;

    // Would prefer to use
    // for await (const chunk of data.file.stream()) {}
    // instead but it's not available on streams.

    // eslint-disable-next-line no-constant-condition
    while (true) {
      // Check if the import has been cancelled
      let shouldCancel = false;
      const index = this.cancelList.findIndex((item) => item === data.id);

      if (index !== -1) {
        // Remove from cancel list
        this.cancelList.splice(index, 1);
        shouldCancel = true;
      }
      if (shouldCancel) {
        // Close the file
        await closeFile(data.workspaceHandle, fd);
        // Delete the file
        await deleteFile(data.workspaceHandle, filePath);
        // Inform about the cancellation
        await this.sendState(ImportState.Cancelled, data);
        return;
      }

      const buffer = await reader.read();

      if (buffer.value) {
        const writeResult = await writeFile(data.workspaceHandle, fd, writtenData, buffer.value);

        if (!writeResult.ok) {
          await closeFile(data.workspaceHandle, fd);
          await deleteFile(data.workspaceHandle, filePath);
          await this.sendState(ImportState.WriteError, data, { error: writeResult.error.tag });
          return;
        } else {
          writtenData += writeResult.value;
          await this.sendState(ImportState.FileProgress, data, { progress: (writtenData / (data.file.size || 1)) * 100 });
        }
      }
      if (buffer.done) {
        break;
      }
    }
    await closeFile(data.workspaceHandle, fd);
    await this.sendState(ImportState.FileImported, data);
  }

  async stop(): Promise<void> {
    await this.cancelAll();
    while (this.importJobs.length > 0) {
      await wait(100);
    }
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
            await this.sendState(ImportState.Cancelled, elem);
            // Remove from file list
            this.importData.slice(index, 1);
          }
        }

        await wait(500);
        continue;
      }
      let elem: ImportData | undefined = undefined;
      elem = this.importData.pop();

      if (elem) {
        if (!importStarted) {
          await this.sendState(ImportState.ImportAllStarted);
          importStarted = true;
        }
        const elemId = elem.id;
        // check if elem is in cancel list
        const index = this.cancelList.findIndex((item) => item === elemId);
        if (index !== -1) {
          this.cancelList.splice(index, 1);
          await this.sendState(ImportState.Cancelled, elem as ImportData);
          continue;
        }
        const job = this.doImport(elem);
        this.importJobs.push([elem.id, job]);
        job.then(async () => {
          const index = this.importJobs.findIndex((item) => item[1] === job);
          if (index !== -1) {
            this.importJobs.splice(index, 1);
          }
          if (this.importJobs.length === 0 && this.importData.length === 0) {
            await this.sendState(ImportState.ImportAllFinished);
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

// Keeping this in case we have trouble with the no-mutex version
/*
class ImportManager {
  private importData: Array<ImportData>;
  private callbacks: Array<[string, FileImportCallback]>;
  private cancelList: Array<ImportID>;
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
    // Sending the information before adding to the list, else the file may get imported before
    // we've even informed that it was added
    this.sendState(ImportState.FileAdded, newData);
    await this.fileMutex
      .runExclusive(async () => {
        console.log(`Adding file ${file.name} to import to ${path}`);
        this.importData.unshift(newData);
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

  private async doImport(data: ImportData): Promise<void> {
    this.sendState(ImportState.ImportStarted, data);
    const reader = data.file.stream().getReader();
    const filePath = await Path.join(data.path, data.file.name);

    const result = await createFolder(data.path);
    if (result.ok) {
      this.sendState(ImportState.FolderCreated, undefined, { path: data.path });
    } else if (!result.ok && result.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
      console.log(`Failed to create folder ${data.path} (reason: ${result.error.tag}), cancelling...`);
      // No need to go further if the folder creation failed
      return;
    }

    const openResult = await openFile(filePath, { write: true, truncate: true, create: true });

    if (!openResult.ok) {
      this.sendState(ImportState.CreateFailed, data, { error: openResult.error.tag });
      return;
    }

    const fd = openResult.value;

    const resizeResult = await resizeFile(fd, data.file.size);
    if (!resizeResult.ok) {
      await closeFile(fd);
      await deleteFile(filePath);
      this.sendState(ImportState.WriteError, data, { error: resizeResult.error.tag as unknown as WorkspaceFdWriteErrorTag });
      return;
    }

    this.sendState(ImportState.FileProgress, data, { progress: 0 });
    let writtenData = 0;

    // Would prefer to use
    // for await (const chunk of data.file.stream()) {}
    // instead but it's not available on streams.

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
        // Close the file
        await closeFile(fd);
        // Delete the file
        await deleteFile(filePath);
        // Inform about the cancellation
        this.sendState(ImportState.Cancelled, data);
        return;
      }

      const buffer = await reader.read();

      if (buffer.value) {
        const writeResult = await writeFile(fd, writtenData, buffer.value);

        if (!writeResult.ok) {
          await closeFile(fd);
          await deleteFile(filePath);
          this.sendState(ImportState.WriteError, data, { error: writeResult.error.tag });
          return;
        } else {
          writtenData += writeResult.value;
          this.sendState(ImportState.FileProgress, data, { progress: (writtenData / (data.file.size || 1)) * 100 });
        }
      }
      if (buffer.done) {
        break;
      }
    }
    await closeFile(fd);
    console.log('File', data.file.name, 'imported');
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
*/
