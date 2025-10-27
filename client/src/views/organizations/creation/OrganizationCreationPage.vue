<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="creation-page page-modal-container">
    <!-- prettier-ignore -->
    <ms-image
      :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string)"
      class="creation-page__logo"
    />
    <ion-text
      class="creation-page__title subtitles-normal"
      v-if="isSmallDisplay"
    >
      {{ $msTranslate('CreateOrganization.loading') }}
    </ion-text>
    <div class="creation-page__spinner-container">
      <ion-text
        v-if="isLargeDisplay"
        class="subtitles-normal container-text"
      >
        {{ $msTranslate('CreateOrganization.loading') }}
      </ion-text>
      <ms-spinner class="creation-page__spinner" />
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonPage, IonText } from '@ionic/vue';
import { LogoIconWhite, MsImage, MsSpinner, useWindowSize } from 'megashark-lib';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
</script>

<style scoped lang="scss">
.creation-page {
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1.5rem;
  position: relative;
  justify-content: center;
  background: var(--parsec-color-light-gradient-background);

  @include ms.responsive-breakpoint('sm') {
    padding: 2rem 0 6rem;
    height: 100vh;
  }

  &::after {
    content: '';
    position: absolute;
    top: -3rem;
    background-image: url('@/assets/images/background/background-shapes-large.svg');
    background-size: cover;
    background-repeat: no-repeat;
    background-position: top left;
    z-index: -1;
    transition: all 0.3s ease-in-out;

    @include ms.responsive-breakpoint('sm') {
      width: 40rem;
      height: 67%;
    }

    @include ms.responsive-breakpoint('xs') {
      width: 30rem;
      height: 49%;
    }
  }

  &__logo {
    width: 3rem;

    @include ms.responsive-breakpoint('sm') {
      height: auto;
    }
  }

  &__title {
    color: var(--parsec-color-light-secondary-white);
    text-align: center;
    margin-bottom: 2rem;
    line-height: 1.7rem;
    max-width: 18rem;
  }

  &__spinner-container {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-white);
  }

  &__spinner {
    height: fit-content;
    background: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-circle);
    margin-inline: auto;
    padding: 0.125rem;
  }
}
</style>
