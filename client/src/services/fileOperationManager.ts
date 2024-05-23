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

export const FileOperationManagerKey = 'fileOperationManager';

enum FileOperationState {
  OperationAllStarted = 1,
  OperationAllFinished,
  OperationProgress,
  FileImported,
  FileAdded,
  ImportStarted,
  CreateFailed,
  WriteError,
  Cancelled,
  FolderCreated,
}

export interface OperationProgressStateData {
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

export type StateData = OperationProgressStateData | CreateFailedStateData | WriteErrorStateData | FolderCreatedStateData;

type FileOperationCallback = (state: FileOperationState, operationData?: FileOperationData, stateData?: StateData) => Promise<void>;
type FileOperationID = string;

export enum FileOperationDataType {
  Base,
  Import,
  Copy,
  Move,
}

interface IFileOperationDataType {
  getDataType(): FileOperationDataType;
}

class FileOperationData implements IFileOperationDataType {
  id: FileOperationID;
  workspaceHandle: WorkspaceHandle;
  workspaceId: WorkspaceID;

  constructor(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID) {
    this.id = uuid4();
    this.workspaceHandle = workspaceHandle;
    this.workspaceId = workspaceId;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.Base;
  }
}

class ImportData extends FileOperationData {
  file: File;
  path: FsPath;

  constructor(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, file: File, path: FsPath) {
    super(workspaceHandle, workspaceId);
    this.file = file;
    this.path = path;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.Import;
  }
}

class CopyData extends FileOperationData {
  srcPath: FsPath;
  dstPath: FsPath;

  constructor(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, srcPath: FsPath, dstPath: FsPath) {
    super(workspaceHandle, workspaceId);
    this.srcPath = srcPath;
    this.dstPath = dstPath;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.Copy;
  }
}

class MoveData extends FileOperationData {
  srcPath: FsPath;
  dstPath: FsPath;

  constructor(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, srcPath: FsPath, dstPath: FsPath) {
    super(workspaceHandle, workspaceId);
    this.srcPath = srcPath;
    this.dstPath = dstPath;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.Move;
  }
}

class FileOperationManager {
  private fileOperationData: Array<FileOperationData>;
  private callbacks: Array<[string, FileOperationCallback]>;
  private cancelList: Array<FileOperationID>;
  private running: boolean;
  private importJobs: Array<[FileOperationID, Promise<void>]>;

  constructor() {
    this.fileOperationData = [];
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
    return this.importJobs.length + this.fileOperationData.length > 0;
  }

  async importFile(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, file: File, path: FsPath): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new ImportData(workspaceHandle, workspaceId, file, path);
    // Sending the information before adding to the list, else the file may get imported before
    // we've even informed that it was added
    await this.sendState(FileOperationState.FileAdded, newData);
    this.fileOperationData.unshift(newData);
  }

  async moveEntry(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, srcPath: FsPath, dstPath: FsPath): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new MoveData(workspaceHandle, workspaceId, srcPath, dstPath);
    await this.sendState(FileOperationState.FileAdded, newData);
    this.fileOperationData.unshift(newData);
  }

  async copyEntry(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, srcPath: FsPath, dstPath: FsPath): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new CopyData(workspaceHandle, workspaceId, srcPath, dstPath);
    await this.sendState(FileOperationState.FileAdded, newData);
    this.fileOperationData.unshift(newData);
  }

  async cancelImport(id: FileOperationID): Promise<void> {
    if (!this.cancelList.find((item) => item === id)) {
      this.cancelList.push(id);
    }
  }

  async cancelAll(): Promise<void> {
    for (const elem of this.importJobs) {
      await this.cancelImport(elem[0]);
    }
    for (const elem of this.fileOperationData) {
      await this.cancelImport(elem.id);
    }
  }

  async registerCallback(cb: FileOperationCallback): Promise<string> {
    const id = uuid4();
    this.callbacks.push([id, cb]);
    return id;
  }

  async removeCallback(id: string): Promise<void> {
    this.callbacks = this.callbacks.filter((elem) => elem[0] !== id);
  }

  private async sendState(state: FileOperationState, operationData?: FileOperationData, stateData?: StateData): Promise<void> {
    for (const elem of this.callbacks) {
      await elem[1](state, operationData, stateData);
    }
  }

  private async doImport(data: ImportData): Promise<void> {
    await this.sendState(FileOperationState.ImportStarted, data);
    const reader = data.file.stream().getReader();
    const filePath = await Path.join(data.path, data.file.name);

    if (data.path !== '/') {
      const result = await createFolder(data.workspaceHandle, data.path);
      if (result.ok) {
        await this.sendState(FileOperationState.FolderCreated, undefined, { path: data.path, workspaceHandle: data.workspaceHandle });
      } else if (!result.ok && result.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
        console.log(`Failed to create folder ${data.path} (reason: ${result.error.tag}), cancelling...`);
        await this.sendState(FileOperationState.CreateFailed, data, { error: result.error.tag });
        // No need to go further if the folder creation failed
        return;
      }
    }

    const openResult = await openFile(data.workspaceHandle, filePath, { write: true, truncate: true, create: true });

    if (!openResult.ok) {
      await this.sendState(FileOperationState.CreateFailed, data, { error: openResult.error.tag });
      return;
    }

    const fd = openResult.value;

    const resizeResult = await resizeFile(data.workspaceHandle, fd, data.file.size);
    if (!resizeResult.ok) {
      await closeFile(data.workspaceHandle, fd);
      await deleteFile(data.workspaceHandle, filePath);
      await this.sendState(FileOperationState.WriteError, data, { error: resizeResult.error.tag as unknown as WorkspaceFdWriteErrorTag });
      return;
    }

    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });
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
        await this.sendState(FileOperationState.Cancelled, data);
        return;
      }

      const buffer = await reader.read();

      if (buffer.value) {
        const writeResult = await writeFile(data.workspaceHandle, fd, writtenData, buffer.value);

        if (!writeResult.ok) {
          await closeFile(data.workspaceHandle, fd);
          await deleteFile(data.workspaceHandle, filePath);
          await this.sendState(FileOperationState.WriteError, data, { error: writeResult.error.tag });
          return;
        } else {
          writtenData += writeResult.value;
          await this.sendState(FileOperationState.OperationProgress, data, { progress: (writtenData / (data.file.size || 1)) * 100 });
        }
      }
      if (buffer.done) {
        break;
      }
    }
    await closeFile(data.workspaceHandle, fd);
    await this.sendState(FileOperationState.FileImported, data);
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
          const index = this.fileOperationData.findIndex((item) => item.id === cancelId);
          if (index !== -1) {
            // Remove from cancelList
            this.cancelList.slice(
              this.cancelList.findIndex((item) => cancelId === item),
              1,
            );
            // Inform of the cancel
            const elem = this.fileOperationData[index];
            await this.sendState(FileOperationState.Cancelled, elem);
            // Remove from file list
            this.fileOperationData.slice(index, 1);
          }
        }

        await wait(500);
        continue;
      }
      const elem = this.fileOperationData.pop();

      if (elem) {
        if (!importStarted) {
          await this.sendState(FileOperationState.OperationAllStarted);
          importStarted = true;
        }
        const elemId = elem.id;
        // check if elem is in cancel list
        const index = this.cancelList.findIndex((item) => item === elemId);
        if (index !== -1) {
          this.cancelList.splice(index, 1);
          await this.sendState(FileOperationState.Cancelled, elem as ImportData);
          continue;
        }
        const job = this.doImport(elem as ImportData);
        this.importJobs.push([elem.id, job]);
        job.then(async () => {
          const index = this.importJobs.findIndex((item) => item[1] === job);
          if (index !== -1) {
            this.importJobs.splice(index, 1);
          }
          if (this.importJobs.length === 0 && this.fileOperationData.length === 0) {
            await this.sendState(FileOperationState.OperationAllFinished);
            importStarted = false;
          }
        });
      } else {
        await wait(500);
      }
    }
  }
}

export { CopyData, FileOperationData, FileOperationManager, FileOperationState, ImportData, MoveData };
