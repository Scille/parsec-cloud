// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import FolderSelectionModal from '@/components/files/FolderSelectionModal.vue';
import { FsPath, Path, WorkspaceHandle } from '@/parsec';
import { modalController } from '@ionic/vue';
import { MsModalResult, Translatable } from 'megashark-lib';

export interface FileImportTuple {
  file: File;
  path: string;
}

export interface FolderSelectionOptions {
  title: Translatable;
  subtitle?: Translatable;
  startingPath: FsPath;
  workspaceHandle: WorkspaceHandle;
  allowStartingPath?: boolean;
  excludePaths?: Array<FsPath>;
  okButtonLabel?: Translatable;
}

export async function getFilesFromDrop(event: DragEvent, path: FsPath): Promise<FileImportTuple[]> {
  if (event.dataTransfer) {
    const entries: FileSystemEntry[] = [];
    const files: File[] = [];

    /*
      In some cases (Playwright, old browsers, ...), `webkitGetAsEntry`
      will fail. In those cases, we use the file list. It's a lower API and
      doesn't allow us to travel the tree, but it's good enough as a backup.
    */

    for (let i = 0; i < event.dataTransfer.items.length; i++) {
      const entry = event.dataTransfer.items[i].webkitGetAsEntry();
      if (entry) {
        entries.push(entry);
      } else if (event.dataTransfer.files[i]) {
        files.push(event.dataTransfer.files[i]);
      }
    }
    const imports: FileImportTuple[] = [];
    if (entries.length) {
      for (const entry of entries) {
        const result = await unwindEntry(path, entry);
        imports.push(...result);
      }
      return imports;
    } else if (files.length) {
      for (const file of files) {
        imports.push({ file: file, path: path });
      }
      return imports;
    }
  }
  return [];
}

export async function selectFolder(options: FolderSelectionOptions): Promise<FsPath | null> {
  const modal = await modalController.create({
    component: FolderSelectionModal,
    canDismiss: true,
    cssClass: 'folder-selection-modal',
    componentProps: options,
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  return result.role === MsModalResult.Confirm ? result.data : null;
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
