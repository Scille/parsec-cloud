// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  createFolder,
  createReadStream,
  EntryStat,
  EntryStatFile,
  EntryStatFolder,
  EntryTree,
  FsPath,
  getWorkspaceInfo,
  HistoryEntryTree,
  listTree,
  listTreeAt,
  moveEntry,
  Path,
  WorkspaceCreateFolderErrorTag,
  WorkspaceHandle,
  WorkspaceHistory,
  WorkspaceHistoryEntryStat,
  WorkspaceHistoryEntryStatFile,
} from '@/parsec';
import { wait } from '@/parsec/internals';
import { stringifyError } from '@/parsec/utils';
import {
  FileEventRegistrationCanceller,
  FileOperationCallback,
  FileOperationEventDistributor,
  FileOperationEvents,
} from '@/services/fileOperation/events';
import {
  FileOperationCopyData,
  FileOperationData,
  FileOperationDataType,
  FileOperationDownloadArchiveData,
  FileOperationDownloadData,
  FileOperationID,
  FileOperationImportData,
  FileOperationMoveData,
  FileOperationRestoreData,
} from '@/services/fileOperation/operationData';
import { copyFile, copyFolder, importFile, moveWithCounter, OperationTransaction, restoreFile } from '@/services/fileOperation/operations';
import { DuplicatePolicy, FileOperationCancelled, FileOperationException, OperationFailedErrors } from '@/services/fileOperation/types';
import * as zipjs from '@zip.js/zip.js';
import { DateTime } from 'luxon';
import { FileSystemFileHandle } from 'native-file-system-adapter';
import { v4 as uuid4 } from 'uuid';

const MAX_SIMULTANEOUS_OPERATIONS = 3;

export const FileOperationManagerKey = 'fileOperationManager';

async function getTemporaryPath(parent: FsPath): Promise<FsPath> {
  return await Path.join(parent, `._${crypto.randomUUID()}`);
}

export class FileOperationManager {
  private running: boolean;
  private operations: Map<FileOperationID, { job: Promise<void>; aborter: AbortController }>;
  private pendingOperations: Array<FileOperationData>;
  private eventDistributor: FileOperationEventDistributor;

  constructor() {
    this.operations = new Map<FileOperationID, { job: Promise<void>; aborter: AbortController }>();
    this.pendingOperations = [];
    this.running = false;
    this.eventDistributor = new FileOperationEventDistributor();
    this.start();
  }

  get isRunning(): boolean {
    return this.running;
  }

  hasOperations(): boolean {
    return this.operations.size + this.pendingOperations.length > 0;
  }

  async import(workspaceHandle: WorkspaceHandle, files: Array<File>, destination: FsPath, dupPolicy: DuplicatePolicy): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationImportData = {
      type: FileOperationDataType.Import,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      workspaceName: workspaceResult.value.currentName,
      files: files,
      destination: destination,
      dupPolicy: dupPolicy,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async move(
    workspaceHandle: WorkspaceHandle,
    sources: Array<EntryStat>,
    destination: FsPath,
    dupPolicy: DuplicatePolicy,
  ): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationMoveData = {
      type: FileOperationDataType.Move,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      workspaceName: workspaceResult.value.currentName,
      sources: sources,
      destination: destination,
      dupPolicy: dupPolicy,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async copy(
    workspaceHandle: WorkspaceHandle,
    sources: Array<EntryStat>,
    destination: FsPath,
    dupPolicy: DuplicatePolicy,
  ): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationCopyData = {
      type: FileOperationDataType.Copy,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      workspaceName: workspaceResult.value.currentName,
      sources: sources,
      destination: destination,
      dupPolicy: dupPolicy,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async restore(
    workspaceHandle: WorkspaceHandle,
    entries: Array<WorkspaceHistoryEntryStat>,
    dateTime: DateTime,
    dupPolicy: DuplicatePolicy,
  ): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationRestoreData = {
      type: FileOperationDataType.Restore,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      workspaceName: workspaceResult.value.currentName,
      entries: entries,
      dateTime: dateTime,
      dupPolicy: dupPolicy,
    };
    await this.eventDistributor.distribute(FileOperationEvents.Added, data);
    this.pendingOperations.unshift(data);
    if (!this.isRunning) {
      this.start();
    }
    return true;
  }

  async download(
    workspaceHandle: WorkspaceHandle,
    entry: EntryStatFile,
    saveHandle: FileSystemFileHandle,
    dateTime?: DateTime,
  ): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationDownloadData = {
      type: FileOperationDataType.Download,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      workspaceName: workspaceResult.value.currentName,
      entry: entry,
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

  async downloadArchive(
    workspaceHandle: WorkspaceHandle,
    trees: Array<EntryTree>,
    saveHandle: FileSystemFileHandle,
    root: FsPath,
  ): Promise<boolean> {
    const workspaceResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceResult.ok) {
      return false;
    }

    const data: FileOperationDownloadArchiveData = {
      type: FileOperationDataType.DownloadArchive,
      id: uuid4(),
      workspaceHandle: workspaceHandle,
      workspaceId: workspaceResult.value.id,
      workspaceName: workspaceResult.value.currentName,
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
    const operation = this.operations.get(id);
    if (operation) {
      operation.aborter.abort();
    } else {
      const index = this.pendingOperations.findIndex((op) => op.id === id);
      if (index !== -1) {
        await this.eventDistributor.distribute(FileOperationEvents.Cancelled, this.pendingOperations[index]);
        this.pendingOperations.splice(index, 1);
      }
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

  private async _doCopy(signal: AbortSignal, data: FileOperationCopyData): Promise<void> {
    const transaction = new OperationTransaction(data.workspaceHandle);
    try {
      const trees = new Map<EntryStat, EntryTree>();

      let globalTotalFileCount = 0;
      let globalTotalSize = 0;

      for (const entry of data.sources) {
        if (signal.aborted) {
          throw new FileOperationCancelled();
        }
        if (entry.isFile()) {
          trees.set(entry, {
            totalSize: Number((entry as EntryStatFile).size),
            entries: [entry as EntryStatFile],
            maxRecursionReached: false,
            maxFilesReached: false,
          });
          globalTotalFileCount += 1;
          globalTotalSize += (entry as EntryStatFile).size;
        } else {
          const tree = await listTree(data.workspaceHandle, entry.path);
          if (tree.maxRecursionReached) {
            throw new FileOperationException(OperationFailedErrors.MaxRecursionReached);
          }
          if (tree.maxFilesReached) {
            throw new FileOperationException(OperationFailedErrors.MaxFilesReached);
          }
          trees.set(entry, tree);
          globalTotalFileCount += tree.entries.length;
          globalTotalSize += tree.totalSize;
        }
      }

      let totalSizeCopied = 0;
      let totalFilesCopied = 0;
      for (const [stat, tree] of trees.entries()) {
        if (signal.aborted) {
          throw new FileOperationCancelled();
        }
        const tmpPath = await getTemporaryPath(data.destination);
        if (stat.isFile()) {
          await copyFile(signal, data.workspaceHandle, stat as EntryStatFile, tmpPath, (size: number, totalSize: number) => {
            this.eventDistributor.distribute(FileOperationEvents.Progress, data, {
              currentFile: {
                name: stat.name,
                currentSize: size,
                totalSize: totalSize,
                progress: Math.min(100, Math.round((size / totalSize) * 100.0)),
              },
              global: {
                totalSize: globalTotalSize,
                currentSize: totalSizeCopied + size,
                fileCount: globalTotalFileCount,
                fileIndex: totalFilesCopied,
                progress: Math.min(100, Math.round(((totalSizeCopied + size) / globalTotalSize) * 100.0)),
              },
            });
          });
          totalSizeCopied += (stat as EntryStatFile).size;
          totalFilesCopied += 1;
        } else {
          await copyFolder(
            signal,
            data.workspaceHandle,
            stat as EntryStatFolder,
            tree,
            tmpPath,
            (currentEntry: EntryStatFile, fileIndex: number, size: number, totalSize: number) => {
              totalSizeCopied += size;
              this.eventDistributor.distribute(FileOperationEvents.Progress, data, {
                currentFile: {
                  name: currentEntry.name,
                  currentSize: size,
                  totalSize: totalSize,
                  progress: Math.min(100, Math.round((size / totalSize) * 100.0)),
                },
                global: {
                  totalSize: globalTotalSize,
                  currentSize: totalSizeCopied,
                  fileCount: globalTotalFileCount,
                  fileIndex: totalFilesCopied + fileIndex,
                  progress: Math.min(100, Math.round((totalSizeCopied / globalTotalSize) * 100.0)),
                },
              });
            },
          );
        }
        transaction.addFile(tmpPath, await Path.join(data.destination, stat.name));
      }
      this.eventDistributor.distribute(FileOperationEvents.Finalizing, data);
      transaction.commit(data.dupPolicy);
    } catch (err: unknown) {
      transaction.clear();
      throw err;
    }
  }

  private async _doMove(signal: AbortSignal, data: FileOperationMoveData): Promise<void> {
    const transaction = new OperationTransaction(data.workspaceHandle);
    try {
      for (const [fileIndex, entry] of data.sources.entries()) {
        if (signal.aborted) {
          throw new FileOperationCancelled();
        }

        let newPath = await Path.join(data.destination, entry.name);
        if (data.dupPolicy === DuplicatePolicy.AddCounter) {
          const result = await moveWithCounter(data.workspaceHandle, entry.path, newPath);
          if (!result) {
            throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, 'Failed to move file');
          }
          newPath = result;
          transaction.addFile(newPath, entry.path);
        } else {
          const moveResult = await moveEntry(data.workspaceHandle, entry.path, newPath, data.dupPolicy === DuplicatePolicy.Replace);
          if (!moveResult.ok) {
            if (data.dupPolicy === DuplicatePolicy.Replace) {
              // If we were authorized to replace the file, we already forced the rename. If it failed, we throw an exception
              throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(moveResult.error));
            }
          } else {
            transaction.addFile(newPath, entry.path);
          }
        }
        this.eventDistributor.distribute(FileOperationEvents.Progress, data, {
          currentFile: {
            name: entry.name,
            currentSize: 0,
            totalSize: 0,
            progress: 100.0,
          },
          global: {
            totalSize: 0,
            currentSize: 0,
            fileCount: data.sources.length,
            fileIndex: fileIndex,
            progress: Math.min(100, Math.round((fileIndex / data.sources.length) * 100.0)),
          },
        });
      }
    } catch (err: unknown) {
      // In this case, we set up the transaction so that a commit will reverse the move
      await transaction.commit(DuplicatePolicy.Ignore);
      throw err;
    } finally {
    }
  }

  private async _doRestore(signal: AbortSignal, data: FileOperationRestoreData): Promise<void> {
    const history = new WorkspaceHistory(data.workspaceId);

    try {
      const trees: Array<HistoryEntryTree> = [];

      await history.start(data.dateTime);

      let globalTotalFileCount = 0;
      let globalTotalSize = 0;

      for (const entry of data.entries) {
        if (signal.aborted) {
          throw new FileOperationCancelled();
        }

        const statResult = await history.entryStat(entry.path);
        if (!statResult.ok) {
          throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(statResult.error));
        }
        if (statResult.value.isFile()) {
          trees.push({
            totalSize: Number((statResult.value as WorkspaceHistoryEntryStatFile).size),
            entries: [statResult.value as WorkspaceHistoryEntryStatFile],
            maxRecursionReached: false,
            maxFilesReached: false,
          });
          globalTotalFileCount += 1;
          globalTotalSize += (statResult.value as WorkspaceHistoryEntryStatFile).size;
        } else {
          const tree = await listTreeAt(history, entry.path);
          if (tree.maxRecursionReached) {
            throw new FileOperationException(OperationFailedErrors.MaxRecursionReached);
          }
          if (tree.maxFilesReached) {
            throw new FileOperationException(OperationFailedErrors.MaxFilesReached);
          }
          trees.push(tree);
          globalTotalFileCount += tree.entries.length;
          globalTotalSize += tree.totalSize;
        }
      }

      const transaction = new OperationTransaction(data.workspaceHandle);

      try {
        let totalSizeRestored = 0;
        let fileIndex = 0;
        for (const tree of trees) {
          for (const entry of tree.entries) {
            if (signal.aborted) {
              throw new FileOperationCancelled();
            }
            const dstDir = await Path.parent(entry.path);
            if (dstDir !== '/') {
              const result = await createFolder(data.workspaceHandle, dstDir);
              if (!result.ok && result.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
                throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(result.error));
              }
            }
            const dstPath = await getTemporaryPath(dstDir);
            await restoreFile(
              signal,
              history,
              { workspace: data.workspaceHandle, path: entry.path },
              { workspace: data.workspaceHandle, path: dstPath },
              (size: number, totalSize: number) => {
                totalSizeRestored += size;
                this.eventDistributor.distribute(FileOperationEvents.Progress, data, {
                  currentFile: {
                    name: entry.name,
                    currentSize: size,
                    totalSize: totalSize,
                    progress: Math.min(100, Math.round((size / totalSize) * 100.0)),
                  },
                  global: {
                    totalSize: globalTotalSize,
                    currentSize: totalSizeRestored,
                    fileCount: globalTotalFileCount,
                    fileIndex: fileIndex,
                    progress: Math.min(100, Math.round((totalSizeRestored / totalSize) * 100.0)),
                  },
                });
              },
            );
            fileIndex += 1;
            transaction.addFile(dstPath, entry.path);
          }
        }
        this.eventDistributor.distribute(FileOperationEvents.Finalizing, data);
        await transaction.commit(data.dupPolicy);
      } catch (err: unknown) {
        await transaction.clear();
        throw err;
      }
    } catch (err: unknown) {
      throw err;
    } finally {
      await history.stop();
    }
  }

  private async _doDownload(signal: AbortSignal, data: FileOperationDownloadData): Promise<void> {
    let writtenSize = 0;

    const rStream = await createReadStream(data.workspaceHandle, data.entry.path);
    const wStream = await data.saveHandle.createWritable();
    const reader = rStream.getReader();

    try {
      while (true) {
        // Check if the download has been cancelled
        if (signal.aborted) {
          throw new FileOperationCancelled();
        }

        const { done, value } = await reader.read();
        if (done) {
          break;
        } else {
          const chunk = value as Uint8Array;
          await wStream.write(chunk);
          writtenSize += chunk.byteLength;
          this.eventDistributor.distribute(FileOperationEvents.Progress, data, {
            currentFile: {
              name: data.entry.name,
              currentSize: writtenSize,
              totalSize: data.entry.size,
              progress: Math.min(100, Math.round((writtenSize / data.entry.size) * 100)),
            },
            global: {
              totalSize: data.entry.size,
              currentSize: writtenSize,
              fileCount: 1,
              fileIndex: 0,
              progress: Math.min(100, Math.round((writtenSize / data.entry.size) * 100)),
            },
          });
        }
      }
      wStream.close();
    } catch (err: unknown) {
      await wStream.abort();
      data.saveHandle.remove();
      throw err;
    }
  }

  private async _doDownloadArchive(signal: AbortSignal, data: FileOperationDownloadArchiveData): Promise<void> {
    let zipWriter: zipjs.ZipWriter<any> | undefined;
    let wStream: FileSystemWritableFileStream | undefined;

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
      let fileIndex = 0;
      for (const tree of data.trees) {
        for (const entry of tree.entries) {
          // Check if the download has been cancelled
          if (signal.aborted) {
            throw new FileOperationCancelled();
          }

          try {
            const rStream = await createReadStream(data.workspaceHandle, entry.path, async (sizeRead: number) => {
              totalSizeRead += sizeRead;
              this.eventDistributor.distribute(FileOperationEvents.Progress, data, {
                currentFile: {
                  name: entry.name,
                  currentSize: 0,
                  totalSize: entry.size,
                  progress: 0.0,
                },
                global: {
                  totalSize: data.totalSize,
                  currentSize: totalSizeRead,
                  fileCount: data.totalFiles,
                  fileIndex: fileIndex,
                  progress: Math.min(100, Math.round((totalSizeRead / data.totalSize) * 100.0)),
                },
              });
            });
            const relPath = entry.path.startsWith(data.rootPath) ? entry.path.slice(data.rootPath.length) : entry.path;
            await zipWriter.add(relPath, rStream);
            fileIndex += 1;
          } catch (e: any) {
            window.electronAPI.log('error', 'Failed to add file to archive');
            throw e;
          }
        }
      }
    } catch (err: unknown) {
      if (wStream) {
        await wStream.abort();
      }
      data.saveHandle.remove();
      throw err;
    } finally {
      if (zipWriter) {
        zipWriter.close();
      }
    }
  }

  private async _doImport(signal: AbortSignal, data: FileOperationImportData): Promise<void> {
    const transaction = new OperationTransaction(data.workspaceHandle);
    const globalTotalSize = data.files.reduce((sum, file) => sum + file.size, 0);

    try {
      let globalCurrentSize = 0;
      for (const [i, file] of data.files.entries()) {
        if (signal.aborted) {
          throw new FileOperationCancelled();
        }
        const destinationDir = await Path.parent(await Path.joinPaths(data.destination, (file as any).relativePath));
        const mkdirResult = await createFolder(data.workspaceHandle, destinationDir);
        if (!mkdirResult.ok && mkdirResult.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
          throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(mkdirResult.error));
        }
        const tmpPath = await getTemporaryPath(destinationDir);
        await importFile(signal, file, { workspace: data.workspaceHandle, path: tmpPath }, (size: number, totalSize: number) => {
          this.eventDistributor.distribute(FileOperationEvents.Progress, data, {
            currentFile: {
              name: file.name,
              currentSize: size,
              totalSize: totalSize,
              progress: Math.min(100, Math.round((size / totalSize) * 100.0)),
            },
            global: {
              totalSize: globalTotalSize,
              currentSize: globalCurrentSize + size,
              fileCount: data.files.length,
              fileIndex: i,
              progress: Math.min(100, Math.round(((globalCurrentSize + size) / globalTotalSize) * 100.0)),
            },
          });
        });
        globalCurrentSize += file.size;
        transaction.addFile(tmpPath, await Path.join(destinationDir, file.name));
      }
      this.eventDistributor.distribute(FileOperationEvents.Finalizing, data);
      await transaction.commit(data.dupPolicy);
    } catch (err: unknown) {
      await transaction.clear();
      throw err;
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

      if (this.operations.size >= MAX_SIMULTANEOUS_OPERATIONS) {
        await wait(500);
        continue;
      }
      const elem = this.pendingOperations.pop();

      if (!elem) {
        await wait(500);
        continue;
      }
      let job: Promise<void>;
      const aborter = new AbortController();
      switch (elem.type) {
        case FileOperationDataType.Import: {
          job = this._doImport(aborter.signal, elem as FileOperationImportData);
          break;
        }
        case FileOperationDataType.Move: {
          job = this._doMove(aborter.signal, elem as FileOperationMoveData);
          break;
        }
        case FileOperationDataType.Copy: {
          job = this._doCopy(aborter.signal, elem as FileOperationCopyData);
          break;
        }
        case FileOperationDataType.Restore: {
          job = this._doRestore(aborter.signal, elem as FileOperationRestoreData);
          break;
        }
        case FileOperationDataType.Download: {
          job = this._doDownload(aborter.signal, elem as FileOperationDownloadData);
          break;
        }
        case FileOperationDataType.DownloadArchive: {
          job = this._doDownloadArchive(aborter.signal, elem as FileOperationDownloadArchiveData);
          break;
        }
        default:
          window.electronAPI.log('warn', `Unhandled file operation '${elem.type}'`);
          continue;
      }
      await this.eventDistributor.distribute(FileOperationEvents.Started, elem);
      this.operations.set(elem.id, { job: job, aborter: aborter });
      job
        .then(() => {
          this.eventDistributor.distribute(FileOperationEvents.Finished, elem);
        })
        .catch((exc: any) => {
          if (exc instanceof FileOperationCancelled) {
            this.eventDistributor.distribute(FileOperationEvents.Cancelled, elem);
          } else if (exc instanceof FileOperationException) {
            this.eventDistributor.distribute(FileOperationEvents.Failed, elem, { error: exc.error, details: exc.details });
          } else {
            this.eventDistributor.distribute(FileOperationEvents.Failed, elem, {
              error: OperationFailedErrors.Unhandled,
              details: exc.toString(),
            });
          }
        })
        .finally(async () => {
          this.operations.delete(elem.id);
          if (this.operations.size === 0 && this.pendingOperations.length === 0) {
            await this.eventDistributor.distribute(FileOperationEvents.AllFinished);
          }
        });
    }
  }
}
