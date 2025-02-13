<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="file-viewer-background">
    <img src="@/assets/images/background/background-fileviewer.svg" />
    <ion-icon
      v-if="icon"
      :icon="icon"
      class="file-viewer-background-icon"
    />
  </div>
</template>

<script setup lang="ts">
import { IonIcon } from '@ionic/vue';

defineProps<{
  icon?: string;
}>();
</script>

<style scoped lang="scss">
.file-viewer-background {
  height: 100%;
  width: 100%;
  max-height: 37.5rem;
  max-width: 75rem;
  padding: 2rem;
  display: flex;

  img {
    height: 100%;
    margin: auto;
    width: 100%;
    border-radius: var(--parsec-radius-12);
    overflow: hidden;
    box-shadow: var(--parsec-shadow-light);
    object-fit: cover;
  }

  &-icon {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 7vw;
    max-width: 5rem;
    max-height: 5rem;
    min-width: 3rem;
    min-height: 3rem;
    padding: 1vw;
    background: var(--parsec-color-light-primary-30-opacity15);
    border-radius: var(--parsec-radius-12);
    color: var(--parsec-color-light-secondary-white);
    box-shadow: var(--parsec-shadow-3d-strong);
    backdrop-filter: blur(25px);
  }
}
</style>
