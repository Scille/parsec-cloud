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
      v-if="invoices"
    >
      <div
        class="invoices-year"
        v-for="year in years"
        :key="year"
        @click="toggleVisibility()"
        :class="{ 'visible': isVisible }"
      >
        <!-- year -->
        <div class="invoices-year-text">
          <ion-text class="invoices-year-text__title title-h3">
            {{ year }}
          </ion-text>
          <ion-icon
            class="invoices-year-text__icon"
            :icon="chevronDown"
          />
        </div>

        <!-- row header -->
        <ion-list
          class="invoices-year-header-list ion-no-padding"
          v-if="isVisible"
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
            v-for="invoice in getInvoicesByYear(year)"
            :key="invoice.id"
          >
            <ion-text class="invoices-year-content-list-item__data invoices-date subtitles-sm">
              {{ invoice.start.toFormat('LLLL yyyy') }}
            </ion-text>
            <ion-text class="invoices-year-content-list-item__data invoices-number body">{{ invoice.number }}</ion-text>
            <ion-text class="invoices-year-content-list-item__data invoices-organization body">{{ invoice.organizationId }}</ion-text>
            <ion-text class="invoices-year-content-list-item__data invoices-amount body">
              {{ invoice.total }}
            </ion-text>
            <ion-text class="invoices-year-content-list-item__data invoices-status">
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
      {{ $msTranslate('clientArea.invoices.noInvoice') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { download, chevronDown, calendar } from 'ionicons/icons';
import { IonText, IonTitle, IonList, IonItem, IonSkeletonText, IonIcon, popoverController } from '@ionic/vue';
import { BmsAccessInstance, BmsInvoice, BmsOrganization, DataType } from '@/services/bms';
import TimeFilterPopover from '@/components/bms/TimeFilterPopover.vue';
import { ref, onMounted } from 'vue';
import { Info } from 'luxon';

defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Array<BmsInvoice>>([]);
const isVisible = ref(true);
const years = ref<Array<number>>([]);
const months = ref(Info.months('short'));

function toggleVisibility() : void {
  isVisible.value = !isVisible.value;
}

async function openYearFilterPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const popover = await popoverController.create({
    component: TimeFilterPopover,
    alignment: 'end',
    event: event,
    cssClass: 'time-filter-popover',
    showBackdrop: false,
    componentProps: {
      times: years.value,
    },
  });
  await popover.present();
  const result = await popover.onDidDismiss();
  console.log(result);
  await popover.dismiss();
}

async function openMonthFilterPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const popover = await popoverController.create({
    component: TimeFilterPopover,
    alignment: 'end',
    event: event,
    cssClass: 'time-filter-popover',
    showBackdrop: false,
    componentProps: {
      times: months.value,
    },
  });
  await popover.present();
  const { role, data } = await popover.onDidDismiss();
  await popover.dismiss();
}

onMounted(async () => {
  const response = await BmsAccessInstance.get().getInvoices();
  if (!response.isError && response.data && response.data.type === DataType.Invoices) {
    invoices.value = response.data.invoices;
  }

  // get each different year of the invoices
  years.value = invoices.value.map((invoice) => invoice.start.year);
  years.value = Array.from(new Set(years.value));
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
    transition: all 0.20s ease;

    &:hover {
      background: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-6);

      .invoices-year-text__title {
        color: var(--parsec-color-light-primary-500);
      }
    }

    &__title {
      color: var(--parsec-color-light-secondary-text);
      transition: all 0.20s ease;
    }

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
      transform: rotate(0deg);
      transition: transform 0.20s;
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
