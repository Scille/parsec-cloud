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
          @organization-selected="onOrganizationSelected"
        />
        <div class="main-content">
          <client-area-header
            :title="getTitleByPage()"
            @page-selected="switchPage"
          />

          <div
            class="main-page"
            v-if="currentOrganization"
          >
            <billing-details-page
              v-if="currentPage === ClientAreaPages.BillingDetails"
              :organization="currentOrganization"
            />
            <contracts-page
              v-if="currentPage === ClientAreaPages.Contracts"
              :organization="currentOrganization"
            />
            <dashboard-page
              v-if="currentPage === ClientAreaPages.Dashboard"
              :organization="currentOrganization"
            />
            <invoices-page
              v-if="currentPage === ClientAreaPages.Invoices"
              :organization="currentOrganization"
            />
            <payment-methods-page
              v-if="currentPage === ClientAreaPages.PaymentMethods"
              :organization="currentOrganization"
            />
            <personal-data-page
              v-if="currentPage === ClientAreaPages.PersonalData"
              :organization="currentOrganization"
            />
            <statistics-page
              v-if="currentPage === ClientAreaPages.Statistics"
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
import BillingDetailsPage from '@/views/client-area/BillingDetailsPage.vue';
import ContractsPage from '@/views/client-area/ContractsPage.vue';
import DashboardPage from '@/views/client-area/DashboardPage.vue';
import InvoicesPage from '@/views/client-area/InvoicesPage.vue';
import PaymentMethodsPage from '@/views/client-area/PaymentMethodsPage.vue';
import PersonalDataPage from '@/views/client-area/PersonalDataPage.vue';
import StatisticsPage from '@/views/client-area/StatisticsPage.vue';
import { navigateTo, Routes } from '@/router';
import { Translatable } from 'megashark-lib';

const organizations = ref<Array<BmsOrganization>>([]);
const currentPage = ref<ClientAreaPages>(ClientAreaPages.Dashboard);
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

async function onOrganizationSelected(organization: BmsOrganization): Promise<void> {
  currentOrganization.value = organization;
}

function getTitleByPage(): Translatable {
  switch (currentPage.value) {
    case ClientAreaPages.BillingDetails:
      return 'clientArea.header.titles.billingDetails';
    case ClientAreaPages.Contracts:
      return 'clientArea.header.titles.contracts';
    case ClientAreaPages.Dashboard:
      return 'clientArea.header.titles.dashboard';
    case ClientAreaPages.Invoices:
      return 'clientArea.header.titles.invoices';
    case ClientAreaPages.PaymentMethods:
      return 'clientArea.header.titles.paymentMethods';
    case ClientAreaPages.PersonalData:
      return 'clientArea.header.titles.personalData';
    case ClientAreaPages.Statistics:
      return 'clientArea.header.titles.statistics';
    default:
      return '';
  }
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
