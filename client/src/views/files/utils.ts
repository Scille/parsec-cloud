// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryModel } from '@/components/files';
import { WorkspaceRole } from '@/parsec';
import { FileAction, FileContextMenu, SmallDisplayFileContextMenu } from '@/views/files';
import { modalController, popoverController } from '@ionic/vue';

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
  } else {
    const modal = await modalController.create({
      component: SmallDisplayFileContextMenu,
      cssClass: 'file-context-sheet-modal',
      breakpoints: [0, 0.5, 1],
      initialBreakpoint: 1,
      showBackdrop: false,
      componentProps: {
        role: ownRole,
        multipleFiles: selectedEntries.length > 1 && selectedEntries.includes(entry),
        isFile: entry.isFile(),
      },
    });

    await modal.present();
    data = (await modal.onDidDismiss()).data;
  }

  return data;
}
