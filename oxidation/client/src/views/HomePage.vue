<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->
<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      color="secondary"
    >
      <div id="page">
        <div id="topbar">
          <ion-grid>
            <ion-row>
              <ion-col
                size="8"
                offset="2"
              >
                <img src="../assets/images/Logo/logo_inline_blue2.png">
              </ion-col>
              <ion-col size="2">
                <ion-button
                  fill="clear"
                  @click="$router.push('settings')"
                  id="settings-button"
                >
                  <ion-icon
                    slot="start"
                    :icon="cogOutline"
                  />
                  {{ $t('HomePage.topbar.settings') }}
                </ion-button>
              </ion-col>
            </ion-row>
          </ion-grid>
        </div>
        <div id="container">
          <transition-group
            :name="showOrganizationList ? 'slide-left' : 'slide-right'"
          >
            <ion-card
              v-if="showOrganizationList"
              id="organization-list-container"
            >
              <ion-card-content class="organization-list">
                <ion-card-title color="tertiary">
                  {{ $t('HomePage.organizationList.title') }}

                  <!-- No use in showing the sort/filter options for less than 2 devices -->
                  <template v-if="deviceList.length > 2">
                    <ion-grid>
                      <ion-row class="ion-justify-content-between">
                        <ion-col size="1">
                          <ion-item fill="solid">
                            <ion-label
                              position="floating"
                            >
                              {{ $t('HomePage.organizationList.search') }}
                            </ion-label>
                            <ion-icon
                              :icon="searchOutline"
                              slot="start"
                            />
                            <ion-input
                              v-model="orgSearchString"
                              :clear-input="true"
                            />
                          </ion-item>
                        </ion-col>
                        <ion-col size="auto">
                          <MsSelect
                            id="filter-select"
                            label="t('HomePage.organizationList.labelSortBy')"
                            :options="msSelectOptions"
                            default-option="organization"
                            :sort-by-labels="msSelectSortByLabels"
                            @change="onMsSelectChange($event)"
                          />
                        </ion-col>
                      </ion-row>
                    </ion-grid>
                  </template>
                </ion-card-title>
                <ion-grid class="organization-list-grid">
                  <ion-row>
                    <ion-col
                      size="1"
                      v-for="device in filteredDevices"
                      :key="device.slug"
                    >
                      <ion-card
                        button
                        class="organization-card-container"
                        @click="onOrganizationCardClick(device)"
                      >
                        <ion-card-content>
                          <ion-grid>
                            <OrganizationCard
                              :device="device"
                              class="organization-card"
                            />
                            <ion-row class="organization-card-footer">
                              <ion-col
                                size="auto"
                                v-if="deviceStoredDataDict[device.slug]"
                              >
                                <p>{{ $t('HomePage.organizationList.lastLogin') }}</p>
                                <p>{{ formatLastLogin(deviceStoredDataDict[device.slug].lastLogin) }}</p>
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
                          @change="onPasswordChange($event)"
                          @enter="login()"
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
                  <div id="login-button-container">
                    <ion-button
                      @click="login()"
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
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonInput,
  IonPage,
  IonCard,
  IonCardContent,
  IonCardTitle,
  IonButton,
  IonIcon,
  IonItem,
  IonLabel,
  IonRow,
  IonCol,
  IonGrid,
  modalController
} from '@ionic/vue';
import {
  add,
  link,
  chevronBackOutline,
  searchOutline,
  cogOutline,
  logIn
} from 'ionicons/icons'; // We're forced to import icons for the moment, see : https://github.com/ionic-team/ionicons/issues/1032
import { useI18n } from 'vue-i18n';
import { onMounted, ref, toRaw, computed } from 'vue';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import OrganizationCard from '@/components/OrganizationCard.vue';
import PasswordInput from '@/components/PasswordInput.vue';
import MsSelect from '@/components/MsSelect.vue';
import { MsSelectChangeEvent, MsSelectOption } from '@/components/MsSelectOption';
import { createAlert } from '@/components/AlertConfirmation';
import { AvailableDevice } from '../plugins/libparsec/definitions';
import { Storage } from '@ionic/storage';

export interface DeviceStoredData {
    lastLogin: Date;
}

const { t, d } = useI18n();
const deviceList: AvailableDevice[] = [
  {
    organizationId: 'Planet Express',
    humanHandle: 'Dr. John A. Zoidberg',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug1',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'PPTH',
    humanHandle: 'Dr. Gregory House',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug2',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Black Mesa',
    humanHandle: 'Dr. Gordon Freeman',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug3',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'OsCorp',
    humanHandle: 'Dr. Otto G. Octavius',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug4',
    ty: {tag: 'Password'}
  }
];
let selectedDevice: AvailableDevice;
const password = ref('');
const orgSearchString = ref('');
const showOrganizationList = ref(true);
const store = new Storage();
const sortBy = ref('organization');
const sortByAsc = ref(true);

const msSelectOptions: MsSelectOption[] = [
  { label: t('HomePage.organizationList.sortByOrganization'), key: 'organization' },
  { label: t('HomePage.organizationList.sortByUserName'), key: 'user_name' },
  { label: t('HomePage.organizationList.sortByLastLogin'), key: 'last_login' }
];

const msSelectSortByLabels = {
  asc: t('HomePage.organizationList.sortOrderAsc'),
  desc: t('HomePage.organizationList.sortOrderDesc')
};

const filteredDevices = computed(() => {
  return deviceList.filter((item) => {
    const lowerSearchString = orgSearchString.value.toLocaleLowerCase();
    return (item.humanHandle?.toLocaleLowerCase().includes(lowerSearchString) ||
      item.organizationId?.toLocaleLowerCase().includes(lowerSearchString));
  }).sort((a, b) => {
    if (sortBy.value === 'organization') {
      if (sortByAsc.value) {
        return a.organizationId.localeCompare(b.organizationId);
      } else {
        return b.organizationId.localeCompare(a.organizationId);
      }
    } else if (sortBy.value === 'user_name') {
      if (sortByAsc.value) {
        return a.humanHandle?.localeCompare(b.humanHandle ? b.humanHandle : '');
      } else {
        return b.humanHandle?.localeCompare(a.humanHandle ? a.humanHandle : '');
      }
    } else if (sortBy.value === 'last_login') {
      const aLastLogin = (a.slug in deviceStoredDataDict.value && deviceStoredDataDict.value[a.slug].lastLogin !== undefined) ?
        deviceStoredDataDict.value[a.slug].lastLogin.valueOf() : 0;
      const bLastLogin = (b.slug in deviceStoredDataDict.value && deviceStoredDataDict.value[b.slug].lastLogin !== undefined) ?
        deviceStoredDataDict.value[b.slug].lastLogin.valueOf() : 0;
      if (sortByAsc.value) {
        return bLastLogin - aLastLogin;
      } else {
        return aLastLogin - bLastLogin;
      }
    }
    return 0;
  });
});

const deviceStoredDataDict = ref<{[slug: string]: DeviceStoredData}>({});

onMounted(async (): Promise<void> => {
  await store.create();

  store.get('devicesData').then((val) => {
    // This is needed because for some weird reason,
    // ionic-storage deserializes dates correctly in web
    // but keep them as strings during tests.
    Object.keys(val).forEach((slug, _) => {
      const obj = val[slug];
      if (obj && obj.lastLogin) {
        if (typeof obj.lastLogin === 'string') {
          obj.lastLogin = new Date(obj.lastLogin);
        }
      }
    });
    deviceStoredDataDict.value = val;
  });
});

function onPasswordChange(pwd: string): void {
  password.value = pwd;
}

function onMsSelectChange(event: MsSelectChangeEvent): void {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;
}

function onOrganizationCardClick(device: AvailableDevice): void {
  showOrganizationList.value = !showOrganizationList.value;
  selectedDevice = device;
}

async function login(): Promise<void> {
  await store.create();
  if (!deviceStoredDataDict.value[selectedDevice.slug]) {
    deviceStoredDataDict.value[selectedDevice.slug] = {
      lastLogin: new Date()
    };
  } else {
    deviceStoredDataDict.value[selectedDevice.slug].lastLogin = new Date();
  }
  console.log(`Log in to ${selectedDevice.organizationId} with password "${password.value}"`);
  await store.set('devicesData', toRaw(deviceStoredDataDict.value));
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
  return d(lastLogin, 'long');
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
#page {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  margin: 0 auto;
  align-items: center;

  #topbar {
    display: flex;
    align-items: center;
    flex-basis: 5em;
    flex-grow: 0;
    flex-shrink: 0;
    background: #fefefe;
    width: 100vw;
    justify-content: center;
    margin-bottom: 5em;

    ion-col {
      display: flex;
      align-items: center;
    }

    ion-col:first-child {
      justify-content: center;
    }

    ion-col:last-child {
      justify-content: end;
      padding-right: 3em;
    }

    img {
      max-height: 3em;
    }
  }

  #container {
    width: 70vw;
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
      --ion-grid-columns: 3;
    }

    .organization-list-grid {
      max-height: 30em;
      overflow-y: auto;
      scrollbar-color: #0058CC #F9F9FB;
      scrollbar-width: thin;

    &::-webkit-scrollbar {
      width: 1em;
      border-radius: 1em;
      background: #F9F9FB;
      border: 1px solid #EAEAF1;
    }

    &::-webkit-scrollbar-thumb {
      border-radius: 1em;
      background: #0058CC;
      border: 0.25em solid transparent;
      background-clip: content-box;
    }
    }

    ion-item {
      align-items: center;
    }

    ion-card-title {
      ion-grid {
        margin-top: 1em;
      }
    }

    .organization-card-container {
      background: #F9F9FB;
      margin: 1em 1.5em;
      user-select: none;

      ion-card-content {
        padding: 0 !important;
      }

      .organization-card {
        padding: 1em;
      }

      .organization-card-footer {
        padding: 0.5em 1em;
        background: #F3F3F7;
        border-top: 1px solid #EAEAF1;
        color: #8585AD;
        height: 4.6em;

        p {
          font-size: 0.8em;
        }
      }

      &:hover {
        background: #E5F1FF;
        cursor: pointer;

        .organization-card-footer {
          background: #E5F1FF;
          border-top: 1px solid #CCE2FF;
        }
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
