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
          :auth-method="DeviceSaveStrategyTag.Smartcard"
          :state="AuthenticationCardState.Current"
          v-show="currentDevice && currentDevice.ty.tag === AvailableDeviceTypeTag.Smartcard"
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
          <ion-label class="update-auth-button__label">
            <span v-if="multipleAuthAvailable()">{{ $msTranslate('Authentication.changeAuthenticationButton') }}</span>
            <span v-else>{{ $msTranslate('Authentication.changePasswordButton') }}</span>
          </ion-label>
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
import {
  AvailableDevice,
  AvailableDeviceTypeOpenBao,
  AvailableDeviceTypeTag,
  DeviceSaveStrategyTag,
  getCurrentAvailableDevice,
  getServerConfig,
  isWeb,
  ServerConfig,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { isSSOProviderHandled, OpenBaoClient, openBaoConnect, OpenBaoConnectionInfo } from '@/services/openBao';
import UpdateAuthenticationModal from '@/views/users/UpdateAuthenticationModal.vue';
import { IonButton, IonIcon, IonLabel, IonText, modalController } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { Answer, askQuestion, MsModalResult } from 'megashark-lib';
import { inject, onMounted, Ref, ref } from 'vue';

const currentDevice: Ref<AvailableDevice | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const error = ref('');
const serverConfig = ref<ServerConfig | undefined>(undefined);

function multipleAuthAvailable(): boolean {
  if (!isWeb()) {
    return true;
  }

  return (serverConfig.value?.openbao && serverConfig.value?.openbao.auths.some((auth) => isSSOProviderHandled(auth.tag))) === true;
}

async function getOpenBaoClient(): Promise<OpenBaoClient | void> {
  if (!serverConfig.value || !currentDevice.value) {
    window.electronAPI.log('error', 'Server config or current device not found');
    return;
  }
  if (!serverConfig.value.openbao) {
    window.electronAPI.log('error', 'OpenBao not enabled on this server');
    return;
  }
  const provider = (currentDevice.value.ty as AvailableDeviceTypeOpenBao).openbaoPreferredAuthId;
  const auth = serverConfig.value.openbao.auths.find((v) => v.tag === provider);
  if (!auth) {
    window.electronAPI.log('error', `Provider '${provider}' selected but is not available in server config`);
    return;
  }
  const result = await openBaoConnect(
    serverConfig.value.openbao.serverUrl,
    auth.tag,
    auth.mountPath,
    serverConfig.value.openbao.secret.mountPath,
  );
  if (!result.ok) {
    window.electronAPI.log('error', `Error while connecting with SSO: ${JSON.stringify(result.error)}`);
  } else {
    return result.value;
  }
}

async function openChangeAuthentication(): Promise<void> {
  let openBaoConnInfo: OpenBaoConnectionInfo | undefined = undefined;

  if (currentDevice.value && currentDevice.value.ty.tag === AvailableDeviceTypeTag.Smartcard) {
    const answer = await askQuestion('Authentication.method.smartcard.warn.title', 'Authentication.method.smartcard.warn.subtitle', {
      yesText: 'Authentication.method.smartcard.warn.yes',
      noText: 'Authentication.method.smartcard.warn.no',
    });

    if (answer === Answer.No) {
      return;
    }
  } else if (currentDevice.value && currentDevice.value.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    const connResult = await getOpenBaoClient();
    if (!connResult) {
      return;
    }
    openBaoConnInfo = connResult.getConnectionInfo();
  }

  const modal = await modalController.create({
    component: UpdateAuthenticationModal,
    cssClass: 'change-authentication-modal',
    componentProps: {
      currentDevice: currentDevice.value,
      informationManager: informationManager,
      serverConfig: serverConfig.value,
      openBaoConnInfo: openBaoConnInfo,
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
    currentDevice.value = deviceResult.value;
    const configResult = await getServerConfig(currentDevice.value.serverAddr);
    if (configResult.ok) {
      serverConfig.value = configResult.value;
    }
    error.value = '';
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
