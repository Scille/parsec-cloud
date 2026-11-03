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
    <ion-text class="sso-provider-name">
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
      <ion-text class="sso-provider-tag__text">
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
  gap: ms.spacing('gap-2xl');
  width: 100%;
  min-height: 3rem;
  padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
  background: ms.color('surface-elevated-default');
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('4xl');
  box-shadow: ms.shadow('elevation-sm');
  color: ms.color('text-base-body');
  text-align: left;
  cursor: pointer;
  transition:
    background 150ms ease-in-out,
    border-color 150ms ease-in-out;

  &:hover {
    background: ms.color('surface-elevated-default-hover');
    border-color: ms.color('border-base-default-hover');
  }

  &:focus-visible {
    outline: ms.border('thick') solid ms.color('border-brand-default');
    outline-offset: 2px;
  }

  &__logo {
    width: 1.75rem;
    height: 1.75rem;
    object-fit: contain;
    flex-shrink: 0;
  }

  &--connected {
    background: ms.color('surface-success-default-subtle');
    border-color: ms.color('border-success-default');
    cursor: default;

    &:hover {
      background: ms.color('surface-success-default-subtle');
      border-color: ms.color('border-success-default');
    }
  }
}

.sso-provider-tag {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-md');

  &__icon {
    font-size: 1rem;
    color: ms.color('icon-success-default');
  }

  &__text {
    color: ms.color('text-success-default');
  }
}
</style>
