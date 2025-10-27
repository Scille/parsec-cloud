<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <template v-if="orders.length > 0">
    <orders-list-item
      v-for="order in orders"
      :key="order.id"
      :request="order"
    />
  </template>

  <!-- no orders -->
  <ion-text
    class="body-lg no-orders"
    v-else
  >
    {{ $msTranslate('clientArea.orders.noOrders') }}
  </ion-text>
</template>

<script setup lang="ts">
import OrdersListItem from '@/components/client-area/orders/OrdersListItem.vue';
import { CustomOrderRequest } from '@/services/bms';
import { IonText } from '@ionic/vue';

defineProps<{
  orders: CustomOrderRequest[];
}>();
</script>

<style scoped lang="scss"></style>
