// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  DEFAULT_READ_SIZE,
  EntryName,
  EntryStatFile,
  EntryTree,
  FileDescriptor,
  FsPath,
  HistoryEntryTree,
  Path,
  WorkspaceCreateFolderErrorTag,
  WorkspaceFdWriteErrorTag,
  WorkspaceHandle,
  WorkspaceHistory,
  WorkspaceHistoryEntryStatFile,
  WorkspaceID,
  WorkspaceMoveEntryErrorTag,
  WorkspaceOpenFileErrorTag,
  closeFile,
  createFolder,
  createReadStream,
  deleteFile,
  entryStat,
  listTree,
  listTreeAt,
  moveEntry,
  openFile,
  readFile,
  rename,
  resizeFile,
  writeFile,
} from '@/parsec';
import { wait } from '@/parsec/internals';
import * as zipjs from '@zip.js/zip.js';
import { DateTime } from 'luxon';
import { FileSystemFileHandle } from 'native-file-system-adapter';
import { v4 as uuid4 } from 'uuid';

const MAX_SIMULTANEOUS_OPERATIONS = 3;
const MIN_OPERATION_TIME_MS = 500;

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
  ImportFailed,
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
  DownloadAdded,
  DownloadStarted,
  DownloadFailed,
  EntryDownloaded,
  DownloadArchiveAdded,
  DownloadArchiveStarted,
  DownloadArchiveFailed,
  ArchiveDownloaded,
}

export interface OperationProgressStateData {
  progress: number;
}

export interface DownloadOperationProgressStateData extends OperationProgressStateData {
  currentFile: EntryName;
  currentFileSize: number;
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
  | RestoreFailedStateData
  | DownloadOperationProgressStateData;

type FileOperationCallback = (state: FileOperationState, operationData?: FileOperationData, stateData?: StateData) => Promise<void>;
type FileOperationID = string;

export enum FileOperationDataType {
  Base,
  Import,
  Copy,
  Move,
  Restore,
  Download,
  DownloadArchive,
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

class DownloadData extends FileOperationData {
  path: FsPath;
  dateTime?: DateTime;
  saveHandle: FileSystemFileHandle;

  constructor(
    workspaceHandle: WorkspaceHandle,
    workspaceId: WorkspaceID,
    path: FsPath,
    saveHandle: FileSystemFileHandle,
    dateTime?: DateTime,
  ) {
    super(workspaceHandle, workspaceId);
    this.saveHandle = saveHandle;
    this.path = path;
    this.dateTime = dateTime;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.Download;
  }
}

class DownloadArchiveData extends FileOperationData {
  trees: Array<EntryTree>;
  saveHandle: FileSystemFileHandle;
  rootPath: FsPath;
  totalFiles: number;
  totalSize: number;

  constructor(
    workspaceHandle: WorkspaceHandle,
    workspaceId: WorkspaceID,
    saveHandle: FileSystemFileHandle,
    trees: Array<EntryTree>,
    rootPath: FsPath,
    totalFiles: number,
    totalSize: number,
  ) {
    super(workspaceHandle, workspaceId);
    this.saveHandle = saveHandle;
    this.trees = trees;
    this.rootPath = rootPath;
    this.totalFiles = totalFiles;
    this.totalSize = totalSize;
  }

  getDataType(): FileOperationDataType {
    return FileOperationDataType.DownloadArchive;
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

  async downloadEntry(
    workspaceHandle: WorkspaceHandle,
    workspaceId: WorkspaceID,
    saveStream: FileSystemFileHandle,
    path: FsPath,
    dateTime?: DateTime,
  ): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new DownloadData(workspaceHandle, workspaceId, path, saveStream, dateTime);
    await this.sendState(FileOperationState.DownloadAdded, newData);
    this.fileOperationData.unshift(newData);
  }

  async downloadArchive(
    workspaceHandle: WorkspaceHandle,
    workspaceId: WorkspaceID,
    saveStream: FileSystemFileHandle,
    trees: Array<EntryTree>,
    rootPath: FsPath,
    totalFiles: number,
    totalSize: number,
  ): Promise<void> {
    if (!this.isRunning) {
      this.start();
    }
    const newData = new DownloadArchiveData(workspaceHandle, workspaceId, saveStream, trees, rootPath, totalFiles, totalSize);
    await this.sendState(FileOperationState.DownloadArchiveAdded, newData);
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
    const start = DateTime.now();

    const statResult = await entryStat(data.workspaceHandle, data.srcPath);
    if (!statResult.ok) {
      await this.sendState(FileOperationState.CopyFailed, data, { error: CopyFailedError.SourceDoesNotExist });
      return;
    }
    const srcEntry = statResult.value;
    if (statResult.value.isFile()) {
      tree = {
        totalSize: Number((statResult.value as EntryStatFile).size),
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
      let filenameCount = 2;
      try {
        // Open the source
        const openReadResult = await openFile(data.workspaceHandle, entry.path, { read: true });
        if (!openReadResult.ok) {
          continue;
        }
        fdR = openReadResult.value;
        // Try to open the destination
        let openWriteResult = await openFile(data.workspaceHandle, dstPath, { write: true, createNew: true });
        const filename = (await Path.filename(dstPath)) || '';
        // If opened failed because the file already exists, we append a number to its name and try opening it again,
        // until that number reaches 10 or until we manage to open it
        while (
          !openWriteResult.ok &&
          openWriteResult.error.tag === WorkspaceOpenFileErrorTag.EntryExistsInCreateNewMode &&
          filenameCount < 10
        ) {
          const ext = Path.getFileExtension(dstPath);
          let newFilename = '';
          if (ext.length) {
            newFilename = `${Path.filenameWithoutExtension(filename)} (${filenameCount}).${ext}`;
          } else {
            newFilename = `${Path.filenameWithoutExtension(filename)} (${filenameCount})`;
          }
          dstPath = await Path.join(await Path.parent(dstPath), newFilename);
          openWriteResult = await openFile(data.workspaceHandle, dstPath, { write: true, createNew: true });
          filenameCount += 1;
        }
        // No luck, cancel the copy
        if (!openWriteResult.ok) {
          throw Error('Failed to open destination');
        }
        fdW = openWriteResult.value;

        // Resize the destination
        await resizeFile(data.workspaceHandle, fdW, Number(entry.size));

        let loop = true;
        let offset = 0;
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
          const readResult = await readFile(data.workspaceHandle, fdR, offset, DEFAULT_READ_SIZE);

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
          if (chunk.byteLength < DEFAULT_READ_SIZE) {
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
          return;
        }
        if (!copied) {
          // We don't want to delete anything if count is 10 since no file has been created
          if (!Path.areSame(data.srcPath, dstPath) && filenameCount < 10) {
            await deleteFile(data.workspaceHandle, dstPath);
          }
          await this.sendState(FileOperationState.CopyFailed, data, { error: CopyFailedError.OneFailed });
          return;
        }
      }
    }
    const end = DateTime.now();
    const diff = end.toMillis() - start.toMillis();

    if (diff < MIN_OPERATION_TIME_MS) {
      await wait(MIN_OPERATION_TIME_MS - diff);
    }
    await this.sendState(FileOperationState.EntryCopied, data);
  }

  private async doMove(data: MoveData): Promise<void> {
    await this.sendState(FileOperationState.MoveStarted, data);
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    const start = DateTime.now();
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
      const end = DateTime.now();
      const diff = end.toMillis() - start.toMillis();

      if (diff < MIN_OPERATION_TIME_MS) {
        await wait(MIN_OPERATION_TIME_MS - diff);
      }
      await this.sendState(FileOperationState.EntryMoved, data);
    }
  }

  private async doRestore(data: RestoreData): Promise<void> {
    await this.sendState(FileOperationState.RestoreStarted, data);
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    let tree: HistoryEntryTree | undefined;

    const start = DateTime.now();
    const history = new WorkspaceHistory(data.workspaceId);

    try {
      await history.start(data.dateTime);

      const statResult = await history.entryStat(data.path);
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
          totalSize: Number((statResult.value as WorkspaceHistoryEntryStatFile).size),
          entries: [statResult.value as WorkspaceHistoryEntryStatFile],
          maxRecursionReached: false,
          maxFilesReached: false,
        };
      } else {
        tree = await listTreeAt(history, data.path);
      }

      if (!tree) {
        return;
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
          const openReadResult = await history.openFile(entry.path);
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
          await resizeFile(data.workspaceHandle, fdW, Number(entry.size));

          let loop = true;
          let offset = 0;
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
            const readResult = await history.readFile(fdR, offset, DEFAULT_READ_SIZE);

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
            if (chunk.byteLength < DEFAULT_READ_SIZE) {
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
            await history.closeFile(fdR);
          }
          if (fdW !== null) {
            await closeFile(data.workspaceHandle, fdW);
          }
          if (cancelled) {
            await deleteFile(data.workspaceHandle, dstPath);
            await this.sendState(FileOperationState.Cancelled, data);
            return;
          }
          if (!restored) {
            await this.sendState(FileOperationState.RestoreFailed, data, {
              path: data.path,
              workspaceHandle: data.workspaceHandle,
              error: RestoreFailedError.OneFailed,
            });
            return;
          }
        }
      }
      const end = DateTime.now();
      const diff = end.toMillis() - start.toMillis();

      if (diff < MIN_OPERATION_TIME_MS) {
        await wait(MIN_OPERATION_TIME_MS - diff);
      }
      await this.sendState(FileOperationState.EntryRestored, data);
    } catch (e: any) {
      window.electronAPI.log('error', `Error while restoring: ${e.toString()}`);
    } finally {
      await history.stop();
    }
  }

  private async doDownload(data: DownloadData): Promise<void> {
    await this.sendState(FileOperationState.DownloadStarted, data);
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    // First, get the file size
    const statsResult = await entryStat(data.workspaceHandle, data.path);
    if (!statsResult.ok || !statsResult.value.isFile()) {
      await this.sendState(FileOperationState.DownloadFailed);
      return;
    }
    const fileSize = (statsResult.value as EntryStatFile).size;
    let writtenSize = 0;

    const rStream = await createReadStream(data.workspaceHandle, data.path);
    const wStream = await data.saveHandle.createWritable();
    const reader = rStream.getReader();
    let cancelled = false;

    try {
      const start = DateTime.now();
      while (true) {
        // Check if the download has been cancelled
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

        const { done, value } = await reader.read();
        if (done) {
          break;
        } else {
          const chunk = value as Uint8Array;
          await wStream.write(chunk);
          writtenSize += chunk.byteLength;
          await this.sendState(FileOperationState.OperationProgress, data, { progress: (writtenSize / (Number(fileSize) || 1)) * 100 });
        }
      }
      wStream.close();
      const end = DateTime.now();
      const diff = end.toMillis() - start.toMillis();

      if (diff < MIN_OPERATION_TIME_MS) {
        await wait(MIN_OPERATION_TIME_MS - diff);
      }
      await this.sendState(FileOperationState.EntryDownloaded, data);
    } catch (e: any) {
      await wStream.abort();
      data.saveHandle.remove();
      if (cancelled) {
        await this.sendState(FileOperationState.Cancelled, data);
      } else {
        window.electronAPI.log('warn', `Error while downloading file: ${e?.toString()}`);
        await this.sendState(FileOperationState.DownloadFailed, data);
      }
    }
  }

  private async doDownloadArchive(data: DownloadArchiveData): Promise<void> {
    await this.sendState(FileOperationState.DownloadArchiveStarted, data);
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    const totalSize = data.totalSize;
    let zipWriter: zipjs.ZipWriter<any> | undefined;
    let wStream: FileSystemWritableFileStream | undefined;
    let cancelled = false;

    try {
      wStream = await data.saveHandle.createWritable();
      zipWriter = new zipjs.ZipWriter(wStream, {
        level: 3,
        compressionMethod: 0x08,
        useUnicodeFileNames: true,
        supportZip64SplitFile: false,
        zip64: false,
      });

      const start = DateTime.now();
      let totalSizeRead = 0;

      for (const tree of data.trees) {
        for (const entry of tree.entries) {
          // Check if the download has been cancelled
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

          try {
            const rStream = await createReadStream(data.workspaceHandle, entry.path, async (sizeRead: number) => {
              totalSizeRead += sizeRead;
              await this.sendState(FileOperationState.OperationProgress, data, {
                progress: (totalSizeRead / totalSize) * 100,
                currentFile: entry.name,
                currentFileSize: entry.size,
              });
            });
            const relPath = entry.path.startsWith(data.rootPath) ? entry.path.slice(data.rootPath.length) : entry.path;
            await zipWriter.add(relPath, rStream);
          } catch (e: any) {
            window.electronAPI.log('error', `Failed to add file '${entry.name}' to archive`);
            throw e;
          }
        }
      }

      const end = DateTime.now();
      const diff = end.toMillis() - start.toMillis();

      if (diff < MIN_OPERATION_TIME_MS) {
        await wait(MIN_OPERATION_TIME_MS - diff);
      }
      await this.sendState(FileOperationState.ArchiveDownloaded, data);
    } catch (e: any) {
      if (wStream) {
        await wStream.abort();
      }
      data.saveHandle.remove();
      if (cancelled) {
        await this.sendState(FileOperationState.Cancelled, data);
      } else {
        window.electronAPI.log('warn', `Error while downloading archive: ${e?.toString()}`);
        await this.sendState(FileOperationState.DownloadArchiveFailed, data);
      }
    } finally {
      if (zipWriter) {
        zipWriter.close();
      }
    }
  }

  private async doImport(data: ImportData): Promise<void> {
    await this.sendState(FileOperationState.ImportStarted, data);
    const tmpFileName = `._${crypto.randomUUID()}`;
    const reader = data.file.stream().getReader();
    const tmpFilePath = await Path.join(data.path, tmpFileName);

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

    const start = DateTime.now();

    const openResult = await openFile(data.workspaceHandle, tmpFilePath, { write: true, truncate: true, create: true });

    if (!openResult.ok) {
      await this.sendState(FileOperationState.CreateFailed, data, { error: openResult.error.tag });
      return;
    }

    const fd = openResult.value;

    const resizeResult = await resizeFile(data.workspaceHandle, fd, data.file.size);
    if (!resizeResult.ok) {
      await closeFile(data.workspaceHandle, fd);
      await deleteFile(data.workspaceHandle, tmpFilePath);
      await this.sendState(FileOperationState.WriteError, data, { error: resizeResult.error.tag as unknown as WorkspaceFdWriteErrorTag });
      return;
    }

    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });
    let writtenData = 0;

    // Would prefer to use
    // for await (const chunk of data.file.stream()) {}
    // instead but it's not available on streams.

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
        await deleteFile(data.workspaceHandle, tmpFilePath);
        // Inform about the cancellation
        await this.sendState(FileOperationState.Cancelled, data);
        return;
      }

      const buffer = await reader.read();

      if (buffer.value) {
        const writeResult = await writeFile(data.workspaceHandle, fd, writtenData, buffer.value);

        if (!writeResult.ok) {
          await closeFile(data.workspaceHandle, fd);
          await deleteFile(data.workspaceHandle, tmpFilePath);
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

    // Rename tmp file to expected name
    const result = await rename(data.workspaceHandle, tmpFilePath, data.file.name, true);
    if (!result.ok) {
      await deleteFile(data.workspaceHandle, tmpFilePath);
      await this.sendState(FileOperationState.ImportFailed, data, { error: result.error.tag });
      return;
    }

    const end = DateTime.now();
    const diff = end.toMillis() - start.toMillis();

    if (diff < MIN_OPERATION_TIME_MS) {
      await wait(MIN_OPERATION_TIME_MS - diff);
    }

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
        } else if (elem.getDataType() === FileOperationDataType.Download) {
          job = this.doDownload(elem as DownloadData);
        } else if (elem.getDataType() === FileOperationDataType.DownloadArchive) {
          job = this.doDownloadArchive(elem as DownloadArchiveData);
        } else {
          console.warn(`Unhandled file operation '${elem.getDataType()}'`);
          continue;
        }
        this.operationJobs.push([elem.id, job]);
        job
          .catch((reason: any) => {
            window.electronAPI.log('error', `File operation unexpected failure: ${reason}`);
          })
          .finally(async () => {
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

export {
  CopyData,
  DownloadArchiveData,
  DownloadData,
  FileOperationData,
  FileOperationManager,
  FileOperationState,
  ImportData,
  MoveData,
  RestoreData,
};
