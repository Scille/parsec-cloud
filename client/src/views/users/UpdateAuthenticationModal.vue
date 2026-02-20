<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="ChangeAuthenticationStep[pageStep]"
  >
    <!-- close button -->
    <ion-button
      slot="icon-only"
      @click="cancel()"
      class="closeBtn"
      v-if="isLargeDisplay"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>

    <!-- modal content -->
    <div class="modal">
      <ion-header
        class="modal-header"
        v-if="isLargeDisplay"
      >
        <ion-title class="modal-header__title title-h3">
          {{ $msTranslate(texts.title) }}
        </ion-title>
      </ion-header>

      <small-display-modal-header
        v-else
        @close-clicked="cancel()"
        :title="texts.title"
      />

      <div class="modal-content inner-content">
        <div
          v-show="pageStep === ChangeAuthenticationStep.CurrentAuthentication"
          class="step"
        >
          <ms-password-input
            v-if="currentDevice.ty.tag === AvailableDeviceTypeTag.Password"
            v-model="currentPassword"
            @change="updateError"
            label="Password.currentPassword"
            @on-enter-keyup="nextStep()"
            :password-is-invalid="passwordIsInvalid"
            :error-message="errorMessage"
            ref="currentPasswordInput"
          />
          <div
            class="provider-card"
            v-if="currentDevice.ty.tag === AvailableDeviceTypeTag.OpenBao"
          >
            <sso-provider-card
              :provider="(currentDevice.ty as AvailableDeviceTypeOpenBao).openbaoPreferredAuthId as OpenBaoAuthConfigTag"
              :is-connected="openBaoClient !== undefined"
              @sso-selected="onSSOLoginClicked"
            />
            <ms-spinner
              v-if="querying"
              class="provider-card-spinner"
            />
          </div>
        </div>
        <div
          v-show="pageStep === ChangeAuthenticationStep.ChooseNewAuthMethod"
          class="step"
        >
          <choose-authentication
            ref="chooseAuth"
            :active-auth="currentDevice.ty.tag"
            :server-config="serverConfig"
          />
        </div>
      </div>

      <ion-footer class="modal-footer">
        <div class="modal-footer-buttons">
          <ion-button
            fill="clear"
            size="default"
            @click="cancel"
          >
            {{ $msTranslate('MyProfilePage.cancelButton') }}
          </ion-button>
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            @click="nextStep"
            :disabled="!canGoForward || querying"
          >
            {{ $msTranslate(texts.button) }}
            <ms-spinner
              v-show="querying"
              class="modal-footer-buttons-spinner"
            />
          </ion-button>
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { SsoProviderCard } from '@/components/devices';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import {
  AvailableDevice,
  AvailableDeviceTypeOpenBao,
  AvailableDeviceTypeTag,
  DevicePrimaryProtectionStrategy,
  DevicePrimaryProtectionStrategyOpenBao,
  DevicePrimaryProtectionStrategyPassword,
  OpenBaoAuthConfigTag,
  PrimaryProtectionStrategy,
  ServerConfig,
  UpdateDeviceErrorTag,
  constructAccessStrategy,
  isAuthenticationValid,
  updateDeviceChangeAuthentication,
} from '@/parsec';
import { AvailableDeviceTypePKI } from '@/plugins/libparsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { OpenBaoClient, OpenBaoErrorType, openBaoConnect } from '@/services/openBao';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonTitle, modalController } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { MsModalResult, MsPasswordInput, MsSpinner, Translatable, asyncComputed, useWindowSize } from 'megashark-lib';
import { Ref, computed, onMounted, ref, useTemplateRef } from 'vue';

enum ChangeAuthenticationStep {
  Undefined,
  ChooseNewAuthMethod,
  CurrentAuthentication,
}

const props = defineProps<{
  currentDevice: AvailableDevice;
  informationManager: InformationManager;
  serverConfig?: ServerConfig;
}>();

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(ChangeAuthenticationStep.Undefined);
const chooseAuthRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('chooseAuth');
const currentPassword = ref('');
const errorMessage: Ref<Translatable> = ref('');
const passwordIsInvalid = ref(false);
const currentPasswordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('currentPasswordInput');
const querying = ref(false);
const openBaoClient = ref<OpenBaoClient | undefined>(undefined);

const texts = computed(() => {
  switch (pageStep.value) {
    case ChangeAuthenticationStep.CurrentAuthentication:
      switch (props.currentDevice.ty.tag) {
        case AvailableDeviceTypeTag.Password:
          return { title: 'MyProfilePage.titleCurrentPassword', button: 'MyProfilePage.nextButton' };
        case AvailableDeviceTypeTag.OpenBao:
          return { title: 'MyProfilePage.titleCurrentOpenBao', button: 'MyProfilePage.nextButton' };
      }
    case ChangeAuthenticationStep.ChooseNewAuthMethod:
      return { title: 'MyProfilePage.titleNewAuthentication', button: 'MyProfilePage.changeAuthenticationButton' };
    default:
      return { title: '', button: '' };
  }
});

onMounted(async () => {
  if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Password || props.currentDevice.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    pageStep.value = ChangeAuthenticationStep.CurrentAuthentication;
    if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Password) {
      await currentPasswordInputRef.value?.setFocus();
    }
  } else {
    pageStep.value = ChangeAuthenticationStep.ChooseNewAuthMethod;
  }
});

async function nextStep(): Promise<void> {
  if (pageStep.value === ChangeAuthenticationStep.CurrentAuthentication) {
    let primaryProtection!: DevicePrimaryProtectionStrategyPassword | DevicePrimaryProtectionStrategyOpenBao;
    if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Password) {
      primaryProtection = PrimaryProtectionStrategy.usePassword(currentPassword.value);
    } else if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.OpenBao) {
      if (!openBaoClient.value) {
        return;
      }
      primaryProtection = PrimaryProtectionStrategy.useOpenBao(openBaoClient.value.getConnectionInfo());
    }
    const access = constructAccessStrategy(props.currentDevice, primaryProtection);
    const result = await isAuthenticationValid(props.currentDevice, access);
    if (result) {
      pageStep.value = ChangeAuthenticationStep.ChooseNewAuthMethod;
    } else {
      errorMessage.value = 'MyProfilePage.errors.wrongAuthentication';
      passwordIsInvalid.value = true;
    }
  } else if (pageStep.value === ChangeAuthenticationStep.ChooseNewAuthMethod) {
    querying.value = true;
    await changeAuthentication();
    querying.value = false;
  }
}

const canGoForward = asyncComputed(async () => {
  if (pageStep.value === ChangeAuthenticationStep.CurrentAuthentication) {
    if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Password && currentPassword.value.length > 0) {
      return true;
    } else if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.OpenBao && openBaoClient.value) {
      return true;
    }
  } else if (pageStep.value === ChangeAuthenticationStep.ChooseNewAuthMethod && chooseAuthRef.value) {
    return await chooseAuthRef.value.areFieldsCorrect();
  }
  return false;
});

async function onSSOLoginClicked(): Promise<void> {
  if (querying.value) {
    window.electronAPI.log('warn', 'Clicked on SSO login while already login in');
    return;
  }
  if (!props.serverConfig || !props.serverConfig.openbao) {
    window.electronAPI.log('error', 'Server config or current device not found');
    return;
  }
  if (props.currentDevice.ty.tag !== AvailableDeviceTypeTag.OpenBao) {
    window.electronAPI.log('error', 'Device is not OpenBao device');
    return;
  }
  const provider = (props.currentDevice.ty as AvailableDeviceTypeOpenBao).openbaoPreferredAuthId;
  const auth = props.serverConfig.openbao.auths.find((v) => v.tag === provider);
  if (!auth) {
    window.electronAPI.log('error', `Provider '${provider}' selected but is not available in server config`);
    return;
  }
  try {
    querying.value = true;
    const result = await openBaoConnect(
      props.serverConfig.openbao.serverUrl,
      auth.tag,
      auth.mountPath,
      props.serverConfig.openbao.secret.mountPath,
      props.serverConfig.openbao.transitMountPath,
    );
    if (!result.ok) {
      if (result.error.type === OpenBaoErrorType.PopupFailed) {
        errorMessage.value = 'Authentication.popupBlocked';
      } else {
        errorMessage.value = 'Authentication.invalidOpenBaoData';
      }
      window.electronAPI.log('error', `Error while connecting with SSO: ${JSON.stringify(result.error)}`);
    } else {
      openBaoClient.value = result.value;
    }
  } finally {
    querying.value = false;
  }
}

async function changeAuthentication(): Promise<void> {
  let protection!: DevicePrimaryProtectionStrategy;

  if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Keyring) {
    protection = PrimaryProtectionStrategy.useKeyring();
  } else if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Password) {
    protection = PrimaryProtectionStrategy.usePassword(currentPassword.value);
  } else if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.PKI) {
    protection = PrimaryProtectionStrategy.useSmartcard((props.currentDevice as any as AvailableDeviceTypePKI).certificateRef);
  } else if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    if (!openBaoClient.value) {
      window.electronAPI.log('error', 'OpenBaoClient should not be undefined at this step');
      return;
    }
    protection = PrimaryProtectionStrategy.useOpenBao(openBaoClient.value.getConnectionInfo());
  } else {
    // Should not happen
    window.electronAPI.log('error', `Unhandled authentication type for this device: ${props.currentDevice.ty.tag}`);
    props.informationManager.present(
      new Information({
        message: 'MyProfilePage.errors.cannotChangeAuthentication',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const saveStrategy = chooseAuthRef.value?.getSaveStrategy();
  if (!saveStrategy) {
    return;
  }

  const result = await updateDeviceChangeAuthentication(constructAccessStrategy(props.currentDevice, protection), saveStrategy);

  if (result.ok) {
    props.informationManager.present(
      new Information({
        message: 'MyProfilePage.authenticationUpdated',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await modalController.dismiss(undefined, MsModalResult.Confirm);
  } else {
    switch (result.error.tag) {
      case UpdateDeviceErrorTag.DecryptionFailed: {
        errorMessage.value = 'MyProfilePage.errors.wrongPassword';
        break;
      }
      default:
        props.informationManager.present(
          new Information({
            message: 'MyProfilePage.errors.cannotChangeAuthentication',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
    }
  }
}

function updateError(): void {
  passwordIsInvalid.value = false;
  errorMessage.value = '';
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.modal-header__title {
  margin-right: 1.5rem;
}

.modal-footer-buttons-spinner {
  width: 1rem;
  height: 1rem;
  margin-left: 0.5rem;
}

.provider-card {
  position: relative;
  margin-top: 1.25rem;

  .provider-card-spinner {
    position: absolute;
    top: 2rem;
    right: 2rem;
    transform: translate(-50%, -50%);
  }
}
</style>
