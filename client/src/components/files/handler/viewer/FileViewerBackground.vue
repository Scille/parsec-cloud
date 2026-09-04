<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="file-viewer-background">
    <img :src="isDarkTheme ? BackgroundFileViewerDark : BackgroundFileViewer" />
    <ion-icon
      v-if="icon"
      :icon="icon"
      class="file-viewer-background-icon"
    />
  </div>
</template>

<script setup lang="ts">
import BackgroundFileViewerDark from '@/assets/images/background/background-fileviewer-dark.svg';
import BackgroundFileViewer from '@/assets/images/background/background-fileviewer.svg';
import { IonIcon } from '@ionic/vue';
import { useTheme } from 'megashark-lib';

const { isDarkTheme } = useTheme();

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
  padding: ms.spacing('padding-5xl');
  display: flex;

  img {
    height: 100%;
    margin: auto;
    width: 100%;
    border-radius: ms.radius('2xl');
    overflow: hidden;
    box-shadow: ms.shadow('light');
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
    border-radius: ms.radius('2xl');
    color: ms.color('icon-on-color-heading');
    box-shadow: ms.shadow('3d-strong');
    backdrop-filter: blur(25px);
  }
}
</style>
