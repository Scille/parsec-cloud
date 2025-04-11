<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="creation-page page-modal-container">
    <ms-image
      :image="isSmallDisplay ? LogoIconWhite : LogoIconGradient"
      class="creation-page__logo"
    />
    <ion-text
      class="creation-page__title subtitles-normal"
      v-if="isSmallDisplay"
    >
      {{ $msTranslate('CreateOrganization.loading') }}
    </ion-text>
    <ms-spinner
      :title="isLargeDisplay ? 'CreateOrganization.loading' : ''"
      class="creation-page__spinner"
    />
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonText } from '@ionic/vue';
import { MsImage, MsSpinner, LogoIconGradient, LogoIconWhite, useWindowSize } from 'megashark-lib';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
</script>

<style scoped lang="scss">
.creation-page {
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1.5rem;
  position: relative;

  @include ms.responsive-breakpoint('sm') {
    justify-content: center;
    padding: 2rem 0 6rem;
    height: 100vh;
    background: var(--parsec-color-light-gradient-background);
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

  &__spinner {
    height: fit-content;
    background: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-circle);
    margin-inline: auto;
    padding: 0.25rem;
  }
}
</style>
