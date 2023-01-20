<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      color="secondary"
    >
      <div id="container">
        <div class="logo">
          <img src="../assets/images/Logo/logo_blue.png">
        </div>
        <transition-group :name="showOrganizationList ? 'slide-left' : 'slide-right'">
          <ion-card
            v-if="showOrganizationList"
            id="organization-list-container"
          >
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
                      @click="onOrganizationCardClick(device)"
                    >
                      <ion-card-content>
                        <ion-grid>
                          <OrganizationCard :device="device" />
                          <ion-row>
                            <ion-col
                              size="auto"
                              v-if="getDeviceLocalStorageData(device.slug)"
                            >
                              {{ formatLastLogin(getDeviceLocalStorageData(device.slug).lastLogin) }}
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
                id="join-by-link-button"
              >
                <ion-icon
                  slot="start"
                  :icon="link"
                />
                {{ $t('HomePage.noExistingOrganization.joinOrganization') }}
              </ion-button>
            </ion-card-content>
          </ion-card>
          <ion-card
            v-if="!showOrganizationList"
            id="login-popup-container"
          >
            <ion-card-content class="organization-list">
              <ion-card-title color="tertiary">
                <ion-button
                  fill="clear"
                  @click="showOrganizationList = !showOrganizationList"
                  id="back-to-list-button"
                >
                  <ion-icon
                    slot="start"
                    :icon="chevronBackOutline"
                  />
                  {{ $t('HomePage.organizationLogin.backToList') }}
                </ion-button>
              </ion-card-title>
              <div id="login-container">
                <ion-card id="login-card-container">
                  <ion-card-content>
                    <ion-grid>
                      <OrganizationCard :device="selectedDevice" />
                      <PasswordInput
                        :label="t('HomePage.organizationLogin.passwordLabel')"
                        @change="onPasswordChange"
                        @enter="login"
                      />
                      <ion-button
                        fill="clear"
                        @click="onForgottenPasswordClick"
                        id="forgotten-password-button"
                      >
                        {{ $t('HomePage.organizationLogin.forgottenPassword') }}
                      </ion-button>
                    </ion-grid>
                  </ion-card-content>
                </ion-card>
                <div id="login-button-container">
                  <ion-button
                    @click="login"
                    size="large"
                    :disabled="password.length == 0"
                    id="login-button"
                  >
                    <ion-icon
                      slot="start"
                      :icon="logIn"
                    />
                    {{ $t("HomePage.organizationLogin.login") }}
                  </ion-button>
                </div>
              </div>
            </ion-card-content>
          </ion-card>
        </transition-group>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonPage,
  IonCard,
  IonCardContent,
  IonCardTitle,
  IonButton,
  IonIcon,
  IonRow,
  IonCol,
  IonGrid,
  modalController
} from '@ionic/vue';
import {
  add,
  link,
  chevronBackOutline,
  logIn
} from 'ionicons/icons'; // We're forced to import icons for the moment, see : https://github.com/ionic-team/ionicons/issues/1032
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import OrganizationCard from '@/components/OrganizationCard.vue';
import PasswordInput from '@/components/PasswordInput.vue';
import { createAlert } from '@/components/AlertConfirmation';
import { libparsec } from '../plugins/libparsec';
import { AvailableDevice } from '../plugins/libparsec/definitions';

export interface DeviceLocalStorageData {
    slug: string;
    lastLogin: Date;
}

const { t, d } = useI18n();

// Not ready yet
// const deviceList = libparsec.listAvailableDevices();
const deviceList: AvailableDevice[] = [
  {
    organizationId: 'MegaShark',
    humanHandle: 'Maxime Grandcolas',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug1',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Resana',
    humanHandle: 'Maxime Grandcolas',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug2',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Oxymore',
    humanHandle: 'Maxime Grandcolas',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug3',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Eddy',
    humanHandle: 'Maxime Grandcolas',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug4',
    ty: {tag: 'Password'}
  }
];
let selectedDevice: AvailableDevice;
const password = ref('');
const showOrganizationList = ref(true);

const deviceLocalStorageDataList = [
  {slug: 'slug1', lastLogin: new Date('01/11/2023')},
  {slug: 'slug2', lastLogin: new Date('01/12/2023 12:03:05')},
  {slug: 'slug3', lastLogin: new Date('01/12/2023 15:12:04')}
];

function getDeviceLocalStorageData(deviceSlug: string): DeviceLocalStorageData {
  return deviceLocalStorageDataList.find((device) => {
    return device.slug === deviceSlug;
  });
}

function onPasswordChange(pwd: string): void {
  password.value = pwd;
}

function onOrganizationCardClick(device: AvailableDevice): void {
  showOrganizationList.value = !showOrganizationList.value;
  selectedDevice = device;
}

function login(): void {
  console.log(`Log in to ${selectedDevice.organizationId} with password "${password.value}"`);
}

function formatLastLogin(lastLogin: Date | undefined) : string {
  if (!lastLogin) {
    return '';
  }
  // Get the difference in ms
  let diff = Date.now().valueOf() - lastLogin.valueOf();

  // To seconds
  diff = Math.ceil(diff / 1000);
  if (diff < 60) {
    return t('HomePage.organizationList.lastLoginSeconds', {seconds: diff}, diff);
  }

  // To minutes
  diff = Math.ceil(diff / 60);
  if (diff < 60) {
    return t('HomePage.organizationList.lastLoginMinutes', {minutes: diff}, diff);
  }

  // To hours
  diff = Math.ceil(diff / 60);
  if (diff < 24) {
    return t('HomePage.organizationList.lastLoginHours', {hours: diff}, diff);
  }

  // Too long, let's use the date as is
  return t('HomePage.organizationList.lastLogin', {date: d(lastLogin, 'long')});
}

function onForgottenPasswordClick(): void {
  console.log('forgotten password!');
}

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
#container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;

  max-width: 50vw;
  margin: 0 auto;

  .logo {
    max-width: 10em;
    align-self: center;
    display: flex;
    align-items: end;
    padding-bottom: 2em;
    flex-basis: 25%;
    flex-grow: 0;
    flex-shrink: 0;
  }

  ion-card {
    flex-grow: 0;
    flex-shrink: 0;
  }

  .slide-left-enter-active, .slide-right-enter-active {
    transition: 0.5s ease-in-out;
    transition-delay: 0.5s;
  }

  .slide-left-leave-active, .slide-right-leave-active {
    transition: 0.5s;
  }

  .slide-left-enter-from, .slide-right-leave-to {
    opacity: 0;
    transform: translate(-100%, 0);
  }

  .slide-left-leave-to, .slide-right-enter-from {
    opacity: 0;
    transform: translate(100%, 0);
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

  #login-container {
    margin-right: 3em;
    margin-left: 3em;

    #login-card-container {
      background: #f9f9fb;
      border-radius: 8px;
      padding: 2em;

      .organization-card {
        margin-bottom: 2em;
      }
    }

    #login-button-container {
      text-align: right;
    }
  }
}
</style>
