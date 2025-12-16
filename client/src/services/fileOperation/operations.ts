// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  closeFile,
  createFolder,
  DEFAULT_READ_SIZE,
  deleteFile,
  deleteFolder,
  EntryStatFile,
  EntryStatFolder,
  EntryTree,
  FileDescriptor,
  FsPath,
  moveEntry,
  openFile,
  Path,
  readFile,
  resizeFile,
  WorkspaceCreateFolderErrorTag,
  WorkspaceHandle,
  WorkspaceHistory,
  WorkspaceHistoryEntryStatFile,
  WorkspaceMoveEntryErrorTag,
  WorkspaceRemoveEntryErrorTag,
  writeFile,
} from '@/parsec';
import { stringifyError } from '@/parsec/utils';
import {
  DuplicatePolicy,
  FileEntryPointer,
  FileOperationCancelled,
  FileOperationException,
  FileProgressCallback,
  FolderProgressCallback,
  OperationFailedErrors,
} from '@/services/fileOperation/types';

export async function importFile(
  signal: AbortSignal,
  input: File,
  destination: FileEntryPointer,
  progressFunction?: FileProgressCallback,
): Promise<void> {
  let fdW: FileDescriptor | null = null;
  try {
    window.electronAPI.log('debug', `Importing file ${(input as any).relativePath} into ${destination.path} (${input.size} bytes)`);
    const fdWResult = await openFile(destination.workspace, destination.path, { write: true, createNew: true });

    if (!fdWResult.ok) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(fdWResult.error));
    }
    fdW = fdWResult.value;
    const resizeResult = await resizeFile(destination.workspace, fdW, input.size);
    if (!resizeResult.ok) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(resizeResult.error));
    }
    if (progressFunction) {
      progressFunction(0, input.size);
    }
    const reader = input.stream().getReader();
    let offset = 0;
    while (true) {
      if (signal.aborted) {
        throw new FileOperationCancelled();
      }

      const buffer = await reader.read();
      if (buffer.value) {
        const writeResult = await writeFile(destination.workspace, fdW, offset, buffer.value);

        if (!writeResult.ok) {
          throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(writeResult.error));
        } else {
          offset += writeResult.value;
          if (progressFunction) {
            progressFunction(offset, input.size);
          }
        }
      }
      if (buffer.done) {
        break;
      }
    }
  } catch (err: unknown) {
    await deleteFile(destination.workspace, destination.path);
    throw err;
  } finally {
    if (fdW) {
      await closeFile(destination.workspace, fdW);
    }
  }
}

export async function restoreFile(
  signal: AbortSignal,
  history: WorkspaceHistory,
  source: FileEntryPointer,
  destination: FileEntryPointer,
  progressFunction?: FileProgressCallback,
): Promise<void> {
  let fdR: FileDescriptor | null = null;
  let fdW: FileDescriptor | null = null;
  try {
    window.electronAPI.log('debug', `Restoring file ${source.path} into ${destination.path}`);
    const statsResult = await history.entryStat(source.path);
    if (!statsResult.ok) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(statsResult.error));
    }
    if (!statsResult.value.isFile()) {
      throw new FileOperationException(OperationFailedErrors.OneFailed, 'Not a file');
    }
    const sourceSize = (statsResult.value as WorkspaceHistoryEntryStatFile).size;
    const openReadResult = await history.openFile(source.path);
    if (!openReadResult.ok) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(openReadResult.error));
    }
    fdR = openReadResult.value;
    const openWriteResult = await openFile(destination.workspace, destination.path, { write: true, create: true });
    if (!openWriteResult.ok) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(openWriteResult.error));
    }
    fdW = openWriteResult.value;

    // Resize the destination
    await resizeFile(destination.workspace, fdW, Number(sourceSize));

    let offset = 0;
    if (progressFunction) {
      progressFunction(0, sourceSize);
    }
    while (true) {
      // Check if the copy has been cancelled
      if (signal.aborted) {
        throw new FileOperationCancelled();
      }

      // Read the source
      const readResult = await history.readFile(fdR, offset, DEFAULT_READ_SIZE);

      // Failed to read, cancel the copy
      if (!readResult.ok) {
        throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(readResult.error));
      }
      const chunk = readResult.value;
      const writeResult = await writeFile(destination.workspace, fdW, offset, new Uint8Array(chunk));

      // Failed to write, or not everything's been written
      if (!writeResult.ok) {
        throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(writeResult.error));
      }
      if (writeResult.value < chunk.byteLength) {
        throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, 'Partial write');
      }
      offset += chunk.byteLength;
      if (progressFunction) {
        progressFunction(offset, sourceSize);
      }

      // Smaller that what we asked for, we're at the end of the file
      if (chunk.byteLength < DEFAULT_READ_SIZE) {
        break;
      }
    }
  } catch (err: unknown) {
    await deleteFile(destination.workspace, destination.path);
    throw err;
  } finally {
    if (fdR !== null) {
      await history.closeFile(fdR);
    }
    if (fdW !== null) {
      await closeFile(destination.workspace, fdW);
    }
  }
}

export class OperationTransaction {
  files: Array<[FsPath, FsPath]>;
  workspace: WorkspaceHandle;

  constructor(workspace: WorkspaceHandle) {
    this.workspace = workspace;
    this.files = [];
  }

  addFile(source: FsPath, destination: FsPath): void {
    this.files.push([source, destination]);
  }

  async clear(): Promise<void> {
    for (const [path, _file] of this.files) {
      await deleteFile(this.workspace, path);
    }
    this.files = [];
  }

  async commit(policy: DuplicatePolicy): Promise<void> {
    for (const [source, destination] of this.files) {
      if (policy === DuplicatePolicy.AddCounter) {
        const result = await moveWithCounter(this.workspace, source, destination);
        if (!result) {
          throw new FileOperationException(OperationFailedErrors.OneFailed, 'Failed to rename file');
        }
      } else {
        window.electronAPI.log('debug', `Moving file '${source}' to '${destination}'`);
        const moveResult = await moveEntry(this.workspace, source, destination, policy === DuplicatePolicy.Replace);
        if (!moveResult.ok) {
          if (policy === DuplicatePolicy.Replace) {
            // If we were authorized to replace the file, we already forced the rename. If it failed, we throw an exception
            throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(moveResult.error));
          } else if (policy === DuplicatePolicy.Ignore) {
            // If we should ignore existing file, then we delete the tmp file and break
            const deleteResult = await deleteFile(this.workspace, source);
            if (!deleteResult.ok && deleteResult.error.tag === WorkspaceRemoveEntryErrorTag.EntryIsFolder) {
              await deleteFolder(this.workspace, source);
            }
          }
        }
      }
    }
    this.files = [];
  }
}

export async function moveWithCounter(workspace: WorkspaceHandle, source: FsPath, destination: FsPath): Promise<FsPath | undefined> {
  const MAX_COUNTER = 10;
  let counter = 1;
  const parentDir = await Path.parent(destination);
  const baseName = await Path.filename(destination);

  if (!baseName) {
    return undefined;
  }

  while (true) {
    let newName = baseName;

    if (counter > 1) {
      const ext = Path.getFileExtension(baseName);
      if (ext.length) {
        newName = `${Path.filenameWithoutExtension(baseName)} (${counter}).${ext}`;
      } else {
        newName = `${Path.filenameWithoutExtension(baseName)} (${counter})`;
      }
    }
    const newDestination = await Path.join(parentDir, newName);
    window.electronAPI.log('debug', `Trying to move file '${source}' to '${newDestination}'`);
    const moveResult = await moveEntry(workspace, source, newDestination, false);
    if (moveResult.ok) {
      return newDestination;
    }
    if (moveResult.error.tag === WorkspaceMoveEntryErrorTag.DestinationExists && counter < MAX_COUNTER) {
      counter += 1;
    } else {
      return undefined;
    }
  }
}

export async function copyFile(
  signal: AbortSignal,
  workspace: WorkspaceHandle,
  source: EntryStatFile,
  destination: FsPath,
  progressFunction?: FileProgressCallback,
): Promise<void> {
  let fdR: FileDescriptor | null = null;
  let fdW: FileDescriptor | null = null;
  try {
    window.electronAPI.log('debug', `Copying file ${source.name} into ${destination}`);
    // Open the source
    const openReadResult = await openFile(workspace, source.path, { read: true });
    if (!openReadResult.ok) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(openReadResult.error));
    }
    fdR = openReadResult.value;
    const openWriteResult = await openFile(workspace, destination, { write: true, createNew: true });
    if (!openWriteResult.ok) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(openWriteResult.error));
    }
    fdW = openWriteResult.value;
    await resizeFile(workspace, fdW, Number(source.size));

    let offset = 0;
    if (progressFunction) {
      progressFunction(0, source.size);
    }
    while (true) {
      if (signal.aborted) {
        throw new FileOperationCancelled();
      }
      const readResult = await readFile(workspace, fdR, offset, DEFAULT_READ_SIZE);

      if (!readResult.ok) {
        throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(readResult.error));
      }
      const chunk = readResult.value;
      const writeResult = await writeFile(workspace, fdW, offset, new Uint8Array(chunk));

      if (!writeResult.ok) {
        throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(writeResult.error));
      }
      if (writeResult.value < chunk.byteLength) {
        throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, 'Partial write');
      }
      offset += chunk.byteLength;
      if (progressFunction) {
        progressFunction(offset, source.size);
      }
      if (chunk.byteLength < DEFAULT_READ_SIZE) {
        break;
      }
    }
  } catch (err: unknown) {
    await deleteFile(workspace, destination);
    throw err;
  } finally {
    if (fdR !== null) {
      await closeFile(workspace, fdR);
    }
    if (fdW !== null) {
      await closeFile(workspace, fdW);
    }
  }
}

export async function copyFolder(
  signal: AbortSignal,
  workspace: WorkspaceHandle,
  source: EntryStatFolder,
  tree: EntryTree,
  destination: FsPath,
  progressFunction?: FolderProgressCallback,
): Promise<void> {
  const mkdirResult = await createFolder(workspace, destination);

  if (!mkdirResult.ok) {
    throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(mkdirResult.error));
  }
  window.electronAPI.log('debug', `Copying folder ${source.name}/ into ${destination}`);
  for (const [index, entry] of tree.entries.entries()) {
    const relativePath = entry.path.substring(source.path.length);
    const parts = (await Path.parse(relativePath)).slice(0, -1);
    const dstFolder = await Path.joinMultiple(destination, parts);
    const mkdirResult = await createFolder(workspace, dstFolder);
    if (!mkdirResult.ok && mkdirResult.error.tag !== WorkspaceCreateFolderErrorTag.EntryExists) {
      throw new FileOperationException(OperationFailedErrors.LibParsecCallFailed, stringifyError(mkdirResult.error));
    }
    const dstPath = await Path.join(dstFolder, entry.name);

    await copyFile(signal, workspace, entry, dstPath, (size: number, totalSize: number) => {
      if (progressFunction) {
        progressFunction(entry, index, size, totalSize);
      }
    });
  }
}
