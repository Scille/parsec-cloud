<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="invoices-year-content">
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
  </div>
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
.invoices-year-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  overflow: auto;
}

.invoices-year-header-list {
  display: flex;
  width: 100%;
  margin-bottom: 0.5rem;
  position: sticky;
  top: 0;
  background: var(--parsec-color-light-secondary-premiere);
  z-index: 2;
  min-width: 35rem;

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
  min-width: fit-content;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.invoices {
  &-date {
    width: 100%;
    max-width: var(--max-width-date);
    min-width: 8rem;
  }

  &-number {
    width: 100%;
    max-width: var(--max-width-number);
    min-width: 10rem;
  }

  &-organization {
    width: 100%;
    max-width: var(--max-width-organization);
    min-width: 10rem;
  }

  &-amount {
    width: 100%;
    max-width: var(--max-width-amount);
    min-width: 10rem;
  }

  &-contract-period {
    width: 100%;
    max-width: var(--max-width-contract-period);
    min-width: 10rem;
  }

  &-status {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    flex-grow: 1;
    flex-basis: var(--max-width-date);
    min-width: 12rem;
  }
}
</style>
