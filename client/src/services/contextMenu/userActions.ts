// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  ClientUserUpdateProfileError,
  ClientUserUpdateProfileErrorTag,
  revokeUser as parsecRevokeUser,
  updateProfile as parsecUpdateProfile,
  UserInfo,
  UserProfile,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import BulkRoleAssignmentModal from '@/views/users/BulkRoleAssignmentModal.vue';
import UpdateProfileModal from '@/views/users/UpdateProfileModal.vue';
import UserDetailsModal from '@/views/users/UserDetailsModal.vue';
import { modalController } from '@ionic/vue';
import { Answer, askQuestion, MsModalResult } from 'megashark-lib';
import { inject, Ref } from 'vue';

export function useUserActions() {
  const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;

  async function _revokeUser(user: UserInfo): Promise<void> {
    const answer = await askQuestion(
      { key: 'UsersPage.revocation.revokeTitle', count: 1 },
      { key: 'UsersPage.revocation.revokeQuestion', data: { user: user.humanHandle.label }, count: 1 },
      {
        yesIsDangerous: true,
        yesText: 'UsersPage.revocation.revokeYes',
        noText: 'UsersPage.revocation.revokeNo',
      },
    );
    if (answer === Answer.No) {
      return;
    }
    const result = await parsecRevokeUser(user.id);

    if (!result.ok) {
      informationManager.value.present(
        new Information({
          message: { key: 'UsersPage.revocation.revokeFailed', count: 1 },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.value.present(
        new Information({
          message: { key: 'UsersPage.revocation.revokeSuccess', data: { user: user.humanHandle.label }, count: 1 },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  }

  async function revokeUsers(users: Array<UserInfo>): Promise<void> {
    if (users.length === 1) {
      return await _revokeUser(users[0]);
    }

    const answer = await askQuestion(
      { key: 'UsersPage.revocation.revokeTitle', count: users.length },
      { key: 'UsersPage.revocation.revokeQuestion', data: { count: users.length }, count: users.length },
      {
        yesIsDangerous: true,
        yesText: 'UsersPage.revocation.revokeYes',
        noText: 'UsersPage.revocation.revokeNo',
      },
    );
    if (answer === Answer.No) {
      return;
    }
    let errorCount = 0;

    for (const user of users) {
      const result = await parsecRevokeUser(user.id);
      if (!result.ok) {
        errorCount += 1;
      }
    }
    if (errorCount === 0) {
      informationManager.value.present(
        new Information({
          message: {
            key: 'UsersPage.revocation.revokeSuccess',
            data: { count: users.length },
            count: users.length,
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else if (errorCount < users.length) {
      informationManager.value.present(
        new Information({
          message: 'UsersPage.revocation.revokeSomeFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.value.present(
        new Information({
          message: { key: 'UsersPage.revocation.revokeFailed', count: users.length },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }

  async function openDetails(user: UserInfo): Promise<void> {
    const modal = await modalController.create({
      component: UserDetailsModal,
      cssClass: 'user-details-modal',
      componentProps: {
        user: user,
        informationManager: informationManager.value,
      },
    });
    await modal.present();
    await modal.onWillDismiss();
    await modal.dismiss();
  }

  async function updateProfiles(users: Array<UserInfo>): Promise<void> {
    const modal = await modalController.create({
      component: UpdateProfileModal,
      cssClass: 'update-profile-modal',
      componentProps: {
        users: users,
      },
    });
    await modal.present();
    const { data, role } = await modal.onWillDismiss();
    await modal.dismiss();

    if (role !== MsModalResult.Confirm) {
      return;
    }
    const newProfile = data.profile;
    let firstError: ClientUserUpdateProfileError | undefined = undefined;
    const affectedUsers = users.filter((u) => u.currentProfile !== UserProfile.Outsider);

    for (const user of affectedUsers) {
      if (user.currentProfile === newProfile) {
        continue;
      }
      const result = await parsecUpdateProfile(user.id, newProfile);
      if (!result.ok) {
        if (!firstError) {
          firstError = result.error;
        }
      }
    }
    let message = '';
    if (!firstError) {
      message = 'UsersPage.updateProfile.success';
    } else {
      switch (firstError.tag) {
        case ClientUserUpdateProfileErrorTag.Offline:
          message = 'UsersPage.updateProfile.failedOffline';
          break;
        default:
          message = 'UsersPage.updateProfile.failedGeneric';
          break;
      }
    }
    informationManager.value.present(
      new Information({
        message: { key: message, count: affectedUsers.length },
        level: firstError === undefined ? InformationLevel.Success : InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }

  async function copyWorkspaceRoles(user: UserInfo, currentUser: UserInfo): Promise<void> {
    const modal = await modalController.create({
      component: BulkRoleAssignmentModal,
      cssClass: 'role-assignment-modal',
      componentProps: {
        sourceUser: user,
        currentUser: currentUser,
        informationManager: informationManager.value,
      },
    });
    await modal.present();
    await modal.onWillDismiss();
    await modal.dismiss();
  }

  return {
    revokeUsers,
    openDetails,
    updateProfiles,
    copyWorkspaceRoles,
  };
}
