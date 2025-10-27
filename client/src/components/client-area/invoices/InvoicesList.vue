<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- row header -->
  <ion-list class="invoices-year-header-list ion-no-padding">
    <ion-item class="invoices-year-header-list-item invoices-date">
      <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.date') }}</ion-text>
    </ion-item>
    <ion-item class="invoices-year-header-list-item invoices-number">
      <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.number') }}</ion-text>
    </ion-item>
    <ion-item class="invoices-year-header-list-item invoices-organization">
      <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.organization') }}</ion-text>
    </ion-item>
    <ion-item class="invoices-year-header-list-item invoices-amount">
      <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.price.title') }}</ion-text>
    </ion-item>
    <ion-item
      v-if="invoices.some((invoice: Invoice) => invoice.getType() === InvoiceType.Sellsy)"
      class="invoices-year-header-list-item invoices-contract-period"
    >
      <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.contractPeriod.title') }}</ion-text>
    </ion-item>
    <ion-item class="invoices-year-header-list-item invoices-status">
      <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.status') }}</ion-text>
    </ion-item>
  </ion-list>

  <!-- row invoices -->
  <ion-list class="invoices-year-content-list ion-no-padding">
    <invoices-list-item
      v-for="invoice in invoices"
      :invoice="invoice"
      :key="invoice.getId()"
      v-show="monthsFilter === undefined || monthsFilter.length === 0 || monthsFilter.includes(invoice.getDate().month)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import InvoicesListItem from '@/components/client-area/invoices/InvoicesListItem.vue';
import { Invoice, InvoiceType } from '@/services/bms';
import { IonItem, IonList, IonText } from '@ionic/vue';

defineProps<{
  invoices: Array<Invoice>;
  monthsFilter?: Array<number>;
}>();
</script>

<style lang="scss" scoped>
.invoices-year-header-list {
  display: flex;
  width: 100%;
  background: none;
  margin-bottom: 0.5rem;
  position: sticky;
  top: 0rem;
  background: var(--parsec-color-light-secondary-premiere);
  z-index: 2;

  &-item {
    --padding-start: 0;
    --padding-end: 0;
    --padding-top: 0;
    --padding-bottom: 0;
    --background: none;
    padding: 0.375rem 1rem;
    color: var(--parsec-color-light-secondary-grey);
  }
}

.invoices-year-content-list {
  display: flex;
  flex-direction: column;
  --ion-safe-area-left: 0;
  gap: 0.5rem;
  background: none;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.invoices {
  &-date {
    width: 100%;
    max-width: var(--max-width-date);
  }

  &-number {
    width: 100%;
    max-width: var(--max-width-number);
  }

  &-organization {
    width: 100%;
    max-width: var(--max-width-organization);
  }

  &-amount {
    width: 100%;
    max-width: var(--max-width-amount);
  }

  &-contract-period {
    width: 100%;
    max-width: var(--max-width-contract-period);
  }

  &-status {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }
}
</style>
