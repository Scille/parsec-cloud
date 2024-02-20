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
          <div class="counter">
            <ion-text class="body">
              {{ $t('UsersPage.invitation.invitationCount', { count: invitations.length }, invitations.length) }}
            </ion-text>
          </div>
          <ms-grid-list-toggle v-model="displayView" />
        </div>
      </ms-action-bar>

      <!-- content -->
      <div class="invitation-container scroll">
        <div
          v-if="invitations.length === 0"
          class="no-invitation body-lg"
        >
          <div class="no-invitation-content">
            <ms-image :image="NoInvitation" />
            <ion-text>
              {{ $t('UsersPage.invitation.noInvitations') }}
            </ion-text>
          </div>
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
import { MsImage, NoInvitation } from '@/components/core/ms-image';
import InvitationCard from '@/components/users/InvitationCard.vue';
import InvitationListItem from '@/components/users/InvitationListItem.vue';
import {
  ClientCancelInvitationErrorTag,
  ClientNewUserInvitationErrorTag,
  InvitationEmailSentStatus,
  UserInvitation,
  cancelInvitation as parsecCancelInvitation,
  inviteUser as parsecInviteUser,
  isAdmin as parsecIsAdmin,
  listUserInvitations as parsecListUserInvitations,
} from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, navigateTo, watchRoute } from '@/router';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import { IonContent, IonLabel, IonList, IonListHeader, IonPage, IonText, modalController } from '@ionic/vue';
import { personAdd } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, onUpdated, ref } from 'vue';

const invitations: Ref<UserInvitation[]> = ref([]);
const displayView = ref(DisplayState.List);
const isAdmin = ref(false);
const informationManager: InformationManager = inject(InformationKey)!;

const routeWatchCancel = watchRoute(async () => {
  const query = getCurrentRouteQuery();
  if (query.openInvite) {
    await inviteUser();
  }
});

onMounted(async () => {
  isAdmin.value = await parsecIsAdmin();
  await refreshInvitationsList();
  const query = getCurrentRouteQuery();
  if (query.openInvite) {
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
    informationManager.present(
      new Information({
        message: translate('UsersPage.invitation.invitationsListFailed'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
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
    await navigateTo(Routes.ActiveUsers);
  }
}

async function rejectUser(invitation: UserInvitation): Promise<void> {
  const result = await parsecCancelInvitation(invitation.token);

  if (result.ok) {
    informationManager.present(
      new Information({
        message: translate('UsersPage.invitation.cancelSuccess'),
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await refreshInvitationsList();
  } else {
    // In both those cases we can just refresh the list and the invitation should disappear, no need
    // to warn the user.
    if (
      result.error.tag === ClientCancelInvitationErrorTag.NotFound ||
      result.error.tag === ClientCancelInvitationErrorTag.AlreadyDeleted
    ) {
      await refreshInvitationsList();
    } else {
      informationManager.present(
        new Information({
          message: translate('UsersPage.invitation.cancelFailed'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
}
</script>

<style scoped lang="scss">
.no-invitation {
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
    max-width: 15vw;
    flex-grow: 0;
  }

  .label-status {
    width: 100%;
    max-width: 10vw;
    flex-grow: 0;
  }

  .label-space {
    min-width: 4rem;
    max-width: 20rem;
    width: 100%;
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
