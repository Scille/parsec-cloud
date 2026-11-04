<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-item class="invoices-year-content-list-item ion-no-padding">
    <ion-text class="invoices-year-content-list-item__data invoices-date">
      {{ $msTranslate(I18n.formatDate(invoice.getDate(), 'narrow')) }}
    </ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-number">{{ invoice.getNumber() }}</ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-organization">{{ invoice.getOrganizationId() }}</ion-text>
    <ion-text class="invoices-year-content-list-item__data invoices-amount">
      {{ $msFormatCurrency(invoice.getAmount()) }}
    </ion-text>
    <template v-if="invoice.getType() === InvoiceType.Sellsy">
      <ion-text
        class="invoices-year-content-list-item__data invoices-contract-period"
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
        class="invoices-year-content-list-item__separator"
      >
        /
      </ion-text>
    </template>
    <ion-text class="invoices-year-content-list-item__data invoices-status">
      <template v-if="invoice.getType() === InvoiceType.Sellsy">
        <span
          class="badge-status"
          :class="{
            paid: (invoice as SellsyInvoice).getDueAmount() > 0,
            toBePaid: (invoice as SellsyInvoice).getDueAmount() === 0,
          }"
        >
          {{ $msTranslate(getCustomOrderStatusInvoiceStep()) }}
        </span>
      </template>
      <template v-if="invoice.getType() === InvoiceType.Stripe">
        <span
          class="badge-status"
          :class="{
            paid: invoice.getStatus() === 'paid',
            toBePaid: invoice.getStatus() === 'open',
            unavailable: invoice.getStatus() === 'uncollectible' || invoice.getStatus() === 'draft' || invoice.getStatus() === 'void',
          }"
        >
          {{ $msTranslate(getInvoiceStatusTranslationKey(invoice.getStatus())) }}
        </span>
      </template>
      <ion-button
        fill="clear"
        class="invoices-year-content-list-item__download color-secondary"
        @click="Env.Links.openUrl(invoice.getLink())"
      >
        <ms-image
          :image="DownloadIcon"
          class="invoices-year-content-list-item__download-icon button-icon-left"
        />
        <span class="invoices-year-content-list-item__download-label">{{ $msTranslate('clientArea.invoices.cell.download') }}</span>
      </ion-button>
    </ion-text>
  </ion-item>
</template>

<script setup lang="ts">
import { Invoice, InvoiceType, SellsyInvoice } from '@/services/bms';
import { Env } from '@/services/environment';
import { getInvoiceStatusTranslationKey } from '@/services/translation';
import { IonButton, IonItem, IonText } from '@ionic/vue';
import { DownloadIcon, I18n, MsImage, Translatable } from 'megashark-lib';

const props = defineProps<{
  invoice: Invoice;
}>();

function getCustomOrderStatusInvoiceStep(): Translatable {
  if ((props.invoice as SellsyInvoice).getDueAmount() > 0) {
    return 'clientArea.invoices.status.paid';
  } else {
    return 'clientArea.invoices.status.toBePaid';
  }
}
</script>

<style lang="scss" scoped>
.invoices-year-content-list-item {
  background: ms.color('surface-base-default');
  display: flex;
  justify-content: space-between;
  --background: none;
  border-radius: ms.radius('lg');
  position: relative;
  --inner-padding-end: #{ms.spacing('padding-none')};
  align-items: center;

  &:hover {
    background: ms.color('surface-neutral-default-subtle-hover');
    .invoices-year-content-list-item__data {
      color: ms.color('text-base-heading');
    }
  }

  &__separator {
    @include ms.font('body-md-medium');
  }

  &__data {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
    align-self: stretch;
    display: flex;
    align-items: center;
    padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');

    &.invoices-date {
      @include ms.font('body-md-medium');
    }
  }

  &__download {
    &-icon {
      width: 1rem;
      --fill-color: #{ms.color('icon-neutral-default')};
      flex-shrink: 0;
    }

    &-label {
      @include ms.responsive-breakpoint('md') {
        display: none;
      }
    }
  }

  .badge-status {
    @include ms.font('body-sm-regular');
    border-radius: ms.radius('6xl');
    padding-inline: ms.spacing('padding-lg');
    flex-shrink: 0;
    background: ms.color('surface-disabled-default');
    color: ms.color('text-neutral-default');

    &.paid {
      background: ms.color('surface-success-default-subtle');
      color: ms.color('text-success-default');
    }

    &.toBePaid {
      background: ms.color('surface-information-default-subtle');
      color: ms.color('text-information-default');
    }

    &.unavailable {
      background: ms.color('surface-base-default-secondary');
      color: ms.color('text-neutral-default');
    }
  }
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
    gap: ms.spacing('gap-lg');
    flex-grow: 1;
    flex-basis: var(--max-width-date);
    min-width: 12rem;
  }
}
</style>
