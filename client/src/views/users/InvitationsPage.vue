<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <!-- replace all texts -->
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
        <div v-if="displayView === DisplayState.List">
          coucou
        </div>
        <div v-else>
          <ion-list class="invitation-list">
            <ion-item
              v-for="invitation in invitations"
              :key="invitation.token"
              class="invitation-list-item"
            >
              <invitation-card
                :invitation="invitation"
                @greet-user="openGreetUser"
                @reject-user="openRejectUser"
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
  modalController,
} from '@ionic/vue';
import {
  personAdd,
} from 'ionicons/icons';
import { onMounted, ref, Ref } from 'vue';
import MsActionBar from '@/components/core/ms-action-bar/MsActionBar.vue';
import MsActionBarButton from '@/components/core/ms-action-bar/MsActionBarButton.vue';
import MsGridListToggle from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { DisplayState } from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { isAdmin } from '@/common/permissions';
import { MockInvitation, getInvitations } from '@/common/mocks';
import { useI18n } from 'vue-i18n';
import { ModalResultCode } from '@/common/constants';
import { createAlert } from '@/components/core/ms-alert/MsAlertConfirmation';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import InvitationCard from '@/components/users/InvitationCard.vue';

const invitations: Ref<MockInvitation[]> = ref([]);

const { t } = useI18n();

const displayView = ref(DisplayState.List);

onMounted(async () => {
  invitations.value = await getInvitations();
});

function inviteUser(): void {
  console.log('Invite user clicked');
}

async function canDismissModal(_data?: any, modalRole?: string): Promise<boolean> {
  if (modalRole === ModalResultCode.Confirm) {
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
  return role === ModalResultCode.Confirm;
}

async function openGreetUser(invitation: MockInvitation): Promise<void> {
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

function openRejectUser(invitation: MockInvitation) : void {
  console.log(`Reject user ${invitation.email}`);
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

.invitation-list {
  padding: 0px;
  display: flex;
  flex-wrap: wrap;
  gap: 2rem;
}

.invitation-list-item {
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
