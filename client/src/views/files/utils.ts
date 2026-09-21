// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, EntryStat, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { DuplicatePolicy } from '@/services/fileOperation';
import { FileOperationManager } from '@/services/fileOperation/manager';
import { InformationManager } from '@/services/informationManager';
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
  workspaceId: WorkspaceID;
  informationManager: InformationManager;
  fileOperationManager: FileOperationManager;
  entries: Array<EntryStat>;
  asArchive?: {
    archiveName: EntryName;
    relativePath: string;
  };
}

// TODO: downloads are being moved to the streaming worker (public/streaming-worker.js), so that
// the browser handles them like any other download. Nothing is downloaded until that is done.
export async function downloadFiles(_options: DownloadOptions): Promise<void> {
  window.nativeAPI.log('error', 'Downloads are not available yet');
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
