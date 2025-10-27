<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    class="file-controls-button-container button-medium"
    fill="clear"
  >
    <div class="file-controls-button">
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
    </div>
  </ion-button>
</template>

<script setup lang="ts">
import { IonButton, IonIcon } from '@ionic/vue';
import { MsImage, Translatable } from 'megashark-lib';

defineProps<{
  label?: Translatable;
  icon?: string;
  image?: string;
}>();
</script>

<style lang="scss" scoped>
.file-controls-button-container {
  --background: none;
  --background-hover: none;
  border-radius: var(--parsec-radius-6);
  min-height: 0;
  transition: all 0.2s ease-in-out;
  height: fit-content;

  &::part(native) {
    margin: 0;
    padding: 0;
  }

  .file-controls-button {
    display: flex;
    align-items: center;
    gap: 0.5em;
    height: 100%;
    width: 100%;
    color: var(--parsec-color-light-secondary-text);
    padding: 0.25rem;

    &-icon {
      font-size: 1.25rem;
      width: 1.25rem;
      height: 1.25rem;
      display: flex;
      flex-grow: 1;
      --fill-color: var(--parsec-color-light-secondary-text);
    }
  }

  &.button-disabled {
    opacity: 0.3;
  }

  &:hover {
    background: var(--parsec-color-light-secondary-background);
    box-shadow: var(--parsec-shadow-soft);
    color: var(--parsec-color-light-primary-400);

    .file-controls-button-icon {
      color: var(--parsec-color-light-primary-400);
      --fill-color: var(--parsec-color-light-primary-400);
    }

    .file-controls-button-label {
      color: var(--parsec-color-light-primary-400);
    }
  }

  &:active {
    box-shadow: none;
  }
}
</style>
