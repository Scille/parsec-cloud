<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <h1>PAYMENT METHODS</h1>
    <div v-if="billingDetails">
      <ms-stripe-card-details
        v-if="cardDetails"
        :card="cardDetails as MsPaymentMethod.Card"
      />
      {{ billingDetails }}
    </div>
    <div v-if="!billingDetails && !error">
      <ms-spinner />
    </div>
    <div v-if="!billingDetails && error">
      {{ $msTranslate(error) }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { BmsOrganization, BmsAccessInstance, DataType, BillingDetailsResultData, PaymentMethod } from '@/services/bms';
import { onMounted, ref, computed } from 'vue';
import { MsSpinner, MsStripeCardDetails, PaymentMethod as MsPaymentMethod } from 'megashark-lib';

defineProps<{
  organization: BmsOrganization;
}>();

const billingDetails = ref<BillingDetailsResultData | undefined>(undefined);
const error = ref('');
const cardDetails = computed(() => {
  if (!billingDetails.value) {
    return;
  }
  const defaultMethod = billingDetails.value.paymentMethods.find((method) => method.isDefault);
  if (!defaultMethod || defaultMethod.type !== PaymentMethod.Card) {
    return;
  }
  return {
    last4: defaultMethod.lastDigits,
    // eslint-disable-next-line camelcase
    exp_year: defaultMethod.expirationDate.year,
    // eslint-disable-next-line camelcase
    exp_month: defaultMethod.expirationDate.month,
    brand: defaultMethod.brand,
  };
});

onMounted(async () => {
  const response = await BmsAccessInstance.get().getBillingDetails();
  if (!response.isError && response.data && response.data.type === DataType.BillingDetails) {
    billingDetails.value = response.data;
  } else {
    console.log(`Failed to retrieve billing details: ${response.errors}`);
    error.value = 'ClientApplication.billingDetails.retrieveFailed';
  }
});
</script>

<style scoped lang="scss"></style>
