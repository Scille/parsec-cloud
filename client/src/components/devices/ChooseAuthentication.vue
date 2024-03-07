<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="choose-auth-page">
    <ion-text
      class="body choose-auth-page__label"
      v-show="showTitle"
    >
      {{ $t('Authentication.description') }}
    </ion-text>
    <ion-radio-group
      v-model="authentication"
      class="radio-list"
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
        <ion-text class="body-lg item-radio__label">
          {{ $t('Authentication.useKeyring') }}
        </ion-text>
        <ion-text
          v-show="!keyringAvailable && isDesktop()"
          class="body-sm item-radio__text"
        >
          {{ $t('Authentication.keyringUnavailableOnSystem') }}
        </ion-text>
        <ion-text
          v-show="!keyringAvailable && isWeb()"
          class="body-sm item-radio__text"
        >
          {{ $t('Authentication.keyringUnavailableOnWeb') }}
        </ion-text>
      </ion-radio>
      <ion-radio
        class="item-radio radio-list-item"
        label-placement="end"
        justify="start"
        :value="DeviceSaveStrategyTag.Password"
      >
        <ion-text class="body-lg item-radio__label">
          {{ $t('Authentication.usePassword') }}
        </ion-text>
      </ion-radio>
    </ion-radio-group>
    <div>
      <keyring-information v-show="authentication === DeviceSaveStrategyTag.Keyring" />
    </div>
    <div v-show="authentication === DeviceSaveStrategyTag.Password">
      <ms-choose-password-input ref="choosePassword" />
    </div>
  </ion-list>
</template>

<script setup lang="ts">
import { MsChoosePasswordInput } from '@/components/core';
import KeyringInformation from '@/components/devices/KeyringInformation.vue';
import { DeviceSaveStrategy, DeviceSaveStrategyTag, SaveStrategy, isDesktop, isKeyringAvailable, isWeb } from '@/parsec';
import { IonList, IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import { onMounted, ref } from 'vue';

const authentication = ref(DeviceSaveStrategyTag.Keyring);
const keyringAvailable = ref(false);
const choosePassword = ref();

defineProps<{
  showTitle?: boolean;
}>();

defineExpose({
  areFieldsCorrect,
  authentication,
  getSaveStrategy,
});

defineEmits<{
  (e: 'fieldUpdate'): void;
}>();

onMounted(async () => {
  keyringAvailable.value = await isKeyringAvailable();

  if (!keyringAvailable.value) {
    authentication.value = DeviceSaveStrategyTag.Password;
  }
});

async function onChange(_value: any): Promise<void> {
  if (authentication.value === DeviceSaveStrategyTag.Password && choosePassword.value) {
    await choosePassword.value.setFocus();
  }
}

function getSaveStrategy(): DeviceSaveStrategy {
  if (authentication.value === DeviceSaveStrategyTag.Keyring) {
    return SaveStrategy.useKeyring();
  }
  return SaveStrategy.usePassword(choosePassword.value.password);
}

async function areFieldsCorrect(): Promise<boolean> {
  if (keyringAvailable.value && authentication.value === DeviceSaveStrategyTag.Keyring) {
    return true;
  } else if (authentication.value === DeviceSaveStrategyTag.Password && choosePassword.value) {
    return await choosePassword.value.areFieldsCorrect();
  }
  return false;
}
</script>

<style scoped lang="scss">
.choose-auth-page {
  &__label {
    color: var(--parsec-color-light-primary-700);
    margin-bottom: 1rem;
    display: block;
  }
  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  .choose-password {
    padding: 0.75rem;
  }
}

.radio-list {
  display: flex;
  flex-direction: column;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.radio-list-item {
  display: flex;
  flex-direction: column;
  border-radius: var(--parsec-radius-6);
  width: 100%;
  z-index: 2;
  &:hover {
    background: var(--parsec-color-light-secondary-background);
  }

  &:nth-child(2) {
    border-radius: var(--parsec-radius-6) var(--parsec-radius-6);

    &:hover {
      border-radius: var(--parsec-radius-6);
    }
  }

  &.radio-checked {
    background: var(--parsec-color-light-secondary-medium);
  }

  &:first-of-type {
    margin-bottom: 0.5rem;
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.item-radio {
  gap: 0.5rem;
  padding: 1em;
  width: 100%;

  &::part(mark) {
    background: none;
    transition: none;
    transform: none;
  }

  &__label,
  &__text {
    color: var(--parsec-color-light-secondary-grey);
    display: block;
    margin-left: 1rem;
    width: 100%;
  }

  &__text {
    display: flex;
    align-items: center;
    background: var(--parsec-color-light-secondary-text);
    color: var(--parsec-color-light-secondary-white);
    padding: 0.125rem 0.25rem;
    border-radius: var(--parsec-radius-4);
    width: fit-content;
    margin-left: auto;
  }

  &.radio-disabled {
    &::part(container) {
      opacity: 0.3;
    }
    &::part(label) {
      opacity: 1;
      display: flex;
      align-items: center;
      gap: 1rem;
    }
    .item-radio__label {
      opacity: 0.5;
    }
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.item-radio.radio-checked {
  &::part(container) {
    border-color: var(--parsec-color-light-primary-600);
  }

  &::part(mark) {
    width: 0.85rem;
    height: 0.85rem;
    border: 1.5px solid var(--parsec-color-light-secondary-inversed-contrast);
    background: var(--parsec-color-light-primary-600);
    border-radius: var(--parsec-radius-circle);
  }

  .item-radio__label {
    color: var(--parsec-color-light-primary-600);
  }
}
</style>
