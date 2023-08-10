<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      color="secondary"
    >
      <div id="page">
        <!-- sidebar -->
        <div class="sidebar">
          <div class="sidebar-content">
            <div class="sidebar-content__titles">
              <h1 class="title-h1-lg">
                {{ $t('HomePage.organizationList.title') }}
              </h1>
              <ion-text class="subtitles-normal">
                {{ $t('HomePage.organizationList.subtitle') }}
              </ion-text>
            </div>
            <div class="sidebar-content__logo">
              <img
                src="@/assets/images/Logo/logo_row_white.svg"
                alt="Parsec logo"
                class="logo-img"
              >
            </div>
            <div
              class="sidebar-content__version"
              @click="openAboutModal"
            >
              {{ getAppVersion() }}
            </div>
          </div>
        </div>

        <!-- organization list -->
        <!-- organization -->
        <div class="right-side">
          <!-- topbar -->
          <ion-card-content class="topbar">
            <ion-card-title
              color="tertiary"
              v-if="!showOrganizationList"
            >
              <ion-button
                fill="clear"
                @click="showOrganizationList = !showOrganizationList"
                id="back-to-list-button"
              >
                <ion-icon
                  slot="start"
                  :icon="chevronBack"
                />
                {{ $t('HomePage.organizationLogin.backToList') }}
              </ion-button>
            </ion-card-title>
            <ms-search-input
              :label="t('HomePage.organizationList.search')"
              v-if="showOrganizationList"
              @change="onSearchChange($event)"
              id="ms-search-input"
            />
            <ion-button
              @click="isPopoverOpen = !isPopoverOpen; openPopover($event)"
              size="large"
              id="create-organization-button"
              class="button-default"
            >
              {{ $t('HomePage.noExistingOrganization.createOrJoin') }}
            </ion-button>
            <ion-buttons
              slot="primary"
              class="topbar-icon__settings"
            >
              <ion-button
                slot="icon-only"
                id="trigger-settings-button"
                class="topbar-button__item"
                @click="openSettingsModal()"
              >
                <ion-icon
                  slot="icon-only"
                  :icon="cog"
                />
              </ion-button>
            </ion-buttons>
          </ion-card-content>
          <slide-horizontal
            :reverse-direction="!showOrganizationList"
          >
            <ion-card
              v-if="showOrganizationList"
              class="right-side-container"
            >
              <ion-card-content class="organization-container">
                <ion-card-title class="organization-filter">
                  <!-- No use in showing the sort/filter options for less than 2 devices -->
                  <template v-if="deviceList.length > 2">
                    <ms-select
                      id="organization-filter-select"
                      label="t('HomePage.organizationList.labelSortBy')"
                      :options="msSelectOptions"
                      default-option="organization"
                      :sort-by-labels="msSelectSortByLabels"
                      @change="onMsSelectChange($event)"
                    />
                  </template>
                </ion-card-title>
                <ion-grid class="organization-list">
                  <ion-row
                    class="organization-list-row"
                  >
                    <ion-col
                      v-for="device in filteredDevices"
                      :key="device.slug"
                      class="organization-list-row__col"
                    >
                      <ion-card
                        button
                        class="organization-card"
                        @click="onOrganizationCardClick(device)"
                      >
                        <ion-card-content class="card-content">
                          <ion-grid>
                            <organization-card
                              :device="device"
                              class="card-content__body"
                            />
                            <ion-row class="card-content__footer">
                              <ion-col size="auto">
                                <p>{{ $t('HomePage.organizationList.lastLogin') }}</p>
                                <p>
                                  {{ device.slug in storedDeviceDataDict ?
                                    timeSince(storedDeviceDataDict[device.slug].lastLogin, '--') : '--' }}
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
            </ion-card>
            <!-- after animation -->
            <ion-card
              v-if="!showOrganizationList"
              class="login-popup"
            >
              <ion-card-content class="organization-container">
                <ion-text class="title-h1">
                  {{ $t('HomePage.organizationLogin.login') }}
                </ion-text>
                <!-- login -->
                <div id="login-container">
                  <ion-card class="login-card">
                    <ion-card-content class="login-card__content">
                      <ion-grid>
                        <organization-card :device="selectedDevice" />
                        <ms-password-input
                          :label="$t('HomePage.organizationLogin.passwordLabel')"
                          @change="onPasswordChange($event)"
                          @enter="onLoginClick()"
                          id="ms-password-input"
                        />
                        <ion-button
                          fill="clear"
                          @click="onForgottenPasswordClick()"
                          id="forgotten-password-button"
                        >
                          {{ $t('HomePage.organizationLogin.forgottenPassword') }}
                        </ion-button>
                      </ion-grid>
                    </ion-card-content>
                  </ion-card>
                  <div class="login-button-container">
                    <ion-button
                      @click="onLoginClick()"
                      size="large"
                      :disabled="password.length == 0"
                      class="login-button"
                    >
                      <ion-icon
                        slot="start"
                        :icon="logIn"
                      />
                      {{ $t("HomePage.organizationLogin.login") }}
                    </ion-button>
                  </div>
                </div>
                <!-- end of login -->
              </ion-card-content>
            </ion-card>
          </slide-horizontal>
        </div>
        <!-- end of organization -->
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
  IonButtons,
  IonIcon,
  IonRow,
  IonCol,
  IonGrid,
  IonText,
  popoverController,
  modalController,
} from '@ionic/vue';
import {
  chevronBack,
  cog,
  logIn,
} from 'ionicons/icons'; // We're forced to import icons for the moment, see : https://github.com/ionic-team/ionicons/issues/1032
import { useI18n } from 'vue-i18n';
import { onMounted, ref, toRaw, computed, inject, Ref } from 'vue';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import MsPasswordInput from '@/components/core/ms-input/MsPasswordInput.vue';
import MsSearchInput from '@/components/core/ms-input/MsSearchInput.vue';
import MsSelect from '@/components/core/ms-select/MsSelect.vue';
import { MsSelectChangeEvent, MsSelectOption } from '@/components/core/ms-select/MsSelectOption';
import { AvailableDevice } from '@/plugins/libparsec/definitions';
import { libparsec } from '@/plugins/libparsec';
import SlideHorizontal from '@/transitions/SlideHorizontal.vue';
import { mockLastLogin } from '@/common/mocks';
import { StoredDeviceData, StorageManager } from '@/services/storageManager';
import { DateTime } from 'luxon';
import { useRouter } from 'vue-router';
import HomePagePopover from '@/views/home/HomePagePopover.vue';
import SettingsModal from '@/views/settings/SettingsModal.vue';
import { ConfigPathKey, Formatters, FormattersKey, StorageManagerKey } from '@/common/injectionKeys';
import { ModalResultCode } from '@/common/constants';
import { getAppVersion } from '@/common/mocks';
import AboutModal from '@/views/about/AboutModal.vue';

const router = useRouter();
const { t } = useI18n();
const deviceList: Ref<AvailableDevice[]> = ref([]);
let selectedDevice: AvailableDevice;
const password = ref('');
const orgSearchString = ref('');
const showOrganizationList = ref(true);
const sortBy = ref('organization');
const sortByAsc = ref(true);
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince } = inject(FormattersKey)! as Formatters;
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const storageManager: StorageManager = inject(StorageManagerKey)!;
const configPath = inject(ConfigPathKey, '/');  // Must be a valid Unix path !
const isPopoverOpen = ref(false);

const msSelectOptions: MsSelectOption[] = [
  { label: t('HomePage.organizationList.sortByOrganization'), key: 'organization' },
  { label: t('HomePage.organizationList.sortByUserName'), key: 'user_name' },
  { label: t('HomePage.organizationList.sortByLastLogin'), key: 'last_login' },
];

const msSelectSortByLabels = {
  asc: t('HomePage.organizationList.sortOrderAsc'),
  desc: t('HomePage.organizationList.sortOrderDesc'),
};

const filteredDevices = computed(() => {
  return deviceList.value.filter((item) => {
    const lowerSearchString = orgSearchString.value.toLocaleLowerCase();
    return ((item.humanHandle || item.deviceId)?.toLocaleLowerCase().includes(lowerSearchString) ||
      item.organizationId?.toLocaleLowerCase().includes(lowerSearchString));
  }).sort((a, b) => {
    if (sortBy.value === 'organization') {
      if (sortByAsc.value) {
        return a.organizationId.localeCompare(b.organizationId);
      } else {
        return b.organizationId.localeCompare(a.organizationId);
      }
    } else if (sortBy.value === 'user_name' && (a.humanHandle || a.deviceId) && (b.humanHandle || b.deviceId)) {
      if (sortByAsc.value) {
        return (a.humanHandle || a.deviceId)?.localeCompare((b.humanHandle || b.deviceId) ?? '');
      } else {
        return (b.humanHandle || b.deviceId)?.localeCompare((a.humanHandle || a.deviceId) ?? '');
      }
    } else if (sortBy.value === 'last_login') {
      const aLastLogin = (a.slug in storedDeviceDataDict.value && storedDeviceDataDict.value[a.slug].lastLogin !== undefined) ?
        storedDeviceDataDict.value[a.slug].lastLogin : DateTime.fromMillis(0);
      const bLastLogin = (b.slug in storedDeviceDataDict.value && storedDeviceDataDict.value[b.slug].lastLogin !== undefined) ?
        storedDeviceDataDict.value[b.slug].lastLogin : DateTime.fromMillis(0);
      if (sortByAsc.value) {
        return bLastLogin.diff(aLastLogin).toObject().milliseconds!;
      } else {
        return aLastLogin.diff(bLastLogin).toObject().milliseconds!;
      }
    }
    return 0;
  });
});

const storedDeviceDataDict = ref<{ [slug: string]: StoredDeviceData }>({});

onMounted(async (): Promise<void> => {
  await mockLastLogin(storageManager);

  deviceList.value = await libparsec.listAvailableDevices(configPath);

  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();
});

function onPasswordChange(pwd: string): void {
  password.value = pwd;
}

function onSearchChange(search: string): void {
  orgSearchString.value = search;
}

function onMsSelectChange(event: MsSelectChangeEvent): void {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;
}

function onOrganizationCardClick(device: AvailableDevice): void {
  showOrganizationList.value = !showOrganizationList.value;
  selectedDevice = device;
}

async function onLoginClick(): Promise<void> {
  await login(selectedDevice, password.value);
}

async function login(device: AvailableDevice, password: string): Promise<void> {
  if (!storedDeviceDataDict.value[device.slug]) {
    storedDeviceDataDict.value[device.slug] = {
      lastLogin: DateTime.now(),
    };
  } else {
    storedDeviceDataDict.value[device.slug].lastLogin = DateTime.now();
  }
  console.log(`Log in to ${device.organizationId} with password "${password}"`);
  await storageManager.storeDevicesData(toRaw(storedDeviceDataDict.value));

  showOrganizationList.value = true;

  // name: define where the user will be move, query: add parameters
  router.push({ name: 'workspaces', params: {deviceId: device.deviceId} });
}

function onForgottenPasswordClick(): void {
  console.log('forgotten password!');
}

async function openAboutModal(): Promise<void> {
  const modal = await modalController.create({
    component: AboutModal,
    cssClass: 'about-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}

async function openPopover(ev: Event): Promise<void> {
  const popover = await popoverController.create({
    component: HomePagePopover,
    cssClass: 'homepage-popover',
    event: ev,
    showBackdrop: false,
  });
  await popover.present();
  const result = await popover.onWillDismiss();
  if (result.role === ModalResultCode.Confirm) {
    login(result.data.device, result.data.password);
  }
}

async function openSettingsModal(): Promise<void> {
  const modal = await modalController.create({
    component: SettingsModal,
    cssClass: 'settings-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}
</script>

<style lang="scss" scoped>
#page {
  height: 100vh;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  display: flex;
  flex-direction: row;
  overflow: hidden;
  margin: 0 auto;
  align-items: center;
  position: relative;
  z-index: -4;
}
.sidebar {
  display: flex;
  height: 100vh;
  width: 40vw;
  padding: 2rem 0;
  background: var(--parsec-color-light-gradient);
  position: relative;
  justify-content: flex-end;
  z-index: -3;
  &::before {
    content: '';
    position: absolute;
    z-index: -2;
    top: 0;
    right: 0;
    width: 100vw;
    height: 100vh;
    background: url('@/assets/images/shapes-bg.svg') repeat center;
    background-size: cover;
  }
  .sidebar-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-left: 2rem;
    &__titles {
      display: flex;
      flex-direction: column;
      justify-content: center;
      flex-grow: 2;
      margin-right: 2rem;
      max-width: var(--parsec-max-title-width);
      position: relative;
      gap: 1rem;
    }
    &__logo {
      display: flex;
      width: 100%;
      .logo-img {
        max-height: 3em;
        width: 25%;
        height: 100%;
      }
    }
  }
}
.right-side {
  height: 100vh;
  width: 60vw;
  max-width: var(--parsec-max-content-width);
  background: var(--parsec-color-light-secondary-inversed-contrast);
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: -5;
  .right-side-container {
    margin-inline: 0px;
    margin-top: 0px;
    margin-bottom: 0px;
    border-radius: 0;
    box-shadow: none;
    flex-grow: 0;
    flex-shrink: 0;
  }
}
.topbar {
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  padding: 3.5rem 3.5rem 1.5rem;
  display: flex;
  align-items: center;
  #create-organization-button {
    margin-left: auto;
    margin-right: 1.5rem;
  }
  .topbar-button__item, .sc-ion-buttons-md-s .button {
    border: 1px solid var(--parsec-color-light-secondary-light);
    color: var(--parsec-color-light-primary-700);
    border-radius: 50%;
    --padding-top: 0;
    --padding-end: 0;
    --padding-bottom: 0;
    --padding-start: 0;
    width: 3em;
    height: 3em;
    &:hover {
      --background-hover: var(--parsec-color-light-primary-50);
      background: var(--parsec-color-light-primary-50);
      border: var(--parsec-color-light-primary-50);
    }
    ion-icon {
      font-size: 1.375rem;
    }
  }
}
.organization-container {
  padding: 1.5rem 3.5rem 0;
  .organization-filter {
    display: flex;
    margin: 0 0 .5rem;
    justify-content: flex-end;
  }
  .organization-list {
    max-height: 30em;
    overflow-y: auto;
    --ion-grid-columns: 3;
  }
  .organization-list-row {
    gap: 1rem;
    &__col {
      display: flex;
      align-items: center;
      padding: 0;
    }
  }
  .organization-card {
    background: var(--parsec-color-light-secondary-background);
    user-select: none;
    transition: box-shadow 150ms linear;
    box-shadow: none;
    border-radius: 0.5em;
    margin-inline: 0;
    margin-top: 0;
    margin-bottom: 0;
    &:hover {
      box-shadow: 0px 4px 12px rgba(0, 0, 0, 0.08);
    }
    .card-content {
      padding-top: 0px;
      padding-bottom: 0px;
      padding-inline-end: 0px;
      padding-inline-start: 0px;
      &__footer {
        padding: 0.5em 1em;
        background: var(--parsec-color-light-secondary-medium);
        border-top: 1px solid var(--parsec-color-light-secondary-disabled);
        color: var(--parsec-color-light-secondary-grey);
        height: 4.6em;
      }
      &:hover {
        background: var(--parsec-color-light-primary-50);
        cursor: pointer;
        .card-content__footer {
          background: var(--parsec-color-light-primary-50);
          border-top: 1px solid var(--parsec-color-light-primary-100);
        }
      }
    }
  }
}
.login-popup {
  box-shadow: none;
  display: flex;
  margin: auto 0;
  align-items: center;
  justify-content: center;
  flex-grow: 1;
  .organization-container {
    max-width: 62.5rem;
    padding: 0 3.5rem 3.5rem;
    flex-grow: 1;
  }
  .title-h1 {
    color: var(--parsec-color-light-primary-700);
  }
  #login-container {
    margin-top: 2.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  .login-card {
      background: var(--parsec-color-light-secondary-background);
      border-radius: 8px;
      padding: 2em;
      box-shadow: none;
      margin: 0;
      &__content {
        padding: 0;
        #ms-password-input {
          margin: 1.5rem 0 1rem;
        }
      }
      .organization-card {
        margin-bottom: 2em;
        display: flex;
        &__body {
          padding: 0;
        }
      }
    }
  .login-button-container {
    text-align: right;
    .login-button {
      margin: 0;
    }
  }
}
</style>
