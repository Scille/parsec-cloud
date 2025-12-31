<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- header -->
  <div class="invoices-header">
    <ion-text class="invoices-header-title title-h2">
      {{ $msTranslate(title) }}
    </ion-text>
    <div
      class="invoices-header-filter"
      v-if="invoices.size > 0"
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
  <template v-if="invoices.size > 0 || querying">
    <div
      class="invoices-container"
      v-show="!querying"
    >
      <year-invoices-list-item
        v-for="year in invoices.keys()"
        v-show="selectedYears.length === 0 || selectedYears.includes(year)"
        :key="year"
        :invoices="filterInvoicesByYear(year)"
        :title="{ key: 'common.date.asIs', data: { date: year.toString() } }"
        :months-filter="selectedMonths"
      />
    </div>

    <!-- skeleton invoices -->
    <year-invoices-list-skeleton v-if="querying" />
  </template>

  <!-- no invoices -->
  <template v-if="!querying && invoices.size === 0 && !error">
    <ion-text class="body-lg no-invoices">
      {{ $msTranslate('clientArea.invoices.noInvoice') }}
    </ion-text>
  </template>
  <span
    v-show="error"
    class="form-error body"
  >
    {{ $msTranslate(error) }}
  </span>
</template>

<script setup lang="ts">
import TimeFilterPopover from '@/components/client-area/TimeFilterPopover.vue';
import YearInvoicesListItem from '@/components/client-area/invoices/YearInvoicesListItem.vue';
import YearInvoicesListSkeleton from '@/components/client-area/invoices/YearInvoicesListSkeleton.vue';
import { Invoice } from '@/services/bms';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { calendar, chevronDown, close } from 'ionicons/icons';
import { Info } from 'luxon';
import { I18n, MsOptions, Translatable } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  invoices: Map<number, Array<Invoice>>;
  title: Translatable;
  querying: boolean;
  error?: Translatable;
}>();

const selectedYears = ref<Array<number>>([]);
const selectedMonths = ref<Array<number>>([]);

async function openYearFilterPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const years = Array.from(props.invoices.keys());
  const yearOptions = new MsOptions(
    years.map((year) => {
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

function filterInvoicesByYear(year: number): Array<Invoice> {
  return props.invoices.get(year) || [];
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

.invoices-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex-wrap: wrap;

  &-title {
    color: var(--parsec-color-light-primary-700);
    width: fit-content;
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
</style>
