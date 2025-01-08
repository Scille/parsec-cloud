<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container">
    <div class="devices-container">
      <!-- header -->
      <div class="devices-header">
        <ion-button
          class="devices-header-button"
          id="add-device-button"
          fill="clear"
          @click="onAddDeviceClick()"
        >
          <ion-icon :icon="add" />
          <span>{{ $msTranslate('DevicesPage.addDevice') }}</span>
        </ion-button>
      </div>

      <!-- device list -->
      <div class="devices-content">
        <ion-text
          class="no-device"
          v-if="devices.length === 0"
        >
          {{ $msTranslate('DevicesPage.noDevices') }}
        </ion-text>
        <ion-list
          class="devices-list"
          id="devices-list"
          v-if="devices.length > 0"
        >
          <ion-item
            v-for="device in devices"
            :key="device.id"
            class="device-list-item ion-no-padding"
            v-show="!device.isRecovery"
          >
            <device-card
              :device="device"
              :is-current="device.isCurrent"
              :show-id="true"
            />
          </ion-item>
        </ion-list>
      </div>
    </div>

    <!-- restore password card -->
    <div>
      <!-- files not downloaded -->
      <div
        class="restore-password"
        v-if="!hasRecoveryDevice"
      >
        <ion-label class="body-sm danger">
          {{ $msTranslate('DevicesPage.restorePassword.notDone.label') }}
        </ion-label>
        <div class="restore-password-header">
          <ms-image
            :image="PasswordLock"
            class="restore-password-header-img"
          />
          <h3 class="title-h3 restore-password-header__title">
            {{ $msTranslate('DevicesPage.restorePassword.title') }}
          </h3>
        </div>
        <div class="restore-password-subtitles">
          <ion-text class="body">
            {{ $msTranslate('DevicesPage.restorePassword.notDone.subtitle') }}
          </ion-text>
          <ion-text class="body">
            {{ $msTranslate('DevicesPage.restorePassword.notDone.subtitle2') }}
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
            {{ $msTranslate('DevicesPage.restorePassword.notDone.button') }}
          </ion-button>
        </div>
      </div>
      <!-- files downloaded -->
      <div
        class="restore-password"
        v-else
      >
        <ion-label class="body-sm done">
          {{ $msTranslate('DevicesPage.restorePassword.done.label') }}
        </ion-label>
        <div class="restore-password-header">
          <ms-image
            :image="PasswordLock"
            class="restore-password-header-img"
          />
          <h3 class="title-h3 restore-password-header__title">
            {{ $msTranslate('DevicesPage.restorePassword.title') }}
          </h3>
        </div>
        <div class="restore-password-subtitles">
          <ion-text class="body">
            {{ $msTranslate('DevicesPage.restorePassword.done.subtitle') }}
          </ion-text>
          <ion-text class="body">
            {{ $msTranslate('DevicesPage.restorePassword.done.subtitle2') }}
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
            {{ $msTranslate('DevicesPage.restorePassword.done.button') }}
          </ion-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Answer, askQuestion, MsImage, MsModalResult, PasswordLock } from 'megashark-lib';
import DeviceCard from '@/components/devices/DeviceCard.vue';
import { OwnDeviceInfo, createDeviceInvitation, listOwnDevices } from '@/parsec';
import { Routes, navigateTo, watchRoute, getCurrentRouteName } from '@/router';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import GreetDeviceModal from '@/views/devices/GreetDeviceModal.vue';
import { IonButton, IonIcon, IonItem, IonLabel, IonList, IonText, modalController } from '@ionic/vue';
import { add, download, sparkles } from 'ionicons/icons';
import { Ref, inject, onMounted, ref, computed, onUnmounted } from 'vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const devices: Ref<OwnDeviceInfo[]> = ref([]);
const hasRecoveryDevice = computed(() => {
  return devices.value.find((device) => device.isRecovery) !== undefined;
});

const routeWatchCancel = watchRoute(async () => {
  if (getCurrentRouteName() !== Routes.MyProfile) {
    return;
  }
  await refreshDevicesList();
});

onMounted(async () => {
  await refreshDevicesList();
});

onUnmounted(async () => {
  routeWatchCancel();
});

async function goToExportRecoveryDevice(): Promise<void> {
  if (hasRecoveryDevice.value) {
    const answer = await askQuestion(
      'DevicesPage.restorePassword.done.recreateQuestionTitle',
      'DevicesPage.restorePassword.done.recreateQuestionMessage',
      { yesText: 'DevicesPage.restorePassword.done.recreateYes', noText: 'DevicesPage.restorePassword.done.recreateNo' },
    );
    if (answer === Answer.No) {
      return;
    }
  }
  await navigateTo(Routes.RecoveryExport);
}

async function refreshDevicesList(): Promise<void> {
  const result = await listOwnDevices();
  if (result.ok) {
    devices.value = result.value;
  } else {
    informationManager.present(
      new Information({
        message: 'DevicesPage.greet.errors.retrieveDeviceInfoFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    window.electronAPI.log('error', `Failed to list devices ${JSON.stringify(result.error)}`);
  }
}

async function onAddDeviceClick(): Promise<void> {
  const result = await createDeviceInvitation(false);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: 'DevicesPage.greet.errors.startFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const modal = await modalController.create({
    component: GreetDeviceModal,
    canDismiss: true,
    cssClass: 'greet-organization-modal',
    componentProps: {
      informationManager: informationManager,
      invitationLink: result.value.addr,
      token: result.value.token,
    },
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
  flex-direction: column;
  flex: 1;
}

.devices-header {
  padding-inline: 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;

  &-button {
    margin-left: auto;

    span {
      margin-left: 0.625rem;
    }
  }
}

.devices-content {
  margin-top: 2rem;
  max-height: 16em;
  overflow-y: auto;

  .devices-list {
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
}

.restore-password {
  max-width: 75%;
  margin: 6rem 3rem 0 auto;
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
