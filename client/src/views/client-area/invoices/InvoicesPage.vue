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
import { YearInvoicesList } from '@/components/client-area';
import { BmsAccessInstance, BmsOrganization, DataType, StripeInvoice } from '@/services/bms';
import { isDefaultOrganization } from '@/views/client-area/types';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Map<number, Array<StripeInvoice>>>(new Map());
const error = ref<string>('');
const querying = ref(true);

onMounted(async () => {
  querying.value = true;
  const response = await BmsAccessInstance.get().getMonthlySubscriptionInvoices();
  if (!response.isError && response.data && response.data.type === DataType.MonthlySubscriptionInvoices) {
    const filteredInvoices = response.data.invoices
      .filter((invoice) => isDefaultOrganization(props.organization) || invoice.getOrganizationId() === props.organization.parsecId)
      .sort((invoice1, invoice2) => {
        return invoice2.getDate().diff(invoice1.getDate()).toMillis();
      });

    filteredInvoices.forEach((invoice) => {
      const year = invoice.getDate().year;
      if (!invoices.value.has(year)) {
        invoices.value.set(year, []);
      }
      invoices.value.get(year)!.push(invoice);
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
