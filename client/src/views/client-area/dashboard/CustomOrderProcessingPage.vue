<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-processing">
    <div class="processing-text">
      <ion-text class="processing-text__title title-h1">
        {{ $msTranslate('clientArea.dashboard.processing.title') }}
      </ion-text>
      <ion-text class="processing-text__subtitle body-lg">
        {{ $msTranslate('clientArea.dashboard.processing.subtitle') }}
      </ion-text>
    </div>

    <div class="processing-step">
      <ion-text class="processing-step__title title-h4">{{ $msTranslate('clientArea.dashboard.processing.stepTitle') }}</ion-text>
      <ion-text class="processing-step__description body">{{ $msTranslate('clientArea.dashboard.processing.stepDescription') }}</ion-text>
      <ms-image
        class="processing-step__image"
        :image="LogoRowGradient"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { LogoRowGradient, MsImage } from 'megashark-lib';
</script>

<style scoped lang="scss">
.client-page-processing {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  margin-left: 1rem;
}

.processing-text {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  .processing-text__title {
    background: var(--parsec-color-light-gradient-background);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-hard-grey);
    max-width: 28rem;
  }
}

.processing-step {
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  gap: 0.5rem;
  max-width: 25rem;
  border-radius: var(--parsec-radius-12);
  box-shadow: var(--parsec-shadow-light);
  padding: 1.5rem;

  &__title {
    color: var(--parsec-color-light-primary-600);
  }

  &__description {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &__image {
    width: 100%;
    max-width: 5rem;
    height: 1rem;
    margin-top: 0.5rem;
    display: flex;
    align-self: flex-end;
  }
}
</style>
