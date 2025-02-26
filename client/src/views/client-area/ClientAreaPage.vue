<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="page">
    <div
      class="resize-divider"
      ref="divider"
      v-show="isVisible()"
    />
    <ion-split-pane
      when="xs"
      content-id="main"
    >
      <ion-menu
        content-id="main"
        class="sidebar"
      >
        <template v-if="!querying && loggedIn">
          <client-area-sidebar
            v-if="currentOrganization"
            :current-page="currentPage"
            :organization="currentOrganization"
            @page-selected="switchPage"
            @organization-selected="onOrganizationSelected"
            :key="refresh"
          />
        </template>
        <template v-else-if="loggedIn">
          <div class="skeleton">
            <div class="skeleton-header">
              <ion-skeleton-text
                :animated="true"
                class="skeleton-header-organization"
              />
              <ion-skeleton-text
                :animated="true"
                class="skeleton-header-button"
              />
            </div>
            <div class="skeleton-menu">
              <ion-skeleton-text
                :animated="true"
                class="skeleton-menu-item"
                v-for="index in 3"
                :key="index"
              />
            </div>
          </div>
        </template>
      </ion-menu>

      <div
        class="ion-page"
        id="main"
      >
        <client-area-header
          v-if="loggedIn && !querying"
          :title="getTitleByPage()"
          @page-selected="switchPage"
        />
        <ion-content class="main-container">
          <div class="main-content">
            <div
              class="main-page"
              v-if="loggedIn && !querying && currentOrganization"
              :key="refresh"
            >
              <billing-details-page
                v-if="currentPage === ClientAreaPages.BillingDetails"
                :organization="currentOrganization"
              />
              <dashboard-page
                v-if="currentPage === ClientAreaPages.Dashboard"
                :organization="currentOrganization"
                @switch-page-request="switchPage"
              />
              <statistics-page
                v-if="currentPage === ClientAreaPages.Statistics"
                :organization="currentOrganization"
                @organization-selected="onOrganizationSelected"
              />
              <invoices-page
                v-if="currentPage === ClientAreaPages.Invoices"
                :organization="currentOrganization"
              />
              <payment-methods-page
                v-if="currentPage === ClientAreaPages.PaymentMethods"
                :organization="currentOrganization"
                :information-manager="informationManager"
              />
              <personal-data-page
                v-if="currentPage === ClientAreaPages.PersonalData"
                :organization="currentOrganization"
                :information-manager="informationManager"
              />
              <!-- CustomOrder -->
              <custom-order-statistics-page
                v-if="currentPage === ClientAreaPages.CustomOrderStatistics"
                :organization="currentOrganization"
              />
              <contracts-page
                v-if="currentPage === ClientAreaPages.Contracts"
                :organization="currentOrganization"
                @organization-selected="onOrganizationSelected"
              />
              <orders-page
                v-if="currentPage === ClientAreaPages.Orders"
                :organization="currentOrganization"
                @organization-selected="onOrganizationSelected"
                :information-manager="informationManager"
              />
              <custom-order-billing-details-page
                v-if="currentPage === ClientAreaPages.CustomOrderBillingDetails"
                :organization="currentOrganization"
              />
              <custom-order-processing-page v-if="currentPage === ClientAreaPages.CustomOrderProcessing" />
            </div>
          </div>
        </ion-content>
      </div>
    </ion-split-pane>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent, IonSkeletonText, IonSplitPane, IonMenu, GestureDetail, createGesture } from '@ionic/vue';
import ClientAreaHeader from '@/views/client-area/ClientAreaHeader.vue';
import ClientAreaSidebar from '@/views/client-area/ClientAreaSidebar.vue';
import { BillingSystem, BmsAccessInstance, BmsOrganization, CustomOrderStatus, DataType } from '@/services/bms';
import { inject, onMounted, onUnmounted, ref, watch } from 'vue';
import { DefaultBmsOrganization, ClientAreaPages, isDefaultOrganization } from '@/views/client-area/types';
import BillingDetailsPage from '@/views/client-area/billing-details/BillingDetailsPage.vue';
import ContractsPage from '@/views/client-area/contracts/ContractsPage.vue';
import DashboardPage from '@/views/client-area/dashboard/DashboardPage.vue';
import InvoicesPage from '@/views/client-area/invoices/InvoicesPage.vue';
import PaymentMethodsPage from '@/views/client-area/payment-methods/PaymentMethodsPage.vue';
import PersonalDataPage from '@/views/client-area/personal-data/PersonalDataPage.vue';
import StatisticsPage from '@/views/client-area/statistics/StatisticsPage.vue';
import CustomOrderBillingDetailsPage from '@/views/client-area/billing-details/CustomOrderBillingDetailsPage.vue';
import CustomOrderStatisticsPage from '@/views/client-area/statistics/CustomOrderStatisticsPage.vue';
import OrdersPage from '@/views/client-area/orders/OrdersPage.vue';
import useSidebarMenu from '@/services/sidebarMenu';
import { Translatable } from 'megashark-lib';
import { ClientAreaQuery, getCurrentRouteQuery, navigateTo, Routes } from '@/router';
import { InformationManager } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import CustomOrderProcessingPage from '@/views/client-area/dashboard/CustomOrderProcessingPage.vue';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const informationManager: InformationManager = injectionProvider.getDefault().informationManager;
const { computedWidth: computedWidth, isVisible: isVisible } = useSidebarMenu();
const organizations = ref<Array<BmsOrganization>>([]);
const divider = ref();
const currentPage = ref<ClientAreaPages>(ClientAreaPages.Dashboard);
const currentOrganization = ref<BmsOrganization>(DefaultBmsOrganization);
const sidebarWidthProperty = ref('');
const loggedIn = ref(false);
const refresh = ref(0);
const querying = ref(true);

const watchSidebarWidthCancel = watch(computedWidth, (value: number) => {
  sidebarWidthProperty.value = `${value}px`;
  // set toast offset
  setToastOffset(value);
});

function setToastOffset(width: number): void {
  window.document.documentElement.style.setProperty('--ms-toast-offset', `${width}px`);
}

onMounted(async () => {
  sidebarWidthProperty.value = `${computedWidth.value}px`;
  querying.value = true;
  if (!BmsAccessInstance.get().isLoggedIn()) {
    loggedIn.value = await BmsAccessInstance.get().tryAutoLogin();
    if (!loggedIn.value) {
      await navigateTo(Routes.ClientAreaLogin, { skipHandle: true });
      querying.value = false;
      return;
    }
  } else {
    loggedIn.value = true;
  }
  const billingSystem = BmsAccessInstance.get().getPersonalInformation().billingSystem;
  const query = getCurrentRouteQuery<ClientAreaQuery>();
  const response = await BmsAccessInstance.get().listOrganizations();
  if (!response.isError && response.data && response.data.type === DataType.ListOrganizations) {
    organizations.value = response.data.organizations;
    if (organizations.value.length > 0) {
      if (query.organization) {
        const org = organizations.value.find((org) => org.bmsId === query.organization);
        if (org) {
          currentOrganization.value = org;
        }
      } else {
        if ((organizations.value.length > 0 && billingSystem !== BillingSystem.Stripe) || organizations.value.length === 1) {
          currentOrganization.value = organizations.value[0];
        } else {
          currentOrganization.value = DefaultBmsOrganization;
        }
      }
    }
    if (billingSystem === BillingSystem.CustomOrder || billingSystem === BillingSystem.ExperimentalCandidate) {
      currentPage.value = ClientAreaPages.Contracts;
      const statusResp = await BmsAccessInstance.get().getCustomOrderStatus(currentOrganization.value);
      if (!statusResp.isError && statusResp.data && statusResp.data.type === DataType.CustomOrderStatus) {
        if (statusResp.data.status === CustomOrderStatus.NothingLinked) {
          currentPage.value = ClientAreaPages.CustomOrderProcessing;
        }
      }
    } else {
      currentPage.value = ClientAreaPages.Dashboard;
    }
    if (query.page) {
      currentPage.value = query.page as ClientAreaPages;
    }
  }
  querying.value = false;

  setToastOffset(computedWidth.value);
  if (divider.value) {
    const gesture = createGesture({
      gestureName: 'resize-menu',
      el: divider.value,
      onMove,
    });
    gesture.enable();
  }
});

onUnmounted(() => {
  watchSidebarWidthCancel();
  setToastOffset(0);
});

async function switchPage(page: ClientAreaPages): Promise<void> {
  currentPage.value = page;
  await navigateTo(Routes.ClientArea, {
    skipHandle: true,
    query: {
      organization: isDefaultOrganization(currentOrganization.value) ? undefined : currentOrganization.value.bmsId,
      page: currentPage.value,
    },
  });
}

function onMove(detail: GestureDetail): void {
  requestAnimationFrame(() => {
    if (detail.currentX < 250) {
      computedWidth.value = 250;
    } else if (detail.currentX > 370) {
      computedWidth.value = 370;
    } else {
      computedWidth.value = detail.currentX;
    }
  });
}

async function onOrganizationSelected(organization: BmsOrganization): Promise<void> {
  await navigateTo(Routes.ClientArea, {
    skipHandle: true,
    query: { organization: isDefaultOrganization(organization) ? undefined : organization.bmsId, page: currentPage.value },
  });
  currentOrganization.value = organization;
  refresh.value += 1;
}

function getTitleByPage(): Translatable {
  switch (currentPage.value) {
    case ClientAreaPages.BillingDetails:
      return 'clientArea.header.titles.billingDetails';
    case ClientAreaPages.Contracts:
      return 'clientArea.header.titles.contract';
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
    case ClientAreaPages.CustomOrderStatistics:
      return 'clientArea.header.titles.customOrderStatistics';
    case ClientAreaPages.CustomOrderBillingDetails:
      return 'clientArea.header.titles.customOrderBillingDetails';
    case ClientAreaPages.Orders:
      return 'clientArea.header.titles.orders';
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

.resize-divider {
  width: 0.25rem;
  height: 100%;
  position: absolute;
  left: calc(v-bind(sidebarWidthProperty) - 2px);
  top: 0;
  z-index: 10000;
  cursor: ew-resize;
  display: flex;
  justify-content: center;

  &::after {
    content: '';
    width: 0.125rem;
    height: 100%;
    padding: 20rem 0;
  }

  &:hover::after,
  &:active::after {
    background: var(--parsec-color-light-secondary-soft-grey);
  }
}

.sidebar {
  --side-min-width: var(--parsec-sidebar-menu-min-width);
  --side-max-width: var(--parsec-sidebar-menu-max-width);
  --side-width: v-bind(sidebarWidthProperty);
  --background: var(--parsec-color-light-secondary-background);
  border-right: 1px solid var(--parsec-color-light-secondary-disabled);
  user-select: none;

  &::part(container) {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .skeleton {
    padding: 2rem 1.5rem 0;

    &-header {
      display: flex;
      flex-direction: column;
      margin-bottom: 2rem;

      &-organization {
        width: 100%;
        height: 3rem;
        margin-bottom: 3rem;
      }

      &-button {
        width: 100%;
        height: 2rem;
      }
    }

    &-menu {
      display: flex;
      flex-direction: column;

      gap: 0.625rem;

      &-item {
        width: 100%;
        height: 2rem;
      }
    }
  }
}

// -------- main content ------------
.main-content {
  // multiple lines for cross-browser compatibility
  width: 100%;
  width: -webkit-fill-available;
  width: -moz-available;
  width: stretch;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.main-page {
  // multiple lines for cross-browser compatibility
  height: 100%;
  height: -webkit-fill-available;
  height: -moz-available;
  height: stretch;
  // multiple lines for cross-browser compatibility
  width: 100%;
  width: -webkit-fill-available;
  width: -moz-available;
  width: stretch;
}
</style>
