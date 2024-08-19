<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-header class="sidebar-header">
    <ion-card class="organization-card">
      <ion-card-header
        class="organization-card-header"
        @click="openOrganizationChoice($event)"
      >
        <div
          class="card-header"
          v-if="!isDefaultOrganization(organization)"
        >
          <ion-avatar class="card-header-avatar">
            <span>{{ organization.parsecId.substring(0, 2) }}</span>
          </ion-avatar>
          <ion-card-title class="card-header-title title-h3">
            {{ organization.parsecId }}
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

        <div
          class="card-header-icon"
          v-show="organizations.length > 0"
        >
          <ms-image :image="ChevronExpand" />
        </div>
      </ion-card-header>

      <!-- show it only when there is one organization selected -->
      <template v-if="!isDefaultOrganization(organization) && status">
        <ion-text class="organization-card-state body">
          {{ $msTranslate('clientArea.sidebar.state.title') }}
          <span>{{ $msTranslate(status.isFrozen ? 'clientArea.sidebar.state.frozen' : 'clientArea.sidebar.state.active') }}</span>
        </ion-text>
      </template>

      <!-- button: go to organization -->
      <div
        class="organization-card-button custom-button custom-button-fill"
        @click="goToHome"
      >
        <ion-text
          class="button-medium"
          button
        >
          {{ $msTranslate(isDefaultOrganization(organization) ? 'clientArea.sidebar.goToHome' : 'clientArea.sidebar.goToOrganization') }}
        </ion-text>
      </div>
    </ion-card>
  </ion-header>

  <ion-content class="ion-padding sidebar-content">
    <!-- menu client -->
    <div class="menu-client">
      <!-- menu list -->
      <ion-list
        v-if="billingSystem === BillingSystem.Stripe"
        class="menu-client-list"
      >
        <!-- summary -->
        <ion-item
          button
          lines="none"
          class="menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Dashboard }"
          @click="goToPageClicked(ClientAreaPages.Dashboard)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="grid"
          />
          {{ $msTranslate('clientArea.sidebar.menu.summary') }}
        </ion-item>

        <!-- stats -->
        <ion-item
          button
          lines="none"
          class="menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Statistics }"
          @click="goToPageClicked(ClientAreaPages.Statistics)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="podium"
          />
          {{ $msTranslate('clientArea.sidebar.menu.stats') }}
        </ion-item>

        <!-- invoices -->
        <ion-item
          button
          lines="none"
          class="sidebar-item menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Invoices }"
          @click="goToPageClicked(ClientAreaPages.Invoices)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="newspaper"
          />
          {{ $msTranslate('clientArea.sidebar.menu.invoices') }}
        </ion-item>

        <!-- payment -->
        <ion-item
          button
          lines="none"
          class="menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.PaymentMethods }"
          @click="goToPageClicked(ClientAreaPages.PaymentMethods)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="card"
          />
          {{ $msTranslate('clientArea.sidebar.menu.billingMethod') }}
        </ion-item>

        <!-- billing -->
        <ion-item
          button
          lines="none"
          class="menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.BillingDetails }"
          @click="goToPageClicked(ClientAreaPages.BillingDetails)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="idCard"
          />
          {{ $msTranslate('clientArea.sidebar.menu.billingDetails') }}
        </ion-item>
      </ion-list>

      <ion-list
        v-if="billingSystem === BillingSystem.CustomOrder || billingSystem === BillingSystem.ExperimentalCandidate"
        class="menu-client-list"
      >
        <!-- contracts -->
        <ion-item
          button
          lines="none"
          class="menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.Contracts }"
          @click="goToPageClicked(ClientAreaPages.Contracts)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="grid"
          />
          {{ $msTranslate('clientArea.sidebar.menu.contracts') }}
        </ion-item>

        <!-- stats -->
        <ion-item
          button
          lines="none"
          class="menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.CustomOrderStatistics }"
          @click="goToPageClicked(ClientAreaPages.CustomOrderStatistics)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="podium"
          />
          {{ $msTranslate('clientArea.sidebar.menu.stats') }}
        </ion-item>

        <!-- invoices -->
        <ion-item
          button
          lines="none"
          class="sidebar-item menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.CustomOrderInvoices }"
          @click="goToPageClicked(ClientAreaPages.CustomOrderInvoices)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="newspaper"
          />
          {{ $msTranslate('clientArea.sidebar.menu.invoices') }}
        </ion-item>

        <!-- billing -->
        <ion-item
          button
          lines="none"
          class="menu-default menu-client-list-item"
          :class="{ 'current-page menu-active': currentPage === ClientAreaPages.CustomOrderBillingDetails }"
          @click="goToPageClicked(ClientAreaPages.CustomOrderBillingDetails)"
        >
          <ion-icon
            class="menu-client-list-item__icon"
            :icon="idCard"
          />
          {{ $msTranslate('clientArea.sidebar.menu.billingDetails') }}
        </ion-item>
      </ion-list>
    </div>

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
  </ion-content>
</template>

<script setup lang="ts">
import { ChevronExpand, MsImage, MsModalResult } from 'megashark-lib';
import { card, chatbubbleEllipses, podium, grid, idCard, newspaper } from 'ionicons/icons';
import { BmsAccessInstance, BmsOrganization, OrganizationStatusResultData, BillingSystem, DataType } from '@/services/bms';
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
  IonText,
  popoverController,
} from '@ionic/vue';
import OrganizationSwitchClientPopover from '@/components/organizations/OrganizationSwitchClientPopover.vue';
import { ClientAreaPages, isDefaultOrganization } from '@/views/client-area/types';
import { navigateTo, Routes } from '@/router';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  organization: BmsOrganization;
  currentPage: ClientAreaPages;
}>();
const status = ref<OrganizationStatusResultData | null>(null);
const billingSystem = ref(BmsAccessInstance.get().getPersonalInformation().billingSystem);
const organizations = ref<Array<BmsOrganization>>([]);

const emits = defineEmits<{
  (e: 'pageSelected', page: ClientAreaPages): void;
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

async function goToPageClicked(page: ClientAreaPages): Promise<void> {
  emits('pageSelected', page);
}

onMounted(async () => {
  if (isDefaultOrganization(props.organization)) {
    status.value = null;
  } else {
    const response = await BmsAccessInstance.get().getOrganizationStatus(props.organization.bmsId);
    if (!response.isError && response.data) {
      status.value = response.data as OrganizationStatusResultData;
    }
  }
  const response = await BmsAccessInstance.get().listOrganizations();
  if (!response.isError && response.data && response.data.type === DataType.ListOrganizations) {
    organizations.value = response.data.organizations;
  }
});

async function openOrganizationChoice(event: Event): Promise<void> {
  if (organizations.value.length === 0) {
    return;
  }
  const popover = await popoverController.create({
    component: OrganizationSwitchClientPopover,
    componentProps: {
      currentOrganization: props.organization,
    },
    cssClass: 'dropdown-popover',
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

async function goToHome(): Promise<void> {
  if (isDefaultOrganization(props.organization)) {
    await navigateTo(Routes.Home, { skipHandle: true });
  } else {
    await navigateTo(Routes.Home, { skipHandle: true, query: { bmsOrganizationId: props.organization.parsecId } });
  }
}
</script>

<style scoped lang="scss">
.sidebar-header {
  padding: 2rem 1.5rem 0;
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
    height: -webkit-fill-available;
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

  &-state {
    margin-inline: 0.5rem;
    color: var(--parsec-color-light-secondary-grey);

    span {
      color: var(--parsec-color-light-primary-500);
      font-weight: bold;
    }
  }
}

.menu-client {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  position: relative;
  margin-left: 0.5rem;

  &::after {
    content: '';
    position: absolute;
    bottom: -3rem;
    left: -2rem;
    width: calc(100% + 4rem);
    height: 1px;
    background: var(--parsec-color-light-secondary-disabled);
    z-index: 2;
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
      --padding-start: 0.75rem;
      --padding-end: 0.75rem;
      --padding-top: 0.5rem;
      --padding-bottom: 0.5rem;
      width: fit-content;
      border-radius: var(--parsec-radius-6);
      font-weight: 500;

      &__icon {
        color: var(--parsec-color-light-secondary-soft-grey);
        font-size: 1rem;
        margin-inline-end: 0.5rem;
        flex-shrink: 0;
      }

      &.current-page {
        --background: var(--parsec-color-light-secondary-medium);
        --background-hover: var(--parsec-color-light-secondary-medium);
        --color: var(--parsec-color-light-primary-700);

        .menu-client-list-item__icon {
          color: var(--parsec-color-light-primary-700);
        }
      }
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
</style>
