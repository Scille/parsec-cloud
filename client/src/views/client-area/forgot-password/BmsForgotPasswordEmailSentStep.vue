<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-forgot-password-content">
    <ion-text class="subtitles-normal saas-forgot-password-content__email">
      {{ email }}
    </ion-text>

    <!-- button -->
    <div class="saas-forgot-password-button">
      <ion-text
        class="button-medium custom-button custom-button-ghost saas-forgot-password-button__item"
        button
        @click="$emit('loginRequested')"
      >
        {{ $msTranslate('clientArea.forgotPassword.emailSentStep.nextButton') }}
        <ion-icon :icon="arrowForward" />
      </ion-text>
    </div>

    <!-- error -->
    <ion-text
      class="form-error body forgot-password-button-error"
      v-show="error"
    >
      <ion-icon
        class="form-error-icon"
        :icon="warning"
      />{{ $msTranslate(error) }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import { arrowForward, warning } from 'ionicons/icons';
import { ref } from 'vue';

defineProps<{
  email: string;
}>();

defineEmits<{
  (e: 'loginRequested'): void;
  (e: 'closeRequested'): void;
}>();

const error = ref<string>('');
</script>

<style scoped lang="scss">
.saas-forgot-password-content {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;

  &__email {
    font-weight: 500;
    color: var(--parsec-color-light-secondary-100);
  }

  .saas-forgot-password-button {
    display: flex;
    gap: 1rem;
    justify-content: end;
    align-items: center;
    margin-top: 0.5rem;
    width: 100%;

    &__item {
      height: 2.5rem;
      border-radius: var(--parsec-radius-6);
      font-size: 1rem;
      color: var(--parsec-color-light-secondary-contrast);
      border: 1px solid transparent;

      &:hover {
        border: 1px solid var(--parsec-color-light-secondary-contrast);
      }
    }
  }
}
</style>
