<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="invoices-year-content-list-item ion-no-padding"
  >
    <ion-text class="invoices-year-content-list-item__data invoices-date subtitles-sm">
      {{ $msTranslate(I18n.formatDate(invoice.date, 'narrow')) }}
    </ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-number body">{{ invoice.number }}</ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-organization body">{{ invoice.organizationId }}</ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-amount body">
      {{ $msFormatCurrency(invoice.price) }}
    </ion-text>
    <template v-if="invoice.type === DataType.CustomOrderDetails">
      <ion-text
        class="invoices-year-content-list-item__data invoices-contract-period body"
        v-if="invoice.licenseStart && invoice.licenseEnd"
      >
        {{
          $msTranslate({
            key: 'clientArea.invoices.cell.contractPeriod.from',
            data: { date: $msTranslate(I18n.formatDate(invoice.licenseStart, 'narrow')) },
          })
        }}
        {{
          $msTranslate({
            key: 'clientArea.invoices.cell.contractPeriod.to',
            data: { date: $msTranslate(I18n.formatDate(invoice.licenseEnd, 'narrow')) },
          })
        }}
      </ion-text>
      <ion-text
        v-else
        class="subtitles-sm"
      >
        /
      </ion-text>
    </template>
    <ion-text class="invoices-year-content-list-item__data invoices-status">
      <span
        class="badge-status body-sm"
        :class="invoice.status"
      >
        {{ $msTranslate(getInvoiceStatusTranslationKey(invoice.status)) }}
      </span>
      <a
        class="custom-button custom-button-ghost button-medium"
        :href="invoice.link"
        download
      >
        <ms-image
          :image="downloaded"
          class="custom-button__icon"
        />
        {{ $msTranslate('clientArea.invoices.cell.download') }}
      </a>
    </ion-text>
  </ion-item>
</template>

<script setup lang="ts">
import { DataType } from '@/services/bms';
import { getInvoiceStatusTranslationKey } from '@/services/translation';
import { IonItem, IonText } from '@ionic/vue';
import { MsImage, Download as downloaded, I18n } from 'megashark-lib';
import { InvoiceData } from '@/components/client-area/invoices/types';

defineProps<{
  invoice: InvoiceData;
}>();
</script>

<style lang="scss" scoped>
.invoices-year-content-list-item {
  background: var(--parsec-color-light-secondary-white);
  display: flex;
  justify-content: space-between;
  --background: none;
  border-radius: var(--parsec-radius-8);
  position: relative;
  --inner-padding-end: 0;
  align-items: center;

  &:hover {
    background: var(--parsec-color-light-secondary-background);
    .invoices-year-content-list-item__data {
      color: var(--parsec-color-light-secondary-contrast);
    }
  }

  &__data {
    color: var(--parsec-color-light-secondary-hard-grey);
    align-self: stretch;
    display: flex;
    align-items: center;
    padding: 0.5rem 1rem;

    .custom-button-ghost {
      color: var(--parsec-color-light-primary-500);

      &:hover {
        color: var(--parsec-color-light-primary-600);
      }
    }
  }

  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  .badge-status {
    border-radius: var(--parsec-radius-32);
    padding-inline: 0.5rem;
    background: var(--parsec-color-light-secondary-disabled);
    color: var(--parsec-color-light-secondary-text);

    &.paid {
      background: var(--parsec-color-light-success-100);
      color: var(--parsec-color-light-success-700);
    }

    &.open {
      background: var(--parsec-color-light-info-100);
      color: var(--parsec-color-light-info-700);
    }
  }

  .custom-button {
    &__icon {
      width: 1rem;
      --fill-color: var(--parsec-color-light-primary-500);
    }

    &:hover {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-50);

      .custom-button__icon {
        --fill-color: var(--parsec-color-light-primary-600);
      }
    }
  }
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
