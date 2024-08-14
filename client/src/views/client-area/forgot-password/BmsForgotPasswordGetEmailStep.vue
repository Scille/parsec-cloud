<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-forgot-password-content">
    <!-- description -->
    <ion-text class="body saas-forgot-password-content__description">
      {{ $msTranslate('clientArea.forgotPassword.getEmailStep.description') }}
    </ion-text>

    <!-- email -->
    <ms-input
      class="saas-forgot-password-content__input"
      ref="emailInputRef"
      v-model="email"
      @keyup="error = ''"
      @on-enter-keyup="submit"
      label="clientArea.app.emailLabel"
      :validator="emailValidator"
    />
    <!-- button -->
    <div class="saas-forgot-password-button">
      <ion-button
        :disabled="querying"
        @click="$emit('cancel')"
        class="saas-forgot-password-button__item"
        fill="clear"
        size="large"
      >
        <ion-icon :icon="arrowBack" />
        {{ $msTranslate('clientArea.forgotPassword.getEmailStep.previousButton') }}
      </ion-button>
      <ion-button
        :disabled="!emailInputRef || emailInputRef.validity !== Validity.Valid || querying"
        @click="submit"
        class="saas-forgot-password-button__item"
        size="large"
      >
        {{ $msTranslate('clientArea.forgotPassword.getEmailStep.nextButton') }}
      </ion-button>
      <ms-spinner v-show="querying" />
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
import { MsInput, Translatable, Validity, MsSpinner, I18n } from 'megashark-lib';
import { emailValidator } from '@/common/validators';
import { warning, arrowBack } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { BmsApi } from '@/services/bms';
import { longLocaleCodeToShort } from '@/services/translation';

const emits = defineEmits<{
  (e: 'cancel'): void;
  (e: 'emailSent', email: string): void;
  (e: 'closeRequested'): void;
}>();

const email = ref<string>('');
const emailInputRef = ref();
const querying = ref(false);
const error = ref<Translatable>('');

onMounted(async () => {
  if (emailInputRef.value) {
    await emailInputRef.value.setFocus();
  }
});

async function submit(): Promise<boolean> {
  if (emailInputRef.value && emailInputRef.value.validity !== Validity.Valid) {
    return false;
  }
  querying.value = true;
  const response = await BmsApi.changePassword({
    email: email.value,
    lang: longLocaleCodeToShort(I18n.getLocale()),
  });

  if (response.isError) {
    error.value = 'clientArea.forgotPassword.getEmailStep.error';
    querying.value = false;
    return false;
  }
  querying.value = false;
  emits('emailSent', email.value);
  return true;
}
</script>

<style scoped lang="scss">
.saas-forgot-password-content {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;

  &__description {
    color: var(--parsec-color-light-secondary-grey);
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
