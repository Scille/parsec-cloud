<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
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
      <div class="invitation-container">
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
          <ion-list class="invitation-card">
            <ion-item
              v-for="invitation in invitations"
              :key="invitation.token[0]"
              class="invitation-card-item"
            >
              <invitation-card
                :invitation="invitation"
                @greet-user="greetUser"
                @reject-user="rejectUser"
              />
            </ion-item>
          </ion-list>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent, IonList, IonItem, IonListHeader, IonLabel, modalController } from '@ionic/vue';
import { personAdd } from 'ionicons/icons';
import { onUpdated, ref, Ref, onMounted, inject, watch, onUnmounted } from 'vue';
import MsActionBar from '@/components/core/ms-action-bar/MsActionBar.vue';
import MsActionBarButton from '@/components/core/ms-action-bar/MsActionBarButton.vue';
import MsGridListToggle from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { DisplayState } from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { useI18n } from 'vue-i18n';
import { MsModalResult } from '@/components/core/ms-types';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import InvitationCard from '@/components/users/InvitationCard.vue';
import InvitationListItem from '@/components/users/InvitationListItem.vue';
import {
  isRoute,
  UserInvitation,
  listUserInvitations as parsecListUserInvitations,
  inviteUser as parsecInviteUser,
  cancelInvitation as parsecCancelInvitation,
  isAdmin as parsecIsAdmin,
  InvitationEmailSentStatus,
  InvitationErrorTag,
  DeleteInvitationErrorTag,
} from '@/parsec';
import { NotificationCenter, NotificationKey, NotificationLevel, Notification } from '@/services/notificationCenter';
import { useRoute } from 'vue-router';
import { routerNavigateTo } from '@/router';
import { getTextInputFromUser } from '@/components/core/ms-modal/MsTextInputModal.vue';
import { emailValidator } from '@/common/validators';

const invitations: Ref<UserInvitation[]> = ref([]);
const { t } = useI18n();
const displayView = ref(DisplayState.List);
const isAdmin = ref(false);
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationCenter: NotificationCenter = inject(NotificationKey)!;
const currentRoute = useRoute();

const routeUnwatch = watch(currentRoute, async (newRoute) => {
  if (newRoute.query.openInvite) {
    await inviteUser();
  }
});

onMounted(async () => {
  isAdmin.value = await parsecIsAdmin();
  await refreshInvitationsList();
  if (currentRoute.query.openInvite) {
    await inviteUser();
  }
});

onUpdated(async () => {
  if (isRoute('invitations')) {
    await refreshInvitationsList();
  }
});

onUnmounted(() => {
  routeUnwatch();
});

async function refreshInvitationsList(): Promise<void> {
  const result = await parsecListUserInvitations();
  if (result.ok) {
    invitations.value = result.value;
  } else {
    notificationCenter.showToast(
      new Notification({
        message: t('UsersPage.invitation.invitationsListFailed'),
        level: NotificationLevel.Error,
      }),
    );
  }
}

async function inviteUser(): Promise<void> {
  const email = await getTextInputFromUser({
    title: t('UsersPage.CreateUserInvitationModal.pageTitle'),
    trim: true,
    validator: emailValidator,
    inputLabel: t('UsersPage.CreateUserInvitationModal.label'),
    placeholder: t('UsersPage.CreateUserInvitationModal.placeholder'),
    okButtonText: t('UsersPage.CreateUserInvitationModal.create'),
  });

  if (!email) {
    return;
  }

  const result = await parsecInviteUser(email);

  if (result.ok) {
    await refreshInvitationsList();
    if (result.value.emailSentStatus === InvitationEmailSentStatus.Success) {
      await notificationCenter.showToast(
        new Notification({
          message: t('UsersPage.invitation.inviteSuccessMailSent', {
            email: email,
          }),
          level: NotificationLevel.Success,
        }),
      );
    } else {
      await notificationCenter.showToast(
        new Notification({
          message: t('UsersPage.invitation.inviteSuccessNoMail', {
            email: email,
          }),
          level: NotificationLevel.Success,
        }),
      );
    }
  } else {
    let message = '';
    switch (result.error.tag) {
      case InvitationErrorTag.AlreadyMember:
        message = t('UsersPage.invitation.inviteFailedAlreadyMember');
        break;
      case InvitationErrorTag.Offline:
        message = t('UsersPage.invitation.inviteFailedOffline');
        break;
      case InvitationErrorTag.NotAllowed:
        message = t('UsersPage.invitation.inviteFailedNotAllowed');
        break;
      default:
        message = t('UsersPage.invitation.inviteFailedUnknown', {
          reason: result.error.tag,
        });
        break;
    }

    await notificationCenter.showToast(
      new Notification({
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
    routerNavigateTo('activeUsers', {}, { refresh: true });
  }
}

async function rejectUser(invitation: UserInvitation): Promise<void> {
  const result = await parsecCancelInvitation(invitation.token);

  if (result.ok) {
    await notificationCenter.showToast(
      new Notification({
        message: t('UsersPage.invitation.cancelSuccess'),
        level: NotificationLevel.Success,
      }),
    );
    await refreshInvitationsList();
  } else {
    // In both those cases we can just refresh the list and the invitation should disappear, no need
    // to warn the user.
    if (result.error.tag === DeleteInvitationErrorTag.NotFound || result.error.tag === DeleteInvitationErrorTag.AlreadyDeleted) {
      await refreshInvitationsList();
    } else {
      await notificationCenter.showToast(
        new Notification({
          message: t('UsersPage.invitation.cancelFailed'),
          level: NotificationLevel.Error,
        }),
      );
    }
  }
}
</script>

<style scoped lang="scss">
.invitation-container {
  margin: 7em 2em 2em;
}

.right-side {
  margin-left: auto;
  display: flex;
}

.invitation-card {
  padding: 0;
  display: flex;
  flex-wrap: wrap;
  gap: 2rem;
}

.invitation-list-header {
  color: var(--parsec-color-light-secondary-grey);
  padding-inline-start: 0;

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

.invitation-card-item {
  width: 20rem;
  padding: 1rem;
  border: var(--parsec-color-light-secondary-disabled) 1px solid;
  --background: var(--parsec-color-light-secondary-background);
  background: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-6);
  position: relative;
  z-index: 2;

  &::part(native) {
    display: flex;
    flex-direction: column;
    padding: 0;
  }
}
</style>
