<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div id="page">
        <!-- sidebar -->
        <home-page-sidebar />
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
            @back-click="backToPreviousPage"
            :back-button-title="getBackButtonTitle()"
            @customer-area-click="goToCustomerAreaLogin"
            :show-back-button="
              state === HomePageState.Login || state === HomePageState.ForgottenPassword || state === HomePageState.CustomerArea
            "
          />
          <slide-horizontal
            :appear-from="slidePositions.appearFrom"
            :disappear-to="slidePositions.disappearTo"
          >
            <template v-if="state === HomePageState.OrganizationList">
              <organization-list-page
                @create-organization-click="openCreateOrganizationModal"
                @organization-select="onOrganizationSelected"
                @join-organization-click="onJoinOrganizationClicked"
                @join-organization-with-link-click="openJoinByLinkModal"
                @bootstrap-organization-with-link-click="openCreateOrganizationModal"
                @recover-click="onForgottenPasswordClicked"
                ref="organizationListRef"
              />
            </template>
            <template v-else-if="state === HomePageState.CustomerArea">
              <client-area-login-page />
            </template>
            <template v-else-if="state === HomePageState.Login">
              <login-page
                v-if="selectedDevice"
                :device="selectedDevice"
                @login-click="login"
                @forgotten-password-click="onForgottenPasswordClicked"
                :login-in-progress="loginInProgress"
                ref="loginPageRef"
              />
            </template>
            <template v-else-if="state === HomePageState.ForgottenPassword">
              <import-recovery-device-page
                :device="selectedDevice"
                @organization-selected="onOrganizationSelected"
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
import {
  bootstrapLinkValidator,
  claimAndBootstrapLinkValidator,
  claimDeviceLinkValidator,
  claimUserLinkValidator,
} from '@/common/validators';
import {
  AccessStrategy,
  archiveDevice,
  AvailableDevice,
  ClientStartError,
  ClientStartErrorTag,
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
import AboutModal from '@/views/about/AboutModal.vue';
import ImportRecoveryDevicePage from '@/views/devices/ImportRecoveryDevicePage.vue';
import CreateOrganizationModal from '@/views/organizations/creation/CreateOrganizationModal.vue';
import DeviceJoinOrganizationModal from '@/views/home/DeviceJoinOrganizationModal.vue';
import HomePageHeader from '@/views/home/HomePageHeader.vue';
import HomePageSidebar from '@/views/home/HomePageSidebar.vue';
import LoginPage from '@/views/home/LoginPage.vue';
import OrganizationListPage from '@/views/home/OrganizationListPage.vue';
import UserJoinOrganizationModal from '@/views/home/UserJoinOrganizationModal.vue';
import { openSettingsModal } from '@/views/settings';
import { IonContent, IonPage, modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { Base64, Validity, MsModalResult, Position, SlideHorizontal, getTextFromUser, askQuestion, Answer } from 'megashark-lib';
import { Ref, inject, nextTick, onMounted, onUnmounted, ref, toRaw, watch } from 'vue';
import ClientAreaLoginPage from '@/views/client-area/ClientAreaLoginPage.vue';
import { getServerTypeFromAddress, ServerType } from '@/services/parsecServers';
import { getDurationBeforeExpiration, isExpired, isTrialOrganizationDevice } from '@/common/organization';

enum HomePageState {
  OrganizationList = 'organization-list',
  Login = 'login',
  ForgottenPassword = 'forgotten-password',
  CustomerArea = 'customer-area',
}

const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

const state = ref(HomePageState.OrganizationList);
const storedDeviceDataDict = ref<{ [deviceId: string]: StoredDeviceData }>({});
const selectedDevice: Ref<AvailableDevice | undefined> = ref();
const loginPageRef = ref();
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const informationManager: InformationManager = injectionProvider.getDefault().informationManager;
const loginInProgress = ref(false);
const queryInProgress = ref(false);
const organizationListRef = ref();

const slidePositions = ref({ appearFrom: Position.Left, disappearTo: Position.Right });

let hotkeys: HotkeyGroup | null = null;

const stateWatchCancel = watch(state, (newState, oldState) => {
  // we use the enum ordering to determine the direction of the slide
  if (oldState > newState) {
    slidePositions.value = { appearFrom: Position.Right, disappearTo: Position.Left };
  } else {
    slidePositions.value = { appearFrom: Position.Left, disappearTo: Position.Right };
  }
});

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
  stateWatchCancel();
});

async function handleQuery(): Promise<void> {
  if (queryInProgress.value === true) {
    return;
  }
  queryInProgress.value = true;
  const query = getCurrentRouteQuery();
  if (query.claimLink) {
    await openJoinByLinkModal(query.claimLink);
  } else if (query.bootstrapLink) {
    await openCreateOrganizationModal(query.bootstrapLink);
  } else if (query.deviceId) {
    const availableDevices = await listAvailableDevices();
    const device = availableDevices.find((d) => d.deviceId === query.deviceId);
    if (device) {
      await onOrganizationSelected(device);
    } else {
      console.error('Could not find the corresponding device');
    }
  } else if (query.bmsOrganizationId) {
    const availableDevices = await listAvailableDevices();
    const device = availableDevices.find((d) => {
      const serverType = getServerTypeFromAddress(d.serverUrl);
      return serverType === ServerType.Saas && d.organizationId === query.bmsOrganizationId;
    });
    if (device) {
      await onOrganizationSelected(device);
    } else {
      informationManager.present(
        new Information({
          message: {
            key: 'HomePage.bmsOrganizationNotFound',
            data: { organization: query.bmsOrganizationId },
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } else if (query.createOrg) {
    openCreateOrganizationModal(undefined, query.createOrg);
  }
  queryInProgress.value = false;
}

async function openCreateOrganizationModal(bootstrapLink?: string, defaultServerChoice?: ServerType): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganizationModal,
    canDismiss: true,
    cssClass: 'create-organization-modal',
    backdropDismiss: false,
    componentProps: {
      informationManager: informationManager,
      bootstrapLink: bootstrapLink,
      defaultChoice: defaultServerChoice,
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
    window.electronAPI.log('error', 'Trying to open join link modal with invalid link');
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

async function onOrganizationSelected(device: AvailableDevice): Promise<void> {
  if (isDeviceLoggedIn(device)) {
    const handle = getDeviceHandle(device);
    switchOrganization(handle, false);
  } else {
    if (isTrialOrganizationDevice(device) && isExpired(getDurationBeforeExpiration(device.createdOn))) {
      const answer = await askQuestion('HomePage.expiredDevice.questionTitle', 'HomePage.expiredDevice.questionMessage', {
        yesIsDangerous: true,
        yesText: 'HomePage.expiredDevice.questionYes',
        noText: 'HomePage.expiredDevice.questionNo',
        backdropDismiss: false,
      });
      if (answer === Answer.Yes) {
        const result = await archiveDevice(device);
        if (result.ok) {
          informationManager.present(
            new Information({
              message: 'HomePage.expiredDevice.archiveSuccess',
              level: InformationLevel.Success,
            }),
            PresentationMode.Toast,
          );
          await organizationListRef.value.refreshDeviceList();
          return;
        } else {
          informationManager.present(
            new Information({
              message: 'HomePage.expiredDevice.archiveFailure',
              level: InformationLevel.Error,
            }),
            PresentationMode.Toast,
          );
        }
      }
    }
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
    if (error.tag === ClientStartErrorTag.LoadDeviceDecryptionFailed) {
      const answer = await askQuestion('HomePage.loginErrors.keyringFailedTitle', 'HomePage.loginErrors.keyringFailedQuestion', {
        yesIsDangerous: false,
        yesText: 'HomePage.loginErrors.keyringFailedUsedRecovery',
        noText: 'HomePage.loginErrors.keyringFailedAbort',
      });
      if (answer === Answer.Yes) {
        selectedDevice.value = device;
        state.value = HomePageState.ForgottenPassword;
      }
    } else {
      informationManager.present(
        new Information({
          message: 'HomePage.loginErrors.keyringFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    window.electronAPI.log('error', `Unhandled device authentication type ${device.ty}`);
  }
}

async function login(device: AvailableDevice, access: DeviceAccessStrategy): Promise<void> {
  const eventDistributor = new EventDistributor();
  loginInProgress.value = true;
  const result = await parsecLogin(eventDistributor, device, access);
  if (result.ok) {
    if (!storedDeviceDataDict.value[device.deviceId]) {
      storedDeviceDataDict.value[device.deviceId] = {
        lastLogin: DateTime.now(),
      };
    } else {
      storedDeviceDataDict.value[device.deviceId].lastLogin = DateTime.now();
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
    async (event: Events, _data?: EventData) => {
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

async function backToPreviousPage(): Promise<void> {
  if (state.value === HomePageState.ForgottenPassword && selectedDevice.value) {
    state.value = HomePageState.Login;
  } else if (
    state.value === HomePageState.Login ||
    state.value === HomePageState.CustomerArea ||
    state.value === HomePageState.ForgottenPassword
  ) {
    state.value = HomePageState.OrganizationList;
    selectedDevice.value = undefined;
  }
}

function onForgottenPasswordClicked(device?: AvailableDevice): void {
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

async function goToCustomerAreaLogin(): Promise<void> {
  state.value = HomePageState.CustomerArea;
}

async function onJoinOrganizationClicked(): Promise<void> {
  const link = await getTextFromUser({
    title: 'JoinByLinkModal.pageTitle',
    subtitle: 'JoinByLinkModal.pleaseEnterUrl',
    trim: true,
    validator: claimAndBootstrapLinkValidator,
    inputLabel: 'JoinOrganization.linkFormLabel',
    placeholder: 'JoinOrganization.linkFormPlaceholder',
    okButtonText: 'JoinByLinkModal.join',
  });

  if (link) {
    if ((await bootstrapLinkValidator(link)).validity === Validity.Valid) {
      await openCreateOrganizationModal(link);
    } else {
      await openJoinByLinkModal(link);
    }
  }
}

function getBackButtonTitle(): string {
  if (state.value === HomePageState.Login) {
    return 'HomePage.topbar.backToList';
  } else if (state.value === HomePageState.ForgottenPassword) {
    return 'HomePage.topbar.backToLogin';
  } else if (state.value === HomePageState.CustomerArea) {
    return 'HomePage.topbar.backToList';
  }
  return '';
}
</script>

<style lang="scss" scoped>
#page {
  position: relative;
  height: 100vh;
  display: flex;
  overflow: hidden;
  align-items: self-start;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  z-index: -10;

  .content {
    width: 100%;
    height: 100%;
    display: flex;
    gap: 2rem;
    flex-direction: column;
    position: relative;
    max-width: var(--parsec-max-content-width);
    padding: 6.26rem 5rem 0;

    &::after {
      content: '';
      position: absolute;
      height: 100%;
      width: 100%;
      max-width: 317px;
      max-height: 326px;
      bottom: 0;
      right: 0;
      background-image: url('@/assets/images/background/blob-shape.svg');
      background-size: contain;
      background-repeat: no-repeat;
      background-position: top center;
      opacity: 0.5;
      filter: blur(250px);
    }
  }
}
</style>
