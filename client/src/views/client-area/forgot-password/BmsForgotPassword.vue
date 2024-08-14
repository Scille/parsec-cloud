<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-forgot-password">
    <ion-buttons
      slot="end"
      class="closeBtn-container"
      v-if="!hideHeader"
    >
      <ion-button
        slot="icon-only"
        class="closeBtn"
        @click="$emit('closeRequested')"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
    <div class="saas-forgot-password-container">
      <!-- here we should normally use the hide-close-button given by the component not the custom one above -->
      <!-- will be addressed in https://github.com/Scille/parsec-cloud/issues/8056 -->
      <create-organization-modal-header
        v-if="!hideHeader"
        @close-clicked="$emit('closeRequested')"
        title="clientArea.app.titleLinkOrganization"
        :hide-close-button="true"
      />
      <ion-text
        v-if="hideHeader"
        class="saas-forgot-password__title title-h2"
      >
        {{ $msTranslate(getCurrentStepTitle()) }}
      </ion-text>

      <bms-forgot-password-get-email-step
        v-if="currentStep === Steps.GetEmail"
        @email-sent="onEmailSent"
        @cancel="$emit('cancel')"
      />

      <bms-forgot-password-email-sent-step
        v-if="currentStep === Steps.EmailSent"
        :email="emailRef"
        @login-requested="$emit('loginRequested')"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonText, IonButtons, IonIcon } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
import { close } from 'ionicons/icons';
import { ref } from 'vue';
import BmsForgotPasswordGetEmailStep from '@/views/client-area/forgot-password/BmsForgotPasswordGetEmailStep.vue';
import BmsForgotPasswordEmailSentStep from '@/views/client-area/forgot-password/BmsForgotPasswordEmailSentStep.vue';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

const enum Steps {
  GetEmail,
  EmailSent,
}

defineProps<{
  hideHeader?: boolean;
}>();

defineEmits<{
  (e: 'cancel'): void;
  (e: 'closeRequested'): void;
  (e: 'loginRequested'): void;
}>();

const currentStep = ref<Steps>(Steps.GetEmail);
const emailRef = ref<string>('');

function getCurrentStepTitle(): Translatable {
  switch (currentStep.value) {
    case Steps.GetEmail:
      return 'clientArea.forgotPassword.getEmailStep.title';
    case Steps.EmailSent:
      return 'clientArea.forgotPassword.emailSentStep.title';
  }
}

function onEmailSent(email: string): void {
  emailRef.value = email;
  currentStep.value = Steps.EmailSent;
}
</script>

<style scoped lang="scss">
.saas-forgot-password {
  display: flex;
  flex-direction: row;
  width: 100%;
  background: var(--parsec-color-light-primary-50);
  position: relative;
  padding: 2.5rem;
  min-width: 32em;

  &__title {
    color: var(--parsec-color-light-primary-800);
    margin-bottom: 2rem;
  }

  &-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    position: relative;
    z-index: 2;
  }
}
</style>
