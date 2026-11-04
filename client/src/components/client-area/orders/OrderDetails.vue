<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<!-- eslint-disable vue/html-indent -->
<template>
  <div class="order-content-details">
    <ion-text class="order-content-details-header">
      {{ $msTranslate('clientArea.orders.new.details') }}
    </ion-text>
    <!-- No contract created at this step, users and storage estimation requested only -->
    <div
      class="details-list"
      v-if="[OrderStep.Received, OrderStep.Processing, OrderStep.Confirmed].includes(orderStep)"
    >
      <div class="details-list-item">
        <ion-text class="details-list-item__title">
          <ion-icon :icon="people" />
          {{ $msTranslate('clientArea.orders.new.usersNeed') }}
        </ion-text>
        <ion-text class="details-list-item__data">{{ $msTranslate(getUserOptions(request.standardUsers)) }}</ion-text>
      </div>
      <div class="details-list-item">
        <ion-text class="details-list-item__title">
          <ion-icon :icon="pieChart" />
          {{ $msTranslate('clientArea.orders.new.dataNeeded') }}
        </ion-text>
        <ion-text class="details-list-item__data">{{ $msTranslate(getStorageOptions(request.storage)) }}</ion-text>
      </div>
      <!-- comment -->
      <div
        class="details-list-item"
        id="detail-comment"
      >
        <ion-text class="details-list-item__title">
          <ion-icon :icon="chatbubbleEllipses" />
          {{ $msTranslate('clientArea.orders.new.comment') }}
        </ion-text>
        <ion-text class="details-list-item__data">
          {{ request.describedNeeds }}
        </ion-text>
      </div>
    </div>

    <!-- Contract created, users and storage are known -->
    <div
      class="details-list"
      v-if="orderDetails && [OrderStep.InvoiceToBePaid, OrderStep.Available].includes(orderStep)"
    >
      <div class="details-list-item">
        <ion-text class="details-list-item__title">
          <ion-icon :icon="people" />
          {{ $msTranslate('clientArea.orders.new.usersNeed') }}
        </ion-text>
        <ion-text class="details-list-item__data">
          {{
            $msTranslate({
              key: 'clientArea.orders.quantityOrdered.administrator',
              data: { count: orderDetails.administrators.quantityOrdered },
              count: orderDetails.administrators.quantityOrdered,
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
        <ion-text class="details-list-item__data">
          {{
            $msTranslate({
              key: 'clientArea.orders.quantityOrdered.standard',
              data: { count: orderDetails.standards.quantityOrdered },
              count: orderDetails.standards.quantityOrdered,
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
        <ion-text class="details-list-item__data">
          {{
            $msTranslate({
              key: 'clientArea.orders.quantityOrdered.outsider',
              data: { count: orderDetails.outsiders.quantityOrdered },
              count: orderDetails.outsiders.quantityOrdered,
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <div class="details-list-item">
        <ion-text class="details-list-item__title">
          <ion-icon :icon="pieChart" />
          {{ $msTranslate('clientArea.orders.new.dataNeeded') }}
        </ion-text>
        <ion-text class="details-list-item__data">
          {{ $msTranslate(formatFileSize(orderDetails.storage.quantityOrdered)) }}

          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <!-- Starting date -->
      <div
        class="details-list-item"
        v-if="request.organizationId"
      >
        <ion-text class="details-list-item__title">
          <ion-icon :icon="time" />
          {{ $msTranslate('clientArea.orders.new.startDate') }}
        </ion-text>
        <ion-text class="details-list-item__data">
          {{ $msTranslate(I18n.formatDate(orderDetails.created, 'short')) }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <!-- Ending date -->
      <div
        class="details-list-item"
        v-if="request.organizationId"
      >
        <ion-text class="details-list-item__title">
          <ion-icon :icon="time" />
          {{ $msTranslate('clientArea.orders.new.endDate') }}
        </ion-text>
        <ion-text class="details-list-item__data">
          {{ $msTranslate(I18n.formatDate(orderDetails.dueDate, 'short')) }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <div
        v-if="orderStep === OrderStep.Available && orderDetails"
        class="details-list-item"
      >
        <ion-text class="details-list-item__title">
          <ion-icon :icon="wallet" />
          {{ $msTranslate('clientArea.orders.invoiceToBePaid.paid') }}
        </ion-text>
        <ion-text class="details-list-item__data">
          {{
            $msTranslate({
              key: 'clientArea.orders.invoiceToBePaid.price',
              data: { amount: orderDetails.amountDue },
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
    </div>

    <!-- invoice details -->
    <div
      v-if="orderStep === OrderStep.InvoiceToBePaid && orderDetails"
      class="details-invoice"
    >
      <div class="details-invoice-text">
        <ion-text class="details-invoice-text__title">
          {{ $msTranslate('clientArea.orders.invoiceToBePaid.title') }}
        </ion-text>
        <ion-text class="details-invoice-text__description">
          {{ $msTranslate('clientArea.orders.invoiceToBePaid.description') }}
        </ion-text>
      </div>
      <ion-text class="details-invoice-price">
        <span class="details-invoice-price__amount">
          {{
            $msTranslate({
              key: 'clientArea.orders.invoiceToBePaid.price',
              data: { amount: orderDetails.amountDue },
            })
          }}
        </span>
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { OrderStep } from '@/components/client-area/orders/utils';
import { BmsAccessInstance, BmsOrganization, CustomOrderDetailsResultData, CustomOrderRequest, DataType } from '@/services/bms';
import { IonIcon, IonText } from '@ionic/vue';
import { chatbubbleEllipses, checkmark, people, pieChart, time, wallet } from 'ionicons/icons';
import { I18n, Translatable } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const error = ref<string>('');
const querying = ref(true);
const orderDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);

const props = defineProps<{
  orderStep: OrderStep;
  request: CustomOrderRequest;
  organization: BmsOrganization | undefined;
}>();

function getUserOptions(userNeeds: number): Translatable {
  switch (userNeeds) {
    case 50:
      return 'clientArea.orders.request.userNeeds.choices.50';
    case 100:
      return 'clientArea.orders.request.userNeeds.choices.100';
    case 300:
      return 'clientArea.orders.request.userNeeds.choices.300';
    case 9999:
      return 'clientArea.orders.request.userNeeds.choices.more';
    default:
      return '';
  }
}

function getStorageOptions(storage: number): Translatable {
  switch (storage) {
    case 100:
      return 'clientArea.orders.request.storageNeeds.choices.100';
    case 500:
      return 'clientArea.orders.request.storageNeeds.choices.500';
    case 1000:
      return 'clientArea.orders.request.storageNeeds.choices.1000';
    case 9999:
      return 'clientArea.orders.request.storageNeeds.choices.more';
    default:
      return '';
  }
}

onMounted(async () => {
  try {
    querying.value = true;

    // If the order is in a step where no organization is created yet, we don't need to fetch details
    if ([OrderStep.Received, OrderStep.Processing, OrderStep.Confirmed].includes(props.orderStep) || !props.organization) {
      return;
    }

    const orderDetailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);
    if (orderDetailsRep.isError || orderDetailsRep.data?.type !== DataType.CustomOrderDetails) {
      error.value = 'clientArea.contracts.errors.noInfo';
      return;
    }
    orderDetails.value = orderDetailsRep.data;
  } finally {
    querying.value = false;
  }
});
</script>

<style scoped lang="scss">
.order-content-details {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  max-width: 28rem;
  margin: 2.5rem;
  width: 100%;

  &-header {
    @include ms.font('body-md-medium');
    color: ms.color('text-base-description');
  }

  .details-list {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');

    &-item {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-3xl');
      justify-content: space-between;
      flex-wrap: wrap;

      &__title {
        @include ms.font('body-lg-regular');
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-lg');
        color: ms.color('text-base-description');
      }

      &__data {
        @include ms.font('body-lg-medium');
        color: ms.color('text-neutral-default');
        padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
        background: ms.color('surface-base-page-secondary');
        border-radius: ms.radius('lg');
        max-width: 15rem;
        margin-left: auto;
        text-align: end;
        width: 100%;
        position: relative;

        .checkmark-icon {
          position: absolute;
          right: -1.5rem;
          color: ms.color('icon-brand-default');
          margin-left: 0.5rem;
        }
      }

      &#detail-comment {
        flex-direction: column;
        align-items: flex-start;

        .details-list-item__data {
          max-width: 100%;
          text-align: start;
        }
      }

      ion-icon {
        flex-shrink: 0;
      }
    }
  }

  .details-invoice {
    display: flex;
    justify-content: space-between;
    gap: ms.spacing('gap-3xl');
    margin-top: 0.5rem;
    border-top: ms.border('thin') solid ms.color('border-base-default');
    padding-top: ms.spacing('padding-4xl');

    &-text {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-lg');

      &__title {
        @include ms.font('heading-h5');
        color: ms.color('text-base-body');
      }

      &__description {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }
    }

    &-price {
      color: ms.color('text-neutral-default');
      text-align: end;

      &__amount {
        @include ms.font('heading-h5');
      }
    }
  }
}
</style>
