<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="invoices-year"
    :class="{ visible: isVisible }"
  >
    <!-- year -->
    <div
      class="invoices-year-text"
      @click="isVisible = !isVisible"
    >
      <ion-text class="invoices-year-text__title title-h3">
        {{ $msTranslate(title) }}
      </ion-text>
      <ion-icon
        class="invoices-year-text__icon"
        :icon="chevronDown"
      />
    </div>

    <!-- row header -->
    <ion-list
      class="invoices-year-header-list ion-no-padding"
      v-if="invoices.length > 0 && isVisible"
    >
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
      <ion-item class="invoices-year-header-list-item invoices-status">
        <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.status') }}</ion-text>
      </ion-item>
    </ion-list>

    <!-- row invoices -->
    <ion-list
      class="invoices-year-content-list ion-no-padding"
      v-if="invoices.length > 0 && isVisible"
    >
      <ion-item
        class="invoices-year-content-list-item ion-no-padding"
        v-for="invoice in invoices"
        :key="invoice.id"
        v-show="monthsFilter === undefined || monthsFilter.length === 0 || monthsFilter.includes(invoice.start.month)"
      >
        <ion-text class="invoices-year-content-list-item__data invoices-date subtitles-sm">
          {{ $msTranslate(I18n.formatDate(invoice.start, 'narrow')) }}
        </ion-text>
        <ion-text class="invoices-year-content-list-item__data invoices-number body">{{ invoice.number }}</ion-text>
        <ion-text class="invoices-year-content-list-item__data invoices-organization body">{{ invoice.organizationId }}</ion-text>
        <ion-text class="invoices-year-content-list-item__data invoices-amount body">
          {{ $msFormatCurrency(invoice.total) }}
        </ion-text>
        <ion-text class="invoices-year-content-list-item__data invoices-status">
          <span
            class="badge-status body-sm"
            :class="invoice.status"
          >
            {{ $msTranslate(getInvoiceStatusTranslationKey(invoice.status)) }}
          </span>
          <a
            class="custom-button custom-button-ghost button-medium"
            :href="invoice.pdfLink"
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
  </div>
</template>

<script setup lang="ts">
import { getInvoiceStatusTranslationKey } from '@/services/translation';
import { BmsInvoice } from '@/services/bms';
import { IonItem, IonList, IonText, IonIcon } from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
import { MsImage, Download as downloaded, Translatable, I18n } from 'megashark-lib';
import { ref } from 'vue';

defineProps<{
  invoices: Array<BmsInvoice>;
  title: Translatable;
  monthsFilter?: Array<number>;
}>();

const isVisible = ref(true);
</script>

<style scoped lang="scss">
.invoices-year {
  display: flex;
  flex-direction: column;
  padding: 0.5rem;
  margin-top: 0.5rem;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-premiere);
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;

  &:last-of-type {
    margin-bottom: 3rem;
  }

  &-text {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    cursor: pointer;
    position: sticky;
    top: -2rem;
    background: var(--parsec-color-light-secondary-premiere);
    z-index: 3;
    transition: all 0.2s ease;

    &:hover {
      background: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-6);

      .invoices-year-text__title {
        color: var(--parsec-color-light-primary-500);
      }
    }

    &__title {
      color: var(--parsec-color-light-secondary-text);
      transition: all 0.2s ease;
    }

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
      transform: rotate(0deg);
      transition: transform 0.2s;
    }
  }

  &.visible {
    padding: 0.5rem;

    .invoices-year-text__icon {
      transform: rotate(180deg);
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

    &-status {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.5rem;
    }
  }

  &-header-list {
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

  &-content-list {
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
}
</style>
