<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <h1>INVOICES</h1>

    <div
      v-for="invoice in invoices"
      :key="invoice.id"
    >
      {{ invoice }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, BmsInvoice, BmsOrganization, DataType } from '@/services/bms';
import { ref, onMounted } from 'vue';

defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Array<BmsInvoice>>([]);

onMounted(async () => {
  const response = await BmsAccessInstance.get().getInvoices();
  if (!response.isError && response.data && response.data.type === DataType.Invoices) {
    invoices.value = response.data.invoices;
  }
});
</script>

<style scoped lang="scss"></style>
