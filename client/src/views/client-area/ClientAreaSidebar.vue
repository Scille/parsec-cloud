<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-header class="sidebar-header">
    <ion-card class="organization-card">
      <template v-if="organizations.length > 0">
        <ion-card-header
          class="organization-card-header"
          @click="openOrganizationChoice($event)"
        >
          <div
            class="card-header"
            v-if="!isDefaultOrganization(currentOrganization)"
          >
            <ion-avatar class="card-header-avatar">
              <span>{{ currentOrganization.parsecId.substring(0, 2) }}</span>
            </ion-avatar>
            <ion-card-title class="card-header-title title-h3">
              {{ currentOrganization.parsecId }}
            </ion-card-title>
          </div>
          <div
            class="card-header"
            v-else
          >
            <ion-card-title class="card-header-title title-h3">
              {{ $msTranslate('clientArea.sidebar.allOrganization') }}
            </ion-card-title>
          </div>

          <div class="card-header-icon">
            <ms-image :image="ChevronExpand" />
          </div>
        </ion-card-header>
      </template>
      <template v-else-if="!querying && organizations.length === 0">
        <ion-card-header
          class="organization-card-header no-organization"
          @click="openOrganizationChoice($event)"
        >
          <div class="no-organization-title">
            <ion-text class="no-organization-title__text subtitles-normal">
              {{ $msTranslate('clientArea.sidebar.noOrganization') }}
            </ion-text>
            <ms-information-tooltip
              :text="$msTranslate('clientArea.sidebar.noOrganizationDescription')"
              slot="end"
              class="no-organization-title__icon"
            />
          </div>
          <ion-text
            button
            class="organization-card-button custom-button custom-button-fill button-medium"
            v-show="showMenu"
            @click="createOrganization"
          >
            <ion-icon :icon="add" />
            {{ $msTranslate('clientArea.sidebar.createFirstOrganization') }}
          </ion-text>
        </ion-card-header>
      </template>
      <template v-else-if="querying">
        <div class="skeleton-header">
          <ion-skeleton-text
            :animated="true"
            class="skeleton-header-organization"
          />
        </div>
      </template>

      <!-- show it only when there is one organization selected -->
      <template v-if="!isDefaultOrganization(currentOrganization) && orgStatus">
        <ion-text class="organization-card-state body">
          {{ $msTranslate('clientArea.sidebar.state.title') }}
          <span>{{ $msTranslate(orgStatus.isFrozen ? 'clientArea.sidebar.state.frozen' : 'clientArea.sidebar.state.active') }}</span>
        </ion-text>
      </template>

      <!-- button: go to organization -->
      <div
        class="organization-card-button custom-button custom-button-fill"
        v-show="!isDefaultOrganization(currentOrganization) && orgStatus?.isBootstrapped"
        @click="goToOrganization"
      >
        <ion-icon
          class="navigate-icon"
          :icon="arrowForward"
        />
        <ion-text
          class="button-medium"
          button
        >
          {{ $msTranslate('clientArea.sidebar.goToOrganization') }}
        </ion-text>
      </div>
    </ion-card>
  </ion-header>

  <ion-content class="ion-padding sidebar-content">
    <!-- global menu client -->
    <div class="menu-client">
      <div class="menu-client-block">
        <!-- menu list -->
        <ion-list
          class="menu-client-list"
          :class="{ 'menu-client-list-mobile': !isLargeDisplay }"
        >
          <!-- dashboard -->
          <ion-item
            button
            lines="none"
            v-if="billingSystem === BillingSystem.Stripe"
            class="menu-default menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Dashboard }"
            @click="goToPageClicked(ClientAreaPages.Dashboard)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="grid"
              />
              {{ $msTranslate('clientArea.sidebar.menu.summary') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="grid"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.summary') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- orders -->
          <ion-item
            button
            lines="none"
            v-if="billingSystem === BillingSystem.CustomOrder || billingSystem === BillingSystem.ExperimentalCandidate"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Orders }"
            @click="goToPageClicked(ClientAreaPages.Orders)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="cube"
              />
              {{ $msTranslate('clientArea.sidebar.menu.orders') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="cube"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.orders') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- profile -->
          <ion-item
            button
            lines="none"
            class="menu-default menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.PersonalData }"
            @click="goToPageClicked(ClientAreaPages.PersonalData)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="person"
              />
              {{ $msTranslate('clientArea.sidebar.menu.personalData') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="person"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.personalData') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>
        </ion-list>
      </div>

      <!-- specific submenu depending on billing system -->
      <div class="menu-client-block">
        <ion-list
          v-if="billingSystem === BillingSystem.Stripe"
          class="menu-client-list"
          :class="{ 'menu-client-list-mobile': !isLargeDisplay }"
        >
          <!-- stats -->
          <ion-item
            button
            lines="none"
            :disabled="clientAreaPagesDisabled[ClientAreaPages.Statistics]"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Statistics }"
            @click="goToPageClicked(ClientAreaPages.Statistics)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="podium"
              />
              {{ $msTranslate('clientArea.sidebar.menu.stats') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="podium"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.stats') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- invoices -->
          <ion-item
            button
            lines="none"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Invoices }"
            @click="goToPageClicked(ClientAreaPages.Invoices)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="newspaper"
              />
              {{ $msTranslate('clientArea.sidebar.menu.invoices') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="newspaper"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.invoices') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- payment -->
          <ion-item
            button
            lines="none"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.PaymentMethods }"
            @click="goToPageClicked(ClientAreaPages.PaymentMethods)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="card"
              />
              {{ $msTranslate('clientArea.sidebar.menu.billingMethod') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="card"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.billingMethod') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- billing -->
          <ion-item
            button
            lines="none"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.BillingDetails }"
            @click="goToPageClicked(ClientAreaPages.BillingDetails)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="idCard"
              />
              {{ $msTranslate('clientArea.sidebar.menu.billingDetails') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="idCard"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.billingDetails') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>
        </ion-list>

        <ion-list
          v-if="billingSystem === BillingSystem.CustomOrder || billingSystem === BillingSystem.ExperimentalCandidate"
          class="menu-client-list"
          :class="{ 'menu-client-list-mobile': !isLargeDisplay }"
        >
          <!-- contracts -->
          <ion-item
            button
            lines="none"
            :disabled="clientAreaPagesDisabled[ClientAreaPages.Contracts]"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Contracts }"
            @click="goToPageClicked(ClientAreaPages.Contracts)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="albums"
              />
              {{ $msTranslate('clientArea.sidebar.menu.contract') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="albums"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.contract') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- stats -->
          <ion-item
            button
            lines="none"
            :disabled="clientAreaPagesDisabled[ClientAreaPages.CustomOrderStatistics]"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.CustomOrderStatistics }"
            @click="goToPageClicked(ClientAreaPages.CustomOrderStatistics)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="podium"
              />
              {{ $msTranslate('clientArea.sidebar.menu.stats') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="podium"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.stats') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- invoices -->
          <ion-item
            button
            lines="none"
            :disabled="clientAreaPagesDisabled[ClientAreaPages.CustomOrderInvoices]"
            class="button-medium menu-client-list-item"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.CustomOrderInvoices }"
            @click="goToPageClicked(ClientAreaPages.CustomOrderInvoices)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="newspaper"
              />
              {{ $msTranslate('clientArea.sidebar.menu.invoices') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="newspaper"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.invoices') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>

          <!-- billing -->
          <!-- TODO: unhide this, when BillingDetails for custom orders is developed -->
          <!-- https://github.com/Scille/parsec-cloud/issues/10415 -->
          <ion-item
            button
            lines="none"
            :disabled="!showMenu"
            class="button-medium menu-client-list-item"
            v-show="false"
            :class="{ 'current-page menu-active': currentPage === ClientAreaPages.CustomOrderBillingDetails }"
            @click="goToPageClicked(ClientAreaPages.CustomOrderBillingDetails)"
          >
            <template v-if="isLargeDisplay">
              <ion-icon
                class="menu-client-list-item__icon"
                :icon="idCard"
              />
              {{ $msTranslate('clientArea.sidebar.menu.billingDetails') }}
            </template>
            <template v-else>
              <ion-menu-button class="menu-button-wrapper">
                <ion-icon
                  class="menu-client-list-item__icon"
                  :icon="idCard"
                />
                <span>{{ $msTranslate('clientArea.sidebar.menu.billingDetails') }}</span>
              </ion-menu-button>
            </template>
          </ion-item>
        </ion-list>
      </div>
    </div>

    <!-- TODO: check if this block still needs to be hidden or should be shown / removed -->
    <!-- https://github.com/Scille/parsec-cloud/issues/10414 -->
    <div
      class="contact"
      v-show="false"
    >
      <ion-text class="contact-title subtitles-sm">
        {{ $msTranslate('clientArea.sidebar.help.title') }}
      </ion-text>
      <ion-text class="contact-button custom-button custom-button-fill button-medium">
        <ion-icon
          class="custom-button custom-button-icon"
          :icon="chatbubbleEllipses"
        />
        {{ $msTranslate('clientArea.sidebar.help.button') }}
      </ion-text>
    </div>

    <!-- button: go to home -->
    <div class="bottom-section">
      <div
        class="bottom-section-buttons bottom-section-buttons-home custom-button custom-button-fill"
        @click="goToHome"
      >
        <ion-icon
          class="bottom-section-icon"
          :icon="home"
        />
        <ion-text
          class="button-medium"
          button
        >
          {{ $msTranslate('clientArea.sidebar.goToHome') }}
        </ion-text>
      </div>

      <!-- button: logout -->
      <div
        class="bottom-section-buttons bottom-section-buttons-logout custom-button custom-button-fill"
        @click="logout"
      >
        <ion-icon
          class="bottom-section-icon"
          :icon="logOut"
        />
        <ion-text
          class="button-medium"
          button
        >
          {{ $msTranslate('clientArea.sidebar.logout') }}
        </ion-text>
      </div>
    </div>
  </ion-content>
</template>

<script setup lang="ts">
import OrganizationSwitchClientPopover from '@/components/organizations/OrganizationSwitchClientPopover.vue';
import { navigateTo, Routes } from '@/router';
import {
  BillingSystem,
  BmsAccessInstance,
  BmsOrganization,
  CustomOrderStatus,
  DataType,
  OrganizationStatusResultData,
} from '@/services/bms';
import { ServerType } from '@/services/parsecServers';
import { ClientAreaPages, isDefaultOrganization } from '@/views/client-area/types';
import {
  IonAvatar,
  IonCard,
  IonCardHeader,
  IonCardTitle,
  IonContent,
  IonHeader,
  IonIcon,
  IonItem,
  IonList,
  IonMenuButton,
  IonSkeletonText,
  IonText,
  popoverController,
} from '@ionic/vue';
import {
  add,
  albums,
  arrowForward,
  card,
  chatbubbleEllipses,
  cube,
  grid,
  home,
  idCard,
  logOut,
  newspaper,
  person,
  podium,
} from 'ionicons/icons';
import { Answer, askQuestion, ChevronExpand, MsImage, MsInformationTooltip, MsModalResult, useWindowSize } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  currentOrganization: BmsOrganization;
  organizations: Array<BmsOrganization>;
  currentPage: ClientAreaPages;
}>();
const orgStatus = ref<OrganizationStatusResultData | null>(null);
const billingSystem = ref(BmsAccessInstance.get().getPersonalInformation().billingSystem);
const showMenu = ref(false);
const querying = ref(false);
const { isLargeDisplay } = useWindowSize();

const clientAreaPagesDisabled = ref<Partial<Record<ClientAreaPages, boolean>>>({
  [ClientAreaPages.Contracts]: false,
  [ClientAreaPages.CustomOrderBillingDetails]: false,
  [ClientAreaPages.CustomOrderInvoices]: false,
  [ClientAreaPages.CustomOrderStatistics]: false,
  [ClientAreaPages.Statistics]: false,
});

const emits = defineEmits<{
  (e: 'pageSelected', page: ClientAreaPages): void;
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

async function goToPageClicked(page: ClientAreaPages): Promise<void> {
  emits('pageSelected', page);
}

onMounted(async () => {
  querying.value = true;
  if (isDefaultOrganization(props.currentOrganization)) {
    orgStatus.value = null;
  } else {
    const response = await BmsAccessInstance.get().getOrganizationStatus(props.currentOrganization.bmsId);
    if (!response.isError && response.data) {
      orgStatus.value = response.data as OrganizationStatusResultData;
    }

    // Check if org have possibly stats, if not disable the pages
    const isBootstrapped = orgStatus.value?.isBootstrapped ?? false;

    let disableStatisticsPages = true;
    if (isBootstrapped) {
      const orgStatsResponse = await BmsAccessInstance.get().getOrganizationStats(props.currentOrganization.bmsId);

      if (!orgStatsResponse.isError && orgStatsResponse.data?.type === DataType.OrganizationStats) {
        disableStatisticsPages = false;
      }
    }

    if (disableStatisticsPages) {
      clientAreaPagesDisabled.value[ClientAreaPages.Statistics] = true;
      clientAreaPagesDisabled.value[ClientAreaPages.CustomOrderStatistics] = true;
    }
  }

  if (billingSystem.value === BillingSystem.CustomOrder || billingSystem.value === BillingSystem.ExperimentalCandidate) {
    showMenu.value = true;
    if (!isDefaultOrganization(props.currentOrganization)) {
      const statusResp = await BmsAccessInstance.get().getCustomOrderStatus(props.currentOrganization);
      if (!statusResp.isError && statusResp.data && statusResp.data.type === DataType.CustomOrderStatus) {
        if ([CustomOrderStatus.NothingLinked, CustomOrderStatus.EstimateLinked].includes(statusResp.data.status)) {
          showMenu.value = false;
          clientAreaPagesDisabled.value[ClientAreaPages.Contracts] = true;
          clientAreaPagesDisabled.value[ClientAreaPages.CustomOrderInvoices] = true;
        }
      }
    }
  } else {
    showMenu.value = true;
  }
  querying.value = false;
});

async function openOrganizationChoice(event: Event): Promise<void> {
  if (props.organizations.length === 0) {
    return;
  }
  const popover = await popoverController.create({
    component: OrganizationSwitchClientPopover,
    componentProps: {
      currentOrganization: props.currentOrganization,
      organizations: props.organizations,
    },
    cssClass: 'dropdown-popover',
    id: 'organization-switch-popover',
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  const { data, role } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm && data) {
    emits('organizationSelected', data.organization);
  }
}

async function goToOrganization(): Promise<void> {
  await navigateTo(Routes.Home, { skipHandle: true, query: { bmsOrganizationId: props.currentOrganization.parsecId }, replace: true });
}

async function goToHome(): Promise<void> {
  await navigateTo(Routes.Home, { skipHandle: true, replace: true });
}

async function logout(): Promise<void> {
  const answer = await askQuestion('clientArea.sidebar.logoutConfirmTitle', 'clientArea.sidebar.logoutConfirmQuestion', {
    yesText: 'clientArea.sidebar.logoutYes',
    noText: 'clientArea.sidebar.logoutNo',
  });
  if (answer === Answer.Yes) {
    await BmsAccessInstance.get().logout();
    await navigateTo(Routes.Home, { replace: true });
  }
}

async function createOrganization(): Promise<void> {
  await navigateTo(Routes.Home, { skipHandle: true, query: { createOrg: ServerType.Saas } });
}
</script>

<style scoped lang="scss">
.sidebar-header {
  padding: 2rem 1.5rem 0;
}

.skeleton-header-organization {
  width: 100%;
  height: 3rem;
  margin-bottom: 3rem;
}

.sidebar-content {
  --padding-start: 1.5rem;
  --padding-end: 1.5rem;
  --padding-top: 0;
  --padding-bottom: 2rem;
  --offset-bottom: 0;
  --background: transparent;

  &::part(scroll) {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    // multiple lines for cross-browser compatibility
    height: 100%;
    height: -webkit-fill-available;
    height: -moz-available;
    height: stretch;
  }
}

.organization-card {
  --background: var(--parsec-color-light-primary-30-opacity15);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin: 0;
  box-shadow: none;

  &-header {
    display: flex;
    justify-content: space-between;
    flex-direction: row;
    border-radius: var(--parsec-radius-12);
    transition: background 150ms ease-in-out;
    padding: 0.5rem 0 0.5rem 0.5rem;

    &:hover {
      cursor: pointer;
      background: var(--parsec-color-light-secondary-premiere);
    }

    .card-header {
      box-shadow: none;
      display: flex;
      align-items: center;
      justify-content: left;
      gap: 0.75em;
      position: relative;
      z-index: 2;
      min-width: 0;
      overflow: hidden;

      &-avatar {
        background-color: var(--parsec-color-light-primary-700);
        color: var(--parsec-color-light-secondary-white);
        width: 2rem;
        height: 2rem;
        border-radius: var(--parsec-radius-8);
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        position: relative;
        z-index: 1;
      }

      &-title {
        padding: 0.1875em 0;
        margin: 0;
        --color: var(--parsec-color-light-secondary-text);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      &-icon {
        white-space: nowrap;
        display: flex;
        align-items: center;
        --fill-color: var(--parsec-color-light-secondary-soft-grey);
      }
    }
  }

  .no-organization {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    border-radius: var(--parsec-radius-12);
    background: var(--parsec-color-light-secondary-premiere);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    padding: 1.5rem 1rem;
    margin-bottom: 1rem;
    cursor: default;

    &-title {
      display: flex;
      flex-direction: row;
      align-items: center;
      justify-content: space-between;
      gap: 0.5rem;

      &__text {
        color: var(--parsec-color-light-secondary-text);
      }

      &__icon {
        color: var(--parsec-color-light-secondary-grey);
        font-size: 1.125rem;
        cursor: pointer;

        &:hover {
          color: var(--parsec-color-light-secondary-text);
        }
      }
    }

    .organization-card-button {
      background: var(--parsec-color-light-secondary-text);
      color: var(--parsec-color-light-secondary-white);
      align-items: center;

      &:hover {
        cursor: pointer;
        background: var(--parsec-color-light-secondary-contrast);
      }
    }
  }
}

.menu-client {
  display: flex;
  flex-direction: column;
  gap: 4rem;
  position: relative;

  &-block {
    position: relative;

    &::after {
      content: '';
      position: absolute;
      bottom: -2rem;
      left: -2rem;
      width: calc(100% + 4rem);
      height: 1px;
      background: var(--parsec-color-light-secondary-disabled);
      z-index: 2;
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    padding: 0;
    background: transparent;

    &-item {
      --background: var(--parsec-color-light-secondary-background);
      --background-hover: var(--parsec-color-light-secondary-premiere);
      --color: var(--parsec-color-light-secondary-hard-grey);
      --background-hover-opacity: 1;
      --inner-padding-end: 0rem;
      --padding-start: 1rem;
      --padding-end: 1rem;
      --padding-top: 0.625rem;
      --padding-bottom: 0.625rem;
      border-radius: var(--parsec-radius-12);
      font-weight: 500;
      font-size: 1rem;

      &__icon {
        color: var(--parsec-color-light-secondary-soft-grey);
        font-size: 1.2rem;
        margin-inline-end: 0.5rem;
        flex-shrink: 0;
      }

      &:hover {
        cursor: pointer;
        box-shadow: var(--parsec-shadow-input);
      }

      &.current-page {
        --background: var(--parsec-color-light-secondary-medium);
        --background-hover: var(--parsec-color-light-secondary-medium);
        --color: var(--parsec-color-light-primary-700);
        box-shadow: var(--parsec-shadow-input);

        .menu-client-list-item__icon {
          color: var(--parsec-color-light-primary-700);
        }
      }

      ion-menu-button {
        display: flex;
        font-size: initial;
        width: 100%;
        height: fit-content;

        &::part(native) {
          display: flex;
          font-size: inherit;
          padding: 0;
          width: 100%;
        }
      }
    }
  }

  &-list-mobile {
    .menu-client-list-item::part(native) {
      padding: 0;
    }

    .menu-button-wrapper {
      display: flex;
      font-size: initial;
      width: 100%;
      height: fit-content;
      padding: 0.625rem 1rem;

      &::part(native) {
        border-radius: 0;
        --background-hover: none;
      }
    }

    span {
      width: 100%;
      text-align: left;
    }
  }
}

.contact {
  display: flex;
  flex-direction: column;
  justify-content: end;
  gap: 0.5rem;
  width: 100%;
  background: var(--parsec-color-light-secondary-background);
  position: relative;
  z-index: 3;

  &-title {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-button {
    background: transparent;

    &:hover {
      border: 1px solid var(--parsec-color-light-secondary-light);
    }
  }
}

.bottom-section {
  padding-top: 3rem;

  &-buttons {
    margin: 1rem 0 0.5rem 0;

    &:hover {
      background-color: var(--parsec-color-light-secondary-premiere);
    }

    &-home {
      background: none;
    }

    &-logout {
      background: none;
      color: var(--parsec-color-light-danger-500);
      border-color: var(--parsec-color-light-danger-100);
    }
  }

  &-icon {
    margin-bottom: 0.05rem;
  }
}
</style>
