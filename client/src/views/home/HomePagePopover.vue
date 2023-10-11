<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
  modalController,
  popoverController,
} from '@ionic/vue';
import {
  addCircle,
  mail,
} from 'ionicons/icons';

import CreateOrganizationModal from '@/views/home/CreateOrganizationModal.vue';
import UserJoinOrganizationModal from '@/views/home/UserJoinOrganizationModal.vue';
import DeviceJoinOrganizationModal from '@/views/home/DeviceJoinOrganizationModal.vue';
import { claimUserLinkValidator, claimDeviceLinkValidator, Validity, claimLinkValidator } from '@/common/validators';
import { useI18n } from 'vue-i18n';
import { MsModalResult } from '@/components/core/ms-types';
import { getTextInputFromUser } from '@/components/core/ms-modal/MsTextInputModal.vue';

const { t } = useI18n();

async function openCreateOrganizationModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganizationModal,
    canDismiss: true,
    cssClass: 'create-organization-modal',
  });
  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await popoverController.dismiss(data, role);
}

// Commented until https://github.com/Scille/parsec-cloud/issues/5429

// async function canDismissModal(data?: any, modalRole?: string): Promise<boolean> {
//   if (modalRole === MsModalResult.Confirm) {
//     return true;
//   }
//   const alert = await createAlert(
//     t('MsAlertConfirmation.areYouSure'),
//     t('MsAlertConfirmation.infoNotSaved'),
//     t('MsAlertConfirmation.cancel'),
//     t('MsAlertConfirmation.ok'),
//   );
//   await alert.present();
//   const { role } = await alert.onDidDismiss();
//   return role === MsModalResult.Confirm;
// }

async function openJoinByLinkModal(): Promise<void> {
  const link = await getTextInputFromUser({
    title: t('JoinByLinkModal.pageTitle'),
    subtitle: t('JoinByLinkModal.pleaseEnterUrl'),
    trim: true,
    validator: claimLinkValidator,
    inputLabel: t('JoinOrganization.linkFormLabel'),
    placeholder: t('JoinOrganization.linkFormPlaceholder'),
    okButtonText: t('JoinByLinkModal.join'),
  });

  if (!link) {
    await popoverController.dismiss(null, MsModalResult.Cancel);
    return;
  }

  if (await claimUserLinkValidator(link) === Validity.Valid) {
    const modal = await modalController.create({
      component: UserJoinOrganizationModal,
      canDismiss: true,
      cssClass: 'join-organization-modal',
      componentProps: {
        invitationLink: link,
      },
    });
    await modal.present();
    const result = await modal.onWillDismiss();
    await modal.dismiss();
    await popoverController.dismiss(result.data, result.role);
  } else if (await claimDeviceLinkValidator(link) === Validity.Valid) {
    const modal = await modalController.create({
      component: DeviceJoinOrganizationModal,
      canDismiss: true,
      cssClass: 'join-organization-modal',
      componentProps: {
        invitationLink: link,
      },
    });
    await modal.present();
    const result = await modal.onWillDismiss();
    await modal.dismiss();
    await popoverController.dismiss(result.data, result.role);
  } else {
    console.error('Invalid data gotten from link, should never happen');
    await popoverController.dismiss(null, MsModalResult.Cancel);
  }
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
