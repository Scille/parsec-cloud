<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="spinner">
    <img
      src="@/assets/images/Logo/logo_icon_gradient.svg"
      alt="loading-image"
      class="orga-loading__logo"
    >
    <ion-text
      class="subtitles-normal orga-loading__text"
    >
      {{ title }}
    </ion-text>
    <ion-spinner
      name="crescent"
      class="orga-loading__spinner"
    />
  </div>
</template>

<script setup lang="ts">

import { IonSpinner, IonText } from '@ionic/vue';

defineProps<{
  title: string
}>();
</script>

<style scoped lang="scss">
.spinner {
  &__logo {
    max-width: 3.625rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
    display: block;
    width: 100%;
  }

  &__spinner {
    --color: var(--parsec-color-light-primary-500);
  }
}
</style>
