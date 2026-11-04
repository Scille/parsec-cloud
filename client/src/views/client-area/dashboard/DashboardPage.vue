<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="client-page-dashboard">
    <!-- summary + invoices section -->
    <div class="dashboard-section-container">
      <!-- month summary -->
      <div
        class="dashboard-section month-summary-container"
        v-if="stats"
      >
        <ion-title class="dashboard-section-title">
          {{ I18n.translate(I18n.formatDate(currentDate, 'narrow')) }}
        </ion-title>
        <div class="month-summary">
          <!-- amount -->
          <div
            class="month-summary-item"
            v-if="estimations"
          >
            <ion-title class="month-summary-item__title">
              {{ $msTranslate('clientArea.dashboard.summary.estimatedAmount') }}
            </ion-title>
            <div class="month-summary-item__data">
              <div class="data-content">
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
              <span class="data-info">
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
            <ion-title class="month-summary-item__title">
              {{ $msTranslate('clientArea.dashboard.summary.users') }}
            </ion-title>
            <div class="month-summary-item__data">
              {{ stats.activeUsers }}
            </div>
          </div>

          <!-- storage -->
          <div class="month-summary-item">
            <ion-title class="month-summary-item__title">
              {{ $msTranslate('clientArea.dashboard.summary.storage') }}
            </ion-title>
            <div class="month-summary-item__data">
              {{ $msTranslate(formatFileSize(stats.dataSize)) }}
            </div>
          </div>
        </div>
      </div>

      <!-- recent invoices -->
      <div class="dashboard-section invoices-container">
        <ion-title class="dashboard-section-title">
          {{ $msTranslate('clientArea.dashboard.invoices.title') }}
        </ion-title>
        <div
          class="invoices"
          v-if="(invoices && invoices.length) || querying"
        >
          <div class="invoices-header">
            <ion-list class="invoices-header-list ion-no-padding">
              <ion-item class="invoices-header-list-item invoices-date">
                <ion-text class="invoices-header-list-item__text">{{ $msTranslate('clientArea.dashboard.invoices.date') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-organization">
                <ion-text class="invoices-header-list-item__text">
                  {{ $msTranslate('clientArea.dashboard.invoices.organization') }}
                </ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-amount">
                <ion-text class="invoices-header-list-item__text">{{ $msTranslate('clientArea.dashboard.invoices.price.title') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-status">
                <ion-text class="invoices-header-list-item__text">{{ $msTranslate('clientArea.dashboard.invoices.status') }}</ion-text>
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
                :key="invoice.getId()"
              >
                <ion-text class="invoices-list-item__data invoices-list-item__date invoices-date">
                  {{ $msTranslate(formatTimeSince(invoice.getDate(), '--', 'short')) }}
                </ion-text>
                <ion-text class="invoices-list-item__data invoices-organization invoices-list-item__text">
                  {{ invoice.getOrganizationId() }}
                </ion-text>
                <ion-text class="invoices-list-item__data invoices-amount invoices-list-item__text">
                  {{ $msFormatCurrency(invoice.getAmount()) }}
                </ion-text>
                <ion-text class="invoices-list-item__data invoices-status">
                  <span
                    class="badge-status"
                    :class="{
                      paid: invoice.getStatus() === 'paid',
                      toBePaid: invoice.getStatus() === 'open',
                    }"
                  >
                    {{ $msTranslate(getInvoiceStatusTranslationKey(invoice.getStatus())) }}
                  </span>
                  <ion-button
                    fill="clear"
                    class="invoices-list-item__download color-secondary"
                    @click="Env.Links.openUrl(invoice.getLink())"
                  >
                    <ms-image
                      :image="DownloadIcon"
                      class="invoices-list-item__download-icon button-icon-left"
                    />
                    <span class="invoices-list-item__download-label">{{ $msTranslate('clientArea.dashboard.invoices.download') }}</span>
                  </ion-button>
                </ion-text>
              </ion-item>
            </ion-list>

            <ion-list
              v-else-if="querying"
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
          class="no-invoices"
          v-else-if="(!invoices || !invoices.length) && !querying"
        >
          {{ $msTranslate('clientArea.dashboard.invoices.noInvoices') }}
        </ion-text>
      </div>
    </div>

    <!-- payment section -->
    <div class="dashboard-section payment-container">
      <div class="dashboard-section-title">
        <ion-title class="payment-container__title">{{ $msTranslate('clientArea.dashboard.payment.title') }}</ion-title>
        <ion-text class="payment-text">
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
        <ion-button
          fill="outline"
          class="payment-update-button color-secondary"
          @click="$emit('switchPageRequest', ClientAreaPages.PaymentMethods)"
        >
          {{ $msTranslate('clientArea.dashboard.payment.update') }}
        </ion-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import {
  BillingDetailsPaymentMethodCard,
  BmsAccessInstance,
  BmsOrganization,
  DataType,
  OrganizationStatsResultData,
  PaymentMethod,
  StripeInvoice,
} from '@/services/bms';
import { Env } from '@/services/environment';
import { getInvoiceStatusTranslationKey } from '@/services/translation';
import { ClientAreaPages, isDefaultOrganization } from '@/views/client-area/types';
import { IonButton, IonItem, IonList, IonSkeletonText, IonText, IonTitle } from '@ionic/vue';
import { DateTime } from 'luxon';
import {
  DownloadIcon,
  I18n,
  MsImage,
  MsInformationTooltip,
  PaymentMethod as MsPaymentMethod,
  MsStripeCardDetails,
  formatTimeSince,
} from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  organization: BmsOrganization;
}>();

defineEmits<{
  (e: 'switchPageRequest', page: ClientAreaPages): void;
}>();

const stats = ref<OrganizationStatsResultData | undefined>(undefined);
const invoices = ref<Array<StripeInvoice>>([]);
const defaultCard = ref<MsPaymentMethod.Card | undefined>(undefined);
const currentDate = DateTime.now();
const querying = ref(false);
// TODO: retrieve this from the backend if it ever becomes available
// https://github.com/Scille/parsec-cloud/issues/10416
const estimations = ref<undefined | { amount: number }>(undefined);

onMounted(async () => {
  querying.value = true;
  if (!isDefaultOrganization(props.organization)) {
    const orgStatsResponse = await BmsAccessInstance.get().getOrganizationStats(props.organization.bmsId);
    if (!orgStatsResponse.isError && orgStatsResponse.data && orgStatsResponse.data.type === DataType.OrganizationStats) {
      stats.value = orgStatsResponse.data;
    }
  }

  const invoicesResponse = await BmsAccessInstance.get().getMonthlySubscriptionInvoices();
  if (!invoicesResponse.isError && invoicesResponse.data && invoicesResponse.data.type === DataType.MonthlySubscriptionInvoices) {
    invoices.value = invoicesResponse.data.invoices
      .filter((invoice) => isDefaultOrganization(props.organization) || invoice.getOrganizationId() === props.organization.parsecId)
      .sort((invoice1, invoice2) => invoice2.getDate().toMillis() - invoice1.getDate().toMillis())
      .slice(0, 3);
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
  querying.value = false;
});
</script>

<style scoped lang="scss">
.client-page-dashboard {
  display: flex;
  gap: ms.spacing('gap-4xl');
  flex-wrap: wrap;

  @include ms.responsive-breakpoint('lg') {
    flex-direction: column;
    flex-wrap: nowrap;
  }
}

// summary + invoices section
.dashboard-section-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  max-width: 60rem;
  width: 100%;
}

//payment section
.payment-container {
  max-width: 20rem;
  height: fit-content;

  @include ms.responsive-breakpoint('lg') {
    max-width: 100%;
  }
}

//every sections
.dashboard-section {
  background: ms.color('surface-base-default');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-5xl');
  padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl');
  border-radius: ms.radius('4xl');
  border: ms.border('thin') solid ms.color('border-base-default');
  box-shadow: ms.shadow('input');

  @include ms.responsive-breakpoint('lg') {
    padding: ms.spacing('padding-4xl');
  }

  &-title {
    @include ms.font('heading-h3');
    color: ms.color('text-brand-default');
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
    gap: ms.spacing('gap-2xl');

    &__title {
      @include ms.font('heading-h5');
      height: fit-content;
      color: ms.color('text-base-description');
    }

    &__data {
      @include ms.font('heading-h1');
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-xl');
      color: ms.color('text-base-heading');

      .data-content {
        display: flex;
        gap: ms.spacing('gap-lg');
        align-items: center;

        &-text {
          color: ms.color('text-base-heading');
        }
      }

      .data-info {
        @include ms.font('label-md-medium');
        color: ms.color('text-base-description');
      }
    }
  }
}

.invoices {
  display: flex;
  flex-direction: column;
  overflow: auto;
  gap: ms.spacing('gap-lg');
  --max-width-date: 10rem;
  --max-width-organization: 12rem;
  --max-width-amount: 5.625rem;

  &-date {
    width: 100%;
    flex-basis: var(--max-width-date);
    min-width: 8rem;
  }

  &-organization {
    flex-basis: var(--max-width-organization);
    min-width: 8rem;
  }

  &-amount {
    flex-basis: var(--max-width-amount);
    min-width: 8rem;
  }

  &-status {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-grow: 1;
    flex-basis: var(--max-width-date);
    min-width: 8rem;
    gap: ms.spacing('gap-3xl');
  }

  &-header {
    display: flex;
    justify-content: space-between;
    background: ms.color('surface-base-default-secondary');
    border-radius: ms.radius('lg');
    min-width: 35rem;

    .invoices-header-list {
      display: flex;
      padding: ms.spacing('padding-2xl') ms.spacing('padding-lg');
      width: 100%;

      &-item {
        --padding-start: #{ms.spacing('padding-none')};
        --padding-end: #{ms.spacing('padding-none')};
        --padding-top: #{ms.spacing('padding-none')};
        --padding-bottom: #{ms.spacing('padding-none')};
        --background: none;
        color: ms.color('text-neutral-default');
      }
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    min-width: 35rem;

    .invoices-list {
      display: flex;
      flex-direction: column;
      --ion-safe-area-left: 0;

      &-item {
        display: flex;
        justify-content: space-between;
        --background: none;
        padding: ms.spacing('padding-2xl') ms.spacing('padding-lg');
        border-radius: ms.radius('lg');
        position: relative;

        &:hover {
          .invoices-list-item__data {
            color: ms.color('text-neutral-default-hover');
          }
        }

        &:not(:last-child) {
          &::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            border-bottom: ms.border('thin') solid ms.color('border-base-default');
          }
        }

        &__data {
          color: ms.color('text-base-description');
        }

        // eslint-disable-next-line vue-scoped-css/no-unused-selector
        .badge-status {
          @include ms.font('body-sm-regular');
          border-radius: ms.radius('6xl');
          padding-inline: ms.spacing('padding-lg');
          background: ms.color('surface-disabled-default');
          color: ms.color('text-base-body');
          flex-shrink: 0;

          &.paid {
            background: ms.color('surface-success-default-subtle');
            color: ms.color('text-success-default');
          }

          &.open {
            background: ms.color('surface-information-default-subtle');
            color: ms.color('text-information-default');
          }
        }

        .invoices-list-item__download {
          &-icon {
            width: 1rem;
            --fill-color: #{ms.color('icon-neutral-default')};
            flex-shrink: 0;
          }

          &-label {
            @include ms.responsive-breakpoint('lg') {
              display: none;
            }
          }
        }
      }

      .skeleton-loading {
        display: flex;
        width: 100%;
        gap: ms.spacing('gap-lg');

        [class^='invoices-'] {
          height: 1rem;
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
  gap: ms.spacing('gap-3xl');
}

.payment {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  &-text {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
  }

  &-card {
    margin-bottom: 1rem;
  }

  .skeleton-loading-card {
    width: 15.625rem;
    height: 8.625rem;
    border-radius: ms.radius('2xl');
  }
}

.no-invoices {
  @include ms.font('body-lg-regular');
}

.invoices-header-list-item__text {
  @include ms.font('label-md-emphasis');
}

.invoices-list-item__date {
  @include ms.font('body-md-medium');
}

.invoices-list-item__text {
  @include ms.font('body-md-regular');
}

.payment-container__title {
  @include ms.font('heading-h3');
}
</style>
