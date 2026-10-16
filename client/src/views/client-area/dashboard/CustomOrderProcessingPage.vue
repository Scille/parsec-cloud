<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="client-page-processing">
    <div class="processing-text">
      <ion-text class="processing-text__title">
        {{ $msTranslate('clientArea.dashboard.processing.title') }}
      </ion-text>
      <ion-text class="processing-text__subtitle">
        {{ $msTranslate('clientArea.dashboard.processing.subtitle') }}
      </ion-text>
    </div>

    <div class="processing-step">
      <ion-text class="processing-step__title">{{ $msTranslate('clientArea.dashboard.processing.stepTitle') }}</ion-text>
      <ion-text class="processing-step__description">{{ $msTranslate('clientArea.dashboard.processing.stepDescription') }}</ion-text>
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
  gap: ms.spacing('gap-5xl');
  margin-left: 1rem;
}

.processing-text {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  .processing-text__title {
    @include ms.font('heading-h2');
    background: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  &__subtitle {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-description');
    max-width: 28rem;
  }
}

.processing-step {
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  gap: ms.spacing('gap-lg');
  max-width: 25rem;
  border-radius: ms.radius('2xl');
  box-shadow: ms.shadow('light');
  padding: ms.spacing('padding-4xl');

  &__title {
    @include ms.font('heading-h5');
    color: ms.color('text-brand-default');
  }

  &__description {
    @include ms.font('body-md-regular');
    color: ms.color('text-neutral-default');
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
