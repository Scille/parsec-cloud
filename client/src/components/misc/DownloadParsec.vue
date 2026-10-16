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
      <ion-text class="download-parsec-text"> {{ $msTranslate('HomePage.topbar.downloadParsec') }} </ion-text>
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
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  padding: ms.spacing('padding-none');
  border-radius: ms.radius('lg');
  overflow: hidden;
  flex-shrink: 0;
  position: relative;

  &-content {
    background: ms.color('surface-base-page-secondary');
    align-items: center;
    padding: ms.spacing('padding-lg');
    display: flex;
    gap: ms.spacing('gap-lg');
    width: 100%;
    transition: all 0.2s ease-in-out;

    &:hover {
      background: ms.color('surface-neutral-default-subtle-pressed');
    }
  }

  &-text {
    @include ms.font('body-md-regular');
    font-weight: 600;
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: ms.color('text-brand-default');
  }

  &-icon {
    width: 1.5rem;
    height: 1.5rem;
    --fill-color: #{ms.color('surface-base-default')};
  }

  &-close {
    font-size: 1.125rem;
    background: ms.color('surface-base-page-secondary');
    color: ms.color('icon-base-default');
    opacity: ms.opacity('8');
    right: 0;
    height: 2rem;
    display: none;
    position: absolute;
    padding: ms.spacing('padding-sm');
    flex-shrink: 0;
    margin-inline: auto 0;
    border-left: ms.border('thin') solid ms.color('border-base-default');

    &:hover {
      background: ms.color('surface-neutral-default-subtle-pressed');
    }
  }

  &:hover .download-parsec-close {
    display: block;
  }
}
</style>
