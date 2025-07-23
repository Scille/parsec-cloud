// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryModel } from '@/components/files';
import { SmallDisplayCategoryFileContextMenu, SmallDisplayFileContextMenu } from '@/components/small-display';
import { EntryName, FsPath, WorkspaceHandle, WorkspaceID, WorkspaceRole } from '@/parsec';
import { FileOperationManager } from '@/services/fileOperationManager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { FileAction, FileContextMenu, FolderGlobalAction, FolderGlobalContextMenu } from '@/views/files';
import DownloadWarningModal from '@/views/files/DownloadWarningModal.vue';
import { modalController, popoverController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';
import { showSaveFilePicker } from 'native-file-system-adapter';

export async function openGlobalContextMenu(
  event: Event,
  ownRole: WorkspaceRole,
  isLargeDisplay: boolean,
  isFolderEmpty: boolean,
): Promise<{ action: FolderGlobalAction } | undefined> {
  let data: { action: FolderGlobalAction } | undefined;

  if (isLargeDisplay) {
    if (ownRole === WorkspaceRole.Reader) {
      return;
    }

    const popover = await popoverController.create({
      component: FolderGlobalContextMenu,
      cssClass: 'folder-global-context-menu',
      event: event,
      reference: event.type === 'contextmenu' ? 'event' : 'trigger',
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      alignment: 'start',
      componentProps: {
        role: ownRole,
      },
    });
    await popover.present();
    data = (await popover.onDidDismiss()).data;
    await popover.dismiss();
  } else {
    const modal = await modalController.create({
      component: SmallDisplayCategoryFileContextMenu,
      cssClass: 'file-context-sheet-modal',
      canDismiss: true,
      breakpoints: [0, 0.25, 1],
      expandToScroll: false,
      initialBreakpoint: 0.25,
      showBackdrop: true,
      componentProps: {
        disableSelect: isFolderEmpty,
      },
    });

    await modal.present();
    data = (await modal.onWillDismiss()).data;
    await modal.dismiss();
  }
  return data;
}

export async function openEntryContextMenu(
  event: Event,
  entry: EntryModel,
  selectedEntries: EntryModel[],
  ownRole: WorkspaceRole,
  isLargeDisplay: boolean,
): Promise<{ action: FileAction } | undefined> {
  let data: { action: FileAction } | undefined;

  if (isLargeDisplay) {
    const popover = await popoverController.create({
      component: FileContextMenu,
      cssClass: 'file-context-menu',
      event: event,
      reference: event.type === 'contextmenu' ? 'event' : 'trigger',
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      alignment: 'start',
      componentProps: {
        role: ownRole,
        multipleFiles: selectedEntries.length > 1 && selectedEntries.includes(entry),
        isFile: entry.isFile(),
      },
    });

    await popover.present();
    data = (await popover.onDidDismiss()).data;
    await popover.dismiss();
  } else {
    const modal = await modalController.create({
      component: SmallDisplayFileContextMenu,
      cssClass: 'file-context-sheet-modal',
      breakpoints: [0, 0.5, 1],
      initialBreakpoint: 0.5,
      expandToScroll: false,
      showBackdrop: true,
      componentProps: {
        role: ownRole,
        multipleFiles: selectedEntries.length > 1 && selectedEntries.includes(entry),
        isFile: entry.isFile(),
      },
    });

    await modal.present();
    data = (await modal.onDidDismiss()).data;
    await modal.dismiss();
  }

  return data;
}

export async function askDownloadConfirmation(): Promise<{ result: MsModalResult; noReminder?: boolean }> {
  const modal = await modalController.create({
    cssClass: 'download-warning-modal',
    showBackdrop: true,
    component: DownloadWarningModal,
  });
  await modal.present();
  const { data, role } = await modal.onDidDismiss();
  await modal.dismiss();
  return { result: role ? (role as MsModalResult) : MsModalResult.Cancel, noReminder: data?.noReminder };
}

interface DownloadEntryOptions {
  name: EntryName;
  workspaceHandle: WorkspaceHandle;
  workspaceId: WorkspaceID;
  path: FsPath;
  informationManager: InformationManager;
  fileOperationManager: FileOperationManager;
}

export async function downloadEntry(options: DownloadEntryOptions): Promise<void> {
  try {
    const saveHandle = await showSaveFilePicker({
      _preferPolyfill: false,
      suggestedName: options.name,
    });
    await options.fileOperationManager.downloadEntry(options.workspaceHandle, options.workspaceId, saveHandle, options.path);
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
