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
          <!-- topbar -->
          <home-page-header
            class="header"
            @settings-click="openSettingsModal"
            @about-click="openAboutModal"
            @back-click="backToOrganizations"
            :show-back-button="state === HomePageState.Login || state === HomePageState.ForgottenPassword"
          />
          <slide-horizontal
            :appear-from="state === HomePageState.OrganizationList ? Position.Left : Position.Right"
            :disappear-to="state === HomePageState.OrganizationList ? Position.Right : Position.Left"
          >
            <template v-if="state === HomePageState.OrganizationList">
              <organization-list-page
                @create-organization-click="openCreateOrganizationModal"
                @organization-select="onOrganizationSelected"
                @join-organization-click="onJoinOrganizationClicked"
                @join-organization-with-link-click="openJoinByLinkModal"
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
                @login-click="login"
                @forgotten-password-click="onForgottenPasswordClicked"
                :login-in-progress="loginInProgress"
                ref="loginPageRef"
              />
            </template>
          </slide-horizontal>
        </div>
        <!-- end of organization -->
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { claimDeviceLinkValidator, claimLinkValidator, claimUserLinkValidator } from '@/common/validators';
import {
  AccessStrategy,
  AvailableDevice,
  ClientStartError,
  DeviceAccessStrategy,
  DeviceFileType,
  getDeviceHandle,
  isDeviceLoggedIn,
  listAvailableDevices,
  login as parsecLogin,
} from '@/parsec';
import { RouteBackup, Routes, currentRouteIs, getCurrentRouteQuery, navigateTo, switchOrganization, watchRoute } from '@/router';
import { EventData, EventDistributor, Events } from '@/services/eventDistributor';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { Position, SlideHorizontal } from '@/transitions';
import AboutModal from '@/views/about/AboutModal.vue';
import ImportRecoveryDevicePage from '@/views/devices/ImportRecoveryDevicePage.vue';
import CreateOrganizationModal from '@/views/home/CreateOrganizationModal.vue';
import DeviceJoinOrganizationModal from '@/views/home/DeviceJoinOrganizationModal.vue';
import HomePageHeader from '@/views/home/HomePageHeader.vue';
import LoginPage from '@/views/home/LoginPage.vue';
import OrganizationListPage from '@/views/home/OrganizationListPage.vue';
import UserJoinOrganizationModal from '@/views/home/UserJoinOrganizationModal.vue';
import { openSettingsModal } from '@/views/settings';
import { IonContent, IonPage, modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { Base64, Validity, MsModalResult, getTextFromUser } from 'megashark-lib';
import { Ref, inject, nextTick, onMounted, onUnmounted, ref, toRaw } from 'vue';

enum HomePageState {
  OrganizationList = 'organization-list',
  Login = 'login',
  ForgottenPassword = 'forgotten-password',
}

const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

const state = ref(HomePageState.OrganizationList);
const storedDeviceDataDict = ref<{ [slug: string]: StoredDeviceData }>({});
const selectedDevice: Ref<AvailableDevice | null> = ref(null);
const loginPageRef = ref();
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const informationManager: InformationManager = injectionProvider.getDefault().informationManager;
const loginInProgress = ref(false);

let hotkeys: HotkeyGroup | null = null;

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIs(Routes.Home)) {
    return;
  }
  await handleQuery();
});

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'n', modifiers: Modifiers.Ctrl | Modifiers.Shift, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    openCreateOrganizationModal,
  );
  hotkeys.add(
    { key: 'j', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    onJoinOrganizationClicked,
  );
  hotkeys.add(
    { key: ',', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    openSettingsModal,
  );
  hotkeys.add(
    { key: 'a', modifiers: Modifiers.Ctrl | Modifiers.Alt, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    openAboutModal,
  );

  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();

  await handleQuery();
});

onUnmounted(() => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  routeWatchCancel();
});

async function handleQuery(): Promise<void> {
  const query = getCurrentRouteQuery();
  if (query.claimLink) {
    await openJoinByLinkModal(query.claimLink);
  } else if (query.deviceId) {
    const availableDevices = await listAvailableDevices();
    const device = availableDevices.find((d) => d.deviceId === query.deviceId);
    if (device) {
      await onOrganizationSelected(device);
    } else {
      console.error('Could not find the corresponding device');
    }
  }
}

async function openCreateOrganizationModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganizationModal,
    canDismiss: true,
    cssClass: 'create-organization-modal',
    backdropDismiss: false,
    componentProps: {
      informationManager: informationManager,
    },
  });
  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role === MsModalResult.Confirm) {
    await login(data.device, data.access);
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
      informationManager: informationManager,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  if (result.role === MsModalResult.Confirm) {
    await login(result.data.device, result.data.access);
  } else {
    await navigateTo(Routes.Home);
  }
}

async function backToOrganizations(): Promise<void> {
  state.value = HomePageState.OrganizationList;
  selectedDevice.value = null;
}

async function onOrganizationSelected(device: AvailableDevice): Promise<void> {
  if (isDeviceLoggedIn(device)) {
    const handle = getDeviceHandle(device);
    switchOrganization(handle, false);
  } else {
    if (device.ty === DeviceFileType.Keyring) {
      await login(device, AccessStrategy.useKeyring(device));
    } else {
      selectedDevice.value = device;
      state.value = HomePageState.Login;
    }
  }
}

async function handleLoginError(device: AvailableDevice, error: ClientStartError): Promise<void> {
  if (device.ty === DeviceFileType.Password) {
    selectedDevice.value = device;
    state.value = HomePageState.Login;
    await nextTick();
    loginPageRef.value.setLoginError(error);
  } else if (device.ty === DeviceFileType.Keyring) {
    informationManager.present(
      new Information({
        message: 'HomePage.keyringFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    console.error('Could not connect to device type', device.ty);
  }
}

async function login(device: AvailableDevice, access: DeviceAccessStrategy): Promise<void> {
  const eventDistributor = new EventDistributor();
  loginInProgress.value = true;
  const result = await parsecLogin(eventDistributor, device, access);
  if (result.ok) {
    if (!storedDeviceDataDict.value[device.slug]) {
      storedDeviceDataDict.value[device.slug] = {
        lastLogin: DateTime.now(),
      };
    } else {
      storedDeviceDataDict.value[device.slug].lastLogin = DateTime.now();
    }
    await storageManager.storeDevicesData(toRaw(storedDeviceDataDict.value));

    const query = getCurrentRouteQuery();
    const routeData: RouteBackup = {
      handle: result.value,
      data: {
        route: Routes.Workspaces,
        params: { handle: result.value },
        query: query.fileLink ? { fileLink: query.fileLink } : {},
      },
    };
    if (!injectionProvider.hasInjections(result.value)) {
      injectionProvider.createNewInjections(result.value, eventDistributor);
      const injections = injectionProvider.getInjections(result.value);
      await associateDefaultEvents(injections.eventDistributor, injections.informationManager);
    }
    await navigateTo(Routes.Loading, { skipHandle: true, replace: true, query: { loginInfo: Base64.fromObject(routeData) } });
    state.value = HomePageState.OrganizationList;
    loginInProgress.value = false;
  } else {
    await handleLoginError(device, result.error);
    loginInProgress.value = false;
  }
}

async function associateDefaultEvents(eventDistributor: EventDistributor, informationManager: InformationManager): Promise<void> {
  let ignoreOnlineEvent = true;

  // Since this is going to be alive the whole time, we don't need to remember the id to clear it later
  await eventDistributor.registerCallback(
    Events.Offline | Events.Online | Events.IncompatibleServer | Events.ExpiredOrganization | Events.ClientRevoked,
    async (event: Events, _data: EventData) => {
      switch (event) {
        case Events.Offline: {
          informationManager.present(
            new Information({
              message: 'notification.serverOffline',
              level: InformationLevel.Warning,
            }),
            PresentationMode.Notification,
          );
          break;
        }
        case Events.Online: {
          if (ignoreOnlineEvent) {
            ignoreOnlineEvent = false;
            return;
          }
          informationManager.present(
            new Information({
              message: 'notification.serverOnline',
              level: InformationLevel.Info,
            }),
            PresentationMode.Notification,
          );
          break;
        }
        case Events.IncompatibleServer: {
          informationManager.present(
            new Information({
              message: 'notification.incompatibleServer',
              level: InformationLevel.Error,
            }),
            PresentationMode.Notification,
          );
          await informationManager.present(
            new Information({
              message: 'globalErrors.incompatibleServer',
              level: InformationLevel.Error,
            }),
            PresentationMode.Modal,
          );
          break;
        }
        case Events.ExpiredOrganization: {
          informationManager.present(
            new Information({
              message: 'notification.expiredOrganization',
              level: InformationLevel.Error,
            }),
            PresentationMode.Notification,
          );
          await informationManager.present(
            new Information({
              message: 'globalErrors.expiredOrganization',
              level: InformationLevel.Error,
            }),
            PresentationMode.Modal,
          );
          break;
        }
        case Events.ClientRevoked: {
          informationManager.present(
            new Information({
              message: 'notification.clientRevoked',
              level: InformationLevel.Error,
            }),
            PresentationMode.Notification,
          );
          await informationManager.present(
            new Information({
              message: 'globalErrors.clientRevoked',
              level: InformationLevel.Error,
            }),
            PresentationMode.Modal,
          );
          break;
        }
      }
    },
  );
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
  await modal.dismiss();
}

async function onJoinOrganizationClicked(): Promise<void> {
  const link = await getTextFromUser({
    title: 'JoinByLinkModal.pageTitle',
    subtitle: 'JoinByLinkModal.pleaseEnterUrl',
    trim: true,
    validator: claimLinkValidator,
    inputLabel: 'JoinOrganization.linkFormLabel',
    placeholder: 'JoinOrganization.linkFormPlaceholder',
    okButtonText: 'JoinByLinkModal.join',
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
