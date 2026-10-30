<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <button
    class="sso-provider-card"
    :class="{ 'sso-provider-card--connected': isConnected }"
    @click="$emit('ssoSelected', provider)"
    :disabled="isConnected"
  >
    <img
      v-if="provider.icon"
      :src="provider.icon"
      class="sso-provider-card__logo"
      alt=""
    />
    <ion-text class="button-large sso-provider-name">
      {{ $msTranslate({ key: 'Authentication.method.sso.continueWith', data: { name: provider.name } }) }}
    </ion-text>
    <div
      class="sso-provider-tag"
      v-if="isConnected"
    >
      <ion-icon
        :icon="checkmarkCircle"
        class="sso-provider-tag__icon"
      />
      <ion-text class="button-medium sso-provider-tag__text">
        {{ $msTranslate('Authentication.method.sso.connected') }}
      </ion-text>
    </div>
  </button>
</template>

<script setup lang="ts">
import { OpenBaoAuthConfig } from '@/parsec';
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';

defineEmits<{
  (e: 'ssoSelected', provider: OpenBaoAuthConfig): void;
}>();

defineProps<{
  provider: OpenBaoAuthConfig;
  isConnected?: boolean;
}>();
</script>

<style scoped lang="scss">
.sso-provider-card {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  min-height: 3rem;
  padding: 0.625rem 1rem;
  background: var(--parsec-color-light-secondary-premiere);
  border: 1px solid var(--parsec-color-light-secondary-disabled);
  border-radius: var(--parsec-radius-18);
  box-shadow: var(--parsec-shadow-input);
  color: var(--parsec-color-light-secondary-text);
  text-align: left;
  cursor: pointer;
  transition:
    background 150ms ease-in-out,
    border-color 150ms ease-in-out;

  &:hover {
    background: var(--parsec-color-light-secondary-medium);
    border-color: var(--parsec-color-light-secondary-light);
  }

  &:focus-visible {
    outline: 2px solid var(--parsec-color-light-primary-500);
    outline-offset: 2px;
  }

  &__logo {
    width: 1.75rem;
    height: 1.75rem;
    object-fit: contain;
    flex-shrink: 0;
  }

  &--connected {
    background: var(--parsec-color-light-success-50);
    border-color: var(--parsec-color-light-success-500);
    cursor: default;

    &:hover {
      background: var(--parsec-color-light-success-50);
      border-color: var(--parsec-color-light-success-500);
    }
  }
}

.sso-provider-tag {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: 0.375rem;

  &__icon {
    font-size: 1rem;
    color: var(--parsec-color-light-success-700);
  }

  &__text {
    color: var(--parsec-color-light-success-700);
  }
}
</style>
