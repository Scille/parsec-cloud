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
import { BmsAccessInstance, BmsOrganization, BmsResponse, DataType, SellsyInvoice } from '@/services/bms';
import { isDefaultOrganization } from '@/views/client-area/types';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  currentOrganization: BmsOrganization;
  organizations: Array<BmsOrganization>;
}>();

const invoices = ref<Map<number, Array<SellsyInvoice>>>(new Map());
const querying = ref(true);
const error = ref<string>('');

onMounted(async () => {
  querying.value = true;
  let response: BmsResponse;
  if (isDefaultOrganization(props.currentOrganization)) {
    response = await BmsAccessInstance.get().getCustomOrderInvoices(...props.organizations);
  } else {
    response = await BmsAccessInstance.get().getCustomOrderInvoices(props.currentOrganization);
  }
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
