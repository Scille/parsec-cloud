<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      color="secondary"
    >
      <div id="container">
        <img
          src="../assets/images/Logo/Logo/PNG/logo_blue.png"
          class="logo"
        >
        <ion-card>
          <ion-card-content class="organization-list">
            <ion-card-title color="tertiary">
              {{ $t('HomePage.organizationList.title') }}
            </ion-card-title>
            <ion-grid>
              <ion-row>
                <ion-col
                  size="1"
                  v-for="organization in organizationList"
                  :key="organization.label"
                >
                  <ion-card
                    class="organization-card"
                  >
                    <ion-card-content>
                      <ion-grid>
                        <ion-row class="ion-align-items-center">
                          <ion-col size="auto">
                            <ion-avatar>
                              <span>{{ organization.label.substring(0, 2) }}</span>
                            </ion-avatar>
                          </ion-col>
                          <ion-col size="auto">
                            <p class="organization-label">
                              {{ organization.label }}
                            </p>
                            <p>
                              {{ organization.username }}
                            </p>
                          </ion-col>
                        </ion-row>
                        <ion-row>
                          <ion-col size="auto">
                            <p>
                              {{ $t('HomePage.organizationList.organizationCard.lastLogin') }} {{$d(organization.lastLogin, 'long')}}
                            </p>
                          </ion-col>
                        </ion-row>
                      </ion-grid>
                    </ion-card-content>
                  </ion-card>
                </ion-col>
              </ion-row>
            </ion-grid>
          </ion-card-content>

          <ion-card-content class="no-existing-organization">
            <ion-card-title color="tertiary">
              {{ $t('HomePage.noExistingOrganization.title') }}
            </ion-card-title>
            <ion-button
              @click="openCreateOrganizationModal()"
              size="large"
              id="create-organization-button"
            >
              <ion-icon
                slot="start"
                :icon="add"
              />
              {{ $t('HomePage.noExistingOrganization.createOrganization') }}
            </ion-button>
            <ion-button
              @click="openJoinByLinkModal()"
              fill="outline"
              size="large"
            >
              <ion-icon
                slot="start"
                :icon="link"
              />
              {{ $t('HomePage.noExistingOrganization.joinOrganization') }}
            </ion-button>
          </ion-card-content>
        </ion-card>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonAvatar,
  IonContent,
  IonPage,
  IonCard,
  IonCardContent,
  IonCardTitle,
  IonCardSubtitle,
  IonButton,
  IonIcon,
  IonRow,
  IonCol,
  IonGrid,
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
const organizationList = [
  {
    label: 'Ionic',
    username: 'Maxime Grandcolas',
    lastLogin: new Date()
  },
  {
    label: 'Ionic 2',
    username: 'Maxime Grandcolas',
    lastLogin: new Date()
  },
  {
    label: 'Ionic 3',
    username: 'Maxime Grandcolas',
    lastLogin: new Date()
  },
  {
    label: 'Ionic 3',
    username: 'Maxime Grandcolas',
    lastLogin: new Date()
  }
];

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
  display: flex;
  justify-content: center;
  flex-direction: column;

  max-width: 50vw;
  margin: 0 auto;

  .logo {
    max-width: 10em;
    align-self: center;
  }

  .organization-list {
    padding: 3em;
    padding-bottom: 4em;

    ion-grid {
      --ion-grid-padding: 1em;
      --ion-grid-columns: 2;
    }

    .organization-card {
      background: #F9F9FB;
      margin: 1em 1.5em;

      ion-avatar {
        background: white;
        color: #0058cc;
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 2em;
        text-transform: uppercase;
      }

      .organization-label {
        color: #004299;
        font-size: 1.5em;
      }

      &:hover {
        background: #E5F1FF;
        cursor: pointer;
      }
    }
  }

  .no-existing-organization {
    border-top: 1px solid #cce2ff;
    background: #fafafa;
    padding: 3em;
    padding-bottom: 4em;
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
