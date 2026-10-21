<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="created-page page-modal-container">
    <!-- prettier-ignore -->
    <ms-image
      :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string)"
      class="created-page__logo"
    />
    <ion-text class="created-page__title">
      {{ $msTranslate('CreateOrganization.organizationCreated') }}
    </ion-text>

    <ion-text
      v-if="isSmallDisplay"
      class="created-page__organization"
    >
      {{ organizationName }}
    </ion-text>

    <ion-footer class="created-page-footer">
      <div class="modal-footer-buttons">
        <ion-button
          fill="solid"
          size="default"
          @click="$emit('goClicked')"
          class="footer-button"
        >
          {{ $msTranslate('CreateOrganization.button.done') }}
        </ion-button>
      </div>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import { OrganizationID } from '@/parsec';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonButton, IonFooter, IonPage, IonText } from '@ionic/vue';
import { LogoIconWhite, MsImage, useWindowSize } from 'megashark-lib';

const { isSmallDisplay } = useWindowSize();

defineProps<{
  organizationName: OrganizationID;
}>();

defineEmits<{
  (e: 'goClicked'): void;
}>();
</script>

<style scoped lang="scss">
.created-page {
  flex-direction: column;
  position: relative;
  gap: ms.spacing('gap-4xl');
  align-items: center;
  text-align: center;
  justify-content: center;
  padding: ms.spacing('padding-8xl') ms.spacing('padding-none') ms.spacing('padding-8xl');
  height: 100vh;
  background: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));

  &::after {
    content: '';
    position: absolute;
    top: -4rem;
    right: -3rem;
    background-image: url('@/assets/images/background/background-shapes-large.svg');
    background-size: cover;
    background-repeat: no-repeat;
    background-position: top left;
    z-index: -1;
    transition: all 0.3s ease-in-out;
    width: 20rem;
    height: 100%;

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
    height: auto;
  }

  &__title {
    @include ms.font('body-xl-medium');
    color: ms.color('text-on-color-heading');
    text-align: center;
    line-height: 1.7rem;
    max-width: 18rem;
  }

  &__organization {
    @include ms.font('heading-h2');
    color: ms.color('text-on-color-heading');
  }

  &-footer {
    .footer-button {
      width: auto;

      @include ms.responsive-breakpoint('sm') {
        width: 100%;
      }

      &::part(native) {
        border-radius: ms.radius('lg');
        --background-hover: ms.color('surface-neutral-default-subtle-hover');
        @include ms.font('label-lg-medium');
        color: ms.color('text-brand-default');
        background: ms.color('surface-base-default');
      }
    }
  }
}
</style>
