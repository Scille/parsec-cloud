<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-invoices">
    <!-- header -->
    <div class="invoices-header">
      <ion-text class="invoices-header-title title-h2">
        {{ $msTranslate('clientArea.invoices.title') }}
      </ion-text>
      <div
        class="invoices-header-filter"
        v-if="invoices.length > 0"
      >
        <ion-text class="invoices-header-filter__title body">{{ $msTranslate('clientArea.invoices.filter.title') }}</ion-text>
        <!-- year filter -->
        <ion-text
          class="invoices-header-filter-button button-medium"
          @click="openYearFilterPopover($event)"
          :class="{ selected: selectedYears.length > 0 }"
        >
          <ion-icon
            :icon="calendar"
            class="invoices-header-filter-button__calendar"
          />
          {{ $msTranslate('clientArea.invoices.filter.year') }}
          <ion-icon
            class="invoices-header-filter-button__chevron"
            :icon="chevronDown"
          />
        </ion-text>
        <!-- month filter -->
        <ion-text
          class="invoices-header-filter-button button-medium"
          @click="openMonthFilterPopover($event)"
          :class="{ selected: selectedMonths.length > 0 }"
        >
          <ion-icon
            :icon="calendar"
            class="invoices-header-filter-button__calendar"
          />
          {{ $msTranslate('clientArea.invoices.filter.month') }}
          <ion-icon
            class="invoices-header-filter-button__chevron"
            :icon="chevronDown"
          />
        </ion-text>
      </div>
    </div>

    <div
      class="selected-choice"
      v-if="selectedYears.length > 0 || selectedMonths.length > 0"
    >
      <ion-text
        v-for="selectedYear in selectedYears"
        :key="selectedYear"
        class="selected-choice-item subtitles-sm"
        @click="removeSelectedYear(selectedYear)"
      >
        {{ $msTranslate({ key: 'common.date.asIs', data: { date: selectedYear } }) }}
        <ion-icon
          class="selected-choice-item__icon"
          :icon="close"
        />
      </ion-text>
      <ion-text
        class="selected-choice-item subtitles-sm"
        v-for="selectedMonth in selectedMonths"
        :key="selectedMonth"
        @click="removeSelectedMonth(selectedMonth)"
      >
        {{ $msTranslate(getMonthName(selectedMonth)) }}
        <ion-icon
          class="selected-choice-item__icon"
          :icon="close"
        />
      </ion-text>
    </div>
    <!-- invoices -->
    <template v-if="invoices.length > 0 || querying">
      <div
        class="invoices-container"
        v-if="invoices.length > 0"
      >
        <invoices-container
          v-for="year in years"
          v-show="selectedYears.length === 0 || selectedYears.includes(year)"
          :key="year"
          :invoices="getInvoicesByYear(year)"
          :title="{ key: 'common.date.asIs', data: { date: year.toString() } }"
          :months-filter="selectedMonths"
        />
      </div>
      <!-- skeleton invoices -->
      <div
        class="skeleton"
        v-if="querying"
      >
        <!-- skeleton year -->
        <ion-skeleton-text
          :animated="true"
          class="skeleton-year"
        />

        <!-- skeleton row header -->
        <div class="skeleton-header-list">
          <ion-skeleton-text
            :animated="true"
            class="invoices-date"
          />
          <ion-skeleton-text
            :animated="true"
            class="invoices-number"
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

        <!-- skeleton row invoices -->
        <div class="skeleton-content-list">
          <div
            class="skeleton-content-list-item"
            v-for="index in 5"
            :key="index"
          >
            <ion-skeleton-text
              :animated="true"
              class="invoices-date"
            />
            <ion-skeleton-text
              :animated="true"
              class="invoices-number"
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
        </div>
      </div>
    </template>

    <!-- no invoices -->
    <template v-if="!querying && invoices.length === 0 && !error">
      <ion-text class="body-lg no-invoices">
        {{ $msTranslate('clientArea.invoices.noInvoice') }}
      </ion-text>
    </template>
    <span
      v-show="error"
      class="form-error"
    >
      {{ $msTranslate(error) }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { chevronDown, calendar, close } from 'ionicons/icons';
import { IonText, IonIcon, popoverController, IonSkeletonText } from '@ionic/vue';
import { BmsAccessInstance, BmsInvoice, BmsOrganization, DataType } from '@/services/bms';
import TimeFilterPopover from '@/components/client-area/TimeFilterPopover.vue';
import { ref, onMounted } from 'vue';
import { Info } from 'luxon';
import InvoicesContainer from '@/components/client-area/InvoicesContainer.vue';
import { MsOptions, Translatable, I18n } from 'megashark-lib';

defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Array<BmsInvoice>>([]);
const years = ref<Array<number>>([]);
const selectedYears = ref<Array<number>>([]);
const selectedMonths = ref<Array<number>>([]);
const error = ref<string>('');
const querying = ref(true);

async function openYearFilterPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const yearOptions = new MsOptions(
    years.value.map((year) => {
      return {
        key: year,
        label: { key: 'common.date.asIs', data: { date: year.toString() } },
      };
    }),
  );
  const popover = await popoverController.create({
    component: TimeFilterPopover,
    alignment: 'end',
    event: event,
    cssClass: 'time-filter-popover',
    showBackdrop: false,
    backdropDismiss: true,
    componentProps: {
      times: yearOptions,
      selected: selectedYears.value,
      sortDesc: true,
    },
  });
  await popover.present();
  await popover.onDidDismiss();
  await popover.dismiss();
}

async function openMonthFilterPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const monthOptions = new MsOptions(
    Info.months('short', { locale: I18n.getLocale() }).map((month, index) => {
      return {
        key: index + 1,
        label: { key: 'common.date.asIs', data: { date: month } },
      };
    }),
  );
  const popover = await popoverController.create({
    component: TimeFilterPopover,
    alignment: 'end',
    event: event,
    cssClass: 'time-filter-popover',
    showBackdrop: false,
    componentProps: {
      times: monthOptions,
      selected: selectedMonths.value,
    },
  });
  await popover.present();
  await popover.onDidDismiss();
  await popover.dismiss();
}

onMounted(async () => {
  const response = await BmsAccessInstance.get().getInvoices();
  if (!response.isError && response.data && response.data.type === DataType.Invoices) {
    invoices.value = response.data.invoices.sort((invoice1, invoice2) => {
      return invoice2.start.diff(invoice1.start).toMillis();
    });
    // get each different year of the invoices
    years.value = invoices.value.map((invoice) => invoice.start.year);
    years.value = Array.from(new Set(years.value));
  } else {
    error.value = 'clientArea.invoices.retrieveError';
  }
  querying.value = false;
});

function getInvoicesByYear(year: number): Array<BmsInvoice> {
  return invoices.value.filter((invoice) => invoice.start.year === year);
}

function getMonthName(month: number): Translatable | undefined {
  return { key: 'common.date.asIs', data: { date: Info.months('short', { locale: I18n.getLocale() })[month - 1] } };
}

function removeSelectedYear(year: number): void {
  const index = selectedYears.value.findIndex((y) => y === year);
  if (index !== -1) {
    selectedYears.value.splice(index, 1);
  }
}

function removeSelectedMonth(month: number): void {
  const index = selectedMonths.value.findIndex((m) => m === month);
  if (index !== -1) {
    selectedMonths.value.splice(index, 1);
  }
}
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

.invoices-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;

  &-title {
    color: var(--parsec-color-light-primary-700);
  }

  &-filter {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-left: auto;

    &__title {
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &-button {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      color: var(--parsec-color-light-secondary-hard-grey);
      padding: 0.375rem 0.625rem;
      border-radius: var(--parsec-radius-6);
      border: 1px solid var(--parsec-color-light-secondary-disabled);
      cursor: pointer;
      position: relative;

      &__calendar {
        color: var(--parsec-color-light-secondary-light);
      }

      &__chevron {
        color: var(--parsec-color-light-secondary-grey);
      }

      &:hover {
        color: var(--parsec-color-light-secondary-text);
        background: var(--parsec-color-light-secondary-background);

        .invoices-header-filter-button__calendar {
          color: var(--parsec-color-light-secondary-hard-grey);
        }

        .invoices-header-filter-button__chevron {
          color: var(--parsec-color-light-secondary-text);
        }
      }

      // selected
      &.selected::after {
        content: '';
        position: absolute;
        top: -4px;
        right: -4px;
        width: 0.625rem;
        height: 0.625rem;
        border-radius: var(--parsec-radius-12);
        background: var(--parsec-color-light-primary-500);
      }
    }
  }
}

.selected-choice {
  display: flex;
  gap: 0.75rem;
  width: 100%;
  flex-wrap: wrap;

  &-item {
    background-color: var(--parsec-color-light-secondary-medium);
    color: var(--parsec-color-light-secondary-text);
    align-self: center;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.25rem 0.25rem 0.625rem;
    border-radius: var(--parsec-radius-32);

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
      border-radius: var(--parsec-radius-12);
      padding: 0.125rem;
    }

    &:hover {
      cursor: pointer;

      .selected-choice-item__icon {
        color: var(--parsec-color-light-secondary-white);
        background-color: var(--parsec-color-light-secondary-text);
      }
    }
  }
}

.invoices-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  position: relative;
}

.skeleton {
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

  [class^='invoices-'] {
    height: 1rem;
    width: 100%;
    border-radius: var(--parsec-radius-8);
  }

  .invoices-date {
    max-width: calc(var(--max-width-date) - 1rem);
  }

  .invoices-number {
    max-width: calc(var(--max-width-date) - 1rem);
  }

  .invoices-organization {
    max-width: calc(var(--max-width-organization) - 1rem);
  }

  .invoices-amount {
    max-width: calc(var(--max-width-amount) - 1rem);
  }

  ion-skeleton-text {
    border-radius: var(--parsec-radius-8);
  }

  &-year {
    margin: 0.5rem 1rem;
    height: 1.5rem;
    width: 5rem;
  }

  &-header-list {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);
    position: relative;
    align-items: center;
  }

  &-content-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &-item {
      background: var(--parsec-color-light-secondary-white);
      display: flex;
      justify-content: space-between;
      gap: 0.5rem;
      border-radius: var(--parsec-radius-8);
      position: relative;
      align-items: center;
      padding: 0.5rem 1rem;
    }
  }
}
</style>
