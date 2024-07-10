<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-buttons
    slot="end"
    class="closeBtn-container"
  >
    <ion-button
      slot="icon-only"
      class="closeBtn"
      v-show="!hideCloseButton"
      @click="$emit('closeClicked')"
    >
      <ion-icon
        :icon="close"
        size="large"
        class="closeBtn__icon"
      />
    </ion-button>
  </ion-buttons>
  <ion-header class="modal-header">
    <ion-title class="modal-header__title title-h2">
      {{ $msTranslate(title) }}
    </ion-title>
    <ion-text class="modal-header__text body-lg">
      {{ $msTranslate(subtitle) }}
    </ion-text>
  </ion-header>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonHeader, IonIcon, IonTitle, IonText } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  subtitle: Translatable;
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
  padding-bottom: 0.5rem;

  &__title {
    padding: 0;
    margin-bottom: 1rem;
    color: var(--parsec-color-light-primary-800);
    display: flex;
    align-items: center;
    max-width: 22rem;

    &-container {
      display: flex;
      align-items: center;
    }

    &-icon {
      color: var(--parsec-color-light-primary-600);
      margin-right: 4px;
    }
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
    padding-bottom: 1.5rem;
  }
}
</style>
