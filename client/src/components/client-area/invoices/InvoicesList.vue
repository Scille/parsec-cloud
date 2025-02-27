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
      v-if="invoicesData.type === DataType.CustomOrderDetails"
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
    <ion-item
      class="invoices-year-content-list-item ion-no-padding"
      v-for="invoice in invoicesData.invoices"
      :key="invoice.id"
      v-show="monthsFilter === undefined || monthsFilter.length === 0 || monthsFilter.includes(invoice.date.month)"
    >
      <ion-text class="invoices-year-content-list-item__data invoices-date subtitles-sm">
        {{ $msTranslate(I18n.formatDate(invoice.date, 'narrow')) }}
      </ion-text>
      <ion-text class="invoices-year-content-list-item__data invoices-number body">{{ invoice.number }}</ion-text>
      <ion-text class="invoices-year-content-list-item__data invoices-organization body">{{ invoice.organizationId }}</ion-text>
      <ion-text class="invoices-year-content-list-item__data invoices-amount body">
        {{ $msFormatCurrency(invoice.price) }}
      </ion-text>
      <template v-if="invoicesData.type === DataType.CustomOrderDetails">
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
  </ion-list>
</template>

<script setup lang="ts">
import { DataType } from '@/services/bms';
import { getInvoiceStatusTranslationKey } from '@/services/translation';
import { IonItem, IonList, IonText } from '@ionic/vue';
import { MsImage, Download as downloaded, I18n } from 'megashark-lib';
import { InvoicesData } from '@/components/client-area/invoices/types';

defineProps<{
  invoicesData: InvoicesData;
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

  &-item {
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
