<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="clientArea.paymentMethodsPage.addPaymentMethod.addCard"
    :close-button="{ visible: true }"
    :confirm-button="{
      label: 'clientArea.paymentMethodsPage.addPaymentMethod.confirm',
      disabled: !cardFormRef?.isValid || querying,
      onClick: confirm,
      queryingSpinner: querying,
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
    <span
      v-show="error"
      id="modal-error"
    >
      {{ $msTranslate(error) }}
    </span>
  </ms-modal>
</template>

<script setup lang="ts">
import { BmsAccessInstance } from '@/services/bms';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonToggle, modalController } from '@ionic/vue';
import { I18n, MsModal, MsModalResult, MsStripeCardForm, PaymentMethodResult, Translatable } from 'megashark-lib';
import { ref, useTemplateRef } from 'vue';

const props = defineProps<{
  informationManager: InformationManager;
}>();

const cardFormRef = useTemplateRef<InstanceType<typeof MsStripeCardForm>>('cardForm');
const setDefault = ref(false);
const error = ref<Translatable | undefined>(undefined);
const querying = ref(false);

async function confirm(): Promise<boolean> {
  try {
    querying.value = true;
    const result: PaymentMethodResult | undefined = await cardFormRef.value!.submit();
    if (!result) {
      return false;
    }
    if (result.error && result.error.message) {
      error.value = I18n.valueAsTranslatable(result.error.message);
      return false;
    }
    if (!result.paymentMethod) {
      error.value = 'clientArea.paymentMethodsPage.addPaymentMethod.failed';
      return false;
    }
    const card = result.paymentMethod;
    const response = await BmsAccessInstance.get().addPaymentMethod(card.id);
    if (response.isError) {
      error.value = 'clientArea.paymentMethodsPage.addPaymentMethod.failed';
      return false;
    }
    let setDefaultFailed = false;
    if (setDefault.value) {
      const defaultResponse = await BmsAccessInstance.get().setDefaultPaymentMethod(card.id);
      setDefaultFailed = defaultResponse.isError;
    }
    props.informationManager.present(
      new Information({
        message: setDefaultFailed
          ? 'clientArea.paymentMethodsPage.addPaymentMethod.successButSetDefaultFailed'
          : 'clientArea.paymentMethodsPage.addPaymentMethod.success',
        level: response.isError ? InformationLevel.Warning : InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    return await modalController.dismiss(undefined, MsModalResult.Confirm);
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.creditCard-modal {
  position: relative;
}

.creditCard-checkbox {
  display: flex;
  align-items: center;
  gap: 1rem;
  color: var(--parsec-color-light-secondary-hard-grey);
  margin-top: 1.5rem;
}
</style>
