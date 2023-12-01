<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container-textinfo">
    <ion-icon
      :icon="icon ?? caretForwardCircle"
      size="default"
      class="container-textinfo__icon"
    />
    <ion-text class="subtitles-normal container-textinfo__text">
      <slot />
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import { caretForwardCircle } from 'ionicons/icons';

defineProps<{
  icon?: string;
}>();
</script>

<style scoped lang="scss">
.container-textinfo {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 1rem;
  background-color: none;
  max-width: 37.5rem;

  &__icon {
    color: var(--parsec-color-light-primary-600);
    font-size: 1.25rem;
    min-width: 1.25rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
