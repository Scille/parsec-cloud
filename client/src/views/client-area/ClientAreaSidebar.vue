<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="sidebar-content">
    <ul>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Summary }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Summary)">Summary</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Stats }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Stats)">Stats</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Invoices }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Invoices)">Invoices</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Payment }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Payment)">Paiement</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Billing }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Billing)">Billing</ion-button>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { BmsOrganization } from '@/services/bms';
import { IonButton } from '@ionic/vue';
import { ClientAreaPages } from '@/views/client-area/types';

defineProps<{
  organization: BmsOrganization;
  currentPage: ClientAreaPages;
}>();

const emits = defineEmits<{
  (e: 'pageSelected', page: ClientAreaPages): void;
}>();

async function goToPageClicked(page: ClientAreaPages): Promise<void> {
  emits('pageSelected', page);
}
</script>

<style scoped lang="scss">
.sidebar-content {
  background-color: green;
  max-width: 16em;
  width: 100%;
  height: 100%;
}

.current-page {
  border: orange;
  color: orange;
  background-color: cyan;
}
</style>
