<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
    >
      <ul class="invitations-list">
        <li
          class="invitation"
          v-for="invitation in invitations"
          :key="invitation.token"
          @click="openGreetUser(invitation)"
        >
          {{ invitation.email }}
        </li>
      </ul>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  IonContent,
  modalController,
} from '@ionic/vue';
import { onMounted, ref, Ref } from 'vue';
import { MockInvitation, getInvitations } from '@/common/mocks';
import { ModalResultCode } from '@/common/constants';
import { createAlert } from '@/components/core/ms-alert/MsAlertConfirmation';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import { useI18n } from 'vue-i18n';

const invitations: Ref<MockInvitation[]> = ref([]);

const { t } = useI18n();

onMounted(async () => {
  invitations.value = await getInvitations();
});

async function canDismissModal(_data?: any, modalRole?: string): Promise<boolean> {
  if (modalRole === ModalResultCode.Confirm) {
    return true;
  }

  const alert = await createAlert(
    t('AlertConfirmation.areYouSure'),
    t('AlertConfirmation.infoNotSaved'),
    t('AlertConfirmation.cancel'),
    t('AlertConfirmation.ok'),
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
</script>

<style scoped lang="scss">
.invitations-list {
  list-style: none;
  padding: 0px;
}
.invitation {
  font-weight: bold;
  font-family: "Comic Sans MS", "Comic Sans", cursive;
  color: red;
  font-size: 4em;
}

.invitation:before {
  content: '\1F346';
  color: purple;
  margin: 0 1em;
}
</style>
