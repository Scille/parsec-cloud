<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-forgot-password">
    <ion-button
      v-if="!hideHeader"
      slot="icon-only"
      class="closeBtn"
      @click="$emit('closeRequested')"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>
    <div class="saas-forgot-password-container">
      <!-- here we should normally use the hide-close-button given by the component not the custom one above -->
      <!-- will be addressed in https://github.com/Scille/parsec-cloud/issues/8056 -->
      <create-organization-modal-header
        @close-clicked="$emit('closeRequested')"
        :icon="getCurrentStepDetails().icon"
        :title="getCurrentStepDetails().title"
        :subtitle="getCurrentStepDetails().subtitle"
        :hide-close-button="true"
      />

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
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import BmsForgotPasswordEmailSentStep from '@/views/client-area/forgot-password/BmsForgotPasswordEmailSentStep.vue';
import BmsForgotPasswordGetEmailStep from '@/views/client-area/forgot-password/BmsForgotPasswordGetEmailStep.vue';
import { IonButton, IonIcon } from '@ionic/vue';
import { close, keypad, mailUnread } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';
import { ref } from 'vue';

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

function getCurrentStepDetails(): { icon: string; title: Translatable; subtitle?: Translatable } {
  switch (currentStep.value) {
    case Steps.GetEmail:
      return {
        icon: keypad,
        title: 'clientArea.forgotPassword.getEmailStep.title',
        subtitle: 'clientArea.forgotPassword.getEmailStep.description',
      };
    case Steps.EmailSent:
      return {
        icon: mailUnread,
        title: 'clientArea.forgotPassword.emailSentStep.title',
        subtitle: 'clientArea.forgotPassword.emailSentStep.description',
      };
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
  min-width: 35.75rem;
  max-width: 35.75rem;

  &-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    position: relative;
    z-index: 2;
  }
}
</style>
