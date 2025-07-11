<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="created-page page-modal-container">
    <ms-informative-text v-if="!isSmallDisplay">
      {{ $msTranslate('CreateOrganization.organizationCreated') }}
    </ms-informative-text>

    <!-- prettier-ignore -->
    <ms-image
      v-if="isSmallDisplay"
      :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string)"
      class="created-page__logo"
    />
    <ion-text
      v-if="isSmallDisplay"
      class="created-page__title subtitles-normal"
    >
      {{ $msTranslate('CreateOrganization.organizationCreated') }}
    </ion-text>

    <ion-text
      v-if="isSmallDisplay"
      class="created-page__organization title-h1"
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
          <span>
            {{ $msTranslate('CreateOrganization.button.done') }}
          </span>
          <ion-icon
            slot="start"
            :icon="chevronForward"
            size="small"
          />
        </ion-button>
      </div>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import { OrganizationID } from '@/parsec';
import { MsInformativeText, useWindowSize, LogoIconWhite, MsImage } from 'megashark-lib';
import { chevronForward } from 'ionicons/icons';
import { IonPage, IonText, IonButton, IonFooter, IonIcon } from '@ionic/vue';
import { Resources, ResourcesManager } from '@/services/resourcesManager';

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

  @include ms.responsive-breakpoint('sm') {
    gap: 1.5rem;
    align-items: center;
    text-align: center;
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
    height: auto;
  }

  &__title {
    color: var(--parsec-color-light-secondary-white);
    text-align: center;
    margin-bottom: 2rem;
    line-height: 1.7rem;
    max-width: 18rem;
  }

  &__organization {
    color: var(--parsec-color-light-primary-700);

    @include ms.responsive-breakpoint('sm') {
      color: var(--parsec-color-light-secondary-white);
    }
  }

  &-footer {
    @include ms.responsive-breakpoint('sm') {
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
    }

    .footer-button {
      @include ms.responsive-breakpoint('sm') {
        width: 100%;
      }

      &::part(native) {
        border-radius: var(--parsec-radius-8);

        @include ms.responsive-breakpoint('sm') {
          --background-hover: var(--parsec-color-light-secondary-premiere);
          font-size: 1rem;
          color: var(--parsec-color-light-primary-600);
          padding: 0.75rem 0;
          background: var(--parsec-color-light-secondary-white);
        }
      }
    }
  }
}
</style>
