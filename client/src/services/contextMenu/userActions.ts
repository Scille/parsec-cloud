// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { matchingStringValidator } from '@/common/validators';
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
import { getTextFromUser, I18n, MsModalResult, MsReportTheme, useWindowSize } from 'megashark-lib';
import { inject, Ref } from 'vue';

export function useUserActions() {
  const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
  const { isLargeDisplay } = useWindowSize();

  async function revokeUser(user: UserInfo): Promise<void> {
    // Ask for confirmation by inputting the user email
    const userEmail = await getTextFromUser(
      {
        title: 'UsersPage.revocation.revokeTitle',
        additionalMessage: {
          message: { key: 'UsersPage.revocation.revokeText', data: { user: user.humanHandle.label, email: user.humanHandle.email } },
          theme: MsReportTheme.Warning,
        },
        trim: true,
        validator: matchingStringValidator(user.humanHandle.email),
        inputLabel: 'UsersPage.revocation.label',
        placeholder: I18n.valueAsTranslatable(user.humanHandle.email),
        okButtonText: 'UsersPage.revocation.revokeYes',
        yesIsDangerous: true,
      },
      isLargeDisplay.value,
    );

    if (!userEmail || userEmail.localeCompare(user.humanHandle.email) !== 0) {
      // Shouldn't happen
      return;
    }

    const result = await parsecRevokeUser(user.id);

    if (!result.ok) {
      informationManager.value.present(
        new Information({
          message: 'UsersPage.revocation.revokeFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.value.present(
        new Information({
          message: { key: 'UsersPage.revocation.revokeSuccess', data: { user: user.humanHandle.label } },
          level: InformationLevel.Success,
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
    revokeUser,
    openDetails,
    updateProfiles,
    copyWorkspaceRoles,
  };
}
