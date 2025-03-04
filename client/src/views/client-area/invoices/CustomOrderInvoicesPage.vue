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
import { BmsAccessInstance, BmsOrganization, DataType, SellsyInvoice } from '@/services/bms';
import { YearInvoicesList } from '@/components/client-area';
import { ref, onMounted } from 'vue';

const props = defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Map<number, Array<SellsyInvoice>>>(new Map());
const querying = ref(true);
const error = ref<string>('');

onMounted(async () => {
  querying.value = true;
  const response = await BmsAccessInstance.get().getCustomOrderInvoices(props.organization);
  if (!response.isError && response.data && response.data.type === DataType.CustomOrderInvoices) {
    const filteredInvoices = response.data.invoices.sort((invoice1, invoice2) => {
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
* {
  transition: all 0.2s ease;
}

.client-page-invoices {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
