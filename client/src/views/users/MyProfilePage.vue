<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="page">
    <ion-content :fullscreen="true">
      <div class="page-content">
        <div class="menu">
          <ion-radio-group
            v-model="myProfileTab"
            :value="MyProfileTabs.Devices"
            class="menu-list"
          >
            <ion-radio
              slot="start"
              :value="MyProfileTabs.Devices"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="phonePortrait" />
                <ion-text class="body">
                  {{ $t('MyProfilePage.tabs.devices') }}
                </ion-text>
              </div>
            </ion-radio>
            <ion-radio
              slot="start"
              :value="MyProfileTabs.Authentication"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="lockClosed" />
                <ion-text class="body">
                  {{ $t('MyProfilePage.tabs.authentication') }}
                </ion-text>
              </div>
            </ion-radio>
          </ion-radio-group>
          <div
            v-if="myProfileTab === MyProfileTabs.Devices"
            class="menu-item-content"
          >
            <devices-page class="devices" />
          </div>
          <div
            v-if="myProfileTab === MyProfileTabs.Authentication"
            class="menu-item-content"
          >
            <template v-if="clientInfo">
              <!-- inputs fields -->
              <div class="user-info">
                <ion-text class="user-info__label body title">
                  {{ $t('MyProfilePage.password') }}
                </ion-text>
                <div
                  class="user-info__password"
                  v-show="currentDevice && currentDevice.ty === DeviceFileType.Password"
                >
                  <ms-input
                    :placeholder="'••••••••••'"
                    name="fullname"
                    :disabled="true"
                    class="user-info__input"
                  />
                  <ion-button
                    id="change-password-button"
                    @click="openChangePassword()"
                    size="small"
                  >
                    {{ $t('MyProfilePage.changePasswordButton') }}
                  </ion-button>
                </div>
                <div v-show="currentDevice && currentDevice.ty === DeviceFileType.Keyring">
                  <ms-informative-text>{{ $t('MyProfilePage.systemAuthentication') }}</ms-informative-text>
                </div>
              </div>
            </template>
            <template v-else>
              <div class="device-not-found">
                <ion-icon
                  :icon="warning"
                  size="large"
                />
                <ion-text class="body">
                  {{ $t('MyProfilePage.errors.failedToRetrieveInformation') }}
                </ion-text>
              </div>
            </template>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { MsInformativeText, MsInput } from '@/components/core';
import { AvailableDevice, ClientInfo, DeviceFileType, getClientInfo, getCurrentAvailableDevice } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import DevicesPage from '@/views/devices/DevicesPage.vue';
import UpdatePasswordModal from '@/views/users/UpdatePasswordModal.vue';
import { IonButton, IonContent, IonIcon, IonPage, IonRadio, IonRadioGroup, IonText, modalController } from '@ionic/vue';
import { lockClosed, phonePortrait, warning } from 'ionicons/icons';
import { Ref, inject, onMounted, ref } from 'vue';

const clientInfo: Ref<ClientInfo | null> = ref(null);
const currentDevice: Ref<AvailableDevice | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;

enum MyProfileTabs {
  Devices = 'Devices',
  Authentication = 'Authentication',
}

const myProfileTab = ref(MyProfileTabs.Devices);

async function openChangePassword(): Promise<void> {
  const modal = await modalController.create({
    component: UpdatePasswordModal,
    cssClass: 'change-password-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

onMounted(async () => {
  const deviceResult = await getCurrentAvailableDevice();
  const result = await getClientInfo();

  if (!result.ok || !deviceResult.ok) {
    informationManager.present(
      new Information({
        message: translate('MyProfilePage.errors.failedToRetrieveInformation'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    clientInfo.value = result.value;
    currentDevice.value = deviceResult.value;
  }
});
</script>

<style scoped lang="scss">
.page-content {
  margin: 3rem 2rem 2rem;
  display: flex;
  gap: 1.5rem;
}

.menu {
  display: flex;
  gap: 2rem;
  width: 100%;
}

.menu-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
  max-width: 11.25rem;

  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  &__item {
    color: var(--parsec-color-light-secondary-grey);
    border-radius: var(--parsec-radius-6);

    .item-container {
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 0.5rem 0.75em;
      gap: 0.375rem;
    }

    &::part(container) {
      display: none;
    }

    &.radio-checked {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-50);
    }

    &:hover:not(.radio-checked) {
      background: var(--parsec-color-light-secondary-premiere);
      color: var(--parsec-color-light-secondary-text);
    }

    ion-icon {
      font-size: 1.25rem;
    }
  }
}

.menu-item-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  max-width: 30rem;
  margin-top: 0.75rem;
}

.user-info {
  &__label {
    min-width: 14rem;
    color: var(--parsec-color-light-primary-700);
    margin-bottom: 1rem;
    display: flex;
  }

  &__input {
    display: flex;
    width: 100%;
    max-width: 20rem;
  }

  &__password {
    display: flex;
    align-items: start;
    gap: 1rem;
    width: 100%;
    flex-direction: column;
  }
}

.device-not-found {
  background: var(--parsec-color-light-danger-100);
  color: var(--parsec-color-light-danger-700);
  padding: 1rem;
  display: flex;
  gap: 0.75rem;
  border-left: 0.25rem solid var(--parsec-color-light-danger-500);

  ion-text {
    padding: 0.25rem 0;
  }
}
</style>
