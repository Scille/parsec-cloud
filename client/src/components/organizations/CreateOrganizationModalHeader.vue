<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-if="!hideCloseButton"
    slot="icon-only"
    class="closeBtn"
    @click="$emit('closeClicked')"
  >
    <ion-icon
      :icon="close"
      class="closeBtn__icon"
    />
  </ion-button>
  <ion-header class="modal-header">
    <div class="modal-header-title">
      <ion-icon
        v-if="icon"
        :icon="icon"
        class="modal-header-title__icon"
      />
      <ion-text class="modal-header-title__text title-h2">
        {{ $msTranslate(title) }}
      </ion-text>
    </div>
    <ion-text
      class="modal-header__text body-lg"
      v-if="subtitle"
    >
      {{ $msTranslate(subtitle) }}
    </ion-text>
  </ion-header>
</template>

<script setup lang="ts">
import { IonButton, IonHeader, IonIcon, IonText } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  subtitle?: Translatable;
  icon?: string;
  hideCloseButton?: boolean;
}>();

defineEmits<{
  (e: 'closeClicked'): void;
}>();
</script>

<style scoped lang="scss">
.modal-header {
  display: flex;
  flex-direction: column;
  text-wrap: wrap;
  margin-bottom: 1.5rem;
  gap: 1rem;

  &-title {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &__icon {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-secondary-white);
      border-radius: var(--parsec-radius-6);
      font-size: 1.5rem;
      padding: 0.25rem;
    }

    &__text {
      color: var(--parsec-color-light-primary-800);
    }
  }

  &__text {
    color: var(--parsec-color-light-secondary-soft-text);
  }
}
</style>
