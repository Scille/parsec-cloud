<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-invoices">
    <!-- header -->
    <div class="invoices-header">
      <ion-title class="invoices-header-title title-h2">
        {{ $msTranslate('clientArea.invoices.title') }}
      </ion-title>
      <div class="invoices-header-filter">
        <ion-text class="body-lg">
          {{ $msTranslate('clientArea.invoices.filter.year') }}
        </ion-text>
        <ion-text class="body-lg">
          {{ $msTranslate('clientArea.invoices.filter.month') }}
        </ion-text>
      </div>
    </div>

    <!-- invoices -->
    <div
      class="invoices-content"
      v-if="invoices"
    >
      <div
        class="invoices-main"
        v-for="year in invoicesByYear"
        :key="year.start.year"
        @click="toggleVisibility"
        :class="{ 'visible': isVisible }"
      >
        <!-- year -->
        <div class="invoices-main-year">
          <ion-text class="invoices-main-year__title title-h3">
            {{ year.start.year }}
          </ion-text>
          <ion-icon
            class="invoices-main-year__icon"
            :icon="chevronDown"
          />
        </div>

        <!-- row header -->
        <ion-list
          class="invoices-main-header-list ion-no-padding"
          v-if="isVisible"
        >
          <ion-item class="invoices-main-header-list-item invoices-date">
            <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.date') }}</ion-text>
          </ion-item>
          <ion-item class="invoices-main-header-list-item invoices-number">
            <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.number') }}</ion-text>
          </ion-item>
          <ion-item class="invoices-main-header-list-item invoices-organization">
            <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.organization') }}</ion-text>
          </ion-item>
          <ion-item class="invoices-main-header-list-item invoices-amount">
            <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.price.title') }}</ion-text>
          </ion-item>
          <ion-item class="invoices-main-header-list-item invoices-status">
            <ion-text class="menu-active">{{ $msTranslate('clientArea.invoices.cell.status') }}</ion-text>
          </ion-item>
        </ion-list>

        <!-- row invoices -->
        <ion-list
          class="invoices-main-content-list ion-no-padding"
          v-if="invoices.length > 0 && isVisible"
        >
          <ion-item
            class="invoices-main-content-list-item ion-no-padding"
            v-for="invoice in invoicesByYear"
            :key="invoice.id"
          >
            <ion-text class="invoices-main-content-list-item__data invoices-date subtitles-sm">
              {{ invoice.start.toFormat('LLLL yyyy') }}
            </ion-text>
            <ion-text class="invoices-main-content-list-item__data invoices-number body">{{ invoice.number }}</ion-text>
            <ion-text class="invoices-main-content-list-item__data invoices-organization body">{{ invoice.organizationId }}</ion-text>
            <ion-text class="invoices-main-content-list-item__data invoices-amount body">{{ invoice.total }}</ion-text>
            <ion-text class="invoices-main-content-list-item__data invoices-status">
              <span class="badge-status body-sm incoming">{{ invoice.status }}</span>
              <a
                class="custom-button custom-button-ghost button-medium"
                :href="invoice.pdfLink"
                download
              >
                <ion-icon :icon="download" />
                {{ $msTranslate('clientArea.invoices.cell.download') }}
              </a>
            </ion-text>
          </ion-item>
        </ion-list>

        <div v-if="invoices.length === 0 && isVisible">
          <ion-list
            class="invoices-list ion-no-padding"
            v-for="index in 3"
            :key="index"
          >
            <ion-item class="invoices-list-item">
              <div class="skeleton-loading">
                <ion-skeleton-text
                  :animated="true"
                  class="invoices-date"
                />
                <ion-skeleton-text
                  :animated="true"
                  class="invoices-organization"
                />
                <ion-skeleton-text
                  :animated="true"
                  class="invoices-amount"
                />
                <ion-skeleton-text
                  :animated="true"
                  class="invoices-status"
                />
              </div>
            </ion-item>
          </ion-list>
        </div>
      </div>
    </div>

    <!-- no invoices -->
    <ion-text
      class="body-lg"
      v-else
    >
      {{ $msTranslate('clientArea.dashboard.invoices.noInvoices') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { download, chevronDown } from 'ionicons/icons';
import { IonText, IonTitle, IonList, IonItem, IonSkeletonText, IonIcon } from '@ionic/vue';
import { BmsAccessInstance, BmsInvoice, BmsOrganization, DataType } from '@/services/bms';
import { ref, onMounted } from 'vue';
import { DateTime } from 'luxon';

defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Array<BmsInvoice>>([]);
const currentYear = DateTime.now().year;
const invoicesByYear = ref<Array<BmsInvoice>>([]);
const isVisible = ref(false);

function toggleVisibility() : void {
  isVisible.value = !isVisible.value;
}

// Add year filter

onMounted(async () => {
  const response = await BmsAccessInstance.get().getInvoices();
  if (!response.isError && response.data && response.data.type === DataType.Invoices) {
    invoices.value = response.data.invoices;
  }

  // Add year filter
  invoicesByYear.value = invoices.value.filter((invoice) => invoice.start.year === currentYear);
  console.log(invoicesByYear.value);
});
</script>

<style scoped lang="scss">
.client-page-invoices {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.invoices-header {
  display: flex;
  gap: 0.5rem;

  &-title {
    color: var(--parsec-color-light-primary-700);
  }

  &-filter {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
}

.invoices-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.invoices-main {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 0.5rem;
  border-radius: var(--parsec-radius-8);
  background: var(--parsec-color-light-secondary-premiere);
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;

  &-year {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 1rem;
    border-radius: var(--parsec-radius-6);
    cursor: pointer;

    &:hover {
      background: var(--parsec-color-light-secondary-background);
    }

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
      transform: rotate(0deg);
      transition: transform 0.20s;
    }
  }

  &.visible {
    padding: 1rem 0.5rem;

    .invoices-main-year__icon {
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
    }
  }

  &-header-list {
    display: flex;
    width: 100%;
    background: none;

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
        .invoices-main-content-list-item__data {
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

        &.incoming {
          background: var(--parsec-color-light-info-100);
          color: var(--parsec-color-light-info-700);
        }

        &.paid {
          background: var(--parsec-color-light-success-100);
          color: var(--parsec-color-light-success-700);
        }
      }
    }

    .skeleton-loading {
      display: flex;
      width: 100%;
      gap: 0.5rem;

      [class^='invoices-'] {
        height: 1rem;
        border-radius: var(--parsec-radius-8);
      }

      .invoices-date {
        max-width: calc(var(--max-width-date) - 1rem);
      }

      .invoices-organization {
        max-width: calc(var(--max-width-organization) - 1rem);
      }

      .invoices-amount {
        max-width: calc(var(--max-width-amount) - 1rem);
      }
    }
  }
}
</style>
