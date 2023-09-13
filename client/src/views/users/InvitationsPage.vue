<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <!-- contextual menu -->
      <ms-action-bar
        id="activate-users-ms-action-bar"
      >
        <ms-action-bar-button
          :icon="personAdd"
          id="button-invite-user"
          :button-label="$t('UsersPage.inviteUser')"
          @click="inviteUser()"
          v-show="isAdmin()"
        />
        <div class="right-side">
          <ms-grid-list-toggle
            v-model="displayView"
          />
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
                {{ $t('UsersPage.invitation.email') }}
              </ion-label>
              <ion-label class="invitation-list-header__label cell-title label-date">
                {{ $t('UsersPage.invitation.date') }}
              </ion-label>
              <ion-label class="invitation-list-header__label cell-title label-status">
                {{ $t('UsersPage.invitation.status') }}
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
import {
  IonPage,
  IonContent,
  IonList,
  IonItem,
  IonListHeader,
  IonLabel,
  modalController,
} from '@ionic/vue';
import {
  personAdd,
} from 'ionicons/icons';
import { onUpdated, ref, Ref } from 'vue';
import MsActionBar from '@/components/core/ms-action-bar/MsActionBar.vue';
import MsActionBarButton from '@/components/core/ms-action-bar/MsActionBarButton.vue';
import MsGridListToggle from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { DisplayState } from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { isAdmin } from '@/common/permissions';
import { useI18n } from 'vue-i18n';
import { MsModalResult } from '@/components/core/ms-types';
import { createAlert } from '@/components/core/ms-alert/MsAlertConfirmation';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import InvitationCard from '@/components/users/InvitationCard.vue';
import InvitationListItem from '@/components/users/InvitationListItem.vue';
import * as Parsec from '@/common/parsec';
import CreateUserInvitationModal from '@/views/users/CreateUserInvitationModal.vue';
import { isRoute } from '@/router/conditions';

const invitations: Ref<Parsec.UserInvitation[]> = ref([]);

const { t } = useI18n();

const displayView = ref(DisplayState.List);

onUpdated(async () => {
  if (isRoute('invitations')) {
    await refreshInvitationsList();
  }
});

async function refreshInvitationsList(): Promise<void> {
  const result = await Parsec.listUserInvitations();
  if (result.ok) {
    console.log('List invitations successful', result.value);
    invitations.value = result.value;
  } else {
    console.log('Failed to list invitations', result.error);
  }
}

async function inviteUser(): Promise<void> {
  const modal = await modalController.create({
    component: CreateUserInvitationModal,
    cssClass: 'create-user-invitation-modal',
  });
  modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    const result = await Parsec.inviteUser(data);

    if (result.ok) {
      console.log('Invite user successful', result.value);
      await refreshInvitationsList();
    } else {
      console.log(`Failed to invite ${data}`, result.error);
    }
  }
}

async function canDismissModal(_data?: any, modalRole?: string): Promise<boolean> {
  if (modalRole === MsModalResult.Confirm) {
    return true;
  }

  const alert = await createAlert(
    t('MsAlertConfirmation.areYouSure'),
    t('MsAlertConfirmation.infoNotSaved'),
    t('MsAlertConfirmation.cancel'),
    t('MsAlertConfirmation.ok'),
  );
  await alert.present();
  const { role } = await alert.onDidDismiss();
  return role === MsModalResult.Confirm;
}

async function greetUser(invitation: Parsec.UserInvitation): Promise<void> {
  const modal = await modalController.create({
    component: GreetUserModal,
    canDismiss: canDismissModal,
    cssClass: 'greet-organization-modal',
    componentProps: {
      invitation: invitation,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
}

async function rejectUser(invitation: Parsec.UserInvitation) : Promise<void> {
  const result = await Parsec.cancelInvitation(invitation.token);

  if (result.ok) {
    await refreshInvitationsList();
  } else {
    console.log('Could not cancel the invitation');
  }
}
</script>

<style scoped lang="scss">
.invitation-container {
  margin: 2rem;
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
  padding-inline-start:0;

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
