<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-invoices">
    <!-- header -->
    <div class="invoices-header">
      <ion-text class="invoices-header-title title-h2">
        {{ $msTranslate('clientArea.invoices.title') }}
      </ion-text>
    </div>

    <!-- invoices -->
    <template v-if="invoices.length > 0 && !querying">
      <div class="invoices-container">
        <ion-list class="ion-no-padding">
          <ion-item
            class="ion-no-padding"
            v-for="invoice in invoices"
            :key="invoice.id"
          >
            <ion-text>{{ invoice.number }}</ion-text>
            <ion-text
              class="subtitles-sm"
              v-if="invoice.licenseStart && invoice.licenseEnd"
            >
              {{ $msTranslate(I18n.formatDate(invoice.licenseStart, 'narrow')) }}
              {{ $msTranslate(I18n.formatDate(invoice.licenseEnd, 'narrow')) }}
            </ion-text>
            <ion-text
              v-else
              class="subtitles-sm"
            >
              /
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
                :href="invoice.link"
                download
              >
                <ms-image
                  :image="Download"
                  class="custom-button__icon"
                />
                {{ $msTranslate('clientArea.invoices.cell.download') }}
              </a>
            </ion-text>
          </ion-item>
        </ion-list>
      </div>
    </template>
    <template v-if="!querying && invoices.length === 0 && !error">
      <ion-text class="body-lg no-invoices">
        {{ $msTranslate('clientArea.invoices.noInvoice') }}
      </ion-text>
    </template>
    <template v-if="querying">
      <!-- skeleton invoices -->
      <div class="skeleton">
        <ion-skeleton-text
          v-for="index in 3"
          :key="index"
          :animated="true"
        />
      </div>
    </template>
    <span
      v-show="error"
      class="form-error body"
    >
      {{ $msTranslate(error) }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon, IonSkeletonText, IonList, IonItem } from '@ionic/vue';
import { BmsAccessInstance, BmsOrganization, CustomOrderDetailsResultData, DataType } from '@/services/bms';
import { ref, onMounted } from 'vue';
import { getInvoiceStatusTranslationKey } from '@/services/translation';
import { I18n, Download, MsImage } from 'megashark-lib';

const props = defineProps<{
  organization: BmsOrganization;
}>();

const invoices = ref<Array<CustomOrderDetailsResultData>>([]);
const querying = ref(true);
const error = ref('');

onMounted(async () => {
  querying.value = true;
  const response = await BmsAccessInstance.get().getCustomOrderInvoices(props.organization);
  if (!response.isError && response.data && response.data.type === DataType.CustomOrderInvoices) {
    invoices.value = response.data.invoices.sort((invoice1, invoice2) => invoice2.created.diff(invoice1.created).toMillis());
  } else {
    error.value = 'clientArea.invoices.retrieveError';
  }
  querying.value = false;
});
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
  background: var(--parsec-color-light-secondary-premiere);
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;
  border-radius: var(--parsec-radius-12);

  [class^='invoices-'] {
    height: 1rem;
    width: 100%;
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
