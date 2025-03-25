// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { SmallDisplayCategoryUserContextMenu, SmallDisplayUserContextMenu } from '@/components/small-display';
import { UserModel } from '@/components/users';
import { UserProfile } from '@/parsec';
import UserContextMenu, { UserAction } from '@/views/users/UserContextMenu.vue';
import { modalController, popoverController } from '@ionic/vue';

async function canUpdateProfile(users: UserModel[], clientIsAdmin: boolean): Promise<boolean> {
  return clientIsAdmin && users.some((u) => u.currentProfile !== UserProfile.Outsider && !u.isRevoked());
}

async function canRevoke(users: UserModel[], clientIsAdmin: boolean): Promise<boolean> {
  return clientIsAdmin && users.some((u) => !u.isRevoked());
}

export async function openUserContextMenu(
  event: Event,
  users: UserModel[],
  isAdmin: boolean,
  isLargeDisplay: boolean,
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
        multipleSelected: users.length > 1,
        canUpdateProfile: await canUpdateProfile(users, isAdmin),
        canRevoke: await canRevoke(users, isAdmin),
      },
    });

    await popover.present();
    data = (await popover.onDidDismiss()).data;
    await popover.dismiss();
  } else {
    const modal = await modalController.create({
      component: SmallDisplayUserContextMenu,
      cssClass: 'user-context-sheet-modal',
      showBackdrop: true,
      breakpoints: [0, 0.5, 1],
      // https://ionicframework.com/docs/api/modal#scrolling-content-at-all-breakpoints
      // expandToScroll: false, should be added to scroll with Ionic 8
      initialBreakpoint: 0.5,
      componentProps: {
        multipleSelected: users.length > 1,
        canUpdateProfile: await canUpdateProfile(users, isAdmin),
        canRevoke: await canRevoke(users, isAdmin),
      },
    });

    await modal.present();
    data = (await modal.onDidDismiss()).data;
    await modal.dismiss();
  }
  return data;
}

export async function openGlobalUserContextMenu(): Promise<{ action: UserAction } | undefined> {
  const modal = await modalController.create({
    component: SmallDisplayCategoryUserContextMenu,
    cssClass: 'user-context-sheet-modal',
    showBackdrop: true,
    breakpoints: [0, 0.25, 1],
    initialBreakpoint: 0.25,
  });
  await modal.present();
  const { data } = await modal.onWillDismiss();
  await modal.dismiss();
  return data;
}
