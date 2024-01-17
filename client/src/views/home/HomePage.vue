<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div id="page">
        <!-- organization list -->
        <!-- organization -->
        <div
          class="content"
          :class="{ 'login-fullscreen': state === HomePageState.Login }"
        >
          <slide-horizontal
            :appear-from="state === HomePageState.OrganizationList ? Position.Left : Position.Right"
            :disappear-to="state === HomePageState.OrganizationList ? Position.Right : Position.Left"
          >
            <template v-if="state === HomePageState.OrganizationList">
              <organization-list-page
                @create-organization-click="openCreateOrganizationModal"
                @organization-select="onOrganizationSelected"
                @join-organization-click="onJoinOrganizationClicked"
              />
            </template>
            <template v-if="state === HomePageState.ForgottenPassword && selectedDevice">
              <import-recovery-device-page
                :device="selectedDevice"
                @organization-selected="onOrganizationSelected"
              />
            </template>
            <!-- after animation -->
            <template v-if="state === HomePageState.Login && selectedDevice">
              <login-page
                :device="selectedDevice"
                :show-back-button="state === HomePageState.Login || state === HomePageState.ForgottenPassword"
                @login-click="login"
                @forgotten-password-click="onForgottenPasswordClicked"
                @back-click="backToOrganizations"
                @settings-click="openSettingsModal"
              />
            </template>
          </slide-horizontal>
          <!-- topbar -->
          <home-page-header
            class="header"
            @settings-click="openSettingsModal"
            @about-click="openAboutModal"
            @back-click="backToOrganizations"
            :show-back-button="state === HomePageState.Login || state === HomePageState.ForgottenPassword"
          />
        </div>
        <!-- end of organization -->
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { NotificationKey } from '@/common/injectionKeys';
import { Validity, claimDeviceLinkValidator, claimLinkValidator, claimUserLinkValidator } from '@/common/validators';
import { MsModalResult, getTextInputFromUser } from '@/components/core';
import { AvailableDevice, login as parsecLogin } from '@/parsec';
import { NavigationOptions, Routes, getCurrentRouteQuery, navigateTo, watchRoute } from '@/router';
import { Notification, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { translate } from '@/services/translation';
import { Position, SlideHorizontal } from '@/transitions';
import AboutModal from '@/views/about/AboutModal.vue';
import ImportRecoveryDevicePage from '@/views/devices/ImportRecoveryDevicePage.vue';
import CreateOrganizationModal from '@/views/home/CreateOrganizationModal.vue';
import DeviceJoinOrganizationModal from '@/views/home/DeviceJoinOrganizationModal.vue';
import HomePageHeader from '@/views/home/HomePageHeader.vue';
import LoginPage from '@/views/home/LoginPage.vue';
import OrganizationListPage from '@/views/home/OrganizationListPage.vue';
import UserJoinOrganizationModal from '@/views/home/UserJoinOrganizationModal.vue';
import SettingsModal from '@/views/settings/SettingsModal.vue';
import { IonContent, IonPage, modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { Ref, inject, onMounted, onUnmounted, ref, toRaw } from 'vue';

enum HomePageState {
  OrganizationList = 'organization-list',
  Login = 'login',
  ForgottenPassword = 'forgotten-password',
}

const notificationManager: NotificationManager = inject(NotificationKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const state = ref(HomePageState.OrganizationList);
const storedDeviceDataDict = ref<{ [slug: string]: StoredDeviceData }>({});
const selectedDevice: Ref<AvailableDevice | null> = ref(null);

const routeWatchCancel = watchRoute(async () => {
  const query = getCurrentRouteQuery();
  if ('link' in query && query.link) {
    await openJoinByLinkModal(query.link as string);
  }
});

onMounted(async () => {
  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();
});

onUnmounted(() => {
  routeWatchCancel();
});

async function openCreateOrganizationModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganizationModal,
    canDismiss: true,
    cssClass: 'create-organization-modal',
    backdropDismiss: false,
  });
  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role === MsModalResult.Confirm) {
    await login(data.device, data.password);
  }
}

async function openJoinByLinkModal(link: string): Promise<void> {
  let component = null;

  if ((await claimUserLinkValidator(link)).validity === Validity.Valid) {
    component = UserJoinOrganizationModal;
  } else if ((await claimDeviceLinkValidator(link)).validity === Validity.Valid) {
    component = DeviceJoinOrganizationModal;
  }

  if (!component) {
    console.log('Invalid link');
    return;
  }
  const modal = await modalController.create({
    component: component,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'join-organization-modal',
    componentProps: {
      invitationLink: link,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  if (result.role === MsModalResult.Confirm) {
    await login(result.data.device, result.data.password);
  }
}

async function backToOrganizations(): Promise<void> {
  state.value = HomePageState.OrganizationList;
  selectedDevice.value = null;
}

function onOrganizationSelected(device: AvailableDevice): void {
  selectedDevice.value = device;
  state.value = HomePageState.Login;
}

async function login(device: AvailableDevice, password: string): Promise<void> {
  const result = await parsecLogin(device, password);
  if (result.ok) {
    if (!storedDeviceDataDict.value[device.slug]) {
      storedDeviceDataDict.value[device.slug] = {
        lastLogin: DateTime.now(),
      };
    } else {
      storedDeviceDataDict.value[device.slug].lastLogin = DateTime.now();
    }
    await storageManager.storeDevicesData(toRaw(storedDeviceDataDict.value));

    const options: NavigationOptions = { params: { handle: result.value }, replace: true };

    await navigateTo(Routes.Workspaces, options);
    state.value = HomePageState.OrganizationList;
  } else {
    const notification = new Notification({
      title: translate('HomePage.loginNotification.title'),
      message: translate('HomePage.loginNotification.message'),
      level: NotificationLevel.Error,
    });
    notificationManager.showToast(notification);
  }
}

function onForgottenPasswordClicked(device: AvailableDevice): void {
  selectedDevice.value = device;
  state.value = HomePageState.ForgottenPassword;
}

async function openAboutModal(): Promise<void> {
  const modal = await modalController.create({
    component: AboutModal,
    cssClass: 'about-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}

async function openSettingsModal(): Promise<void> {
  const modal = await modalController.create({
    component: SettingsModal,
    cssClass: 'settings-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}

async function onJoinOrganizationClicked(): Promise<void> {
  const link = await getTextInputFromUser({
    title: translate('JoinByLinkModal.pageTitle'),
    subtitle: translate('JoinByLinkModal.pleaseEnterUrl'),
    trim: true,
    validator: claimLinkValidator,
    inputLabel: translate('JoinOrganization.linkFormLabel'),
    placeholder: translate('JoinOrganization.linkFormPlaceholder'),
    okButtonText: translate('JoinByLinkModal.join'),
  });

  if (link) {
    await openJoinByLinkModal(link);
  }
}
</script>

<style lang="scss" scoped>
#page {
  position: relative;
  height: 100vh;
  display: flex;
  overflow: hidden;
  padding: 0 2rem;
  align-items: self-start;
  z-index: -10;
  background: var(--parsec-color-light-gradient);

  &::before {
    content: '';
    position: absolute;
    z-index: -2;
    top: 0;
    right: 0;
    width: 100vw;
    height: 100vh;
    background: url('@/assets/images/background/homepage-rectangle.svg') repeat center;
    background-size: cover;
  }

  .content {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    position: relative;
  }
}

// login view

.login-fullscreen {
  width: 100%;
  margin: auto;
}

// login view

.login-fullscreen {
  width: 100%;
  margin: auto;
}
</style>
