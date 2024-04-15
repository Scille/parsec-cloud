// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FsPath, Path } from '@/parsec';

export interface FileImportTuple {
  file: File;
  path: string;
}

export async function getFilesFromDrop(event: DragEvent, path: FsPath): Promise<FileImportTuple[]> {
  if (event.dataTransfer) {
    const entries: FileSystemEntry[] = [];
    // May have to use event.dataTransfer.files instead of items.
    // .files gets us a FileList, the API is not as good
    // but it's the same used in <input> and it should be compatible with
    // Cypress.
    for (let i = 0; i < event.dataTransfer.items.length; i++) {
      const entry = event.dataTransfer.items[i].webkitGetAsEntry();
      if (entry) {
        entries.push(entry);
      }
    }
    if (entries.length) {
      const imports: FileImportTuple[] = [];
      for (const entry of entries) {
        const result = await unwindEntry(path, entry);
        imports.push(...result);
      }
      return imports;
    }
  }
  return [];
}

async function unwindEntry(currentPath: string, fsEntry: FileSystemEntry): Promise<FileImportTuple[]> {
  const parsecPath = await Path.join(currentPath, fsEntry.name);
  const imports: FileImportTuple[] = [];

  if (fsEntry.isDirectory) {
    const entries = await getEntries(fsEntry as FileSystemDirectoryEntry);
    for (const entry of entries) {
      const result = await unwindEntry(parsecPath, entry);
      imports.push(...result);
    }
  } else {
    const result = await convertEntryToFile(fsEntry);
    imports.push({ file: result, path: currentPath });
  }
  return imports;
}

async function getEntries(fsEntry: FileSystemDirectoryEntry): Promise<FileSystemEntry[]> {
  return new Promise((resolve, reject) => {
    const reader = fsEntry.createReader();
    reader.readEntries(
      (entries) => {
        resolve(entries);
      },
      () => {
        reject();
      },
    );
  });
}

async function convertEntryToFile(fsEntry: FileSystemEntry): Promise<File> {
  return new Promise((resolve, reject) => {
    (fsEntry as FileSystemFileEntry).file(
      (file) => {
        resolve(file);
      },
      () => {
        reject();
      },
    );
  });
}
