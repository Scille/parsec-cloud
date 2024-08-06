<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="clientArea.paymentMethodsPage.addPaymentMethodTitle"
    :close-button="{ visible: true }"
    :confirm-button="{
      label: 'clientArea.paymentMethodsPage.addPaymentMethodConfirm',
      disabled: !cardForm?.isValid,
      onClick: confirm,
    }"
    class="creditCard-modal"
  >
    <ms-stripe-card-form ref="cardForm" />
    <ion-toggle
      v-model="setDefault"
      label-placement="end"
      class="creditCard-checkbox"
      justify="start"
    >
      <span class="subtitles-normal">{{ $msTranslate('clientArea.paymentMethodsPage.useAsDefault') }}</span>
    </ion-toggle>
    <span v-show="error">{{ error }}</span>
  </ms-modal>
</template>

<script setup lang="ts">
import { MsModal, MsModalResult, MsStripeCardForm, PaymentMethodResult } from 'megashark-lib';
import { IonToggle, modalController } from '@ionic/vue';
import { ref } from 'vue';

const cardForm = ref();
const setDefault = ref(false);
const error = ref('');

async function confirm(): Promise<boolean> {
  const result: PaymentMethodResult = await cardForm.value.submit();
  if (result.paymentMethod) {
    return modalController.dismiss({ card: result.paymentMethod, setDefault: setDefault.value }, MsModalResult.Confirm);
  }
  if (result.error && result.error.message) {
    error.value = result.error.message;
  }
  return false;
}
</script>

<style scoped lang="scss">
.creditCard-checkbox {
  display: flex;
  align-items: center;
  gap: 1rem;
  color: var(--parsec-color-light-secondary-hard-grey);
  margin-top: 1.5rem;
}
</style>
