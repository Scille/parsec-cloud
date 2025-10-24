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
          {{ $msTranslate(getTitle()) }}
        </ion-title>
      </ion-header>

      <small-display-modal-header
        v-else
        @close-clicked="cancel()"
        :title="getTitle()"
      />

      <div class="modal-content inner-content">
        <div
          v-show="pageStep === ChangeAuthenticationStep.InputCurrentPassword"
          class="step"
        >
          <ms-password-input
            v-model="currentPassword"
            @change="updateError"
            label="Password.currentPassword"
            @on-enter-keyup="nextStep()"
            :password-is-invalid="passwordIsInvalid"
            :error-message="errorMessage"
            ref="currentPasswordInput"
          />
        </div>
        <div
          v-show="pageStep === ChangeAuthenticationStep.ChooseNewAuthMethod"
          class="step"
        >
          <choose-authentication
            ref="chooseAuth"
            :active-auth="currentDevice.ty.tag"
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
            {{ $msTranslate(getNextButtonText()) }}
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
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import {
  AvailableDevice,
  AvailableDeviceTypeTag,
  DeviceAccessStrategyPassword,
  DeviceAccessStrategyTag,
  UpdateDeviceErrorTag,
  isAuthenticationValid,
  updateDeviceChangeAuthentication,
} from '@/parsec';
import { DeviceAccessStrategyKeyring, DeviceAccessStrategySmartcard } from '@/plugins/libparsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonTitle, modalController } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { MsModalResult, MsPasswordInput, MsSpinner, Translatable, asyncComputed, useWindowSize } from 'megashark-lib';
import { Ref, onMounted, ref, useTemplateRef } from 'vue';

enum ChangeAuthenticationStep {
  Undefined,
  ChooseNewAuthMethod,
  InputCurrentPassword,
}

const props = defineProps<{
  currentDevice: AvailableDevice;
  informationManager: InformationManager;
}>();

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(ChangeAuthenticationStep.Undefined);
const chooseAuthRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('chooseAuth');
const currentPassword = ref('');
const errorMessage: Ref<Translatable> = ref('');
const passwordIsInvalid = ref(false);
const currentPasswordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('currentPasswordInput');
const querying = ref(false);

onMounted(async () => {
  await currentPasswordInputRef.value?.setFocus();
  if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Password) {
    pageStep.value = ChangeAuthenticationStep.InputCurrentPassword;
  } else {
    pageStep.value = ChangeAuthenticationStep.ChooseNewAuthMethod;
  }
});

async function nextStep(): Promise<void> {
  if (pageStep.value === ChangeAuthenticationStep.InputCurrentPassword) {
    const access: DeviceAccessStrategyPassword = {
      tag: DeviceAccessStrategyTag.Password,
      password: currentPassword.value,
      keyFile: props.currentDevice.keyFilePath,
    };
    const result = await isAuthenticationValid(props.currentDevice, access);
    if (result) {
      pageStep.value = ChangeAuthenticationStep.ChooseNewAuthMethod;
    } else {
      errorMessage.value = 'MyProfilePage.errors.wrongPassword';
      passwordIsInvalid.value = true;
    }
  } else if (pageStep.value === ChangeAuthenticationStep.ChooseNewAuthMethod) {
    querying.value = true;
    await changeAuthentication();
    querying.value = false;
  }
}

const canGoForward = asyncComputed(async () => {
  if (pageStep.value === ChangeAuthenticationStep.InputCurrentPassword && currentPassword.value.length > 0) {
    return true;
  } else if (pageStep.value === ChangeAuthenticationStep.ChooseNewAuthMethod && chooseAuthRef.value) {
    return await chooseAuthRef.value.areFieldsCorrect();
  }
  return false;
});

async function changeAuthentication(): Promise<void> {
  let accessStrategy: DeviceAccessStrategyKeyring | DeviceAccessStrategyPassword | DeviceAccessStrategySmartcard;

  if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Keyring) {
    accessStrategy = {
      tag: DeviceAccessStrategyTag.Keyring,
      keyFile: props.currentDevice.keyFilePath,
    };
  } else if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Password) {
    accessStrategy = {
      tag: DeviceAccessStrategyTag.Password,
      keyFile: props.currentDevice.keyFilePath,
      password: currentPassword.value,
    };
  } else if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Smartcard) {
    accessStrategy = {
      tag: DeviceAccessStrategyTag.Smartcard,
      keyFile: props.currentDevice.keyFilePath,
    };
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

  const result = await updateDeviceChangeAuthentication(accessStrategy, saveStrategy);

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

function getTitle(): string {
  switch (pageStep.value) {
    case ChangeAuthenticationStep.InputCurrentPassword:
      return 'MyProfilePage.titleActualPassword';
    case ChangeAuthenticationStep.ChooseNewAuthMethod:
      return 'MyProfilePage.titleNewAuthentication';
    default:
      return '';
  }
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}

function getNextButtonText(): string {
  switch (pageStep.value) {
    case ChangeAuthenticationStep.InputCurrentPassword:
      return 'MyProfilePage.nextButton';
    case ChangeAuthenticationStep.ChooseNewAuthMethod:
      return 'MyProfilePage.changeAuthenticationButton';
    default:
      return '';
  }
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
</style>
