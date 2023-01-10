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
          <transition-group name="slide">
            <template v-if="showOrganizationList">
              <ion-card-content class="organization-list">
                <ion-card-title color="tertiary">
                  {{ $t('HomePage.organizationList.title') }}
                </ion-card-title>
                <ion-grid>
                  <ion-row>
                    <ion-col
                      size="1"
                      v-for="device in deviceList"
                      :key="device.slug"
                    >
                      <ion-card
                        button
                        class="organization-card-container"
                        @click="showOrganizationList = !showOrganizationList"
                      >
                        <ion-card-content>
                          <ion-grid>
                            <OrganizationCard :device="device"></OrganizationCard>
                            <ion-row>
                              <ion-col size="auto" v-if="getDeviceLocalStorageData(device.slug)">
                                {{ $t('HomePage.organizationList.organizationCard.lastLogin') }}
                                {{ $d(getDeviceLocalStorageData(device.slug).lastLogin, 'long') }}
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
            </template>
            <template v-if="!showOrganizationList">
              <ion-card-content class="organization-list">
                <ion-card-title color="tertiary">
                  {{ $t('HomePage.organizationLogin.backToList') }}
                </ion-card-title>
                <ion-grid>
                  <ion-row>
                    <ion-col
                      size="1"
                      v-for="device in deviceList"
                      :key="device.slug"
                    >
                      <ion-card
                        class="organization-card-container"
                      >
                        <ion-card-content>
                          <ion-grid>
                            <OrganizationCard :device="device"></OrganizationCard>
                            <ion-row>
                              <ion-col size="auto" v-if="getDeviceLocalStorageData(device.slug)">
                                {{ $t('HomePage.organizationList.organizationCard.lastLogin') }}
                                {{ $d(getDeviceLocalStorageData(device.slug).lastLogin, 'long') }}
                              </ion-col>
                            </ion-row>
                          </ion-grid>
                        </ion-card-content>
                      </ion-card>
                    </ion-col>
                  </ion-row>
                </ion-grid>
              </ion-card-content>
            </template>
          </transition-group>
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
import { ref } from 'vue';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import OrganizationCard from '@/components/OrganizationCard.vue';
import { createAlert } from '@/components/AlertConfirmation';
import { AvailableDevice } from '../plugins/libparsec/definitions';

const { t } = useI18n();
const deviceList: AvailableDevice[] = [
  {
    'organization_id': 'MegaShark',
    'human_handle': 'Maxime Grandcolas',
    'device_label': 'device_label',
    'key_file_path': 'key_file_path',
    'device_id': 'device_id',
    'slug': 'slug1',
    'ty': {'tag': 'Password'}
  },
  {
    'organization_id': 'Resana',
    'human_handle': 'Maxime Grandcolas',
    'device_label': 'device_label',
    'key_file_path': 'key_file_path',
    'device_id': 'device_id',
    'slug': 'slug2',
    'ty': {'tag': 'Password'}
  },
  {
    'organization_id': 'Oxymore',
    'human_handle': 'Maxime Grandcolas',
    'device_label': 'device_label',
    'key_file_path': 'key_file_path',
    'device_id': 'device_id',
    'slug': 'slug3',
    'ty': {'tag': 'Password'}
  },
  {
    'organization_id': 'EddyMalou',
    'human_handle': 'Maxime Grandcolas',
    'device_label': 'device_label',
    'key_file_path': 'key_file_path',
    'device_id': 'device_id',
    'slug': 'slug4',
    'ty': {'tag': 'Password'}
  }
];
const showOrganizationList = ref(true);

export interface DeviceLocalStorageData {
    slug: string;
    lastLogin: Date;
}

const deviceLocalStorageDataList = [
  {slug: 'slug1', lastLogin: new Date()},
  {slug: 'slug2', lastLogin: new Date()},
  {slug: 'slug3', lastLogin: new Date()}
];

function getDeviceLocalStorageData(deviceSlug: string): DeviceLocalStorageData {
  return deviceLocalStorageDataList.find((device) => {
    return device.slug === deviceSlug;
  });
}

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

  // .organization-list-enter-active {
  //   transition: all .3s ease;
  // }

  // .organization-list-leave-active {
  //   transition: all .8s cubic-bezier(1.0, 0.5, 0.8, 1.0);
  // }
  // .organization-list-enter-from,
  // .organization-list-leave-to {
  //   opacity: 0;
  //   transform: translateX(30px);
  // }

  // .organization-login-enter-active {
  //   transition: all .3s ease;
  // }

  // .organization-login-leave-active {
  //   transition: all .8s cubic-bezier(1.0, 0.5, 0.8, 1.0);
  // }
  // .organization-login-enter-from,
  // .organization-login-leave-to {
  //   opacity: 0;
  //   transform: translateX(-30px);
  // }

.slide-enter-active {
  z-index: 2;
  transition: 0.5s;
}

.slide-leave-active {
  z-index: -1;
  transition: 0.5s;
}

.slide-enter-from {
  z-index: 2;
  opacity: 0;
  transform: translate(-100%, 0);
}

.slide-leave-to {
  z-index: -1;
  opacity: 0;
  transform: translate(100%, 0);
}

  .logo {
    max-width: 10em;
    align-self: center;
    margin-bottom: 2em;
  }

  .organization-list {
    padding: 3em;
    padding-bottom: 4em;

    ion-grid {
      --ion-grid-padding: 1em;
      --ion-grid-columns: 2;
    }

    .organization-card-container {
      background: #F9F9FB;
      margin: 1em 1.5em;
      user-select: none;

      ion-row {
        height: 2em;
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

  #create-organization-button {
    margin-right: 1em;
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
