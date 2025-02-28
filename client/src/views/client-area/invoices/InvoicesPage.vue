<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-invoices">
    <year-invoices-list
      :querying="querying"
      :invoices="invoices"
      title="clientArea.invoices.title"
      :error="error"
    />
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, BmsOrganization, DataType, StripeInvoice } from '@/services/bms';
import { ref, onMounted } from 'vue';
import { isDefaultOrganization } from '@/views/client-area/types';
import { YearInvoicesList } from '@/components/client-area';

const props = defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Array<StripeInvoice>>([]);
const error = ref<string>('');
const querying = ref(true);

onMounted(async () => {
  querying.value = true;
  const response = await BmsAccessInstance.get().getMonthlySubscriptionInvoices();
  if (!response.isError && response.data && response.data.type === DataType.MonthlySubscriptionInvoices) {
    invoices.value = response.data.invoices
      .filter((invoice) => isDefaultOrganization(props.organization) || invoice.getOrganizationId() === props.organization.parsecId)
      .sort((invoice1, invoice2) => {
        return invoice2.getDate().diff(invoice1.getDate()).toMillis();
      });
  } else {
    error.value = 'clientArea.invoices.retrieveError';
  }
  querying.value = false;
});
</script>

<style scoped lang="scss">
.client-page-invoices {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
