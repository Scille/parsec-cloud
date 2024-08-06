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
    <ion-text class="modal-header__title title-h2">
      {{ $msTranslate(title) }}
    </ion-text>
    <ion-text
      class="modal-header__text subtitles-normal"
      v-if="subtitle"
    >
      {{ $msTranslate(subtitle) }}
    </ion-text>
  </ion-header>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonHeader, IonIcon, IonText } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  subtitle?: Translatable;
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
  margin-bottom: 2rem;
  gap: 1rem;

  &__title {
    color: var(--parsec-color-light-primary-800);
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
