<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-forgot-password-content">
    <!-- description -->
    <ion-text class="body saas-forgot-password-content__description">
      {{ $msTranslate('clientArea.forgotPassword.emailSentStep.description') }}
    </ion-text>
    <ion-text class="body saas-forgot-password-content__email">
      {{ email }}
    </ion-text>

    <!-- button -->
    <div class="saas-forgot-password-button">
      <ion-button
        @click="$emit('loginRequested')"
        class="saas-forgot-password-button__item"
        size="large"
        fill="clear"
      >
        {{ $msTranslate('clientArea.forgotPassword.emailSentStep.nextButton') }}
        <ion-icon :icon="arrowForward" />
      </ion-button>
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
import { IonButton, IonText, IonIcon } from '@ionic/vue';
import { warning, arrowForward } from 'ionicons/icons';
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

  &__description {
    color: var(--parsec-color-light-secondary-grey);
  }

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

      &[fill='clear'] {
        --color: var(--parsec-color-light-secondary-text);
      }
    }
  }
}
</style>
