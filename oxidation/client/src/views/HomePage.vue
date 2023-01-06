<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      color="secondary"
    >
      <div id="container">
        <img
          src="../assets/images/parsec.svg"
          class="logo"
        >
        <ion-card>
          <ion-card-content>
            <ion-card-title>{{ $t('HomePage.organizationList.title') }}</ion-card-title>
            <p>{{ $t('HomePage.pleaseConnectToAnOrganization') }}</p>
            <ion-button
              @click="openCreateOrganizationModal()"
              expand="full"
              size="large"
              id="create-organization-button"
            >
              <ion-icon
                slot="start"
                :icon="add"
              />
              {{ $t('HomePage.createOrganization') }}
            </ion-button>
            <ion-button
              @click="openJoinByLinkModal()"
              expand="full"
              size="large"
            >
              <ion-icon
                slot="start"
                :icon="link"
              />
              {{ $t('HomePage.joinByLink') }}
            </ion-button>
            <ion-button
              v-if="isPlatform('mobile')"
              expand="full"
              size="large"
            >
              <ion-icon
                slot="start"
                :icon="qrCodeSharp"
              />
              {{ $t('HomePage.joinByQRcode') }}
            </ion-button>
          </ion-card-content>

          <ion-card-content class="noExistingOrganization">
            <ion-card-title>{{ $t('HomePage.noExistingOrganization.title') }}</ion-card-title>
            <p>{{ $t('HomePage.pleaseConnectToAnOrganization') }}</p>
            <ion-button
              @click="openCreateOrganizationModal()"
              expand="full"
              size="large"
              id="create-organization-button"
            >
              <ion-icon
                slot="start"
                :icon="add"
              />
              {{ $t('HomePage.createOrganization') }}
            </ion-button>
            <ion-button
              @click="openJoinByLinkModal()"
              expand="full"
              size="large"
            >
              <ion-icon
                slot="start"
                :icon="link"
              />
              {{ $t('HomePage.joinByLink') }}
            </ion-button>
            <ion-button
              v-if="isPlatform('mobile')"
              expand="full"
              size="large"
            >
              <ion-icon
                slot="start"
                :icon="qrCodeSharp"
              />
              {{ $t('HomePage.joinByQRcode') }}
            </ion-button>
          </ion-card-content>
        </ion-card>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonPage,
  IonCard,
  IonCardHeader,
  IonCardContent,
  IonCardTitle,
  IonButton,
  IonIcon,
  isPlatform,
  modalController
} from '@ionic/vue';
import {
  add,
  link,
  qrCodeSharp
} from 'ionicons/icons'; // We're forced to import icons for the moment, see : https://github.com/ionic-team/ionicons/issues/1032
import { useI18n } from 'vue-i18n';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import { createAlert } from '@/components/AlertConfirmation';

const { t } = useI18n();

async function openJoinByLinkModal(): Promise<void> {
  const modal = await modalController.create({
    component: JoinByLinkModal,
    cssClass: 'join-by-link-modal'
  });
  modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }
}

async function openCreateOrganizationModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganization,
    canDismiss: canDismissModal
  });
  modal.present();

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
#container {
  height: 100%;
  justify-content: center;
  display: flex;
  flex-direction: column;
  // position: absolute;
  // left: 0;
  // right: 0;
  // top: 50%;
  // transform: translateY(-50%);

  max-width: 680px;
  margin: 0 auto;

  p {
    font-weight: bold;
  }

  .logo {
    max-width: 10em;
    align-self: center;
  }
}

.lang-list {
  .item {
    ion-label {
      margin-left: 3.5em;
    }
  }
}
</style>
