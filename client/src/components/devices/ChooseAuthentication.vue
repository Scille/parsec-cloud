<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="choose-auth-page"
    :class="isWeb() ? 'choose-auth-page--web' : ''"
  >
    <ion-list
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
          @click="$event.preventDefault()"
          :disabled="!keyringAvailable"
        >
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.Keyring"
            :state="!keyringAvailable ? AuthenticationCardState.Unavailable : AuthenticationCardState.Default"
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
            :state="AuthenticationCardState.Default"
            :auth-method="DeviceSaveStrategyTag.Password"
          />
        </ion-radio>

        <!-- TO DO: uncomment for Smartcard -->
        <!-- <ion-radio
          v-if="false"
          class="item-radio radio-list-item"
          label-placement="end"
          justify="start"
        >
          <authentication-card
            @click="onMethodSelected(DeviceSaveStrategyTag.Smartcard)"
            :state="AuthenticationCardState.Default"
            :auth-method="DeviceSaveStrategyTag.Smartcard"
          />
        </ion-radio> -->

        <!-- TO DO: uncomment for SSO -->
        <!-- <ion-radio
          v-if="false"
          class="item-radio radio-list-item"
          label-placement="end"
          :value="DeviceSaveStrategyTag.Sso"
          justify="start"
        >
          <authentication-card
            :state="AuthenticationCardState.Default"
            @click="onMethodSelected(DeviceSaveStrategyTag.Sso)"
            :auth-method="DeviceSaveStrategyTag.Sso"
          />
        </ion-radio> -->
      </ion-radio-group>
    </ion-list>

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

      <!-- TO DO: uncomment for Smartcard -->
      <!-- <div v-if="authentication === DeviceSaveStrategyTag.Smartcard">
        <div class="method-chosen">
          <ion-text class="method-chosen__title subtitles-sm">{{ $msTranslate('Authentication.methodChosen') }}</ion-text>
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.Smartcard"
            :state="AuthenticationCardState.Update"
            @update-clicked="changeAuthenticationMethod"
          />
        </div>
      </div> -->

      <!-- TO DO: uncomment for SSO -->
      <!-- <div v-if="authentication === DeviceSaveStrategyTag.Sso">
        <div class="method-chosen">
          <ion-text class="method-chosen__title subtitles-sm">{{ $msTranslate('Authentication.methodChosen') }}</ion-text>
          <authentication-card
            :auth-method="DeviceSaveStrategyTag.Sso"
            :state="AuthenticationCardState.Update"
            @update-clicked="changeAuthenticationMethod()"
          />
        </div>
      </div> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { MsChoosePasswordInput } from 'megashark-lib';
import KeyringInformation from '@/components/devices/KeyringInformation.vue';
import { DeviceSaveStrategy, DeviceSaveStrategyTag, SaveStrategy, isKeyringAvailable, isWeb } from '@/parsec';
import { IonList, IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import { onMounted, ref, useTemplateRef } from 'vue';
import authenticationCard from '@/components/profile/AuthenticationCard.vue';
import { AuthenticationCardState } from '@/components/profile/types';

const authentication = ref<DeviceSaveStrategyTag | undefined>(undefined);
const keyringAvailable = ref(false);
const choosePasswordRef = useTemplateRef<InstanceType<typeof MsChoosePasswordInput>>('choosePassword');

const props = defineProps<{
  showTitle?: boolean;
  disableKeyring?: boolean;
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
  if (!props.disableKeyring) {
    keyringAvailable.value = await isKeyringAvailable();
  }
});

async function onChange(_value: any): Promise<void> {
  if (authentication.value === DeviceSaveStrategyTag.Password && choosePasswordRef.value) {
    await choosePasswordRef.value.setFocus();
  }
}

async function onMethodSelected(method: DeviceSaveStrategyTag): Promise<void> {
  authentication.value = method;
}

async function changeAuthenticationMethod(): Promise<void> {
  authentication.value = undefined;
}

function getSaveStrategy(): DeviceSaveStrategy {
  if (authentication.value === DeviceSaveStrategyTag.Keyring) {
    return SaveStrategy.useKeyring();
  }
  return SaveStrategy.usePassword(choosePasswordRef.value?.password || '');
}

async function areFieldsCorrect(): Promise<boolean> {
  if (keyringAvailable.value && authentication.value === DeviceSaveStrategyTag.Keyring) {
    return true;
  } else if (authentication.value === DeviceSaveStrategyTag.Password && choosePasswordRef.value) {
    return await choosePasswordRef.value.areFieldsCorrect();
  }
  return false;
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
