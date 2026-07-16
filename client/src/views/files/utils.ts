// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, EntryStat, EntryStatFile, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { DuplicatePolicy } from '@/services/fileOperation';
import { FileOperationManager } from '@/services/fileOperation/manager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { StorageManager } from '@/services/storageManager';
import { FileOperationConflictsModal } from '@/views/files';
import DownloadWarningModal from '@/views/files/DownloadWarningModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';
import { showDirectoryPicker, showSaveFilePicker } from 'native-file-system-adapter';

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

export async function downloadFiles(options: DownloadOptions): Promise<void> {
  if (options.entries.length === 0) {
    return;
  }

  try {
    if (options.asArchive) {
      const saveHandle = await showSaveFilePicker({
        _preferPolyfill: false,
        suggestedName: options.asArchive.archiveName,
      });
      await options.fileOperationManager.downloadArchive(
        options.workspaceHandle,
        options.entries,
        saveHandle,
        options.asArchive.relativePath,
      );
    } else if (options.entries.length === 1 && options.entries[0].isFile()) {
      const saveHandle = await showSaveFilePicker({
        _preferPolyfill: false,
        suggestedName: options.entries[0].name,
      });
      await options.fileOperationManager.download(options.workspaceHandle, options.entries[0] as EntryStatFile, saveHandle);
    } else {
      const saveHandle = await showDirectoryPicker({ _preferPolyfill: false });
      await options.fileOperationManager.downloadFiles(options.workspaceHandle, options.entries, saveHandle);
    }
  } catch (e: any) {
    if (e.name === 'NotAllowedError') {
      window.electronAPI.log('error', 'No permission for showSaveFilePicker');
      options.informationManager.present(
        new Information({
          message: 'FoldersPage.DownloadFile.noPermissions',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
    } else if (e.name === 'AbortError') {
      if ((e.toString() as string).toLocaleLowerCase().includes('user aborted')) {
        window.electronAPI.log('debug', 'User cancelled the showSaveFilePicker');
      } else {
        options.informationManager.present(
          new Information({
            message: 'FoldersPage.DownloadFile.selectFolderFailed',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        window.electronAPI.log('error', `Could not create the file: ${e.toString()}`);
      }
    } else {
      window.electronAPI.log('error', `Failed to select destination file: ${e.toString()}`);
    }
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
