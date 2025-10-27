<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="choose-server-page">
    <ion-text class="body choose-server-page__label">
      {{ $msTranslate('CreateOrganization.serverDescription') }}
    </ion-text>
    <ion-radio-group
      v-model="mode"
      class="radio-list"
      @ion-change="$emit('fieldUpdate')"
    >
      <ion-radio
        class="item-radio radio-list-item"
        label-placement="end"
        justify="start"
        :value="ServerMode.SaaS"
        @click="$event.preventDefault()"
      >
        <ion-text class="body-lg item-radio__label">
          {{ $msTranslate('CreateOrganization.useParsecServer') }}
        </ion-text>
        <ion-text class="body-sm item-radio__text ion-text-wrap">
          {{ $msTranslate('CreateOrganization.acceptTOS.label') }}
          <a
            class="link"
            target="_blank"
            @click="$event.stopPropagation()"
            :href="$msTranslate('CreateOrganization.acceptTOS.tosLink')"
          >
            {{ $msTranslate('CreateOrganization.acceptTOS.tos') }}
          </a>
          {{ $msTranslate('CreateOrganization.acceptTOS.and') }}
          <a
            class="link"
            target="_blank"
            @click="$event.stopPropagation()"
            :href="$msTranslate('CreateOrganization.acceptTOS.privacyPolicyLink')"
          >
            {{ $msTranslate('CreateOrganization.acceptTOS.privacyPolicy') }}
          </a>
        </ion-text>
      </ion-radio>
      <ion-radio
        class="item-radio radio-list-item"
        label-placement="end"
        justify="start"
        :value="ServerMode.Custom"
      >
        <ion-text class="body-lg item-radio__label">
          {{ $msTranslate('CreateOrganization.useMyOwnServer') }}
        </ion-text>
      </ion-radio>
    </ion-radio-group>
    <ms-input
      class="item-radio__input"
      :placeholder="'CreateOrganization.parsecServerUrl'"
      :validator="parsecAddrValidator"
      v-model="serverAddr"
      name="serverUrl"
      v-show="mode === ServerMode.Custom"
      @change="$emit('fieldUpdate')"
    />
  </ion-list>
</template>

<script lang="ts">
export enum ServerMode {
  SaaS = 'saas',
  Custom = 'custom',
}
</script>

<script setup lang="ts">
import { parsecAddrValidator } from '@/common/validators';
import { IonList, IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import { MsInput, Validity } from 'megashark-lib';
import { ref } from 'vue';

const serverAddr = ref('');
const mode = ref(ServerMode.SaaS);

defineExpose({
  areFieldsCorrect,
  ServerMode,
  mode,
  serverAddr,
});

defineEmits<{
  (e: 'fieldUpdate'): void;
}>();

async function areFieldsCorrect(): Promise<boolean> {
  return (
    mode.value === ServerMode.SaaS ||
    (mode.value === ServerMode.Custom && (await parsecAddrValidator(serverAddr.value)).validity === Validity.Valid)
  );
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

  &.radio-checked {
    background: var(--parsec-color-light-secondary-premiere);
  }

  &:first-of-type {
    margin-bottom: 0.5rem;
  }

  .link {
    color: var(--parsec-color-light-primary-600);
    text-decoration: none;
    position: relative;
    pointer-events: initial;
    z-index: 1000000;

    &:hover {
      text-decoration: underline;
    }
  }
}

.item-radio {
  gap: 0.5rem;
  padding: 0.5rem 1rem;
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
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.item-radio.radio-checked {
  &::part(container) {
    border-color: var(--parsec-color-light-primary-600);
  }

  &::part(mark) {
    width: 0.75rem;
    height: 0.75rem;
    border: 1.5px solid var(--parsec-color-light-secondary-inversed-contrast);
    background: var(--parsec-color-light-primary-600);
    border-radius: var(--parsec-radius-circle);
  }

  .item-radio__label {
    color: var(--parsec-color-light-primary-600);
  }
}

.item-radio__input {
  background: var(--parsec-color-light-secondary-white);
  padding: 1rem;
}
</style>
