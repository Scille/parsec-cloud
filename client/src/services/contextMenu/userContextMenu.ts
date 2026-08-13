// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { SmallDisplayCategoryUserContextMenu, SmallDisplayUserContextMenu } from '@/components/small-display';
import { UserInfo, UserProfile } from '@/parsec';
import { UserAction } from '@/services/contextMenu/types';
import { useUserActions } from '@/services/contextMenu/userActions';
import UserContextMenu from '@/views/users/UserContextMenu.vue';
import { modalController, popoverController } from '@ionic/vue';
import { useWindowSize } from 'megashark-lib';

async function _canUpdateProfile(users: UserInfo[], clientIsAdmin: boolean): Promise<boolean> {
  return clientIsAdmin && users.some((u) => u.currentProfile !== UserProfile.Outsider && !u.isRevoked());
}

async function _canRevoke(users: UserInfo[], clientIsAdmin: boolean): Promise<boolean> {
  return clientIsAdmin && users.some((u) => !u.isRevoked());
}

export function useUserContextMenu() {
  const { isLargeDisplay } = useWindowSize();
  const userActions = useUserActions();

  async function openUserContextMenu(users: Array<UserInfo>, currentUser: UserInfo, event: Event): Promise<void> {
    if (users.length === 0) {
      return;
    }

    // a Standard or Outsider user can't do anything with multiple users
    if (currentUser.currentProfile !== UserProfile.Admin && users.length > 1) {
      return;
    }

    let data: { action: UserAction } | undefined;

    if (isLargeDisplay.value) {
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
          canUpdateProfile: await _canUpdateProfile(users, currentUser.currentProfile === UserProfile.Admin),
          canRevoke: await _canRevoke(users, currentUser.currentProfile === UserProfile.Admin),
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
        expandToScroll: false,
        initialBreakpoint: 0.5,
        componentProps: {
          multipleSelected: users.length > 1,
          canUpdateProfile: await _canUpdateProfile(users, currentUser.currentProfile === UserProfile.Admin),
          canRevoke: await _canRevoke(users, currentUser.currentProfile === UserProfile.Admin),
        },
      });

      await modal.present();
      data = (await modal.onDidDismiss()).data;
      await modal.dismiss();
    }

    if (!data) {
      return;
    }

    switch (data.action) {
      case UserAction.Revoke: {
        await userActions.revokeUsers(users);
        break;
      }
      case UserAction.Details: {
        await userActions.openDetails(users[0]);
        break;
      }
      case UserAction.AssignRoles: {
        await userActions.copyWorkspaceRoles(users[0], currentUser);
        break;
      }
      case UserAction.UpdateProfile: {
        await userActions.updateProfiles(users);
        break;
      }
      default: {
        break;
      }
    }
  }

  async function openGlobalUserContextMenu(): Promise<UserAction | undefined> {
    const modal = await modalController.create({
      component: SmallDisplayCategoryUserContextMenu,
      cssClass: 'user-context-sheet-modal',
      showBackdrop: true,
      breakpoints: [0, 0.25, 1],
      expandToScroll: false,
      initialBreakpoint: 0.25,
    });
    await modal.present();
    const { data } = await modal.onWillDismiss();
    await modal.dismiss();
    return data.action;
  }

  return {
    openUserContextMenu,
    openGlobalUserContextMenu,
  };
}
