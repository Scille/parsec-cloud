<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-list class="choose-server-page">
    <ion-text class="body orga-server__label">
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
            A remplacer
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
    <custom-input
      class="item-radio__input"
      :placeholder="$t('CreateOrganization.parsecServerUrl')"
      v-model="backendAddr"
      name="serverUrl"
      v-show="mode === ServerMode.Custom"
    />
  </ion-list>
</template>

<script setup lang="ts">
import {
  IonList,
  IonRadio,
  IonRadioGroup,
  IonItem,
  IonText,
} from '@ionic/vue';
import { ref } from 'vue';
import CustomInput from '@/components/CustomInput.vue';
import { backendAddrValidator, Validity } from '@/common/validators';

enum ServerMode {
  SaaS ='saas',
  Custom = 'custom',
}

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
  --background-hover: var(--parsec-color-light-secondary-background);;
  --background-hover-opacity: 1;

  &:has(.radio-checked) {
    --background: var(--parsec-color-light-secondary-background);
  }

  &:first-of-type {
    margin-bottom: .5rem;
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
</style>
