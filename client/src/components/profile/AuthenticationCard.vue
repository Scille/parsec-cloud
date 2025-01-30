<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <div class="authentication-card">
    <img
      :src="imageSrc"
      :alt="imageAlt"
      class="authentication-card__image"
    />
    <ion-text class="authentication-card__text subtitles-normal">{{ $msTranslate(methodName) }}</ion-text>
    <ion-icon
      :icon="checkmarkCircle"
      class="authentication-card__icon"
    />
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

defineProps<{
  imageSrc: string;
  imageAlt: string;
  methodName: Translatable;
}>();
</script>

<style scoped lang="scss">
.authentication-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding-right: 1rem;
  border: 1px solid var(--parsec-color-light-secondary-disabled);
  border-radius: var(--parsec-radius-8);
  box-shadow: var(--parsec-shadow-1);
  overflow: hidden;

  &__image {
    padding: 1.5rem;
    background: var(--parsec-color-light-secondary-background);
  }

  &__text {
    color: var(--parsec-color-light-primary-700);
    width: 100%;
  }

  &__icon {
    font-size: 1.25rem;
    flex-shrink: 0;
    color: var(--parsec-color-light-primary-600);
  }
}
</style>
