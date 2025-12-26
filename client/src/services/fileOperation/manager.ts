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
  getWorkspaceInfo,
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
import { FileOperationCopyData, FileOperationData, FileOperationDataType, FileOperationDownloadArchiveData, FileOperationDownloadData, FileOperationID, FileOperationImportData, FileOperationMoveData, FileOperationRestoreData } from '@/services/fileOperation/operationData';
import { FileEventRegistrationCanceller, FileOperationCallback, FileOperationEventDistributor, FileOperationEvents, OperationFailedErrors } from '@/services/fileOperation/events';
import { FileOperationCancelled, FileOperationException } from './types';

const MAX_SIMULTANEOUS_OPERATIONS = 3;
const MIN_OPERATION_TIME_MS = 500;

export const FileOperationManagerKey = 'fileOperationManager';

export class FileOperationManager {
  private cancelList: Array<FileOperationID>;
  private running: boolean;
  private operations: Map<FileOperationID, Promise<void>>;
  private pendingOperations: Array<FileOperationData>;
  private eventDistributor: FileOperationEventDistributor;

  constructor() {
    this.operations = new Map<FileOperationID, Promise<void>>;
    this.pendingOperations = [];
    this.cancelList = [];
    this.running = false;
    this.eventDistributor = new FileOperationEventDistributor();
    this.start();
  }

  get isRunning(): boolean {
    return this.running;
  }

  hasOperations(): boolean {
    return (this.operations.size + this.pendingOperations.length) > 0;
  }

  async import(workspaceHandle: WorkspaceHandle, files: Array<File>, destination: FsPath, replace = false): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationImportData = {
      type: FileOperationDataType.Import,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      files: files,
      destination: destination,
      replace: replace,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async move(workspaceHandle: WorkspaceHandle, sources: Array<FsPath>, destination: FsPath, replace = false): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationMoveData = {
      type: FileOperationDataType.Move,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      sources: sources,
      destination: destination,
      replace: replace,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async copy(workspaceHandle: WorkspaceHandle, sources: Array<FsPath>, destination: FsPath, replace = false): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationCopyData = {
      type: FileOperationDataType.Copy,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      sources: sources,
      destination: destination,
      replace: replace,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async restore(workspaceHandle: WorkspaceHandle, paths: Array<FsPath>, dateTime: DateTime, replace = false): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationRestoreData = {
      type: FileOperationDataType.Restore,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      paths: paths,
      dateTime: dateTime,
      replace: replace,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async download(workspaceHandle: WorkspaceHandle, path: FsPath, saveHandle: FileSystemFileHandle, dateTime?: DateTime): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationDownloadData = {
      type: FileOperationDataType.Download,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      path: path,
      saveHandle: saveHandle,
      dateTime: dateTime,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async downloadArchive(workspaceHandle: WorkspaceHandle, trees: Array<EntryTree>, saveHandle: FileSystemFileHandle, root: FsPath): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationDownloadArchiveData = {
      type: FileOperationDataType.DownloadArchive,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      trees: trees,
      saveHandle: saveHandle,
      rootPath: root,
      totalFiles: trees.reduce((acc, el) => acc + el.entries.length, 0),
      totalSize: trees.reduce((acc, el) => acc + el.totalSize, 0),
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async cancelOperation(id: FileOperationID): Promise<void> {
    if (!this.cancelList.find((item) => item === id)) {
      this.cancelList.push(id);
    }
  }

  async cancelAll(): Promise<void> {
    for (const elem of this.operations) {
      await this.cancelOperation(elem[0]);
    }
    for (const elem of this.pendingOperations) {
      await this.cancelOperation(elem.id);
    }
  }

  async registerCallback(cb: FileOperationCallback): Promise<FileEventRegistrationCanceller> {
    return this.eventDistributor.registerCallback(cb);
  }

  private async _doCopy(data: CopyData): Promise<void> {
    let tree: EntryTree;

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
  }

  private async _doMove(data: MoveData): Promise<void> {
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
      await this.sendState(FileOperationState.EntryMoved, data);
    }
  }

  private async _doRestore(data: RestoreData): Promise<void> {
    await this.sendState(FileOperationState.OperationProgress, data, { progress: 0 });

    let tree: HistoryEntryTree | undefined;

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
      await this.sendState(FileOperationState.EntryRestored, data);
    } catch (e: any) {
      window.electronAPI.log('error', `Error while restoring: ${e.toString()}`);
    } finally {
      await history.stop();
    }
  }

  private async _doDownload(data: DownloadData): Promise<void> {
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

  private async _doDownloadArchive(data: DownloadArchiveData): Promise<void> {
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

  private async _doImport(data: ImportData): Promise<void> {

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

  }

  async stop(): Promise<void> {
    await this.cancelAll();
    while (this.operations.size > 0) {
      await wait(100);
    }
    this.running = false;
  }

  async start(): Promise<void> {
    if (this.running) {
      return;
    }
    this.running = true;

    while (true) {
      if (!this.running) {
        break;
      }

      // Remove the files that have been cancelled but have not yet
      // started their import
      for (const cancelId of this.cancelList.slice()) {
        const index = this.pendingOperations.findIndex((item) => item.id === cancelId);
        if (index !== -1) {
          // Inform of the cancel
          const elem = this.pendingOperations[index];
          await this.eventDistributor.distribute(FileOperationEvents.Cancelled, elem);
          // Remove from file list
          this.pendingOperations.slice(index, 1);
          // Remove from cancelList
          this.cancelList.slice(
            this.cancelList.findIndex((item) => cancelId === item),
            1,
          );
        }
      }

      if (this.operations.size >= MAX_SIMULTANEOUS_OPERATIONS) {
        await wait(500);
        continue;
      }
      const elem = this.pendingOperations.pop();

      if (elem) {
        const elemId = elem.id;
        // check if elem is in cancel list
        const index = this.cancelList.findIndex((item) => item === elemId);
        if (index !== -1) {
          this.cancelList.splice(index, 1);
          await this.eventDistributor.distribute(FileOperationEvents.Cancelled, elem);
          continue;
        }
        let job: Promise<void>;
        const start = Date.now();
        switch (elem.type) {
          case FileOperationDataType.Import: {
            job = this._doImport(elem);
            break;
          }
          case FileOperationDataType.Move: {
            job = this._doMove(elem);
            break;
          }
          case FileOperationDataType.Copy: {
            job = this._doCopy(elem);
            break;
          }
          case FileOperationDataType.Restore: {
            job = this._doRestore(elem);
            break;
          }
          case FileOperationDataType.Download: {
            job = this._doDownload(elem);
            break;
          }
          case FileOperationDataType.DownloadArchive: {
            job = this._doDownloadArchive(elem);
            break;
          }
          default:
            window.electronAPI.log('warn', `Unhandled file operation '${elem.type}'`);
            continue;
        }
        await this.eventDistributor.distribute(FileOperationEvents.Started, elem);
        this.operations.set(elem.id, job);
        job
          .then(() => {
            this.eventDistributor.distribute(FileOperationEvents.Finished, elem);
          })
          .catch((exc: any) => {
            if (exc instanceof FileOperationCancelled) {
              this.eventDistributor.distribute(FileOperationEvents.Cancelled, elem);
            }
            if (exc instanceof FileOperationException) {
              this.eventDistributor.distribute(FileOperationEvents.Failed, elem, { error: exc.error, details: exc.details });
            } else {
              this.eventDistributor.distribute(FileOperationEvents.Failed, elem, { error: OperationFailedErrors.Unhandled, detail: exc.toString() });
            }
          })
          .finally(async () => {
            const end = Date.now();
            const diff = end - start;
            if (diff < MIN_OPERATION_TIME_MS) {
              await wait(MIN_OPERATION_TIME_MS - diff);
            }
            this.operations.delete(elem.id);
            if (this.operations.size === 0 && this.pendingOperations.length === 0) {
              await this.eventDistributor.distribute(FileOperationEvents.AllFinished);
            }
          });
      } else {
        await wait(500);
      }
    }
  }
}
