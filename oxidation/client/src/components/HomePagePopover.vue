<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item
      class="container__item"
      @click="openCreateOrganizationModal()"
    >
      <ion-icon
        :icon="addCircle"
        slot="start"
      />
      <div class="text-content">
        <ion-label class="body">
          {{ $t('HomePage.noExistingOrganization.createOrganizationTitle') }}
        </ion-label>
        <ion-text class="body-sm sub-text">
          {{ $t('HomePage.noExistingOrganization.createOrganizationSubtitle') }}
        </ion-text>
      </div>
    </ion-item>
    <ion-item
      class="container__item"
      @click="openJoinByLinkModal()"
    >
      <ion-icon
        :icon="mail"
        slot="start"
      />
      <div class="text-content">
        <ion-label class="body">
          {{ $t('HomePage.noExistingOrganization.joinOrganizationTitle') }}
        </ion-label>
        <ion-text class="body-sm sub-text">
          {{ $t('HomePage.noExistingOrganization.joinOrganizationSubtitle') }}
        </ion-text>
      </div>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import {
  IonList,
  IonItem,
  IonIcon,
  IonLabel,
  IonText,
  modalController
} from '@ionic/vue';
import {
  addCircle,
  mail
} from 'ionicons/icons';
import { createAlert } from '@/components/AlertConfirmation';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

async function openJoinByLinkModal(): Promise<void> {
  const modal = await modalController.create({
    component: JoinByLinkModal,
    cssClass: 'join-by-link-modal'
  });
  await modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }
}

async function openCreateOrganizationModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganization,
    canDismiss: canDismissModal,
    cssClass: 'create-organization-modal'
  });
  await modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }
}

async function canDismissModal(): Promise<boolean> {
  const alert = await createAlert(
    t('AlertConfirmation.areYouSure'),
    t('AlertConfirmation.infoNotSaved'),
    t('AlertConfirmation.cancel'),
    t('AlertConfirmation.ok')
  );

  await alert.present();

  const { role } = await alert.onDidDismiss();
  return role === 'confirm';
}

</script>

<style lang="scss" scoped>
.container {
  padding: 0.5rem;
  border-radius: 0.5rem;
  overflow: hidden;
}

.container__item {
  --background: none;
  width: 100%;
  --min-height: 1rem;
  user-select: none;
  cursor: pointer;
  margin-inline-end: 2px;
  color: var(--parsec-color-light-primary-600);
  border-radius: 0.25rem;

  ion-icon {
    color: var(--parsec-color-light-primary-600);
    margin-inline-end: 0.75rem;
    margin-top: 1rem;
    margin-bottom: 1rem;
  }

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    color: var(--parsec-color-light-primary-600);
  }

  .text-content {
    display: flex;
    flex-direction: column;
    justify-content: center;

    .sub-text {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

</style>
