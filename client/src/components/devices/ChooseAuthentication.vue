<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="choose-auth-page">
    <ion-text
      class="body choose-auth-page__label"
      v-show="showTitle"
    >
      {{ $msTranslate('Authentication.description') }}
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
        :disabled="!keyringAvailable || disableKeyring"
      >
        <ion-text
          v-show="!keyringAvailable && isDesktop()"
          class="body-sm item-radio__text"
        >
          {{ $msTranslate('Authentication.keyringUnavailableOnSystem') }}
        </ion-text>
        <ion-text
          v-show="!keyringAvailable && isWeb()"
          class="body-sm item-radio__text"
        >
          {{ $msTranslate('Authentication.keyringUnavailableOnWeb') }}
        </ion-text>
        <ion-text class="body-lg item-radio__label">
          {{ $msTranslate('Authentication.useKeyring') }}
        </ion-text>
      </ion-radio>
      <ion-radio
        class="item-radio radio-list-item"
        label-placement="end"
        justify="start"
        :value="DeviceSaveStrategyTag.Password"
      >
        <ion-text class="body-lg item-radio__label">
          {{ $msTranslate('Authentication.usePassword') }}
        </ion-text>
      </ion-radio>
    </ion-radio-group>
    <div>
      <keyring-information v-show="authentication === DeviceSaveStrategyTag.Keyring" />
    </div>
    <div v-show="authentication === DeviceSaveStrategyTag.Password">
      <ms-choose-password-input
        ref="choosePassword"
        class="choose-password"
        @on-enter-keyup="$emit('enterPressed')"
      />
    </div>
  </ion-list>
</template>

<script setup lang="ts">
import { MsChoosePasswordInput } from 'megashark-lib';
import KeyringInformation from '@/components/devices/KeyringInformation.vue';
import { DeviceSaveStrategy, DeviceSaveStrategyTag, SaveStrategy, isDesktop, isKeyringAvailable, isWeb } from '@/parsec';
import { IonList, IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import { onMounted, ref } from 'vue';

const authentication = ref(DeviceSaveStrategyTag.Keyring);
const keyringAvailable = ref(false);
const choosePassword = ref();

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
  keyringAvailable.value = await isKeyringAvailable();

  if (!keyringAvailable.value || props.disableKeyring) {
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
    padding: 1.5rem 1rem;
  }
}

.radio-list {
  gap: 1rem;
}

.choose-password {
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12) 0 var(--parsec-radius-12) var(--parsec-radius-12);
  box-shadow: var(--parsec-shadow-soft);
  background: var(--parsec-color-light-secondary-inversed-contrast);

  @include ms.responsive-breakpoint('sm') {
    border-top: 1px solid var(--parsec-color-light-secondary-medium);
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.item-radio {
  gap: 0.5rem;
  padding: 1rem;
  width: 50%;
  border-radius: var(--parsec-radius-12);
  z-index: 2;

  @include ms.responsive-breakpoint('sm') {
    padding: 0.75rem 1rem;
    border-radius: var(--parsec-radius-12);
  }

  &:hover {
    background: var(--parsec-color-light-secondary-background);
  }

  &__label,
  &__text {
    color: var(--parsec-color-light-secondary-grey);
    display: block;
    width: 100%;
  }

  &__text {
    display: flex;
    color: var(--parsec-color-light-danger-700);
    padding: 0.125rem 0;
    border-radius: var(--parsec-radius-4);
    width: fit-content;
    position: absolute;
    top: -1.25rem;
  }

  &.radio-checked {
    background: var(--parsec-color-light-secondary-premiere);
  }

  &.radio-disabled {
    &::part(container) {
      opacity: 0.3;
    }

    &::part(label) {
      opacity: 1;
      display: flex;
      gap: 0.125rem;
    }

    .item-radio__label {
      opacity: 0.5;
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

    @include ms.responsive-breakpoint('sm') {
      margin: 0;
    }
  }

  &::part(container) {
    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }

  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  &.radio-checked {
    border-radius: var(--parsec-radius-12) var(--parsec-radius-12) 0 0;

    @include ms.responsive-breakpoint('sm') {
      border: 2px solid var(--parsec-color-light-secondary-medium);
      border-bottom: none;
    }

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
}
</style>
