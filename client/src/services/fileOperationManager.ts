// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  EntryStatFile,
  EntryTree,
  FileDescriptor,
  FsPath,
  HistoryEntryTree,
  Path,
  WorkspaceCreateFolderErrorTag,
  WorkspaceFdWriteErrorTag,
  WorkspaceHandle,
  WorkspaceHistoryEntryStatFile,
  WorkspaceID,
  WorkspaceMoveEntryErrorTag,
  WorkspaceOpenFileErrorTag,
  closeFile,
  createFolder,
  deleteFile,
  entryStat,
  entryStatAt,
  listTree,
  listTreeAt,
  moveEntry,
  openFile,
  openFileAt,
  readFile,
  readHistoryFile,
  resizeFile,
  writeFile,
} from '@/parsec';
import { wait } from '@/parsec/internals';
import { DateTime } from 'luxon';
import { v4 as uuid4 } from 'uuid';

const MAX_SIMULTANEOUS_OPERATIONS = 3;

export const FileOperationManagerKey = 'fileOperationManager';

enum FileOperationState {
  OperationAllStarted = 1,
  OperationAllFinished,
  OperationProgress,
  FileImported,
  FileAdded,
  MoveAdded,
  CopyAdded,
  ImportStarted,
  MoveStarted,
  CopyStarted,
  CreateFailed,
  MoveFailed,
  CopyFailed,
  WriteError,
  Cancelled,
  FolderCreated,
  EntryMoved,
  EntryCopied,
  RestoreAdded,
  RestoreStarted,
  RestoreFailed,
  EntryRestored,
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

export interface MoveFailedStateData {
  error: WorkspaceMoveEntryErrorTag;
}

export enum CopyFailedError {
  MaxRecursionReached = 'max-recursion-reached',
  MaxFilesReached = 'max-files-reached',
  SourceDoesNotExist = 'source-does-not-exist',
  OneFailed = 'one-failed',
}

export interface CopyFailedStateData {
  error: CopyFailedError;
}

export enum RestoreFailedError {
  MaxRecursionReached = 'max-recursion-reached',
  MaxFilesReached = 'max-files-reached',
  SourceDoesNotExist = 'source-does-not-exist',
  OneFailed = 'one-failed',
}

export interface RestoreFailedStateData {
  error: RestoreFailedError;
  path: FsPath;
  workspaceHandle: WorkspaceHandle;
}

export type StateData =
  | OperationProgressStateData
  | CreateFailedStateData
  | WriteErrorStateData
  | FolderCreatedStateData
  | MoveFailedStateData
  | CopyFailedStateData
  | RestoreFailedStateData;

type FileOperationCallback = (state: FileOperationState, operationData?: FileOperationData, stateData?: StateData) => Promise<void>;
type FileOperationID = string;

export enum FileOperationDataType {
  Base,
  Import,
  Copy,
  Move,
  Restore,
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
  forceReplace: boolean;

  constructor(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, srcPath: FsPath, dstPath: FsPath, forceReplace: boolean) {
    super(workspaceHandle, workspaceId);
    this.srcPath = srcPath;
    this.dstPath = dstPath;
    this.forceReplace = forceReplace;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.Move;
  }
}

class RestoreData extends FileOperationData {
  path: FsPath;
  dateTime: DateTime;

  constructor(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, path: FsPath, dateTime: DateTime) {
    super(workspaceHandle, workspaceId);
    this.path = path;
    this.dateTime = dateTime;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.Restore;
  }
}

class FileOperationManager {
  private fileOperationData: Array<FileOperationData>;
  private callbacks: Array<[string, FileOperationCallback]>;
  private cancelList: Array<FileOperationID>;
  private running: boolean;
  private operationJobs: Array<[FileOperationID, Promise<void>]>;

  constructor() {
    this.fileOperationData = [];
    this.callbacks = [];
    this.cancelList = [];
    this.operationJobs = [];
    this.running = false;
    this.start();
  }

  get isRunning(): boolean {
    return this.running;
  }

  hasOperations(): boolean {
    return this.operationJobs.length + this.fileOperationData.length > 0;
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

  async moveEntry(
    workspaceHandle: WorkspaceHandle,
    workspaceId: WorkspaceID,
    srcPath: FsPath,
    dstPath: FsPath,
    forceReplace = false,
  ): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new MoveData(workspaceHandle, workspaceId, srcPath, dstPath, forceReplace);
    await this.sendState(FileOperationState.MoveAdded, newData);
    this.fileOperationData.unshift(newData);
  }

  async copyEntry(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, srcPath: FsPath, dstPath: FsPath): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new CopyData(workspaceHandle, workspaceId, srcPath, dstPath);
    await this.sendState(FileOperationState.CopyAdded, newData);
    this.fileOperationData.unshift(newData);
  }

  async restoreEntry(workspaceHandle: WorkspaceHandle, workspaceId: WorkspaceID, path: FsPath, dateTime: DateTime): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new RestoreData(workspaceHandle, workspaceId, path, dateTime);
    await this.sendState(FileOperationState.RestoreAdded, newData);
    this.fileOperationData.unshift(newData);
  }

  async cancelOperation(id: FileOperationID): Promise<void> {
    if (!this.cancelList.find((item) => item === id)) {
      this.cancelList.push(id);
    }
  }

  async cancelAll(): Promise<void> {
    for (const elem of this.operationJobs) {
      await this.cancelOperation(elem[0]);
    }
    for (const elem of this.fileOperationData) {
      await this.cancelOperation(elem.id);
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

  private async doCopy(data: CopyData): Promise<void> {
    await this.sendState(FileOperationState.CopyStarted, data);

    let tree: EntryTree;

    const statResult = await entryStat(data.workspaceHandle, data.srcPath);
    if (!statResult.ok) {
      await this.sendState(FileOperationState.CopyFailed, data, { error: CopyFailedError.SourceDoesNotExist });
      return;
    }
    const srcEntry = statResult.value;
    if (statResult.value.isFile()) {
      tree = {
        totalSize: (statResult.value as EntryStatFile).size,
        entries: [statResult.value as EntryStatFile],
        maxRecursionReached: false,
        maxFilesReached: false,
      };
    } else {
      tree = await listTree(data.workspaceHandle, data.srcPath);
    }

    // If we reach max recursion or max files, it's better to simply give up right at the start rather than
    // trying to copy incomplete data
    if (tree.maxRecursionReached) {
      await this.sendState(FileOperationState.CopyFailed, data, { error: CopyFailedError.MaxRecursionReached });
      return;
    } else if (tree.maxFilesReached) {
      await this.sendState(FileOperationState.CopyFailed, data, { error: CopyFailedError.MaxFilesReached });
      return;
    }
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    let totalSizeCopied = 0;

    for (const entry of tree.entries) {
      let dstPath: FsPath;
      if (srcEntry.isFile()) {
        dstPath = await Path.join(data.dstPath, entry.name);
      } else {
        const srcParent = await Path.parent(data.srcPath);
        const relativePath = entry.path.substring(srcParent.length);
        dstPath = `${data.dstPath}/${relativePath}`;
      }
      const dstDir = await Path.parent(dstPath);
      if (dstDir !== '/') {
        const result = await createFolder(data.workspaceHandle, dstDir);
        if (result.ok) {
          await this.sendState(FileOperationState.FolderCreated, undefined, { path: dstDir, workspaceHandle: data.workspaceHandle });
        } else if (!result.ok && result.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
          // No need to go further if the folder creation failed
          continue;
        }
      }
      // Welcome to hell
      let fdR: FileDescriptor | null = null;
      let fdW: FileDescriptor | null = null;
      let copied = false;
      let cancelled = false;
      try {
        // Open the source
        const openReadResult = await openFile(data.workspaceHandle, entry.path, { read: true });
        if (!openReadResult.ok) {
          continue;
        }
        fdR = openReadResult.value;
        // Try to open the destination
        let openWriteResult = await openFile(data.workspaceHandle, dstPath, { write: true, createNew: true });
        let count = 2;
        // If opened failed because the file already exists, we append a number to its name and try opening it again,
        // until that number reaches 10 or until we manage to open it
        while (!openWriteResult.ok && openWriteResult.error.tag === WorkspaceOpenFileErrorTag.EntryExistsInCreateNewMode && count < 10) {
          const filename = (await Path.filename(dstPath)) || '';
          const ext = Path.getFileExtension(dstPath);
          let newFilename = '';
          if (ext.length) {
            newFilename = `${Path.filenameWithoutExtension(filename)} (${count}).${ext}`;
          } else {
            newFilename = `${Path.filenameWithoutExtension(filename)} (${count})`;
          }
          dstPath = await Path.join(await Path.parent(dstPath), newFilename);
          openWriteResult = await openFile(data.workspaceHandle, dstPath, { write: true, createNew: true });
          count += 1;
        }
        // No luck, cancel the copy
        if (!openWriteResult.ok) {
          throw Error('Failed to open destination');
        }
        fdW = openWriteResult.value;

        // Resize the destination
        await resizeFile(data.workspaceHandle, fdW, entry.size);

        let loop = true;
        let offset = 0;
        const READ_CHUNK_SIZE = 1_000_000;
        while (loop) {
          // Check if the copy has been cancelled
          let shouldCancel = false;
          const index = this.cancelList.findIndex((item) => item === data.id);

          if (index !== -1) {
            // Remove from cancel list
            this.cancelList.splice(index, 1);
            shouldCancel = true;
          }
          if (shouldCancel) {
            cancelled = true;
            throw Error('cancelled');
          }

          // Read the source
          const readResult = await readFile(data.workspaceHandle, fdR, offset, READ_CHUNK_SIZE);

          // Failed to read, cancel the copy
          if (!readResult.ok) {
            throw Error('Failed to read the source');
          }
          const chunk = readResult.value;
          const writeResult = await writeFile(data.workspaceHandle, fdW, offset, new Uint8Array(chunk));

          // Failed to write, or not everything's been written
          if (!writeResult.ok || writeResult.value < chunk.byteLength) {
            throw Error('Failed to write the destination');
          }
          // Smaller that what we asked for, we're at the end of the file
          if (chunk.byteLength < READ_CHUNK_SIZE) {
            loop = false;
          } else {
            // Otherwise, move the offset and keep going
            offset += chunk.byteLength;
          }
          totalSizeCopied += chunk.byteLength;
          await this.sendState(FileOperationState.OperationProgress, data, { progress: (totalSizeCopied / tree.totalSize) * 100 });
        }
        copied = true;
      } catch (e: any) {
        console.warn(`Failed to copy file: ${e}`);
      } finally {
        if (fdR !== null) {
          await closeFile(data.workspaceHandle, fdR);
        }
        if (fdW !== null) {
          await closeFile(data.workspaceHandle, fdW);
        }
        if (cancelled) {
          await deleteFile(data.workspaceHandle, dstPath);
          await this.sendState(FileOperationState.Cancelled, data);
          // eslint-disable-next-line no-unsafe-finally
          return;
        }
        if (!copied) {
          await deleteFile(data.workspaceHandle, dstPath);
          await this.sendState(FileOperationState.CopyFailed, data, { error: CopyFailedError.OneFailed });
          // eslint-disable-next-line no-unsafe-finally
          return;
        }
      }
    }
    await this.sendState(FileOperationState.EntryCopied, data);
  }

  private async doMove(data: MoveData): Promise<void> {
    await this.sendState(FileOperationState.MoveStarted, data);
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    let moveResult = await moveEntry(data.workspaceHandle, data.srcPath, data.dstPath, data.forceReplace);
    if (!moveResult.ok) {
      let i = 2;
      // If an entry with the same name already exists, we try appending a number at the end of the file.
      // We only try until we reach 9.
      while (!moveResult.ok && moveResult.error.tag === WorkspaceMoveEntryErrorTag.DestinationExists && i < 10) {
        const filename = (await Path.filename(data.srcPath)) || '';
        const ext = Path.getFileExtension(data.srcPath);
        let newFilename = '';
        if (ext.length) {
          newFilename = `${Path.filenameWithoutExtension(filename)} (${i}).${ext}`;
        } else {
          newFilename = `${Path.filenameWithoutExtension(filename)} (${i})`;
        }
        const dstPath = await Path.join(await Path.parent(data.dstPath), newFilename);
        moveResult = await moveEntry(data.workspaceHandle, data.srcPath, data.dstPath, false);
        if (moveResult.ok) {
          data.dstPath = dstPath;
          await this.sendState(FileOperationState.EntryMoved, data);
        }
        i++;
      }
      if (!moveResult.ok) {
        await this.sendState(FileOperationState.MoveFailed, data, { error: moveResult.error.tag });
      }
    } else {
      await this.sendState(FileOperationState.EntryMoved, data);
    }
  }

  private async doRestore(data: RestoreData): Promise<void> {
    await this.sendState(FileOperationState.RestoreStarted, data);
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    let tree: HistoryEntryTree;

    const statResult = await entryStatAt(data.workspaceHandle, data.path, data.dateTime);
    if (!statResult.ok) {
      await this.sendState(FileOperationState.RestoreFailed, data, {
        path: data.path,
        workspaceHandle: data.workspaceHandle,
        error: RestoreFailedError.SourceDoesNotExist,
      });
      return;
    }
    if (statResult.value.isFile()) {
      tree = {
        totalSize: (statResult.value as WorkspaceHistoryEntryStatFile).size,
        entries: [statResult.value as WorkspaceHistoryEntryStatFile],
        maxRecursionReached: false,
        maxFilesReached: false,
      };
    } else {
      tree = await listTreeAt(data.workspaceHandle, data.path, data.dateTime);
    }

    // If we reach max recursion or max files, it's better to simply give up right at the start rather than
    // trying to copy incomplete data
    if (tree.maxRecursionReached) {
      await this.sendState(FileOperationState.RestoreFailed, data, {
        path: data.path,
        workspaceHandle: data.workspaceHandle,
        error: RestoreFailedError.MaxRecursionReached,
      });
      return;
    } else if (tree.maxFilesReached) {
      await this.sendState(FileOperationState.RestoreFailed, data, {
        path: data.path,
        workspaceHandle: data.workspaceHandle,
        error: RestoreFailedError.MaxFilesReached,
      });
      return;
    }
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    let totalSizeRestored = 0;

    for (const entry of tree.entries) {
      const dstPath = entry.path;
      const dstDir = await Path.parent(dstPath);
      if (dstDir !== '/') {
        const result = await createFolder(data.workspaceHandle, dstDir);
        if (result.ok) {
          await this.sendState(FileOperationState.FolderCreated, undefined, { path: dstDir, workspaceHandle: data.workspaceHandle });
        } else if (!result.ok && result.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
          // No need to go further if the folder creation failed
          continue;
        }
      }
      let fdR: FileDescriptor | null = null;
      let fdW: FileDescriptor | null = null;
      let restored = false;
      let cancelled = false;
      try {
        // Open the source
        const openReadResult = await openFileAt(data.workspaceHandle, entry.path, data.dateTime);
        if (!openReadResult.ok) {
          continue;
        }
        fdR = openReadResult.value;
        // Try to open the destination
        const openWriteResult = await openFile(data.workspaceHandle, dstPath, { write: true, truncate: true, create: true });
        // No luck, cancel the copy
        if (!openWriteResult.ok) {
          throw Error('Failed to open destination');
        }
        fdW = openWriteResult.value;

        // Resize the destination
        await resizeFile(data.workspaceHandle, fdW, entry.size);

        let loop = true;
        let offset = 0;
        const READ_CHUNK_SIZE = 1_000_000;
        while (loop) {
          // Check if the copy has been cancelled
          let shouldCancel = false;
          const index = this.cancelList.findIndex((item) => item === data.id);

          if (index !== -1) {
            // Remove from cancel list
            this.cancelList.splice(index, 1);
            shouldCancel = true;
          }
          if (shouldCancel) {
            cancelled = true;
            throw Error('cancelled');
          }

          // Read the source
          const readResult = await readHistoryFile(data.workspaceHandle, fdR, offset, READ_CHUNK_SIZE);

          // Failed to read, cancel the copy
          if (!readResult.ok) {
            throw Error('Failed to read the source');
          }
          const chunk = readResult.value;
          const writeResult = await writeFile(data.workspaceHandle, fdW, offset, new Uint8Array(chunk));

          // Failed to write, or not everything's been written
          if (!writeResult.ok || writeResult.value < chunk.byteLength) {
            throw Error('Failed to write the destination');
          }
          // Smaller that what we asked for, we're at the end of the file
          if (chunk.byteLength < READ_CHUNK_SIZE) {
            loop = false;
          } else {
            // Otherwise, move the offset and keep going
            offset += chunk.byteLength;
          }
          totalSizeRestored += chunk.byteLength;
          await this.sendState(FileOperationState.OperationProgress, data, { progress: (totalSizeRestored / tree.totalSize) * 100 });
        }
        restored = true;
      } catch (e: any) {
        console.warn(`Failed to restore file: ${e}`);
      } finally {
        if (fdR !== null) {
          await closeFile(data.workspaceHandle, fdR);
        }
        if (fdW !== null) {
          await closeFile(data.workspaceHandle, fdW);
        }
        if (cancelled) {
          await deleteFile(data.workspaceHandle, dstPath);
          await this.sendState(FileOperationState.Cancelled, data);
          // eslint-disable-next-line no-unsafe-finally
          return;
        }
        if (!restored) {
          await this.sendState(FileOperationState.RestoreFailed, data, {
            path: data.path,
            workspaceHandle: data.workspaceHandle,
            error: RestoreFailedError.OneFailed,
          });
          // eslint-disable-next-line no-unsafe-finally
          return;
        }
      }
    }
    await this.sendState(FileOperationState.EntryRestored, data);
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
    while (this.operationJobs.length > 0) {
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

      if (this.operationJobs.length >= MAX_SIMULTANEOUS_OPERATIONS) {
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
        let job: Promise<void>;
        if (elem.getDataType() === FileOperationDataType.Import) {
          job = this.doImport(elem as ImportData);
        } else if (elem.getDataType() === FileOperationDataType.Move) {
          job = this.doMove(elem as MoveData);
        } else if (elem.getDataType() === FileOperationDataType.Copy) {
          job = this.doCopy(elem as CopyData);
        } else if (elem.getDataType() === FileOperationDataType.Restore) {
          job = this.doRestore(elem as RestoreData);
        } else {
          console.warn(`Unhandled file operation '${elem.getDataType()}'`);
          continue;
        }
        this.operationJobs.push([elem.id, job]);
        job.then(async () => {
          const index = this.operationJobs.findIndex((item) => item[1] === job);
          if (index !== -1) {
            this.operationJobs.splice(index, 1);
          }
          if (this.operationJobs.length === 0 && this.fileOperationData.length === 0) {
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

export { CopyData, FileOperationData, FileOperationManager, FileOperationState, ImportData, MoveData, RestoreData };
