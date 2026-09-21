// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, EntryStat, EntryStatFile, FsPath, WorkspaceHandle } from '@/parsec';
import { startDownload } from '@/services/download';
import { DuplicatePolicy } from '@/services/fileOperation';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { StorageManager } from '@/services/storageManager';
import { FileOperationConflictsModal } from '@/views/files';
import DownloadWarningModal from '@/views/files/DownloadWarningModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

export async function askDownloadConfirmation(multipleFiles?: boolean): Promise<{ result: MsModalResult; noReminder?: boolean }> {
  const modal = await modalController.create({
    cssClass: 'download-warning-modal',
    showBackdrop: true,
    component: DownloadWarningModal,
    componentProps: {
      multipleFiles: multipleFiles,
    },
  });
  await modal.present();
  const { data, role } = await modal.onDidDismiss();
  await modal.dismiss();
  return { result: role ? (role as MsModalResult) : MsModalResult.Cancel, noReminder: data?.noReminder };
}

export async function openDownloadConfirmationModal(storageManager: StorageManager, multipleFiles?: boolean): Promise<MsModalResult> {
  const config = await storageManager.retrieveConfig();
  if (!config.disableDownloadWarning) {
    const { result, noReminder } = await askDownloadConfirmation(multipleFiles);

    if (noReminder) {
      config.disableDownloadWarning = true;
      await storageManager.storeConfig(config);
    }
    if (result !== MsModalResult.Confirm) {
      return MsModalResult.Cancel;
    }
  }
  return MsModalResult.Confirm;
}

interface DownloadOptions {
  workspaceHandle: WorkspaceHandle;
  informationManager: InformationManager;
  entries: Array<EntryStat>;
  // A download is an archive if this is set
  asArchive?: {
    archiveName: EntryName;
    // The entries are in the archive with their path relative to this folder
    relativePath: FsPath;
  };
}

// The download is handled by the browser, as any other download: see the streaming worker.
export async function downloadFiles(options: DownloadOptions): Promise<void> {
  if (options.entries.length === 0) {
    return;
  }

  try {
    if (options.asArchive) {
      await startDownload({
        workspaceHandle: options.workspaceHandle,
        name: options.asArchive.archiveName,
        archive: true,
        root: options.asArchive.relativePath,
        entries: options.entries.map((entry) => ({
          path: entry.path,
          isFile: entry.isFile(),
          size: entry.isFile() ? (entry as EntryStatFile).size : undefined,
        })),
      });
    } else {
      await startDownload({
        workspaceHandle: options.workspaceHandle,
        name: options.entries[0].name,
        archive: false,
        root: '/',
        entries: [{ path: options.entries[0].path, isFile: true, size: (options.entries[0] as EntryStatFile).size }],
      });
    }
  } catch (e: any) {
    window.nativeAPI.log('error', `Failed to start the download: ${e.toString()}`);
    options.informationManager.present(
      new Information({
        message: 'FoldersPage.DownloadFile.allFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

export async function getDuplicatePolicy(files: Array<EntryStat | File>): Promise<DuplicatePolicy | undefined> {
  const modal = await modalController.create({
    component: FileOperationConflictsModal,
    cssClass: 'file-operation-conflicts-modal',
    componentProps: {
      files: files,
    },
  });
  await modal.present();
  const { data } = await modal.onDidDismiss();
  return data;
}
