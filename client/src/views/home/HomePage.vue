<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div id="page">
        <!-- sidebar -->
        <home-page-sidebar class="homepage-sidebar" />
        <!-- main content -->
        <div class="homepage-scroll">
          <div
            class="homepage-content"
            :class="{ 'login-fullscreen': state === HomePageState.Login }"
          >
            <!-- topbar -->
            <home-page-header
              class="homepage-header"
              @back-click="backToPreviousPage"
              @customer-area-click="goToCustomerAreaLogin"
              @settings-click="goToAccountSettings"
              @create-or-join-organization-click="openCreateOrJoin"
              @change-tab="onChangeTab"
              :display-create-join="deviceList.length > 0"
              :back-button-title="getBackButtonTitle()"
              :show-secondary-menu="state !== HomePageState.AccountSettings"
              :show-back-button="showBackButton"
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
                  @create-or-join-organization-click="openCreateOrJoin"
                  @invitation-click="onInvitationClicked"
                  @pki-request-click="onPkiRequestClicked"
                  :device-list="deviceList"
                  :invitation-list="invitationList"
                  :pki-request-list="pkiRequestList"
                  :querying="querying"
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
                  ref="loginPage"
                />
              </template>
              <template v-else-if="state === HomePageState.ForgottenPassword">
                <import-recovery-device-page
                  :device="selectedDevice"
                  @organization-selected="login"
                />
              </template>
              <template v-else-if="state === HomePageState.AccountSettings">
                <account-settings-page
                  :active-tab="activeTab"
                  @tab-change="onChangeTab"
                />
              </template>
            </slide-horizontal>
          </div>
        </div>
        <!-- end of organization -->
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { getDurationBeforeExpiration, isExpired, isTrialOrganizationDevice } from '@/common/organization';
import {
  bootstrapLinkValidator,
  claimAndBootstrapLinkValidator,
  claimDeviceLinkValidator,
  claimUserLinkValidator,
  pkiLinkValidator,
} from '@/common/validators';
import { SmallDisplayCreateJoinModal } from '@/components/small-display';
import {
  AccessStrategy,
  AccountInvitation,
  AvailableDevice,
  AvailableDeviceTypeTag,
  ClientStartError,
  ClientStartErrorTag,
  DeviceAccessStrategy,
  DeviceAccessStrategyTag,
  JoinRequestStatus,
  ListAvailableDeviceErrorTag,
  LocalJoinRequest,
  ParsecAccount,
  archiveDevice,
  cancelLocalJoinRequest,
  confirmLocalJoinRequest,
  getDeviceHandle,
  getOrganizationCreationDate,
  isDeviceLoggedIn,
  isWeb,
  listAvailableDevices,
  listAvailableDevicesWithError,
  listLocalJoinRequests,
  parseParsecAddr,
  login as parsecLogin,
  requestJoinOrganization,
} from '@/parsec';
import { RouteBackup, Routes, currentRouteIs, getCurrentRouteQuery, navigateTo, switchOrganization, watchRoute } from '@/router';
import { EventData, EventDistributor, Events } from '@/services/eventDistributor';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { ServerType, getServerTypeFromParsedParsecAddr } from '@/services/parsecServers';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import AccountSettingsPage from '@/views/account/AccountSettingsPage.vue';
import { AccountSettingsTabs } from '@/views/account/types';
import ClientAreaLoginPage from '@/views/client-area/ClientAreaLoginPage.vue';
import ImportRecoveryDevicePage from '@/views/devices/ImportRecoveryDevicePage.vue';
import DeviceJoinOrganizationModal from '@/views/home/DeviceJoinOrganizationModal.vue';
import HomePageButtons, { HomePageAction } from '@/views/home/HomePageButtons.vue';
import HomePageHeader from '@/views/home/HomePageHeader.vue';
import HomePageSidebar from '@/views/home/HomePageSidebar.vue';
import LoginPage from '@/views/home/LoginPage.vue';
import OrganizationListPage from '@/views/home/OrganizationListPage.vue';
import UserJoinOrganizationModal from '@/views/home/UserJoinOrganizationModal.vue';
import CreateOrganizationModal from '@/views/organizations/creation/CreateOrganizationModal.vue';
import { IonContent, IonPage, modalController, popoverController } from '@ionic/vue';
import { DateTime } from 'luxon';
import {
  Answer,
  Base64,
  MsModalResult,
  Position,
  SlideHorizontal,
  Validity,
  askQuestion,
  getTextFromUser,
  useWindowSize,
} from 'megashark-lib';
import { Ref, computed, inject, nextTick, onMounted, onUnmounted, ref, toRaw, useTemplateRef, watch } from 'vue';

enum HomePageState {
  OrganizationList = 'organization-list',
  Login = 'login',
  ForgottenPassword = 'forgotten-password',
  CustomerArea = 'customer-area',
  AccountSettings = 'account-settings',
}

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

const accountLoggedIn = ref(ParsecAccount.isLoggedIn());
const state = ref(HomePageState.OrganizationList);
const storedDeviceDataDict = ref<{ [deviceId: string]: StoredDeviceData }>({});
const selectedDevice: Ref<AvailableDevice | undefined> = ref();
const loginPageRef = useTemplateRef<InstanceType<typeof LoginPage>>('loginPage');
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const informationManager: InformationManager = injectionProvider.getDefault().informationManager;
const loginInProgress = ref(false);
const queryInProgress = ref(false);
const querying = ref(true);
const deviceList: Ref<AvailableDevice[]> = ref([]);
const pkiRequestList: Ref<LocalJoinRequest[]> = ref([]);
const invitationList = ref<Array<AccountInvitation>>([]);
const activeTab = ref(AccountSettingsTabs.Settings);
let eventCallbackId!: string;

const slidePositions = ref({ appearFrom: Position.Left, disappearTo: Position.Right });
const showBackButton = computed(() => {
  return [HomePageState.Login, HomePageState.ForgottenPassword, HomePageState.CustomerArea, HomePageState.AccountSettings].includes(
    state.value,
  );
});

let hotkeys: HotkeyGroup | null = null;

async function onChangeTab(tab: AccountSettingsTabs): Promise<void> {
  state.value = HomePageState.AccountSettings;
  activeTab.value = tab;
}

const stateWatchCancel = watch(state, (newState, oldState) => {
  // we use the enum ordering to determine the direction of the slide
  if (oldState > newState) {
    slidePositions.value = { appearFrom: Position.Right, disappearTo: Position.Left };
  } else {
    slidePositions.value = { appearFrom: Position.Left, disappearTo: Position.Right };
  }
});

const routeWatchCancel = watchRoute(async (newRoute, oldRoute) => {
  if (!currentRouteIs(Routes.Home)) {
    return;
  }
  if (newRoute.name !== oldRoute.name) {
    state.value = HomePageState.OrganizationList;
  }
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
  await handleQuery();
  await refreshDeviceList();
});

onMounted(async () => {
  querying.value = true;
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'n', modifiers: Modifiers.Ctrl | Modifiers.Shift, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    openCreateOrganizationModal,
  );
  hotkeys.add(
    { key: 'j', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    onJoinOrganizationClicked,
  );
  eventCallbackId = await injectionProvider
    .getDefault()
    .eventDistributor.registerCallback(
      Events.ClientStarted | Events.ClientStopped | Events.InvitationUpdated,
      async (event: Events, _data?: EventData): Promise<void> => {
        if (event === Events.InvitationUpdated) {
          refreshInvitationList();
        } else {
          refreshDeviceList();
        }
      },
    );

  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();

  await handleQuery();
  await Promise.allSettled([refreshDeviceList(), refreshInvitationList(), refreshPkiRequestsList()]);
});

onUnmounted(() => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  routeWatchCancel();
  stateWatchCancel();
  injectionProvider.getDefault().eventDistributor.removeCallback(eventCallbackId);
});

async function openCreateOrJoin(event: Event): Promise<void> {
  let result!: { role?: string; data?: { action: HomePageAction } };

  if (isLargeDisplay.value) {
    const popover = await popoverController.create({
      component: HomePageButtons,
      cssClass: 'homepage-popover',
      event: event,
      showBackdrop: false,
      alignment: 'end',
    });
    await popover.present();
    result = await popover.onWillDismiss();
    await popover.dismiss();
  } else {
    const modal = await modalController.create({
      component: SmallDisplayCreateJoinModal,
      cssClass: 'create-join-modal',
      showBackdrop: true,
      handle: false,
      breakpoints: isLargeDisplay.value ? undefined : [1],
      expandToScroll: false,
      initialBreakpoint: isLargeDisplay.value ? undefined : 1,
    });
    await modal.present();
    result = await modal.onWillDismiss();
    await modal.dismiss();
  }

  if (result.role !== MsModalResult.Confirm || !result.data) {
    return;
  }

  if (result.data.action === HomePageAction.CreateOrganization) {
    await openCreateOrganizationModal();
  } else if (result.data.action === HomePageAction.JoinOrganization) {
    await onJoinOrganizationClicked();
  }
}

async function refreshInvitationList(): Promise<void> {
  if (!ParsecAccount.isLoggedIn()) {
    return;
  }
  const result = await ParsecAccount.listInvitations();
  if (result.ok) {
    invitationList.value = result.value;
  } else {
    invitationList.value = [];
  }
}

async function refreshDeviceList(): Promise<void> {
  querying.value = true;
  const result = await listAvailableDevicesWithError();
  if (!result.ok) {
    let message = 'HomePage.organizationList.errors.generic';
    if (result.error.tag === ListAvailableDeviceErrorTag.StorageNotAvailable) {
      if (isWeb()) {
        message = 'HomePage.organizationList.errors.noStorageWeb';
      } else {
        message = 'HomePage.organizationList.errors.noStorageDesktop';
      }
    }
    informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
  } else {
    deviceList.value = result.value;
  }
  querying.value = false;
}

async function refreshPkiRequestsList(): Promise<void> {
  queryInProgress.value = true;
  const result = await listLocalJoinRequests();
  if (result.ok) {
    pkiRequestList.value = result.value;
  } else {
    pkiRequestList.value = [];
  }
  queryInProgress.value = false;
}

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
    let device = undefined;
    for (const d of availableDevices) {
      const result = await parseParsecAddr(d.serverAddr);
      if (!result.ok) {
        throw Error(`Invalid \`serverAddr\` field for AvailableDevice: ${d}`);
      }
      const serverType = getServerTypeFromParsedParsecAddr(result.value);
      if (serverType === ServerType.Saas && d.organizationId === query.bmsOrganizationId) {
        device = d;
      }
    }
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
  } else if (query.bmsLogin) {
    state.value = HomePageState.CustomerArea;
    // Should just reset the query in the URL without reloading the page
    await navigateTo(Routes.Home, { skipHandle: true });
  }
  queryInProgress.value = false;
}

async function onInvitationClicked(invitation: AccountInvitation): Promise<void> {
  await openJoinByLinkModal(invitation.addr);
}

async function onJoinOrganizationClicked(): Promise<void> {
  const link = await getTextFromUser(
    {
      title: 'JoinByLinkModal.pageTitle',
      subtitle: 'JoinByLinkModal.pleaseEnterUrl',
      trim: true,
      validator: claimAndBootstrapLinkValidator,
      inputLabel: 'JoinOrganization.linkFormLabel',
      placeholder: 'JoinOrganization.linkFormPlaceholder',
      okButtonText: 'JoinByLinkModal.join',
    },
    isLargeDisplay.value,
  );

  if (link) {
    if ((await bootstrapLinkValidator(link)).validity === Validity.Valid) {
      await openCreateOrganizationModal(link);
    } else if ((await pkiLinkValidator(link)).validity === Validity.Valid) {
      await handleJoinByPki(link);
    } else {
      await openJoinByLinkModal(link);
    }
  }
}

async function handleJoinByPki(link: string): Promise<void> {
  const result = await requestJoinOrganization(link);

  if (result.ok) {
    await refreshPkiRequestsList();
    informationManager.present(
      new Information({
        message: 'HomePage.organizationRequest.requestSent.success',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: 'HomePage.organizationRequest.requestSent.failure',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function onPkiRequestClicked(pkiRequest: LocalJoinRequest): Promise<void> {
  if (pkiRequest.status === JoinRequestStatus.Pending) {
    const answer = await askQuestion('HomePage.organizationRequest.pending.title', 'HomePage.organizationRequest.pending.message', {
      yesText: 'HomePage.organizationRequest.pending.yes',
      noText: 'HomePage.organizationRequest.pending.no',
    });
    if (answer === Answer.Yes) {
      await cancelLocalJoinRequest(pkiRequest);
      informationManager.present(
        new Information({
          message: 'HomePage.organizationRequest.requestCancelled',
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else if (pkiRequest.status === JoinRequestStatus.Rejected) {
    const answer = await askQuestion('HomePage.organizationRequest.rejected.title', 'HomePage.organizationRequest.rejected.message', {
      yesText: 'HomePage.organizationRequest.rejected.yes',
      noText: 'HomePage.organizationRequest.rejected.no',
    });
    if (answer === Answer.Yes) {
      await cancelLocalJoinRequest(pkiRequest);
      informationManager.present(
        new Information({
          message: 'HomePage.organizationRequest.requestDeleted',
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else if (pkiRequest.status === JoinRequestStatus.Accepted) {
    const result = await confirmLocalJoinRequest(pkiRequest);
    if (result.ok) {
      informationManager.present(
        new Information({
          message: 'HomePage.organizationRequest.accepted.joinSuccess',
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
      await refreshDeviceList();
      // TODO: automatically login with the new device
      // await login(result.value, AccessStrategy.useSmartcard(result.value));
    } else {
      informationManager.present(
        new Information({
          message: 'HomePage.organizationRequest.accepted.joinFailure',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
  await refreshPkiRequestsList();
}

async function openCreateOrganizationModal(bootstrapLink?: string, defaultServerChoice?: ServerType): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganizationModal,
    canDismiss: true,
    cssClass: 'create-organization-modal',
    backdropDismiss: false,
    showBackdrop: true,
    expandToScroll: false,
    handle: false,
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
    const creationData = data as { device: AvailableDevice; access: DeviceAccessStrategy };
    if (creationData.access.tag === DeviceAccessStrategyTag.AccountVault && ParsecAccount.isLoggedIn()) {
      await ParsecAccount.createRegistrationDevice(data.access);
    }
    await login(creationData.device, creationData.access);
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
    cssClass: 'join-organization-modal',
    backdropDismiss: false,
    showBackdrop: true,
    breakpoints: isLargeDisplay.value ? undefined : [0.5, 1],
    expandToScroll: false,
    handle: false,
    initialBreakpoint: isLargeDisplay.value ? undefined : 1,
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
  if (await isDeviceLoggedIn(device)) {
    window.electronAPI.log('debug', 'Selected organization is already logged in, switching to it');
    const handle = await getDeviceHandle(device);
    switchOrganization(handle ?? null, false);
  } else {
    if (
      (await isTrialOrganizationDevice(device)) &&
      storedDeviceDataDict.value[device.deviceId]?.orgCreationDate &&
      isExpired(getDurationBeforeExpiration(storedDeviceDataDict.value[device.deviceId].orgCreationDate as DateTime))
    ) {
      window.electronAPI.log('debug', 'Selected expired trial organization');
      const answer = await askQuestion('HomePage.expiredDevice.questionTitle', 'HomePage.expiredDevice.questionMessage', {
        yesIsDangerous: true,
        yesText: 'HomePage.expiredDevice.questionYes',
        noText: 'HomePage.expiredDevice.questionNo',
        backdropDismiss: true,
      });
      if (answer === Answer.Yes) {
        window.electronAPI.log('debug', 'Archiving expired trial organization');
        const result = await archiveDevice(device);
        if (result.ok) {
          informationManager.present(
            new Information({
              message: 'HomePage.expiredDevice.archiveSuccess',
              level: InformationLevel.Success,
            }),
            PresentationMode.Toast,
          );
          await refreshDeviceList();
          return;
        } else {
          window.electronAPI.log('error', 'Could not archive expired trial organization');
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
    if (device.ty.tag === AvailableDeviceTypeTag.Keyring) {
      window.electronAPI.log('debug', 'Logging in with Keyring');
      await login(device, AccessStrategy.useKeyring(device));
    } else if (device.ty.tag === AvailableDeviceTypeTag.Smartcard) {
      window.electronAPI.log('debug', 'Logging in with Smartcard');
      await login(device, AccessStrategy.useSmartcard(device));
    } else if (device.ty.tag === AvailableDeviceTypeTag.AccountVault) {
      try {
        const strategy = await AccessStrategy.useAccountVault(device);
        await login(device, strategy);
      } catch (e: any) {
        window.electronAPI.log('error', `Failed to log in with vault: ${e.toString()}`);
        informationManager.present(
          new Information({
            message: 'HomePage.loginErrors.vaultFailed',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
      }
    } else {
      window.electronAPI.log('debug', 'Logging in with Password');
      selectedDevice.value = device;
      state.value = HomePageState.Login;
    }
  }
}

async function handleLoginError(device: AvailableDevice, error: ClientStartError): Promise<void> {
  if (device.ty.tag === AvailableDeviceTypeTag.Password) {
    window.electronAPI.log('debug', 'Handling Password login error');
    selectedDevice.value = device;
    state.value = HomePageState.Login;
    await nextTick();
    if (loginPageRef.value) {
      loginPageRef.value.setLoginError(error);
    }
  } else if (device.ty.tag === AvailableDeviceTypeTag.Keyring) {
    window.electronAPI.log('debug', 'Handling Keyring login error');
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
    window.electronAPI.log('error', `Unhandled error for device authentication type ${device.ty.tag}`);
  }
}

async function handleRegistration(device: AvailableDevice, access: DeviceAccessStrategy): Promise<void> {
  if (ParsecAccount.isLoggedIn()) {
    // Check if the device is already among the registration devices and has the right server
    const isRegResult = await ParsecAccount.isDeviceRegistered(device);
    if (isRegResult.ok && !isRegResult.value) {
      // Ask the user if they want to create a registration device
      const answer = await askQuestion('loginPage.storeAccountTitle', 'loginPage.storeAccountQuestion', {
        yesText: 'loginPage.storeAccountYes',
      });
      if (answer === Answer.Yes) {
        // Create the registration device
        const createRegResult = await ParsecAccount.createRegistrationDevice(access);
        if (createRegResult.ok) {
          informationManager.present(
            new Information({
              message: 'loginPage.storeSuccess',
              level: InformationLevel.Success,
            }),
            PresentationMode.Toast,
          );
          // On web, if the existing device uses a password, replace it with a vault authentication
          if (isWeb() && device.ty.tag === AvailableDeviceTypeTag.Password) {
            const regResult = await ParsecAccount.registerNewDevice({ organizationId: device.organizationId, userId: device.userId });
            if (!regResult.ok) {
              window.electronAPI.log('error', `Failed to register new device: ${regResult.error.tag} (${regResult.error.error})`);
            }
          }
        } else {
          window.electronAPI.log(
            'error',
            `Failed to create the registration device: ${createRegResult.error.tag} (${createRegResult.error.error})`,
          );
          informationManager.present(
            new Information({
              message: 'loginPage.storeFailed',
              level: InformationLevel.Error,
            }),
            PresentationMode.Toast,
          );
        }
      }
    }
  }
}

async function login(device: AvailableDevice, access: DeviceAccessStrategy): Promise<void> {
  const eventDistributor = new EventDistributor();
  loginInProgress.value = true;
  window.electronAPI.log('debug', 'Starting Parsec login');
  const result = await parsecLogin(device, access);
  if (result.ok) {
    window.electronAPI.log('debug', 'getOrganizationCreationDate');
    const creationDateResult = await getOrganizationCreationDate(result.value);
    storedDeviceDataDict.value[device.deviceId] = {
      lastLogin: DateTime.now(),
      orgCreationDate: creationDateResult.ok ? creationDateResult.value : undefined,
    };
    await storageManager.storeDevicesData(toRaw(storedDeviceDataDict.value));

    if (device.ty.tag !== AvailableDeviceTypeTag.AccountVault) {
      await handleRegistration(device, access);
    }

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
    }
    window.electronAPI.log('debug', 'navigateTo: Routes.Loading');
    await navigateTo(Routes.Loading, { skipHandle: true, replace: true, query: { loginInfo: Base64.fromObject(routeData) } });
    state.value = HomePageState.OrganizationList;
    loginInProgress.value = false;
  } else {
    window.electronAPI.log('error', `Parsec login error: ${result.error}`);
    await handleLoginError(device, result.error);
    loginInProgress.value = false;
  }
}

async function backToPreviousPage(): Promise<void> {
  if (state.value === HomePageState.ForgottenPassword && selectedDevice.value) {
    state.value = HomePageState.Login;
  } else if (
    state.value === HomePageState.Login ||
    state.value === HomePageState.ForgottenPassword ||
    state.value === HomePageState.CustomerArea ||
    state.value === HomePageState.AccountSettings
  ) {
    state.value = HomePageState.OrganizationList;
    selectedDevice.value = undefined;
  }
}

function onForgottenPasswordClicked(device?: AvailableDevice): void {
  selectedDevice.value = device;
  state.value = HomePageState.ForgottenPassword;
}

async function goToCustomerAreaLogin(): Promise<void> {
  state.value = HomePageState.CustomerArea;
}

async function goToAccountSettings(): Promise<void> {
  state.value = HomePageState.AccountSettings;
}

function getBackButtonTitle(): string {
  if (isSmallDisplay.value) {
    return 'HomePage.topbar.back';
  }
  if (state.value === HomePageState.Login) {
    return 'HomePage.topbar.backToList';
  } else if (state.value === HomePageState.ForgottenPassword) {
    return 'HomePage.topbar.backToLogin';
  } else if (state.value === HomePageState.CustomerArea) {
    return 'HomePage.topbar.backToList';
  } else if (state.value === HomePageState.AccountSettings) {
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
  background: var(--parsec-color-light-secondary-background);
  z-index: -10;

  .homepage-content {
    position: relative;
    display: flex;
    flex-direction: column;
    --background: var(--parsec-color-light-secondary-background);

    &::part(scroll) {
      --keyboard-offset: 0;

      // Disabled for now, as it causes issues with the keyboard on small displays
      @include ms.responsive-breakpoint('xs') {
        // --keyboard-offset: 290px;
      }
    }
  }

  // Should be edited later with responsive
  .homepage-header {
    padding: 1.5rem 4rem 0 4rem;

    @include ms.responsive-breakpoint('lg') {
      flex-direction: column-reverse;
      gap: 1rem;
    }

    @include ms.responsive-breakpoint('md') {
      padding: 2rem 3rem 0;
    }

    @include ms.responsive-breakpoint('sm') {
      padding: 2rem 1.5rem 0;
      margin-bottom: 1rem;
    }
  }

  &::before {
    content: '';
    position: absolute;
    height: 100%;
    width: 100%;
    max-width: 500px;
    max-height: 500px;
    bottom: 0;
    right: 0;
    background-image: url('@/assets/images/background/blob-shape.svg');
    background-size: contain;
    background-repeat: no-repeat;
    background-position: top center;
    opacity: 0.1;
    filter: blur(600px);
  }
}
</style>
