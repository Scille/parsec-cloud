<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container">
    <div class="container-logo">
      <ms-image
        :image="LogoIconGradient"
        class="image"
      />
    </div>
    <ion-text class="subtitles-normal container-text">
      {{ title }}
    </ion-text>
    <ion-spinner
      name="crescent"
      class="container-spinner"
    />
  </div>
</template>

<script setup lang="ts">
import { LogoIconGradient, MsImage } from '@/components/core/ms-image';
import { IonSpinner, IonText } from '@ionic/vue';

defineProps<{
  title: string;
}>();
</script>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  overflow: hidden;
  gap: 1.5rem;

  &-logo {
    max-width: 3.625rem;

    .image {
      width: 100%;
      height: auto;
    }
  }

  &-text {
    color: var(--parsec-color-light-secondary-grey);
    display: block;
    width: 100%;
  }

  &-spinner {
    --color: var(--parsec-color-light-primary-500);
    position: relative;
  }
}
</style>
