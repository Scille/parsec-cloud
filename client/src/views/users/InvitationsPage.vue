<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <!-- contextual menu -->
      <ms-action-bar id="activate-users-ms-action-bar">
        <ms-action-bar-button
          :icon="personAdd"
          id="button-invite-user"
          :button-label="$t('UsersPage.inviteUser')"
          @click="inviteUser()"
          v-show="isAdmin"
        />
        <div class="right-side">
          <ms-grid-list-toggle v-model="displayView" />
        </div>
      </ms-action-bar>

      <!-- content -->
      <div class="invitation-container scroll">
        <div v-if="invitations.length === 0">
          {{ $t('UsersPage.invitation.noInvitations') }}
        </div>
        <div v-if="invitations.length > 0 && displayView === DisplayState.List">
          <ion-list class="invitation-list">
            <ion-list-header
              class="invitation-list-header"
              lines="full"
            >
              <ion-label class="invitation-list-header__label cell-title label-email">
                {{ $t('UsersPage.invitation.emailTitle') }}
              </ion-label>
              <ion-label class="invitation-list-header__label cell-title label-date">
                {{ $t('UsersPage.invitation.dateTitle') }}
              </ion-label>
              <ion-label class="invitation-list-header__label cell-title label-status">
                {{ $t('UsersPage.invitation.statusTitle') }}
              </ion-label>
              <ion-label class="invitation-list-header__label cell-title label-space" />
            </ion-list-header>
            <invitation-list-item
              v-for="invitation in invitations"
              :key="invitation.token[0]"
              :invitation="invitation"
              @greet-user="greetUser"
              @reject-user="rejectUser"
              class="invitation-list-item"
            />
          </ion-list>
        </div>
        <div v-if="invitations.length > 0 && displayView === DisplayState.Grid">
          <ion-list class="invitation-container-grid">
            <invitation-card
              v-for="invitation in invitations"
              :key="invitation.token[0]"
              :invitation="invitation"
              @greet-user="greetUser"
              @reject-user="rejectUser"
            />
          </ion-list>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import { DisplayState, MsActionBar, MsActionBarButton, MsGridListToggle, MsModalResult, getTextInputFromUser } from '@/components/core';
import InvitationCard from '@/components/users/InvitationCard.vue';
import InvitationListItem from '@/components/users/InvitationListItem.vue';
import {
  InvitationEmailSentStatus,
  UserInvitation,
  cancelInvitation as parsecCancelInvitation,
  inviteUser as parsecInviteUser,
  isAdmin as parsecIsAdmin,
  listUserInvitations as parsecListUserInvitations,
  ClientCancelInvitationErrorTag,
  ClientNewUserInvitationErrorTag,
} from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, navigateTo, watchRoute } from '@/router';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { translate } from '@/services/translation';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import { IonContent, IonLabel, IonList, IonListHeader, IonPage, modalController } from '@ionic/vue';
import { personAdd } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, onUpdated, ref } from 'vue';

const invitations: Ref<UserInvitation[]> = ref([]);
const displayView = ref(DisplayState.List);
const isAdmin = ref(false);
const notificationManager: NotificationManager = inject(NotificationKey)!;

const routeWatchCancel = watchRoute(async () => {
  const query = getCurrentRouteQuery();
  if ('openInvite' in query) {
    await inviteUser();
  }
});

onMounted(async () => {
  isAdmin.value = await parsecIsAdmin();
  await refreshInvitationsList();
  const query = getCurrentRouteQuery();
  if ((query as any).openInvite) {
    await inviteUser();
  }
});

onUpdated(async () => {
  if (currentRouteIs(Routes.Invitations)) {
    await refreshInvitationsList();
  }
});

onUnmounted(() => {
  routeWatchCancel();
});

async function refreshInvitationsList(): Promise<void> {
  const result = await parsecListUserInvitations();
  if (result.ok) {
    invitations.value = result.value;
  } else {
    notificationManager.showToast(
      new Notification({
        title: translate('UsersPage.invitation.invitationsListFailed.title'),
        message: translate('UsersPage.invitation.invitationsListFailed.message'),
        level: NotificationLevel.Error,
      }),
    );
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
    await refreshInvitationsList();
    if (result.value.emailSentStatus === InvitationEmailSentStatus.Success) {
      notificationManager.showToast(
        new Notification({
          title: translate('UsersPage.invitation.inviteSuccessMailSent.title'),
          message: translate('UsersPage.invitation.inviteSuccessMailSent.message', {
            email: email,
          }),
          level: NotificationLevel.Success,
        }),
      );
    } else {
      notificationManager.showToast(
        new Notification({
          title: translate('UsersPage.invitation.inviteSuccessNoMail.title'),
          message: translate('UsersPage.invitation.inviteSuccessNoMail.message', {
            email: email,
          }),
          level: NotificationLevel.Success,
        }),
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

    notificationManager.showToast(
      new Notification({
        title: translate('UsersPage.invitation.inviteFailedUnknown.title'),
        message,
        level: NotificationLevel.Error,
      }),
    );
  }
}

async function greetUser(invitation: UserInvitation): Promise<void> {
  const modal = await modalController.create({
    component: GreetUserModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'greet-organization-modal',
    componentProps: {
      invitation: invitation,
    },
  });
  await modal.present();
  const modalResult = await modal.onWillDismiss();
  await modal.dismiss();
  await refreshInvitationsList();
  if (modalResult.role === MsModalResult.Confirm) {
    await navigateTo(Routes.ActiveUsers, { query: { refresh: true } });
  }
}

async function rejectUser(invitation: UserInvitation): Promise<void> {
  const result = await parsecCancelInvitation(invitation.token);

  if (result.ok) {
    notificationManager.showToast(
      new Notification({
        title: translate('UsersPage.invitation.cancelSuccess.title'),
        message: translate('UsersPage.invitation.cancelSuccess.message'),
        level: NotificationLevel.Success,
      }),
    );
    await refreshInvitationsList();
  } else {
    // In both those cases we can just refresh the list and the invitation should disappear, no need
    // to warn the user.
    if (result.error.tag === ClientCancelInvitationErrorTag.NotFound
    || result.error.tag === ClientCancelInvitationErrorTag.AlreadyDeleted) {
      await refreshInvitationsList();
    } else {
      notificationManager.showToast(
        new Notification({
          title: translate('UsersPage.invitation.cancelFailed.title'),
          message: translate('UsersPage.invitation.cancelFailed.message'),
          level: NotificationLevel.Error,
        }),
      );
    }
  }
}
</script>

<style scoped lang="scss">
.invitation-list-header {
  color: var(--parsec-color-light-secondary-grey);
  padding-inline-start: 0;
  width: 100%;

  &__label {
    padding: 0 1rem;
    height: 100%;
    display: flex;
    align-items: center;
  }

  .label-email {
    width: 100%;
    max-width: 30vw;
    white-space: nowrap;
    overflow: hidden;
  }

  .label-date {
    width: 100%;
    max-width: 10vw;
    flex-grow: 0;
  }

  .label-status {
    width: 100%;
    max-width: 10vw;
    flex-grow: 0;
  }

  .label-space {
    width: 100%;
    min-width: 4rem;
    flex-grow: 0;
  }
}

.invitation-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5rem;
  height: 100%;
}
</style>
