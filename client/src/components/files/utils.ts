// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import FolderSelectionModal from '@/components/files/explorer/FolderSelectionModal.vue';
import { SortProperty } from '@/components/files/types';
import { FsPath, Path, WorkspaceHandle, getPathLink } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { modalController } from '@ionic/vue';
import { Clipboard, DisplayState, MsModalResult, Translatable } from 'megashark-lib';

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

export interface FoldersPageSavedData {
  displayState: DisplayState;
  sortProperty: SortProperty;
  sortAscending: boolean;
}

export const FolderDefaultData: Required<FoldersPageSavedData> = {
  displayState: DisplayState.List,
  sortProperty: SortProperty.Name,
  sortAscending: true,
};

export async function copyPathLinkToClipboard(
  path: FsPath,
  workspaceHandle: WorkspaceHandle,
  informationManager: InformationManager,
): Promise<void> {
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }

  const result = await getPathLink(workspaceHandle, path);

  if (result.ok) {
    const [_, invitationAddrAsHttpRedirection] = result.value;
    if (!(await Clipboard.writeText(invitationAddrAsHttpRedirection))) {
      informationManager.present(
        new Information({
          message: 'FoldersPage.linkNotCopiedToClipboard',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: 'FoldersPage.linkCopiedToClipboard',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: 'FoldersPage.getLinkError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

export function getFilesFromDrop(event: DragEvent, batchSize = 200): AsyncGenerator<File[]> {
  const entries: FileSystemEntry[] = [];
  const fallbackFiles: File[] = [];

  // dataTransfer is cleared by the browser at the first async suspension, so we've got
  // to collect the files before.

  /*
    In some cases (Playwright, old browsers, ...), `webkitGetAsEntry`
    will fail. In those cases, we use the file list. It's a lower API and
    doesn't allow us to travel the tree, but it's good enough as a backup.
  */
  if (event.dataTransfer) {
    for (let i = 0; i < event.dataTransfer.items.length; i++) {
      const entry = event.dataTransfer.items[i].webkitGetAsEntry();
      if (entry) {
        entries.push(entry);
      } else if (event.dataTransfer.files[i]) {
        const file = event.dataTransfer.files[i];
        (file as any).relativePath = `/${file.name}`;
        fallbackFiles.push(file);
      }
    }
  }

  return traverseEntries(entries, fallbackFiles, batchSize);
}

async function* traverseEntries(entries: FileSystemEntry[], fallbackFiles: File[], batchSize: number): AsyncGenerator<File[]> {
  let batch: File[] = [];

  for (const file of fallbackFiles) {
    batch.push(file);
    if (batch.length >= batchSize) {
      yield batch;
      batch = [];
    }
  }

  for (const entry of entries) {
    for await (const file of unwindEntry('/', entry)) {
      batch.push(file);
      if (batch.length >= batchSize) {
        yield batch;
        batch = [];
      }
    }
  }

  if (batch.length > 0) {
    yield batch;
  }
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

export function getProgressPercent(completedBytes: number, totalBytes: number): number {
  if (totalBytes === 0) {
    return 0;
  }
  return Math.min(100, Math.round((completedBytes / totalBytes) * 100));
}

async function* unwindEntry(currentPath: string, fsEntry: FileSystemEntry): AsyncGenerator<File> {
  const parsecPath = Path.quickJoin(currentPath, fsEntry.name);

  if (fsEntry.isDirectory) {
    for await (const entry of getEntries(fsEntry as FileSystemDirectoryEntry)) {
      yield* unwindEntry(parsecPath, entry);
    }
  } else {
    yield await convertEntryToFile(fsEntry, parsecPath);
  }
}

async function* getEntries(fsEntry: FileSystemDirectoryEntry): AsyncGenerator<FileSystemEntry> {
  const reader = fsEntry.createReader();
  while (true) {
    const entries = await new Promise<FileSystemEntry[]>((resolve, reject) => {
      // readEntries() returns at most 100 entries per call on Chrome-based browsers
      reader.readEntries(resolve, reject);
    });
    if (entries.length === 0) break;
    for (const entry of entries) {
      yield entry;
    }
  }
}

async function convertEntryToFile(fsEntry: FileSystemEntry, relativePath: FsPath): Promise<File> {
  return new Promise((resolve, reject) => {
    (fsEntry as FileSystemFileEntry).file(
      (file) => {
        (file as any).relativePath = relativePath;
        resolve(file);
      },
      () => {
        reject();
      },
    );
  });
}
