<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    button
    class="download-parsec"
  >
    <div
      button
      class="download-parsec-content"
      @click="Env.Links.openDownloadParsecLink"
    >
      <ms-image
        :image="LogoParsecApp"
        class="download-parsec-icon"
      />
      <ion-text class="body download-parsec-text"> {{ $msTranslate('HomePage.topbar.downloadParsec') }} </ion-text>
    </div>
    <ion-icon
      :icon="close"
      slot="end"
      class="download-parsec-close"
      @click="$emit('hideParsecDownload')"
    />
  </div>
</template>

<script setup lang="ts">
import LogoParsecApp from '@/assets/images/logo-parsec-app.svg?raw';
import { Env } from '@/services/environment';
import { IonIcon, IonText } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { MsImage } from 'megashark-lib';

defineEmits<{
  (e: 'hideParsecDownload'): void;
}>();
</script>

<style lang="scss" scoped>
.download-parsec {
  cursor: pointer;
  display: flex;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  padding: 0;
  border-radius: var(--parsec-radius-8);
  overflow: hidden;
  flex-shrink: 0;
  position: relative;

  &-content {
    background: var(--parsec-color-light-secondary-premiere);
    align-items: center;
    padding: 0.5rem;
    display: flex;
    gap: 0.5rem;
    width: 100%;
    transition: all 0.2s ease-in-out;

    &:hover {
      background: var(--parsec-color-light-secondary-medium);
    }
  }

  &-text {
    font-weight: 600;
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: var(--parsec-color-light-gradient-background);
  }

  &-icon {
    width: 1.5rem;
    height: 1.5rem;
    --fill-color: var(--parsec-color-light-secondary-white);
  }

  &-close {
    font-size: 1.125rem;
    background: var(--parsec-color-light-secondary-premiere);
    color: var(--parsec-color-light-secondary-grey);
    opacity: 0.8;
    right: 0;
    height: 2rem;
    display: none;
    position: absolute;
    padding: 0.25rem;
    flex-shrink: 0;
    margin-inline: auto 0;
    border-left: 1px solid var(--parsec-color-light-secondary-disabled);

    &:hover {
      background: var(--parsec-color-light-secondary-medium);
    }
  }

  &:hover .download-parsec-close {
    display: block;
  }
}
</style>
