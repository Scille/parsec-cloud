<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container-textinfo">
    <ion-icon
      :icon="icon"
      size="default"
      class="container-textinfo__icon"
    />
    <ion-text
      class="subtitles-normal container-textinfo__text"
    >
      {{ text }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';

defineProps<{
  icon: string,
  text: string
}>();
</script>

<style scoped lang="scss">
.container-textinfo {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 1rem;

  &__icon {
    color: var(--parsec-color-light-primary-400);
    font-size: 1.25rem;
    min-width: 1.25rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
