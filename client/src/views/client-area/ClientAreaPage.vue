<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div
        id="page"
        :key="BmsAccessInstance.get().reloadKey"
      >
        <client-area-sidebar
          v-if="currentOrganization"
          :current-page="currentPage"
          :organization="currentOrganization"
          @page-selected="switchPage"
        />
        <div class="main-content">
          <client-area-header v-show="true" />

          <div
            class="main-page"
            v-if="currentOrganization"
          >
            <summary-page
              v-if="currentPage === ClientAreaPages.Summary"
              :organization="currentOrganization"
            />
            <invoices-page
              v-if="currentPage === ClientAreaPages.Invoices"
              :organization="currentOrganization"
            />
            <stats-page
              v-if="currentPage === ClientAreaPages.Stats"
              :organization="currentOrganization"
            />
            <billing-page
              v-if="currentPage === ClientAreaPages.Billing"
              :organization="currentOrganization"
            />
            <payment-page
              v-if="currentPage === ClientAreaPages.Payment"
              :organization="currentOrganization"
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent } from '@ionic/vue';
import ClientAreaHeader from '@/views/client-area/ClientAreaHeader.vue';
import ClientAreaSidebar from '@/views/client-area/ClientAreaSidebar.vue';
import { BmsAccessInstance, BmsOrganization, DataType } from '@/services/bms';
import { onMounted, ref } from 'vue';
import { ClientAreaPages } from '@/views/client-area/types';
import StatsPage from '@/views/client-area/StatsPage.vue';
import InvoicesPage from '@/views/client-area/InvoicesPage.vue';
import SummaryPage from '@/views/client-area/SummaryPage.vue';
import BillingPage from '@/views/client-area/BillingPage.vue';
import PaymentPage from '@/views/client-area/PaymentPage.vue';
import { navigateTo, Routes } from '@/router';

const organizations = ref<Array<BmsOrganization>>([]);
const currentPage = ref<ClientAreaPages>(ClientAreaPages.Summary);
const currentOrganization = ref<BmsOrganization | undefined>(undefined);

onMounted(async () => {
  if (!BmsAccessInstance.get().isLoggedIn()) {
    const loggedIn = await BmsAccessInstance.get().tryAutoLogin();
    if (!loggedIn) {
      await navigateTo(Routes.ClientAreaLogin);
      return;
    }
  }

  const response = await BmsAccessInstance.get().listOrganizations();
  if (!response.isError && response.data && response.data.type === DataType.ListOrganizations) {
    organizations.value = response.data.organizations;
    if (organizations.value.length > 0) {
      currentOrganization.value = organizations.value[0];
    }
  }
});

async function switchPage(page: ClientAreaPages): Promise<void> {
  currentPage.value = page;
}
</script>

<style scoped lang="scss">
#page {
  position: relative;
  display: flex;
  height: 100%;
}

.main-content {
  background-color: blue;
  width: -webkit-fill-available;
  height: 100%;
  color: white;
  font-size: 16px;
  display: flex;
  flex-direction: column;
}
.main-page {
  height: -webkit-fill-available;
  width: -webkit-fill-available;
  background-color: cornsilk;
  color: black;
}
</style>
