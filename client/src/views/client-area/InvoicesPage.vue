<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-invoices">
    <!-- header -->
    <div class="invoices-header">
      <ion-title class="invoices-header-title title-h2">
        {{ $msTranslate('clientArea.invoices.title') }}
      </ion-title>
      <div
        class="invoices-header-filter"
        v-if="invoices"
      >
        <!-- year filter -->
        <ion-text
          class="invoices-header-filter-button button-medium"
          @click="openYearFilterPopover($event)"
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

    <!-- invoices -->
    <div
      class="invoices-container"
      v-if="invoices.length"
    >
      <invoices-container
        v-for="year in years"
        v-show="selectedYear === undefined || selectedYear === year"
        :key="year"
        :invoices="getInvoicesByYear(year)"
        :title="{ key: 'common.date.asIs', data: { date: year.toString() }}"
        :month-filter="selectedMonth"
      />
    </div>

    <!-- no invoices -->
    <ion-text
      class="body-lg"
      v-else
    >
      {{ $msTranslate('clientArea.invoices.noInvoice') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { chevronDown, calendar } from 'ionicons/icons';
import { IonText, IonTitle, IonIcon, popoverController } from '@ionic/vue';
import { BmsAccessInstance, BmsInvoice, BmsOrganization, DataType } from '@/services/bms';
import TimeFilterPopover from '@/components/client-area/TimeFilterPopover.vue';
import { ref, onMounted } from 'vue';
import { Info } from 'luxon';
import InvoicesContainer from '@/components/client-area/InvoicesContainer.vue';
import { MsModalResult, MsOptions } from 'megashark-lib';

defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Array<BmsInvoice>>([]);
const years = ref<Array<number>>([]);
const selectedYear = ref<number | undefined>(undefined);
const selectedMonth = ref<number | undefined>(undefined);

async function openYearFilterPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const yearOptions = new MsOptions(
    years.value.map((year) => {
      return {
        key: year,
        label: year.toString(),
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
      times: yearOptions,
      selected: selectedYear.value,
    },
  });
  await popover.present();
  const { data, role } = await popover.onDidDismiss();
  if (role === MsModalResult.Confirm) {
    selectedYear.value = data.selected;
  }
  await popover.dismiss();
}

async function openMonthFilterPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const monthOptions = new MsOptions(
    Info.months('short').map((month, index) => {
      return {
        key: index + 1,
        label: month,
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
      selected: selectedMonth.value,
    },
  });
  await popover.present();
  const { data, role } = await popover.onDidDismiss();
  if (role === MsModalResult.Confirm) {
    selectedMonth.value = data.selected;
  }
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
  }
});

function getInvoicesByYear(year: number) : Array<BmsInvoice> {
  return invoices.value.filter((invoice) => invoice.start.year === year);
}
</script>

<style scoped lang="scss">
* {
  transition: all 0.20s ease;
}

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

    &-button {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      color: var(--parsec-color-light-secondary-hard-grey);
      padding: 0.375rem 0.625rem;
      border-radius: var(--parsec-radius-6);
      cursor: pointer;

      &__calendar {
        color: var(--parsec-color-light-secondary-light);
      }

      &__chevron {
        color: var(--parsec-color-light-secondary-grey);
      }

      &:hover {
        color: var(--parsec-color-light-secondary-text);
        background: var(--parsec-color-light-primary-100);

        .invoices-header-filter-button__calendar {
          color: var(--parsec-color-light-primary-700);
        }

        .invoices-header-filter-button__chevron {
          color: var(--parsec-color-light-secondary-text);
        }
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
