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
            ref="chooseAuthRef"
            :disable-keyring="currentDevice.ty.tag === AvailableDeviceTypeTag.Keyring"
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
          </ion-button>
          <ms-spinner v-show="querying" />
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { MsModalResult, MsPasswordInput, asyncComputed, MsSpinner } from 'megashark-lib';
import {
  AvailableDevice,
  UpdateDeviceErrorTag,
  updateDeviceChangeAuthentication,
  AvailableDeviceTypeTag,
  DeviceAccessStrategyPassword,
  DeviceAccessStrategyTag,
  isAuthenticationValid,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { Translatable, useWindowSize } from 'megashark-lib';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonTitle, modalController } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import { DeviceAccessStrategyKeyring } from '@/plugins/libparsec';

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
const chooseAuthRef = ref();
const currentPassword = ref('');
const errorMessage: Ref<Translatable> = ref('');
const passwordIsInvalid = ref(false);
const currentPasswordInput = ref();
const querying = ref(false);

onMounted(async () => {
  await currentPasswordInput.value.setFocus();
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
  } else if (pageStep.value === ChangeAuthenticationStep.ChooseNewAuthMethod) {
    return await chooseAuthRef.value.areFieldsCorrect();
  }
  return false;
});

async function changeAuthentication(): Promise<void> {
  let accessStrategy: DeviceAccessStrategyKeyring | DeviceAccessStrategyPassword;

  if (props.currentDevice.ty.tag === AvailableDeviceTypeTag.Keyring) {
    accessStrategy = {
      tag: DeviceAccessStrategyTag.Keyring,
      keyFile: props.currentDevice.keyFilePath,
    };
  } else {
    accessStrategy = {
      tag: DeviceAccessStrategyTag.Password,
      keyFile: props.currentDevice.keyFilePath,
      password: currentPassword.value,
    };
  }

  const result = await updateDeviceChangeAuthentication(accessStrategy, chooseAuthRef.value.getSaveStrategy());

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

<style scoped lang="scss"></style>
