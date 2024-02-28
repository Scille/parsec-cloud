<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <!-- contextual menu -->
      <ms-action-bar id="activate-users-ms-action-bar">
        <div v-show="users.selectedCount() === 0 && isAdmin">
          <ms-action-bar-button
            :icon="personAdd"
            id="button-invite-user"
            :button-label="$t('UsersPage.inviteUser')"
            @click="inviteUser()"
          />
        </div>
        <!-- revoke or view common workspace -->
        <div v-show="activeSelectedCount >= 1 && isAdmin">
          <ms-action-bar-button
            :icon="personRemove"
            class="danger"
            id="button-revoke-user"
            :button-label="$t('UsersPage.userContextMenu.actionRevoke', activeSelectedCount)"
            @click="revokeSelectedUsers"
          />
        </div>
        <div v-show="users.selectedCount() === 1">
          <ms-action-bar-button
            :icon="informationCircle"
            id="button-common-workspaces"
            :button-label="$t('UsersPage.userContextMenu.actionDetails')"
            @click="openSelectedUserDetails"
          />
        </div>
        <div class="right-side">
          <div class="counter">
            <ion-text
              class="body"
              v-show="users.selectedCount() === 0"
            >
              {{ $t('UsersPage.itemCount', { count: users.usersCount() + 1 }, users.usersCount() + 1) }}
            </ion-text>
            <ion-text
              class="body item-selected"
              v-show="users.selectedCount() > 0"
            >
              {{ $t('UsersPage.userSelectedCount', { count: users.selectedCount() }, users.selectedCount()) }}
            </ion-text>
          </div>
          <ms-grid-list-toggle v-model="displayView" />
        </div>
      </ms-action-bar>
      <!-- users -->
      <div class="users-container scroll">
        <div
          v-show="users.usersCount() === 0"
          class="no-active body-lg"
        >
          <div class="no-active-content">
            <ms-image :image="NoActiveUser" />
            <ion-text>
              {{ $t('UsersPage.emptyList') }}
            </ion-text>
          </div>
        </div>
        <div v-show="users.usersCount() > 0">
          <div v-if="displayView === DisplayState.List && currentUser">
            <user-list-display
              :users="users"
              :current-user="currentUser"
              @menu-click="openUserContextMenu"
            />
          </div>
          <div
            v-else-if="currentUser"
            class="users-container-grid"
          >
            <user-grid-display
              :users="users"
              :current-user="currentUser"
              @menu-click="openUserContextMenu"
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import {
  Answer,
  DisplayState,
  MsActionBar,
  MsActionBarButton,
  MsGridListToggle,
  askQuestion,
  getTextInputFromUser,
} from '@/components/core';
import { MsImage, NoActiveUser } from '@/components/core/ms-image';
import { UserCollection, UserModel } from '@/components/users';
import {
  ClientInfo,
  ClientNewUserInvitationErrorTag,
  InvitationEmailSentStatus,
  UserID,
  UserInfo,
  UserProfile,
  getClientInfo as parsecGetClientInfo,
  inviteUser as parsecInviteUser,
  listUsers as parsecListUsers,
  revokeUser as parsecRevokeUser,
} from '@/parsec';
import { getCurrentRouteQuery, watchRoute } from '@/router';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import UserContextMenu, { UserAction } from '@/views/users/UserContextMenu.vue';
import UserDetailsModal from '@/views/users/UserDetailsModal.vue';
import UserGridDisplay from '@/views/users/UserGridDisplay.vue';
import UserListDisplay from '@/views/users/UserListDisplay.vue';
import { IonContent, IonPage, IonText, modalController, popoverController } from '@ionic/vue';
import { informationCircle, personAdd, personRemove } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';

const displayView = ref(DisplayState.List);
const isAdmin = ref(false);
const clientInfo: Ref<ClientInfo | null> = ref(null);
const informationManager: InformationManager = inject(InformationKey)!;

const users = ref(new UserCollection());
const currentUser: Ref<UserModel | null> = ref(null);

const activeSelectedCount = computed(() => {
  return users.value.selectedCount() - users.value.revokedSelectedCount();
});

async function revokeUser(user: UserInfo): Promise<void> {
  const answer = await askQuestion(
    translate('UsersPage.revocation.revokeTitle', {}, 1),
    translate('UsersPage.revocation.revokeQuestion', { user: user.humanHandle.label }, 1),
    {
      yesIsDangerous: true,
      yesText: translate('UsersPage.revocation.revokeYes'),
      noText: translate('UsersPage.revocation.revokeNo'),
    },
  );
  if (answer === Answer.No) {
    return;
  }
  const result = await parsecRevokeUser(user.id);

  if (!result.ok) {
    informationManager.present(
      new Information({
        message: translate('UsersPage.revocation.revokeFailed', {}, 1),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: translate('UsersPage.revocation.revokeSuccess', { user: user.humanHandle.label }, 1),
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  await refreshUserList();
}

async function revokeSelectedUsers(): Promise<void> {
  const selectedUsers = users.value.getSelectedUsers();

  if (selectedUsers.length === 1) {
    return await revokeUser(selectedUsers[0]);
  }

  const answer = await askQuestion(
    translate('UsersPage.revocation.revokeTitle', {}, selectedUsers.length),
    translate('UsersPage.revocation.revokeQuestion', { count: selectedUsers.length }, selectedUsers.length),
    {
      yesIsDangerous: true,
      yesText: translate('UsersPage.revocation.revokeYes'),
      noText: translate('UsersPage.revocation.revokeNo'),
    },
  );
  if (answer === Answer.No) {
    return;
  }
  let errorCount = 0;

  for (const user of selectedUsers) {
    const result = await parsecRevokeUser(user.id);
    if (!result.ok) {
      errorCount += 1;
    }
  }
  if (errorCount === 0) {
    informationManager.present(
      new Information({
        message: translate('UsersPage.revocation.revokeSuccess', { count: selectedUsers.length }, selectedUsers.length),
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  } else if (errorCount < selectedUsers.length) {
    informationManager.present(
      new Information({
        message: translate('UsersPage.revocation.revokeSomeFailed'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: translate('UsersPage.revocation.revokeFailed', {}, selectedUsers.length),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  await refreshUserList();
}

async function openUserDetails(user: UserInfo): Promise<void> {
  const modal = await modalController.create({
    component: UserDetailsModal,
    cssClass: 'user-details-modal',
    componentProps: {
      user: user,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

async function openSelectedUserDetails(): Promise<void> {
  const selectedUsers = users.value.getSelectedUsers();

  if (selectedUsers.length === 1) {
    return await openUserDetails(selectedUsers[0]);
  }
}

function isCurrentUser(userId: UserID): boolean {
  return clientInfo.value !== null && clientInfo.value.userId === userId;
}

async function openUserContextMenu(event: Event, user: UserInfo, onFinished?: () => void): Promise<void> {
  const popover = await popoverController.create({
    component: UserContextMenu,
    cssClass: 'user-context-menu',
    event: event,
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'end',
    componentProps: {
      isRevoked: user.isRevoked(),
      clientIsAdmin: isAdmin.value,
    },
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  const actions = new Map<UserAction, (user: UserInfo) => Promise<void>>([
    [UserAction.Revoke, revokeUser],
    [UserAction.Details, openUserDetails],
  ]);

  if (!data) {
    if (onFinished) {
      onFinished();
    }
    return;
  }

  const fn = actions.get(data.action);
  if (fn) {
    await fn(user);
  }
  if (onFinished) {
    onFinished();
  }
}

async function inviteUser(): Promise<void> {
  const email = await getTextInputFromUser({
    title: translate('UsersPage.CreateUserInvitationModal.pageTitle'),
    trim: true,
    validator: emailValidator,
    inputLabel: translate('UsersPage.CreateUserInvitationModal.label'),
    placeholder: translate('UsersPage.CreateUserInvitationModal.placeholder'),
    okButtonText: translate('UsersPage.CreateUserInvitationModal.create'),
  });
  if (!email) {
    return;
  }
  const result = await parsecInviteUser(email);
  if (result.ok) {
    if (result.value.emailSentStatus === InvitationEmailSentStatus.Success) {
      informationManager.present(
        new Information({
          message: translate('UsersPage.invitation.inviteSuccessMailSent', {
            email: email,
          }),
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: translate('UsersPage.invitation.inviteSuccessNoMail', {
            email: email,
          }),
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    let message = '';
    switch (result.error.tag) {
      case ClientNewUserInvitationErrorTag.AlreadyMember:
        message = translate('UsersPage.invitation.inviteFailedAlreadyMember');
        break;
      case ClientNewUserInvitationErrorTag.Offline:
        message = translate('UsersPage.invitation.inviteFailedOffline');
        break;
      case ClientNewUserInvitationErrorTag.NotAllowed:
        message = translate('UsersPage.invitation.inviteFailedNotAllowed');
        break;
      default:
        message = translate('UsersPage.invitation.inviteFailedUnknown', {
          reason: result.error.tag,
        });
        break;
    }
    informationManager.present(
      new Information({
        message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function refreshUserList(): Promise<void> {
  const result = await parsecListUsers(false);
  const newUsers: UserModel[] = [];
  if (result.ok) {
    for (const user of result.value) {
      (user as UserModel).isSelected = false;
      if (!isCurrentUser(user.id)) {
        newUsers.push(user as UserModel);
      } else {
        currentUser.value = user as UserModel;
      }
    }
    users.value.replace(newUsers);
  } else {
    informationManager.present(
      new Information({
        message: translate('UsersPage.listUsersFailed'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

const routeWatchCancel = watchRoute(async () => {
  const query = getCurrentRouteQuery();
  console.log(query);
  if (query.openInvite) {
    await inviteUser();
  }
  await refreshUserList();
});

onMounted(async (): Promise<void> => {
  const result = await parsecGetClientInfo();

  if (result.ok) {
    clientInfo.value = result.value;
    isAdmin.value = clientInfo.value.currentProfile === UserProfile.Admin;
  }
  await refreshUserList();
  const query = getCurrentRouteQuery();
  if (query.openInvite) {
    await inviteUser();
  }
});

onUnmounted(async () => {
  routeWatchCancel();
});
</script>

<style scoped lang="scss">
.no-active {
  width: 100%;
  height: 100%;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  margin: auto;
  align-items: center;

  &-content {
    border-radius: var(--parsec-radius-8);
    display: flex;
    height: fit-content;
    width: 100%;
    text-align: center;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
    padding: 2rem 1rem;
  }
}

.users-container > div {
  height: 100%;
}

.users-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
  height: 100%;
}
</style>
