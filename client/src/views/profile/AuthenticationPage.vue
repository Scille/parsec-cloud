<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="authentication-container">
    <template v-if="currentDevice">
      <div class="authentication-content">
        <authentication-card
          :auth-method="DeviceSaveStrategyTag.Keyring"
          :state="AuthenticationCardState.Current"
          v-show="currentDevice && currentDevice.ty.tag === AvailableDeviceTypeTag.Keyring"
        />
        <authentication-card
          :auth-method="DeviceSaveStrategyTag.Password"
          :state="AuthenticationCardState.Current"
          v-show="currentDevice && currentDevice.ty.tag === AvailableDeviceTypeTag.Password"
        />
        <authentication-card
          :auth-method="DeviceSaveStrategyTag.PKI"
          :state="AuthenticationCardState.Current"
          v-show="currentDevice && currentDevice.ty.tag === AvailableDeviceTypeTag.PKI"
        />
        <authentication-card
          :auth-method="DeviceSaveStrategyTag.OpenBao"
          :state="AuthenticationCardState.Current"
          v-show="currentDevice && currentDevice.ty.tag === AvailableDeviceTypeTag.OpenBao"
        />
        <ion-button
          id="change-authentication-button"
          class="update-auth-button button-default"
          fill="clear"
          @click="openChangeAuthentication()"
        >
          <ion-text class="update-auth-button__label">
            <span>{{ $msTranslate('Authentication.changeAuthenticationButton') }}</span>
          </ion-text>
        </ion-button>
      </div>
    </template>
    <template v-if="error">
      <div class="device-not-found">
        <ion-icon :icon="warning" />
        <ion-text class="item-container__text body">
          {{ $msTranslate(error) }}
        </ion-text>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import authenticationCard from '@/components/profile/AuthenticationCard.vue';
import { AuthenticationCardState } from '@/components/profile/types';
import { AvailableDevice, AvailableDeviceTypeTag, DeviceSaveStrategyTag, getCurrentAvailableDevice, getServerConfig } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import UpdateAuthenticationModal from '@/views/users/UpdateAuthenticationModal.vue';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { Answer, askQuestion, MsModalResult } from 'megashark-lib';
import { inject, onMounted, Ref, ref } from 'vue';

const currentDevice: Ref<AvailableDevice | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const error = ref('');

async function openChangeAuthentication(): Promise<void> {
  if (!currentDevice.value) {
    return;
  }

  const configResult = await getServerConfig(currentDevice.value.serverAddr);

  if (currentDevice.value.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    if (!configResult.ok || !configResult.value.openbao) {
      error.value = 'Authentication.invalidOpenBaoData';
      return;
    }
  }

  if (currentDevice.value.ty.tag === AvailableDeviceTypeTag.PKI) {
    const answer = await askQuestion('Authentication.method.smartcard.warn.title', 'Authentication.method.smartcard.warn.subtitle', {
      yesText: 'Authentication.method.smartcard.warn.yes',
      noText: 'Authentication.method.smartcard.warn.no',
    });

    if (answer === Answer.No) {
      return;
    }
  }

  const modal = await modalController.create({
    component: UpdateAuthenticationModal,
    cssClass: 'change-authentication-modal',
    componentProps: {
      currentDevice: currentDevice.value,
      informationManager: informationManager,
      serverConfig: configResult.ok ? configResult.value : undefined,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();
  if (role === MsModalResult.Confirm) {
    const result = await getCurrentAvailableDevice();
    if (result.ok) {
      currentDevice.value = result.value;
      error.value = '';
    } else {
      error.value = 'MyProfilePage.errors.failedToRetrieveInformation';
    }
  }
}

onMounted(async () => {
  const deviceResult = await getCurrentAvailableDevice();

  if (!deviceResult.ok) {
    informationManager.present(
      new Information({
        message: 'MyProfilePage.errors.failedToRetrieveInformation',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    error.value = 'MyProfilePage.errors.failedToRetrieveInformation';
  } else {
    error.value = '';
    currentDevice.value = deviceResult.value;
  }
});
</script>

<style scoped lang="scss">
.authentication-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.authentication-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.device-not-found {
  display: flex;
  align-items: center;
  background: var(--parsec-color-light-danger-50);
  color: var(--parsec-color-light-danger-700);
  padding: 0.5rem 1rem;
  border-radius: var(--parsec-radius-8);
  gap: 0.5rem;

  ion-text {
    padding: 0.25rem 0;
  }

  ion-icon {
    font-size: 1.25rem;
  }
}

.update-auth-button {
  margin-top: 1em;
  width: fit-content;
  --background: var(--parsec-color-light-secondary-text);
  --background-hover: var(--parsec-color-light-secondary-contrast);
  color: var(--parsec-color-light-secondary-white);

  @include ms.responsive-breakpoint('xs') {
    position: fixed;
    bottom: 2rem;
    left: 2rem;
    transform: translateX(50%, 50%);
    width: calc(100% - 4rem);
    margin: auto;
    z-index: 2;
    box-shadow: var(--parsec-shadow-strong);
  }
}
</style>
