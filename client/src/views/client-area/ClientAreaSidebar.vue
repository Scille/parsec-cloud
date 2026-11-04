<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
            <ion-card-title class="card-header-title">
              {{ currentOrganization.parsecId }}
            </ion-card-title>
          </div>
          <div
            class="card-header"
            v-else
          >
            <ion-card-title class="card-header-title">
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
            <ion-text class="no-organization-title__text">
              {{ $msTranslate('clientArea.sidebar.noOrganization') }}
            </ion-text>
            <ms-information-tooltip
              :text="$msTranslate('clientArea.sidebar.noOrganizationDescription')"
              slot="end"
              class="no-organization-title__icon"
            />
          </div>
          <ion-button
            class="organization-card-button color-secondary"
            v-show="showMenu"
            @click="createOrganization"
          >
            <ion-icon
              :icon="add"
              class="button-icon-left"
            />
            {{ $msTranslate('clientArea.sidebar.createFirstOrganization') }}
          </ion-button>
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
        <ion-text class="organization-card-state">
          {{ $msTranslate('clientArea.sidebar.state.title') }}
          <span
            class="organization-card-state__badge"
            :class="orgStatus.isFrozen ? 'organization-card-state__badge--frozen' : 'organization-card-state__badge--active'"
          >
            {{ $msTranslate(orgStatus.isFrozen ? 'clientArea.sidebar.state.frozen' : 'clientArea.sidebar.state.active') }}
          </span>
        </ion-text>
      </template>

      <!-- button: go to organization -->
      <ion-button
        class="organization-card-button color-secondary"
        v-show="!isDefaultOrganization(currentOrganization) && orgStatus?.isBootstrapped"
        @click="goToOrganization"
      >
        <ion-icon
          :icon="arrowForward"
          class="button-icon-left"
        />
        {{ $msTranslate('clientArea.sidebar.goToOrganization') }}
      </ion-button>
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.Dashboard }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.Orders }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.PersonalData }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.Statistics }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.Invoices }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.PaymentMethods }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.BillingDetails }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.Contracts }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.CustomOrderStatistics }"
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
            class="menu-client-list-item"
            :class="{ 'current-page': currentPage === ClientAreaPages.CustomOrderInvoices }"
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
            class="menu-client-list-item"
            v-show="false"
            :class="{ 'current-page': currentPage === ClientAreaPages.CustomOrderBillingDetails }"
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
      <ion-text class="contact-title">
        {{ $msTranslate('clientArea.sidebar.help.title') }}
      </ion-text>
      <ion-button
        fill="outline"
        class="contact-button color-secondary"
      >
        <ion-icon
          :icon="chatbubbleEllipses"
          class="button-icon-left"
        />
        {{ $msTranslate('clientArea.sidebar.help.button') }}
      </ion-button>
    </div>

    <!-- button: go to home -->
    <div class="bottom-section">
      <ion-button
        fill="clear"
        class="bottom-section-buttons bottom-section-buttons-home color-secondary"
        @click="goToHome"
      >
        <ion-icon
          :icon="home"
          class="button-icon-left"
        />
        {{ $msTranslate('clientArea.sidebar.goToHome') }}
      </ion-button>

      <!-- button: logout -->
      <ion-button
        fill="clear"
        class="bottom-section-buttons bottom-section-buttons-logout color-secondary"
        @click="logout"
      >
        <ion-icon
          :icon="logOut"
          class="button-icon-left"
        />
        {{ $msTranslate('clientArea.sidebar.logout') }}
      </ion-button>
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
  IonButton,
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
  padding: ms.spacing('padding-5xl') ms.spacing('padding-4xl') ms.spacing('padding-none');
}

.skeleton-header-organization {
  width: 100%;
  height: 3rem;
  margin-bottom: 3rem;
}

.sidebar-content {
  --padding-start: #{ms.spacing('padding-4xl')};
  --padding-end: #{ms.spacing('padding-4xl')};
  --padding-top: #{ms.spacing('padding-none')};
  --padding-bottom: #{ms.spacing('padding-5xl')};
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
  --background: transparent;
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  margin: 0;
  box-shadow: none;

  &-header {
    display: flex;
    justify-content: space-between;
    flex-direction: row;
    border-radius: ms.radius('2xl');
    transition: background 150ms ease-in-out;
    background: ms.color('surface-neutral-default-subtle');
    padding: ms.spacing('padding-lg') ms.spacing('padding-none') ms.spacing('padding-lg') ms.spacing('padding-lg');

    &:hover {
      cursor: pointer;
      background: ms.color('surface-neutral-default-subtle-hover');
    }

    .card-header {
      box-shadow: none;
      display: flex;
      align-items: center;
      justify-content: left;
      gap: ms.spacing('gap-2xl');
      position: relative;
      z-index: 2;
      min-width: 0;
      overflow: hidden;

      &-avatar {
        background-color: ms.color('surface-brand-default');
        color: ms.color('text-neutral-on-color');
        width: 2rem;
        height: 2rem;
        border-radius: ms.radius('lg');
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        position: relative;
        z-index: 1;
      }

      &-title {
        @include ms.font('heading-h4');
        padding: 0.1875em ms.spacing('padding-none');
        margin: 0;
        --color: #{ms.color('text-base-body')};
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      &-icon {
        white-space: nowrap;
        display: flex;
        align-items: center;
        --fill-color: #{ms.color('icon-neutral-default')};
      }
    }
  }

  .no-organization {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    border-radius: ms.radius('2xl');
    background: ms.color('surface-base-default-secondary');
    border: ms.border('thin') solid ms.color('border-base-default');
    padding: ms.spacing('padding-4xl') ms.spacing('padding-3xl');
    margin-bottom: 1rem;
    cursor: default;

    &-title {
      display: flex;
      flex-direction: row;
      align-items: center;
      justify-content: space-between;
      gap: ms.spacing('gap-lg');

      &__text {
        @include ms.font('body-lg-medium');
        color: ms.color('text-base-body');
      }

      &__icon {
        color: ms.color('text-base-description');
        font-size: 1.125rem;
        cursor: pointer;

        &:hover {
          color: ms.color('text-neutral-default-hover');
        }
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
      background: ms.color('surface-disabled-default');
      z-index: 2;
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-xl');
    padding: ms.spacing('padding-none');
    background: transparent;

    &-item {
      @include ms.font('label-md-medium');
      --background: #{ms.color('surface-base-page')};
      --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
      --color: #{ms.color('text-base-description')};
      --background-hover-opacity: #{ms.opacity('10')};
      --inner-padding-end: #{ms.spacing('padding-none')};
      --padding-start: #{ms.spacing('padding-3xl')};
      --padding-end: #{ms.spacing('padding-3xl')};
      --padding-top: #{ms.spacing('padding-xl')};
      --padding-bottom: #{ms.spacing('padding-xl')};
      border-radius: ms.radius('2xl');
      @include ms.font('label-lg-medium');

      &__icon {
        color: ms.color('icon-neutral-default');
        font-size: 1.2rem;
        margin-inline-end: 0.5rem;
        flex-shrink: 0;
      }

      &:hover {
        cursor: pointer;
        box-shadow: ms.shadow('input');
      }

      &.current-page {
        --background: #{ms.color('surface-neutral-default-subtle-pressed')};
        --background-hover: #{ms.color('surface-neutral-default-subtle-pressed')};
        --color: #{ms.color('text-brand-default')};
        box-shadow: ms.shadow('input');

        .menu-client-list-item__icon {
          color: ms.color('text-brand-default');
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
          padding: ms.spacing('padding-none');
          width: 100%;
        }
      }
    }
  }

  &-list-mobile {
    .menu-client-list-item::part(native) {
      padding: ms.spacing('padding-none');
    }

    .menu-button-wrapper {
      display: flex;
      font-size: initial;
      width: 100%;
      height: fit-content;
      padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');

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
  gap: ms.spacing('gap-lg');
  width: 100%;
  background: ms.color('surface-base-page');
  position: relative;
  z-index: 3;

  &-title {
    @include ms.font('body-md-medium');
    color: ms.color('text-base-description');
  }
}

.bottom-section {
  padding-top: ms.spacing('padding-7xl');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-md');

  .bottom-section-buttons {
    width: 100%;
    margin: 0;

    &::part(native) {
      justify-content: flex-start;
    }
  }

  .bottom-section-buttons-logout {
    --color: #{ms.color('text-error-default')};
    --color-hover: #{ms.color('text-error-default-hover')};
    --background-hover: #{ms.color('surface-error-default-subtle-hover')};
    --background-activated: #{ms.color('surface-error-default-subtle-pressed')};
  }
}

.organization-card-state {
  @include ms.font('body-md-regular');
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-md');
  color: ms.color('text-base-description');

  &__badge {
    @include ms.font('label-sm-medium');
    display: inline-flex;
    align-items: center;
    gap: ms.spacing('gap-sm');
    padding: ms.spacing('padding-xs') ms.spacing('padding-md');
    border-radius: ms.radius('full');
    border: ms.border('thin') solid;

    &::before {
      content: '';
      width: 0.375rem;
      height: 0.375rem;
      border-radius: ms.radius('full');
      background: currentColor;
    }

    &--active {
      color: ms.color('text-success-default');
      background: ms.color('surface-success-default-subtle');
      border-color: ms.color('border-success-default-subtle');
    }

    &--frozen {
      color: ms.color('text-error-default');
      background: ms.color('surface-error-default-subtle');
      border-color: ms.color('border-error-default-subtle');
    }
  }
}
</style>
