// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryModel } from '@/components/files';
import { SmallDisplayCategoryFileContextMenu, SmallDisplayFileContextMenu } from '@/components/small-display';
import { WorkspaceRole } from '@/parsec';
import { FileAction, FileContextMenu, FolderGlobalAction, FolderGlobalContextMenu } from '@/views/files';
import DownloadWarningModal from '@/views/files/DownloadWarningModal.vue';
import { modalController, popoverController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

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
