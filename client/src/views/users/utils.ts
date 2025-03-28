// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { SmallDisplayCategoryUserContextMenu, SmallDisplayUserContextMenu } from '@/components/small-display';
import { UserInfo } from '@/parsec';
import UserContextMenu, { UserAction } from '@/views/users/UserContextMenu.vue';
import { modalController, popoverController } from '@ionic/vue';

export async function openUserContextMenu(
  event: Event,
  user: UserInfo,
  isAdmin: boolean,
  isSelectable: boolean,
  someSelected: boolean,
  isLargeDisplay: boolean,
  fromRightClick: boolean,
): Promise<{ action: UserAction } | undefined> {
  let data: { action: UserAction } | undefined;

  if (isLargeDisplay) {
    const popover = await popoverController.create({
      component: UserContextMenu,
      cssClass: 'user-context-menu',
      event: event,
      translucent: true,
      reference: event.type === 'contextmenu' ? 'event' : 'trigger',
      showBackdrop: false,
      dismissOnSelect: true,
      alignment: 'start',
      componentProps: {
        user: user,
        clientIsAdmin: isAdmin,
      },
    });

    await popover.present();
    data = (await popover.onDidDismiss()).data;
    await popover.dismiss();
  } else if (fromRightClick) {
    const modal = await modalController.create({
      component: SmallDisplayUserContextMenu,
      cssClass: 'user-context-sheet-modal',
      showBackdrop: true,
      breakpoints: [0, 0.5, 1],
      // https://ionicframework.com/docs/api/modal#scrolling-content-at-all-breakpoints
      // expandToScroll: false, should be added to scroll with Ionic 8
      initialBreakpoint: 0.5,
      componentProps: {
        user: user,
        clientIsAdmin: isAdmin,
      },
    });

    await modal.present();
    data = (await modal.onDidDismiss()).data;
    await modal.dismiss();
  } else {
    const modal = await modalController.create({
      component: SmallDisplayCategoryUserContextMenu,
      cssClass: 'user-context-sheet-modal',
      showBackdrop: true,
      breakpoints: [0, 0.25, 1],
      initialBreakpoint: 0.25,
      componentProps: {
        selectable: isSelectable,
        someSelected: someSelected,
      },
    });
    await modal.present();
    data = (await modal.onWillDismiss()).data;
    await modal.dismiss();
  }
  return data;
}
