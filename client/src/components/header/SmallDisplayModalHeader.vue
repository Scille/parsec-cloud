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
    <ion-text class="modal-header__title title-h3">
      {{ $msTranslate(title) }}
    </ion-text>
    <ion-text
      class="modal-header__subtitle body-lg"
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

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem;
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  }

  &__title {
    color: var(--parsec-color-light-primary-800);
    margin-bottom: 0 !important;
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-soft-text);
  }
}
</style>
