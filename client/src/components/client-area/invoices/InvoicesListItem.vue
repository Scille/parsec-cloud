<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item class="invoices-year-content-list-item ion-no-padding">
    <ion-text class="invoices-year-content-list-item__data invoices-date subtitles-sm">
      {{ $msTranslate(I18n.formatDate(invoice.getDate(), 'narrow')) }}
    </ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-number body">{{ invoice.getNumber() }}</ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-organization body">{{ invoice.getOrganizationId() }}</ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-amount body">
      {{ $msFormatCurrency(invoice.getAmount()) }}
    </ion-text>
    <template v-if="invoice.getType() === InvoiceType.Sellsy">
      <ion-text
        class="invoices-year-content-list-item__data invoices-contract-period body"
        v-if="(invoice as SellsyInvoice).getLicenseStart() && (invoice as SellsyInvoice).getLicenseEnd()"
      >
        {{
          $msTranslate({
            key: 'clientArea.invoices.cell.contractPeriod.fromTo',
            data: {
              fromDate: $msTranslate(I18n.formatDate((invoice as SellsyInvoice).getLicenseStart()!, 'narrow')),
              toDate: $msTranslate(I18n.formatDate((invoice as SellsyInvoice).getLicenseEnd()!, 'narrow')),
            },
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
        :class="invoice.getStatus()"
      >
        {{ $msTranslate(getInvoiceStatusTranslationKey(invoice.getStatus())) }}
      </span>
      <a
        class="custom-button custom-button-ghost button-medium"
        @click="Env.Links.openUrl(invoice.getLink())"
        download
      >
        <ms-image
          :image="DownloadIcon"
          class="custom-button__icon"
        />
        {{ $msTranslate('clientArea.invoices.cell.download') }}
      </a>
    </ion-text>
  </ion-item>
</template>

<script setup lang="ts">
import { Invoice, InvoiceType, SellsyInvoice } from '@/services/bms';
import { Env } from '@/services/environment';
import { getInvoiceStatusTranslationKey } from '@/services/translation';
import { IonItem, IonText } from '@ionic/vue';
import { MsImage, DownloadIcon, I18n } from 'megashark-lib';

defineProps<{
  invoice: Invoice;
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
    flex-shrink: 0;
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
