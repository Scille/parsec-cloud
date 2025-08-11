// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { formatFileSize } from '@/common/file';
import { EntryModel } from '@/components/files';
import { SmallDisplayCategoryFileContextMenu, SmallDisplayFileContextMenu } from '@/components/small-display';
import { EntryName, EntryStat, EntryStatFile, EntryTree, FsPath, listTree, WorkspaceHandle, WorkspaceID, WorkspaceRole } from '@/parsec';
import { FileOperationManager } from '@/services/fileOperationManager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { StorageManager } from '@/services/storageManager';
import { FileAction, FileContextMenu, FolderGlobalAction, FolderGlobalContextMenu } from '@/views/files';
import DownloadWarningModal from '@/views/files/DownloadWarningModal.vue';
import { modalController, popoverController } from '@ionic/vue';
import { Answer, askQuestion, I18n, MsModalResult } from 'megashark-lib';
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

export async function openDownloadConfirmationModal(storageManager: StorageManager): Promise<MsModalResult> {
  console.log('openDownloadConfirmationModal called');
  const config = await storageManager.retrieveConfig();
  if (!config.disableDownloadWarning) {
    const { result, noReminder } = await askDownloadConfirmation();

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
}

interface DownloadEntryOptions extends DownloadOptions {
  name: EntryName;
  path: FsPath;
}

interface DownloadEntriesOptions extends DownloadOptions {
  entries: Array<EntryStat>;
  archiveName: EntryName;
  relativePath: string;
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

export async function downloadArchive(options: DownloadEntriesOptions): Promise<void> {
  if (options.entries.length === 0) {
    return;
  }

  const trees: Array<EntryTree> = [];
  let totalSize = 0;
  let totalFiles = 0;
  let maxRecursionReached = false;
  let maxFilesReached = false;
  for (const entry of options.entries) {
    if (entry.isFile()) {
      trees.push({
        totalSize: (entry as EntryStatFile).size,
        entries: [entry as EntryStatFile],
        maxRecursionReached: false,
        maxFilesReached: false,
      });
      totalSize += (entry as EntryStatFile).size;
      totalFiles += 1;
    } else {
      const tree = await listTree(options.workspaceHandle, entry.path);
      totalSize += tree.totalSize;
      totalFiles += tree.entries.length;
      maxRecursionReached ||= tree.maxRecursionReached;
      maxFilesReached ||= tree.maxFilesReached;
      trees.push(tree);
    }
  }

  if (maxRecursionReached) {
    window.electronAPI.log('error', 'Maximum recursion reached when downloading archive');
    options.informationManager.present(
      new Information({
        message: 'FoldersPage.DownloadFile.maximumRecursionReached',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  } else if (maxFilesReached) {
    window.electronAPI.log('error', 'Maximum file reached when downloading archive');
    options.informationManager.present(
      new Information({
        message: 'FoldersPage.DownloadFile.maximumFilesReached',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const answer = await askQuestion(
    'FoldersPage.DownloadFile.archiveTitle',
    { key: 'FoldersPage.DownloadFile.archiveQuestion', data: { size: I18n.translate(formatFileSize(totalSize)) } },
    {
      noText: 'FoldersPage.DownloadFile.archiveNo',
      yesText: 'FoldersPage.DownloadFile.archiveYes',
    },
  );
  if (answer === Answer.No) {
    return;
  }

  try {
    const saveHandle = await showSaveFilePicker({
      _preferPolyfill: false,
      suggestedName: options.archiveName,
    });
    await options.fileOperationManager.downloadArchive(
      options.workspaceHandle,
      options.workspaceId,
      saveHandle,
      trees,
      options.relativePath,
      totalFiles,
      totalSize,
    );
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
