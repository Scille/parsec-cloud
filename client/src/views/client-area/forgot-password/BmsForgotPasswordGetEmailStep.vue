<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="saas-forgot-password-content">
    <!-- email -->
    <ms-input
      class="saas-forgot-password-content__input"
      ref="emailInput"
      v-model="email"
      @keyup="error = ''"
      @on-enter-keyup="submit"
      label="clientArea.app.emailLabel"
      :validator="emailValidator"
    />
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
    <!-- button -->
    <div class="saas-forgot-password-button">
      <ion-button
        :disabled="querying"
        fill="clear"
        slot="icon-only"
        class="saas-forgot-password-button__item color-secondary"
        @click="$emit('cancel')"
      >
        <ion-icon :icon="arrowBack" />
      </ion-button>
      <ion-button
        :disabled="!emailInputRef || emailInputRef.validity !== Validity.Valid || querying"
        @click="submit"
        class="saas-forgot-password-button__item"
      >
        {{ $msTranslate('clientArea.forgotPassword.getEmailStep.nextButton') }}
      </ion-button>
      <ms-spinner v-show="querying" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import { BmsApi, BmsLang } from '@/services/bms';
import { longLocaleCodeToShort } from '@/services/translation';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { arrowBack, warning } from 'ionicons/icons';
import { I18n, MsInput, MsSpinner, Translatable, Validity } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const emits = defineEmits<{
  (e: 'cancel'): void;
  (e: 'emailSent', email: string): void;
  (e: 'closeRequested'): void;
}>();

const email = ref<string>('');
const emailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('emailInput');
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
    email: email.value.trim(),
    lang: longLocaleCodeToShort(I18n.getLocale()) as BmsLang,
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

  .saas-forgot-password-button {
    display: flex;
    gap: ms.spacing('gap-3xl');
    justify-content: end;
    align-items: center;
    margin-top: 2.5rem;
    width: 100%;

    &__item:nth-child(2) {
      flex-grow: 1;
    }
  }

  .forgot-password-button-error {
    margin-top: 0.5rem;
    display: flex;
    gap: ms.spacing('gap-lg');
    align-items: baseline;
  }
}
</style>
