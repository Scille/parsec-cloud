<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-dashboard">
    <!-- summary + invoices section -->
    <div class="dashboard-section-container">
      <!-- month summary -->
      <div class="dashboard-section month-summary-container">
        <ion-title class="dashboard-section-title title-h2">
          {{ I18n.translate(I18n.formatDate(currentDate, 'short')) }}
        </ion-title>
        <div class="month-summary">
          <!-- amount -->
          <div
            class="month-summary-item"
            v-if="estimations"
          >
            <ion-title class="month-summary-item__title title-h4">
              {{ $msTranslate('clientArea.dashboard.summary.estimatedAmount') }}
            </ion-title>
            <div class="month-summary-item__data title-h1-xl">
              <div
                class="data-content"
              >
                <ion-text class="data-content-text">
                  {{
                    $msTranslate({
                      key: 'clientArea.dashboard.invoices.price.amount',
                      data: { amount: estimations.amount },
                    })
                  }}
                  {{ estimations.amount }}
                </ion-text>
                <ms-information-tooltip
                  text="clientArea.dashboard.summary.withdrawalDescription"
                  slot="end"
                  class="data-content-icon"
                />
              </div>
              <span class="data-info title-h4 button-medium">
                {{
                  $msTranslate({
                    key: 'clientArea.dashboard.summary.withdrawal',
                    data: { date: I18n.translate(I18n.formatDate(currentDate.endOf('month'), 'short')) },
                  })
                }}
              </span>
            </div>
          </div>

          <!-- users -->
          <div class="month-summary-item">
            <ion-title class="month-summary-item__title title-h4">
              {{ $msTranslate('clientArea.dashboard.summary.users') }}
            </ion-title>
            <div
              class="month-summary-item__data title-h1-xl"
              v-if="stats"
            >
              {{ stats.users }}
            </div>
          </div>

          <!-- storage -->
          <div class="month-summary-item">
            <ion-title class="month-summary-item__title title-h4">
              {{ $msTranslate('clientArea.dashboard.summary.storage') }}
            </ion-title>
            <div
              class="month-summary-item__data title-h1-xl"
              v-if="stats"
            >
              {{ stats.dataSize }}
            </div>
          </div>
        </div>
      </div>

      <!-- recent invoices -->
      <div class="dashboard-section invoices-container">
        <ion-title class="dashboard-section-title title-h2">
          {{ $msTranslate('clientArea.dashboard.invoices.title') }}
        </ion-title>
        <div
          class="invoices"
          v-if="invoices"
        >
          <div class="invoices-header">
            <ion-list class="invoices-header-list ion-no-padding">
              <ion-item class="invoices-header-list-item invoices-date">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.date') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-organization">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.organization') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-amount">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.price.title') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-status">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.status') }}</ion-text>
              </ion-item>
            </ion-list>
          </div>

          <div class="invoices-content">
            <ion-list
              class="invoices-list ion-no-padding"
              v-if="invoices.length > 0"
            >
              <ion-item
                class="invoices-list-item ion-no-padding"
                v-for="invoice in invoices"
                :key="invoice.id"
              >
                <ion-text class="invoices-list-item__data invoices-date subtitles-sm">{{ invoice.start.toFormat('LLLL yyyy') }}</ion-text>
                <ion-text class="invoices-list-item__data invoices-organization body">{{ invoice.organizationId }}</ion-text>
                <ion-text class="invoices-list-item__data invoices-amount body">{{ invoice.total }}</ion-text>
                <ion-text class="invoices-list-item__data invoices-status">
                  <span class="badge-status body-sm incoming">{{ invoice.status }}</span>
                  <ion-text class="custom-button custom-button-ghost button-medium">
                    <ion-icon :icon="download" />
                    {{ $msTranslate('clientArea.dashboard.invoices.download') }}
                  </ion-text>
                </ion-text>
              </ion-item>
            </ion-list>

            <ion-list
              v-else
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
        <ion-text
          class="body-lg"
          v-else
        >
          {{ $msTranslate('clientArea.dashboard.invoices.noInvoices') }}
        </ion-text>
      </div>
    </div>

    <!-- payment section -->
    <div class="dashboard-section payment-container">
      <div class="dashboard-section-title">
        <ion-title class="title-h2">{{ $msTranslate('clientArea.dashboard.payment.title') }}</ion-title>
        <ion-text class="payment-text body">
          {{ $msTranslate('clientArea.dashboard.payment.description') }}
        </ion-text>
      </div>
      <div class="payment">
        <ms-stripe-card-details
          v-if="defaultCard"
          :card="defaultCard"
          class="payment-card"
        />
        <div
          class="skeleton-loading"
          v-else
        >
          <ion-skeleton-text
            :animated="true"
            class="skeleton-loading-card"
          />
        </div>
        <ion-text
          class="custom-button custom-button-fill button-medium"
          @click="$emit('switchPageRequest', ClientAreaPages.PaymentMethods)"
        >
          {{ $msTranslate('clientArea.dashboard.payment.update') }}
        </ion-text>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonTitle, IonText, IonList, IonItem, IonIcon, IonSkeletonText } from '@ionic/vue';
import { download } from 'ionicons/icons';
import { ClientAreaPages } from '@/views/client-area/types';
import {
  BmsAccessInstance,
  DataType,
  BmsOrganization,
  OrganizationStatsResultData,
  BmsInvoice,
  BillingDetailsPaymentMethodCard,
  PaymentMethod,
} from '@/services/bms';
import { MsInformationTooltip, I18n, MsStripeCardDetails } from 'megashark-lib';
import { onMounted, ref } from 'vue';
import { PaymentMethod as MsPaymentMethod } from 'megashark-lib';
import { DateTime } from 'luxon';

const props = defineProps<{
  organization: BmsOrganization;
}>();

defineEmits<{
  (e: 'switchPageRequest', page: ClientPages);
}>();

const stats = ref<OrganizationStatsResultData | undefined>(undefined);
const invoices = ref<Array<BmsInvoice>>([]);
const defaultCard = ref<MsPaymentMethod.Card | undefined>(undefined);
const currentDate = DateTime.now();
// TODO: retrieve this from the backend if it ever becomes available
const estimations = ref<undefined | { amount: number }>(undefined);

onMounted(async () => {
  const orgStatsResponse = await BmsAccessInstance.get().getOrganizationStats(props.organization.bmsId);
  if (!orgStatsResponse.isError && orgStatsResponse.data && orgStatsResponse.data.type === DataType.OrganizationStats) {
    stats.value = orgStatsResponse.data;
  }

  const invoicesResponse = await BmsAccessInstance.get().getInvoices();
  if (!invoicesResponse.isError && invoicesResponse.data && invoicesResponse.data.type === DataType.Invoices) {
    invoices.value = invoicesResponse.data.invoices.filter((invoice) => invoice.organizationId === props.organization.parsecId);
  }

  const billingDetailsResponse = await BmsAccessInstance.get().getBillingDetails();
  if (!billingDetailsResponse.isError && billingDetailsResponse.data && billingDetailsResponse.data.type === DataType.BillingDetails) {
    const card = billingDetailsResponse.data.paymentMethods.find(
      (method) => method.isDefault && method.type === PaymentMethod.Card,
    ) as BillingDetailsPaymentMethodCard;
    if (card) {
      defaultCard.value = {
        last4: card.lastDigits,
        // eslint-disable-next-line camelcase
        exp_year: card.expirationDate.year,
        // eslint-disable-next-line camelcase
        exp_month: card.expirationDate.month,
        brand: card.brand,
      } as MsPaymentMethod.Card;
    }
  }
});
</script>

<style scoped lang="scss">
.client-page-dashboard {
  display: flex;
  gap: 1.5rem;
}

// summary + invoices section
.dashboard-section-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 60rem;
  width: 100%;
}

//payment section
.payment-container {
  max-width: 20rem;
  height: fit-content;
}

//every sections
.dashboard-section {
  background: var(--parsec-color-light-secondary-inversed-contrast);
  display: flex;
  flex-direction: column;
  gap: 2rem;
  padding: 1.5rem 2rem;
  border-radius: var(--parsec-radius-12);
  border: 1px solid var(--parsec-color-light-secondary-premiere);

  &-title {
    color: var(--parsec-color-light-primary-700);
  }
}

.month-summary {
  display: flex;

  &-item {
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 100%;
    height: min-content;
    gap: 0.75rem;

    &__title {
      height: fit-content;
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &__data {
      display: flex;
      flex-direction: column;
      gap: 0.625rem;
      color: var(--parsec-color-light-secondary-contrast);

      .data-content {
        display: flex;
        gap: 0.5rem;
        align-items: center;

        &-text {
          color: var(--parsec-color-light-secondary-contrast);
        }

        &-icon {
          color: var(--parsec-color-light-secondary-grey);
          font-size: 1.5rem;
          cursor: pointer;

          &:hover {
            color: var(--parsec-color-light-secondary-text);
          }
        }
      }

      .data-info {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}

.invoices {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  gap: 0.5rem;
  --max-width-date: 10rem;
  --max-width-organization: 12rem;
  --max-width-amount: 5.625rem;

  &-date {
    width: 100%;
    max-width: var(--max-width-date);
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

  &-header {
    display: flex;
    justify-content: space-between;
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);

    .invoices-header-list {
      display: flex;
      padding: 0.75rem 0.5rem;
      width: 100%;

      &-item {
        --padding-start: 0;
        --padding-end: 0;
        --padding-top: 0;
        --padding-bottom: 0;
        --background: none;
        color: var(--parsec-color-light-secondary-soft-text);
      }
    }
  }

  &-content {
    display: flex;
    flex-direction: column;

    .invoices-list {
      display: flex;
      flex-direction: column;
      --ion-safe-area-left: 0;

      &-item {
        display: flex;
        justify-content: space-between;
        --background: none;
        padding: 0.75rem 0.5rem;
        border-radius: var(--parsec-radius-8);
        position: relative;

        &:hover {
          .invoices-list-item__data {
            color: var(--parsec-color-light-secondary-contrast);
          }
        }

        &:not(:last-child) {
          &::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
          }
        }

        &__data {
          color: var(--parsec-color-light-secondary-hard-grey);

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
}

.payment-container .dashboard-section-title {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.payment {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &-text {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-card {
    margin-bottom: 1rem;
  }

  .skeleton-loading-card {
    width: 15.625rem;
    height: 8.625rem;
    border-radius: var(--parsec-radius-12);
  }
}
</style>
