<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="choose-auth-page"
    :class="isWeb() ? 'choose-auth-page--web' : ''"
  >
    <div
      class="choose-auth-list"
      v-if="authentication === undefined"
    >
      <ion-text
        class="body choose-auth-list__label"
        v-show="showTitle"
      >
        {{ $msTranslate('Authentication.description') }}
      </ion-text>
      <ion-radio-group
        v-model="authentication"
        class="radio-list"
        :class="isWeb() ? 'radio-list--web' : ''"
        @ion-change="onChange"
      >
        <ion-radio
          class="item-radio radio-list-item"
          label-placement="end"
          justify="start"
          :value="DeviceSaveStrategyTag.Keyring"
          :disabled="!keyringAvailable || activeAuth === AvailableDeviceTypeTag.Keyring"
        >
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.Keyring"
            :state="getAuthCardState(AvailableDeviceTypeTag.Keyring)"
            :disabled="!keyringAvailable"
          />
        </ion-radio>
        <ion-radio
          class="item-radio radio-list-item"
          label-placement="end"
          justify="start"
          :value="DeviceSaveStrategyTag.Password"
        >
          <authentication-card
            @click="onMethodSelected(DeviceSaveStrategyTag.Password)"
            :state="getAuthCardState(AvailableDeviceTypeTag.Password)"
            :auth-method="DeviceSaveStrategyTag.Password"
          />
        </ion-radio>

        <ion-radio
          v-show="smartcardAvailable"
          class="item-radio radio-list-item"
          label-placement="end"
          justify="start"
          :value="DeviceSaveStrategyTag.Smartcard"
          :disabled="!smartcardAvailable || activeAuth === AvailableDeviceTypeTag.Smartcard"
        >
          <authentication-card
            @click="onMethodSelected(DeviceSaveStrategyTag.Smartcard)"
            :state="getAuthCardState(AvailableDeviceTypeTag.Smartcard)"
            :auth-method="DeviceSaveStrategyTag.Smartcard"
          />
        </ion-radio>

        <ion-radio
          v-show="isOpenBaoAvailable()"
          class="item-radio radio-list-item"
          label-placement="end"
          :value="DeviceSaveStrategyTag.OpenBao"
          justify="start"
        >
          <authentication-card
            :state="AuthenticationCardState.Default"
            @click="onMethodSelected(DeviceSaveStrategyTag.OpenBao)"
            :auth-method="DeviceSaveStrategyTag.OpenBao"
          />
        </ion-radio>
      </ion-radio-group>
    </div>

    <div
      class="choose-auth-choice"
      v-else
    >
      <div v-if="authentication === DeviceSaveStrategyTag.Keyring">
        <div class="method-chosen">
          <ion-text class="method-chosen__title subtitles-sm">{{ $msTranslate('Authentication.methodChosen') }}</ion-text>
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.Keyring"
            :state="AuthenticationCardState.Update"
            @update-clicked="changeAuthenticationMethod"
          />
        </div>
        <keyring-information />
      </div>

      <div v-if="authentication === DeviceSaveStrategyTag.Password">
        <div class="method-chosen">
          <ion-text class="method-chosen__title subtitles-sm">{{ $msTranslate('Authentication.methodChosen') }}</ion-text>
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.Password"
            :state="AuthenticationCardState.Update"
            @update-clicked="changeAuthenticationMethod"
          />
        </div>
        <ms-choose-password-input
          ref="choosePassword"
          class="choose-password"
          @on-enter-keyup="$emit('enterPressed')"
        />
      </div>

      <div v-if="authentication === DeviceSaveStrategyTag.Smartcard">
        <div class="method-chosen">
          <ion-text class="method-chosen__title subtitles-sm">{{ $msTranslate('Authentication.methodChosen') }}</ion-text>
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.Smartcard"
            :state="AuthenticationCardState.Update"
            @update-clicked="changeAuthenticationMethod"
          />
        </div>
        <choose-certificate ref="chooseCertificate" />
      </div>

      <div v-if="authentication === DeviceSaveStrategyTag.OpenBao">
        <div class="method-chosen">
          <ion-text class="method-chosen__title subtitles-sm">{{ $msTranslate('Authentication.methodChosen') }}</ion-text>
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.OpenBao"
            :state="AuthenticationCardState.Update"
            @update-clicked="changeAuthenticationMethod()"
          />
        </div>
        <sso-provider-card
          :provider="SSOProvider.ProConnect"
          @click="onSSOLoginClicked(SSOProvider.ProConnect)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ChooseCertificate from '@/components/devices/ChooseCertificate.vue';
import KeyringInformation from '@/components/devices/KeyringInformation.vue';
import SsoProviderCard from '@/components/devices/SsoProviderCard.vue';
import { SSOProvider } from '@/components/devices/types';
import authenticationCard from '@/components/profile/AuthenticationCard.vue';
import { AuthenticationCardState } from '@/components/profile/types';
import {
  AvailableDeviceTypeTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyTag,
  SaveStrategy,
  X509CertificateReference,
  isKeyringAvailable,
  isSmartcardAvailable,
  isWeb,
} from '@/parsec';
import { useOpenBao } from '@/services/openBao';
import { IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import { MsChoosePasswordInput } from 'megashark-lib';
import { onMounted, ref, toRaw, useTemplateRef } from 'vue';

const authentication = ref<DeviceSaveStrategyTag | undefined>(undefined);
const keyringAvailable = ref(false);
const choosePasswordRef = useTemplateRef<InstanceType<typeof MsChoosePasswordInput>>('choosePassword');
const chooseCertificateRef = useTemplateRef<InstanceType<typeof ChooseCertificate>>('chooseCertificate');
const { openBaoConnect, isOpenBaoAvailable } = useOpenBao();
const openBaoLoggedIn = ref(false);
const smartcardAvailable = ref(false);

const props = defineProps<{
  showTitle?: boolean;
  activeAuth?: AvailableDeviceTypeTag;
}>();

defineExpose({
  areFieldsCorrect,
  authentication,
  getSaveStrategy,
});

defineEmits<{
  (e: 'enterPressed'): void;
}>();

onMounted(async () => {
  keyringAvailable.value = props.activeAuth === AvailableDeviceTypeTag.Keyring ? true : await isKeyringAvailable();
  smartcardAvailable.value = await isSmartcardAvailable();
});

async function onChange(_value: any): Promise<void> {
  if (authentication.value === DeviceSaveStrategyTag.Password && choosePasswordRef.value) {
    await choosePasswordRef.value.setFocus();
  }
}

function getAuthCardState(auth: AvailableDeviceTypeTag): AuthenticationCardState {
  switch (auth) {
    case AvailableDeviceTypeTag.Password:
      return AuthenticationCardState.Default;
    case AvailableDeviceTypeTag.Keyring:
      if (!keyringAvailable.value) {
        return AuthenticationCardState.Unavailable;
      }
      return auth === props.activeAuth ? AuthenticationCardState.Active : AuthenticationCardState.Default;
    case AvailableDeviceTypeTag.Smartcard:
      if (!smartcardAvailable.value) {
        return AuthenticationCardState.Unavailable;
      }
      return auth === props.activeAuth ? AuthenticationCardState.Active : AuthenticationCardState.Default;
    default:
      return AuthenticationCardState.Default;
  }
}

async function onMethodSelected(method: DeviceSaveStrategyTag): Promise<void> {
  authentication.value = method;
}

async function changeAuthenticationMethod(): Promise<void> {
  authentication.value = undefined;
}

function getSaveStrategy(): DeviceSaveStrategy | undefined {
  if (authentication.value === DeviceSaveStrategyTag.Keyring) {
    return SaveStrategy.useKeyring();
  } else if (authentication.value === DeviceSaveStrategyTag.OpenBao) {
    return SaveStrategy.useOpenBao() as any as DeviceSaveStrategy;
  } else if (authentication.value === DeviceSaveStrategyTag.Smartcard) {
    if (chooseCertificateRef.value && chooseCertificateRef.value.getCertificate()) {
      return SaveStrategy.useSmartCard(toRaw(chooseCertificateRef.value.getCertificate() as X509CertificateReference));
    }
    return undefined;
  }
  if (choosePasswordRef.value?.password) {
    return SaveStrategy.usePassword(choosePasswordRef.value?.password);
  }
  return undefined;
}

async function areFieldsCorrect(): Promise<boolean> {
  if (keyringAvailable.value && authentication.value === DeviceSaveStrategyTag.Keyring) {
    return true;
  } else if (authentication.value === DeviceSaveStrategyTag.Password && choosePasswordRef.value) {
    return await choosePasswordRef.value.areFieldsCorrect();
  } else if (authentication.value === DeviceSaveStrategyTag.OpenBao && openBaoLoggedIn.value) {
    return true;
  } else if (authentication.value === DeviceSaveStrategyTag.Smartcard && chooseCertificateRef.value) {
    return chooseCertificateRef.value.getCertificate() !== undefined;
  }
  return false;
}

async function onSSOLoginClicked(_provider: SSOProvider): Promise<void> {
  const result = await openBaoConnect();
  if (!result.ok) {
    window.electronAPI.log('error', `Error while connecting with SSO: ${(result.error.errors as Array<string>)[0]}`);
  } else {
    openBaoLoggedIn.value = true;
  }
}
</script>

<style scoped lang="scss">
.choose-auth-list {
  padding: 0;

  &__label {
    color: var(--parsec-color-light-primary-700);
    margin-bottom: 1rem;
    display: block;
  }

  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  .item-radio {
    z-index: 2;

    &.radio-disabled {
      &::part(container) {
        opacity: 0.3;
      }

      &::part(label) {
        opacity: 1;
        display: flex;
        gap: 0.125rem;
      }
    }

    &::part(mark) {
      background: none;
      transition: none;
      transform: none;
    }

    &::part(label) {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      margin: 0;
      width: 100%;
    }

    &::part(container) {
      display: none;
    }
  }
}

.choose-auth-choice {
  .method-chosen {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1.5rem;

    &__title {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }

  .choose-password {
    padding: 1.5rem 1rem;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-12) 0 var(--parsec-radius-12) var(--parsec-radius-12);
    box-shadow: var(--parsec-shadow-soft);
    background: var(--parsec-color-light-secondary-inversed-contrast);

    @include ms.responsive-breakpoint('sm') {
      border-top: 1px solid var(--parsec-color-light-secondary-medium);
    }
  }
}

.choose-auth-page--web {
  padding: 0;

  .choose-password {
    border-radius: none;
    border: none;
    box-shadow: none;
    padding: 0 0.25rem;
    background: none;
  }
}
</style>
