<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    class="file-controls-button button-medium"
    size="default"
  >
    <ion-icon
      class="file-controls-button-icon"
      v-if="icon"
      slot="start"
      :icon="icon"
    />
    <ms-image
      class="file-controls-button-icon"
      v-if="!icon && image"
      :image="image"
    />
    <span
      class="file-controls-button-label"
      v-if="label"
    >
      {{ $msTranslate(label) }}
    </span>
  </ion-button>
</template>

<script setup lang="ts">
import { MsImage, Translatable } from 'megashark-lib';
import { IonButton, IonIcon } from '@ionic/vue';

defineProps<{
  label?: Translatable;
  icon?: string;
  image?: string;
}>();
</script>

<style lang="scss" scoped>
.file-controls-button {
  --background: none !important;
  --background-hover: none !important;
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;
  --padding-top: 0;
  --padding-end: 0;
  --padding-bottom: 0;
  --padding-start: 0;
  height: 3em;
  width: fit-content;
  border-radius: 100%;
  color: var(--parsec-color-light-primary-600);
  opacity: 0.6;
  scale: 1;
  transition: all 0.2s ease-in-out;

  &.button-disabled {
    opacity: 0.3;
  }

  &:hover {
    scale: 1.1;
    opacity: 1;
  }

  &-icon,
  &-label {
    margin-inline: 0.625rem;
  }
}
</style>
