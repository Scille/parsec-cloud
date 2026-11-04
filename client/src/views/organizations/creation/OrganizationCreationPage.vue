<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="creation-page page-modal-container">
    <!-- prettier-ignore -->
    <ms-image
      :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string)"
      class="creation-page__logo"
    />
    <ion-text
      class="creation-page__title"
      v-if="isSmallDisplay"
    >
      {{ $msTranslate('CreateOrganization.loading') }}
    </ion-text>
    <div class="creation-page__spinner-container">
      <ion-text
        v-if="isLargeDisplay"
        class="container-text"
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
  gap: ms.spacing('gap-4xl');
  position: relative;
  justify-content: center;
  background: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));

  body.ms-theme-dark & {
    background: linear-gradient(ms.color('surface-brand-default-subtle-pressed'), ms.color('surface-base-page'));
  }

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-5xl') ms.spacing('padding-none') 6rem;
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
      width: 100%;
      height: 100%;
      background-position: center;
    }
  }

  &__logo {
    width: 3rem;

    @include ms.responsive-breakpoint('sm') {
      height: auto;
    }
  }

  &__title {
    @include ms.font('body-lg-medium');
    color: ms.color('text-on-color-heading');
    text-align: center;
    margin-bottom: 2rem;
    line-height: 1.7rem;
    max-width: 18rem;
  }

  &__spinner-container {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    @include ms.font('body-lg-medium');
    color: ms.color('text-on-color-heading');
    height: fit-content;
  }

  &__spinner {
    height: fit-content;
    background: ms.color('surface-base-default');
    border-radius: ms.radius('full');
    margin-inline: auto;
    padding: ms.spacing('padding-xs');
  }
}
</style>
