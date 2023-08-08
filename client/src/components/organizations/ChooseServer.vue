<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="choose-server-page">
    <ion-text class="body choose-server-page__label">
      {{ $t('CreateOrganization.serverDescription') }}
    </ion-text>
    <ion-radio-group
      v-model="mode"
      class="radio-list"
    >
      <ion-item class="radio-list-item">
        <ion-radio
          class="item-radio"
          label-placement="end"
          justify="start"
          :value="ServerMode.SaaS"
        >
          <ion-text class="body-lg item-radio__label">
            {{ $t('CreateOrganization.useParsecServer') }}
          </ion-text>
          <ion-text class="body-sm item-radio__text">
            {{ $t('CreateOrganization.acceptTOS.label') }}
            <a
              class="link"
              target="_blank"
              :href="$t('CreateOrganization.acceptTOS.tosLink')"
            >
              {{ $t('CreateOrganization.acceptTOS.tos') }}
            </a>
            {{ $t('CreateOrganization.acceptTOS.and') }}
            <a
              class="link"
              target="_blank"
              :href="$t('CreateOrganization.acceptTOS.privacyPolicyLink')"
            >
              {{ $t('CreateOrganization.acceptTOS.privacyPolicy') }}
            </a>
          </ion-text>
        </ion-radio>
      </ion-item>

      <ion-item class="radio-list-item">
        <ion-radio
          class="item-radio"
          label-placement="end"
          justify="start"
          :value="ServerMode.Custom"
        >
          <ion-text class="body-lg item-radio__label">
            {{ $t('CreateOrganization.useMyOwnServer') }}
          </ion-text>
        </ion-radio>
      </ion-item>
    </ion-radio-group>
    <ms-input
      class="item-radio__input"
      :placeholder="$t('CreateOrganization.parsecServerUrl')"
      v-model="backendAddr"
      name="serverUrl"
      v-show="mode === ServerMode.Custom"
    />
  </ion-list>
</template>

<script lang="ts">
export enum ServerMode {
  SaaS ='saas',
  Custom = 'custom',
}
</script>

<script setup lang="ts">
import {
  IonList,
  IonRadio,
  IonRadioGroup,
  IonItem,
  IonText,
} from '@ionic/vue';
import { ref } from 'vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { backendAddrValidator, Validity } from '@/common/validators';

const backendAddr = ref('');
const mode = ref(ServerMode.SaaS);

defineExpose({
  areFieldsCorrect,
  ServerMode,
  mode,
  backendAddr,
});

function areFieldsCorrect(): boolean {
  return mode.value === ServerMode.SaaS || (mode.value === ServerMode.Custom && backendAddrValidator(backendAddr.value) === Validity.Valid);
}
</script>

<style scoped lang="scss">
.choose-server-page {
  &__label {
    color: var(--parsec-color-light-primary-700);
    margin-bottom: 1rem;
    display: block;
  }
}

.radio-list {
  display: flex;
  flex-direction: column;
}

.radio-list-item {
  display: flex;
  flex-direction: column;
  border-radius: var(--parsec-radius-6);
  width: 100%;
  --background-hover: var(--parsec-color-light-secondary-background);
  --background-hover-opacity: 1;
  z-index: 2;

  &:nth-child(2) {
    border-radius: var(--parsec-radius-6) var(--parsec-radius-6) 0 0;

    &:hover {
      border-radius: var(--parsec-radius-6);
    }
  }

  &:has(.radio-checked) {
    --background: var(--parsec-color-light-secondary-medium);
    --background-hover: none;
  }

  &:first-of-type {
    margin-bottom: .5rem;
  }

  .link {
    color: var(--parsec-color-light-primary-600);
    text-decoration: none;
    position: relative;
    z-index: 1000000;

    &:hover {
      text-decoration: underline;
    }
  }
}

.item-radio {
  gap: .5rem;
  padding: 1rem 0;
  width: 100%;

  &::part(mark) {
    background: none;
    transition: none;
    transform: none;
  }

  &__label, &__text{
    color: var(--parsec-color-light-secondary-grey);
    display: block;
    margin-left: 1rem;
    width: 100%;
  }
}

.item-radio.radio-checked {
  &::part(container) {
    border-color: var(--parsec-color-light-primary-600);
  }

  &::part(mark) {
    width: .85rem;
    height: .85rem;
    border: 1.5px solid var(--parsec-color-light-secondary-inversed-contrast);
    background: var(--parsec-color-light-primary-600);
    border-radius: var(--parsec-radius-circle);
  }

  .item-radio__label {
    color: var(--parsec-color-light-primary-600);
  }
}

.item-radio__input {
  background: var(--parsec-color-light-secondary-medium);
  padding: 0 1rem 1rem 1rem;
  border-radius: 0 0 var(--parsec-radius-6) var(--parsec-radius-6);
}
</style>
