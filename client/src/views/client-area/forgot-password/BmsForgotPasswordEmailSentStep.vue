<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="saas-forgot-password-content">
    <ion-text class="saas-forgot-password-content__email">
      {{ email }}
    </ion-text>

    <!-- button -->
    <div class="saas-forgot-password-button">
      <ion-button
        class="saas-forgot-password-button__item color-secondary"
        @click="$emit('loginRequested')"
      >
        {{ $msTranslate('clientArea.forgotPassword.emailSentStep.nextButton') }}
        <ion-icon
          :icon="chevronForward"
          class="button-icon-right"
        />
      </ion-button>
    </div>

    <!-- error -->
    <ion-text
      class="form-error forgot-password-button-error"
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
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { chevronForward, warning } from 'ionicons/icons';
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
  gap: ms.spacing('gap-2xl');

  &__email {
    @include ms.font('body-lg-medium');
    color: ms.color('text-base-body');
  }

  .saas-forgot-password-button {
    display: flex;
    gap: ms.spacing('gap-3xl');
    justify-content: end;
    align-items: center;
    margin-top: 0.5rem;
    width: 100%;

    &__item {
      width: 100%;
    }
  }
}
</style>
