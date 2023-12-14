<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="container">
        <div class="devices-container">
          <!-- header -->
          <div class="devices-header">
            <h2 class="title-h2 devices-header-title">
              {{ $t('DevicesPage.title') }}
            </h2>
            <ion-button
              class="devices-header-button"
              fill="clear"
              @click="onAddDeviceClick()"
            >
              <ion-icon :icon="add" />
              <span>{{ $t('DevicesPage.addDevice') }}</span>
            </ion-button>
          </div>

          <!-- device list -->
          <div class="devices-content">
            <ion-text
              class="no-device"
              v-if="devices.length === 0"
            >
              {{ $t('DevicesPage.noDevices') }}
            </ion-text>
            <ion-list
              class="devices-list"
              v-if="devices.length > 0"
            >
              <ion-item
                v-for="device in devices"
                :key="device.id"
                class="device-list-item"
              >
                <device-card
                  :label="device.deviceLabel"
                  :date="device.createdOn"
                  :is-current="device.isCurrent"
                />
              </ion-item>
            </ion-list>
          </div>
        </div>

        <!-- restore password card -->
        <!-- files not downloaded -->
        <div
          class="restore-password"
          v-if="!passwordSaved"
        >
          <ion-label class="body-sm danger">
            {{ $t('DevicesPage.restorePassword.notDone.label') }}
          </ion-label>
          <div class="restore-password-header">
            <ms-image
              :image="PasswordLock"
              class="restore-password-header-img"
            />
            <h3 class="title-h3 restore-password-header__title">
              {{ $t('DevicesPage.restorePassword.title') }}
            </h3>
          </div>
          <div class="restore-password-subtitles">
            <ion-text
              class="body"
              :show="passwordSaved"
            >
              {{ $t('DevicesPage.restorePassword.notDone.subtitle') }}
            </ion-text>
            <ion-text class="body">
              {{ $t('DevicesPage.restorePassword.notDone.subtitle2') }}
            </ion-text>
          </div>
          <div class="restore-password-button">
            <ion-button
              class="button-default"
              @click="goToExportRecoveryDevice()"
            >
              <ion-icon
                :icon="sparkles"
                class="icon"
              />
              {{ $t('DevicesPage.restorePassword.notDone.button') }}
            </ion-button>
          </div>
        </div>
        <!-- files downloaded -->
        <div
          class="restore-password"
          v-else
        >
          <ion-label class="body-sm done">
            {{ $t('DevicesPage.restorePassword.done.label') }}
          </ion-label>
          <div class="restore-password-header">
            <ms-image
              :image="PasswordLock"
              class="restore-password-header-img"
            />
            <h3 class="title-h3 restore-password-header__title">
              {{ $t('DevicesPage.restorePassword.title') }}
            </h3>
          </div>
          <div class="restore-password-subtitles">
            <ion-text class="body">
              {{ $t('DevicesPage.restorePassword.done.subtitle') }}
            </ion-text>
          </div>
          <div class="restore-password-button">
            <ion-button
              @click="goToExportRecoveryDevice()"
              class="button-default"
              fill="clear"
            >
              <ion-icon
                :icon="download"
                class="icon"
              />
              {{ $t('DevicesPage.restorePassword.done.button') }}
            </ion-button>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { NotificationKey } from '@/common/injectionKeys';
import { MsImage, MsModalResult, PasswordLock } from '@/components/core';
import DeviceCard from '@/components/devices/DeviceCard.vue';
import { OwnDeviceInfo, hasRecoveryDevice, listOwnDevices } from '@/parsec';
import { routerNavigateTo } from '@/router';
import { Notification, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import GreetDeviceModal from '@/views/devices/GreetDeviceModal.vue';
import { IonButton, IonContent, IonIcon, IonItem, IonLabel, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { add, download, sparkles } from 'ionicons/icons';
import { Ref, inject, onMounted, ref } from 'vue';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationManager: NotificationManager = inject(NotificationKey)!;
const devices: Ref<OwnDeviceInfo[]> = ref([]);
const passwordSaved = ref(false);

onMounted(async () => {
  await refreshDevicesList();
});

async function goToExportRecoveryDevice(): Promise<void> {
  routerNavigateTo('recoveryExport');
}

async function refreshDevicesList(): Promise<void> {
  const result = await listOwnDevices();
  if (result.ok) {
    devices.value = result.value;
    if (await hasRecoveryDevice()) {
      passwordSaved.value = true;
    }
  } else {
    notificationManager.showToast(
      new Notification({
        message: 'Failed to retrieve devices',
        level: NotificationLevel.Error,
      }),
    );
    console.log('Could not list devices', result.error);
  }
}

async function onAddDeviceClick(): Promise<void> {
  const modal = await modalController.create({
    component: GreetDeviceModal,
    canDismiss: true,
    cssClass: 'greet-organization-modal',
  });
  await modal.present();
  const modalResult = await modal.onWillDismiss();
  await modal.dismiss();
  if (modalResult.role === MsModalResult.Confirm) {
    await refreshDevicesList();
  }
}
</script>

<style scoped lang="scss">
.container {
  display: flex;
  max-width: 70rem;
}

.devices-container {
  margin: 2.5em 2rem 0;
  width: 45%;
}

.devices-header {
  padding-inline: 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);

  &-title {
    margin: 0;
    color: var(--parsec-color-light-primary-700);
  }

  &-button {
    margin: 0;

    span {
      margin-left: 0.625rem;
    }
  }
}

.devices-content {
  margin-top: 2rem;

  .devices-list {
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
}

.restore-password {
  width: 45%;
  margin: 6rem 2rem 0;
  padding: 2rem 1.5rem;
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-disabled);
  border-radius: var(--parsec-radius-6);
  display: flex;
  flex-direction: column;
  position: relative;

  ion-label {
    width: max-content;
    position: absolute;
    top: -0.8rem;
    left: 50%;
    transform: translateX(-50%);
    padding: 0.125rem 1.75rem;
    border-radius: var(--parsec-radius-12);

    &.done {
      color: var(--parsec-color-light-success-500);
      background: var(--parsec-color-light-success-100);
    }

    &.danger {
      background: var(--parsec-color-light-danger-100);
      color: var(--parsec-color-light-danger-500);
    }
  }

  &-header {
    display: flex;
    align-items: center;
    gap: 1rem;

    &__title {
      margin: 0;
      color: var(--parsec-color-light-primary-700);
    }

    &-img {
      width: 3.25rem;
      height: 3.25rem;
      margin-right: 0.5rem;
    }
  }

  &-subtitles {
    margin: 1rem 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-grey);
  }

  &-button {
    .icon {
      margin-right: 0.625rem;
      font-size: 1.125rem;
    }
  }
}
</style>
