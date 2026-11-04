<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="client-contract-page">
    <ms-report-text
      :theme="MsReportTheme.Error"
      v-if="error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>

    <template v-if="organizationStats && contractDetails">
      <div
        class="error-message"
        v-if="errorExceededStorage || errorExceededUsers"
      >
        <ms-report-text
          :theme="MsReportTheme.Error"
          v-if="errorExceededUsers"
        >
          {{ $msTranslate(errorExceededUsers) }}
        </ms-report-text>
        <ms-report-text
          :theme="MsReportTheme.Error"
          v-if="errorExceededStorage"
        >
          {{ $msTranslate(errorExceededStorage) }}
        </ms-report-text>
      </div>
      <div
        class="contract"
        v-if="contractDetails && organizationStats && !error"
      >
        <div class="contract-header">
          <div class="contract-header-title">
            <ion-text class="contract-header-title__text">
              {{ $msTranslate('clientArea.invoicesCustomOrder.contract') }}{{ contractDetails.number }}
            </ion-text>
          </div>
          <div class="contract-header-invoice">
            <ms-image
              class="contract-header-invoice__image"
              alt="pdf"
              :image="File.Pdf"
            />
            <ion-text class="contract-header-invoice__text">
              <span class="contract-header-invoice__label">{{ $msTranslate('clientArea.invoicesCustomOrder.number') }}</span>
              {{ contractDetails.number }}
            </ion-text>
            <div class="contract-header-invoice__button">
              <ion-button
                @click="Env.Links.openUrl(contractDetails.link)"
                class="contract-header-invoice__download color-secondary"
              >
                {{ $msTranslate('clientArea.invoicesCustomOrder.download') }}
              </ion-button>
            </div>
          </div>
        </div>

        <div class="contract-main">
          <!-- info -->
          <div class="contract-main-item">
            <div class="item-header">
              <ion-icon
                class="item-header__icon"
                :icon="informationCircle"
              />
              <ion-title class="item-header__title">
                {{ $msTranslate('clientArea.invoicesCustomOrder.information') }}
              </ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-column">
                <div class="item-content-date">
                  <ion-text class="item-content-date__label">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.startingDate') }}
                  </ion-text>
                  <ion-text
                    class="item-content-date__date"
                    v-if="contractDetails.licenseStart"
                  >
                    {{ $msTranslate(I18n.formatDate(contractDetails.licenseStart, 'short')) }}
                  </ion-text>
                  <ion-text
                    v-else
                    class="item-content-date__unavailable"
                  >
                    {{ $msTranslate('clientArea.contracts.errors.unavailable') }}
                  </ion-text>
                </div>
                <div class="item-content-date">
                  <ion-text class="item-content-date__label">{{ $msTranslate('clientArea.invoicesCustomOrder.endingDate') }}</ion-text>
                  <ion-text
                    class="item-content-date__date"
                    v-if="contractDetails.licenseEnd"
                  >
                    {{ $msTranslate(I18n.formatDate(contractDetails.licenseEnd, 'short')) }}
                  </ion-text>
                  <ion-text
                    v-else
                    class="item-content-date__unavailable"
                  >
                    {{ $msTranslate('clientArea.contracts.errors.unavailable') }}
                  </ion-text>
                </div>
              </div>
            </div>
          </div>
          <div class="contract-main-divider" />
          <!-- user -->
          <div class="contract-main-item">
            <div class="item-header">
              <ion-icon
                class="item-header__icon"
                :icon="person"
              />
              <ion-title class="item-header__title">
                {{
                  $msTranslate({
                    key: 'clientArea.invoicesCustomOrder.users.title',
                    count: contractDetails.administrators.quantityOrdered,
                  })
                }}
              </ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title">
                    {{
                      $msTranslate({
                        key: 'clientArea.invoicesCustomOrder.users.admin',
                        count: contractDetails.administrators.quantityOrdered,
                      })
                    }}
                  </ion-text>
                  <ion-text class="data-text__description">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.users.adminDescription') }}
                  </ion-text>
                </div>
                <ion-text class="data-number">{{ contractDetails.administrators.quantityOrdered }}</ion-text>
              </div>
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title">
                    {{
                      $msTranslate({
                        key: 'clientArea.invoicesCustomOrder.users.standard',
                        count: contractDetails.standards.quantityOrdered,
                      })
                    }}
                  </ion-text>
                </div>
                <ion-text class="data-number">{{ contractDetails.standards.quantityOrdered }}</ion-text>
              </div>
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.users.external') }}
                  </ion-text>
                  <ion-text class="data-text__description">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.users.externalDescription') }}
                  </ion-text>
                </div>
                <ion-text class="data-number">{{ contractDetails.outsiders.quantityOrdered }}</ion-text>
              </div>
            </div>
          </div>
          <div class="contract-main-divider" />
          <!-- storage -->
          <div class="contract-main-item">
            <div class="item-header">
              <ion-icon
                class="item-header__icon"
                :icon="pieChart"
              />
              <ion-title class="item-header__title">
                {{ $msTranslate('clientArea.invoicesCustomOrder.storage.title') }}
              </ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.storage.storage') }}
                  </ion-text>
                  <ion-text class="data-text__description">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.storage.stack') }}
                  </ion-text>
                </div>
                <ion-text class="data-number">
                  {{ contractDetails.storage.quantityOrdered }}
                  <span class="data-number__unit">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.storage.or') }}
                    {{ $msTranslate(formatFileSize(getStorageSize(contractDetails.storage.quantityOrdered))) }}
                  </span>
                </ion-text>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div
        class="organization-content"
        v-if="organizationStats && contractDetails"
      >
        <ion-title class="organization-content__title">{{ $msTranslate('clientArea.contracts.previewOrganization') }}</ion-title>
        <div
          class="organization-users"
          v-if="organizationStats.adminUsersDetail || organizationStats.standardUsersDetail || organizationStats.outsiderUsersDetail"
        >
          <!-- admins-->
          <div
            class="organization-users-item admins"
            v-if="organizationStats.adminUsersDetail"
          >
            <ion-text class="item-title">
              {{
                $msTranslate({
                  key: 'clientArea.contracts.user.admin',
                  data: { count: organizationStats.adminUsersDetail.active },
                })
              }}
            </ion-text>
            <div
              class="item-active"
              :class="getUserClass(organizationStats.adminUsersDetail.active, contractDetails.administrators.quantityOrdered)"
            >
              <ion-text class="item-active__number">
                {{ organizationStats.adminUsersDetail.active }}
                <span class="item-active__label">
                  {{
                    $msTranslate({
                      key: 'clientArea.contracts.user.onTotal',
                      data: { count: contractDetails.administrators.quantityOrdered },
                    })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text">
                  {{ Math.max(contractDetails.administrators.quantityOrdered - organizationStats.adminUsersDetail.active, 0) }}
                  {{ $msTranslate('clientArea.contracts.user.remaining') }}
                </ion-text>
              </div>
            </div>
          </div>
          <!-- standards-->
          <div
            class="organization-users-item members"
            v-if="organizationStats.standardUsersDetail"
          >
            <ion-text class="item-title">
              {{
                $msTranslate({
                  key: 'clientArea.contracts.user.standard',
                  data: { count: organizationStats.standardUsersDetail.active },
                })
              }}
            </ion-text>
            <div
              class="item-active"
              :class="getUserClass(organizationStats.standardUsersDetail.active, contractDetails.standards.quantityOrdered)"
            >
              <ion-text class="item-active__number">
                {{ organizationStats.standardUsersDetail.active }}
                <span class="item-active__label">
                  {{
                    $msTranslate({ key: 'clientArea.contracts.user.onTotal', data: { count: contractDetails.standards.quantityOrdered } })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text">
                  {{ Math.max(contractDetails.standards.quantityOrdered - organizationStats.standardUsersDetail.active, 0) }}
                  {{ $msTranslate('clientArea.contracts.user.remaining') }}
                </ion-text>
              </div>
            </div>
          </div>
          <!-- externals-->
          <div
            class="organization-users-item externals"
            v-if="organizationStats.outsiderUsersDetail"
          >
            <ion-text class="item-title">
              {{
                $msTranslate({
                  key: 'clientArea.contracts.user.external',
                  data: { count: organizationStats.outsiderUsersDetail.active },
                })
              }}
            </ion-text>
            <div
              class="item-active"
              :class="getUserClass(organizationStats.outsiderUsersDetail.active, contractDetails.outsiders.quantityOrdered)"
            >
              <ion-text class="item-active__number">
                {{ organizationStats.outsiderUsersDetail.active }}
                <span class="item-active__label">
                  {{
                    $msTranslate({ key: 'clientArea.contracts.user.onTotal', data: { count: contractDetails.outsiders.quantityOrdered } })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text">
                  {{ Math.max(contractDetails.outsiders.quantityOrdered - organizationStats.outsiderUsersDetail.active, 0) }}
                  {{ $msTranslate('clientArea.contracts.user.remaining') }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>

        <div class="organization-storage">
          <!-- storage-->
          <div class="organization-storage-item">
            <ion-text class="item-title">{{ $msTranslate('clientArea.contracts.storage.title') }}</ion-text>
            <div
              class="item-active"
              :class="getStorageClass()"
            >
              <ion-text class="item-active__number">
                {{ $msTranslate(formatFileSize(organizationStats.metadataSize + organizationStats.dataSize)) }}
                <span class="item-active__label">
                  {{
                    $msTranslate({
                      key: 'clientArea.contracts.storage.onTotal',
                      data: { count: I18n.translate(formatFileSize(getStorageSize(contractDetails.storage.quantityOrdered))) },
                    })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text">
                  {{ $msTranslate({ key: 'clientArea.contracts.storage.used', data: { percentage: getStoragePercentage() } }) }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
    <template v-if="querying">
      <div class="loading-container">
        <ms-spinner />
        <ion-text class="loading-text">{{ $msTranslate('clientArea.contracts.loading') }}</ion-text>
      </div>
    </template>
    <template v-if="isDefaultOrganization(currentOrganization)">
      <ion-text class="organization-choice-title">
        {{ $msTranslate('clientArea.contracts.multipleOrganizations') }}
      </ion-text>

      <div class="organization-list">
        <div
          class="organization-list-item"
          v-for="org in organizations"
          :key="org.bmsId"
          @click="onOrganizationSelected(org)"
        >
          {{ org.name }}
          <ion-icon
            :icon="arrowForward"
            slot="end"
            class="organization-list-item__icon"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import {
  BmsAccessInstance,
  BmsOrganization,
  CustomOrderDetailsResultData,
  CustomOrderStatus,
  DataType,
  OrganizationStatsResultData,
} from '@/services/bms';
import { Env } from '@/services/environment';
import { isDefaultOrganization } from '@/views/client-area/types';
import { IonButton, IonIcon, IonText, IonTitle } from '@ionic/vue';
import { arrowForward, informationCircle, person, pieChart } from 'ionicons/icons';
import { File, I18n, MsImage, MsReportText, MsReportTheme, MsSpinner } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  currentOrganization: BmsOrganization;
  organizations: Array<BmsOrganization>;
}>();

const emits = defineEmits<{
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

const contractStatus = ref(CustomOrderStatus.Unknown);
const contractDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);
const querying = ref(true);
const error = ref('');
const errorExceededUsers = ref('');
const errorExceededStorage = ref('');
const organizationStats = ref<OrganizationStatsResultData | undefined>(undefined);
const progressWidthStorage = ref('');

const progressWidthAdmin = computed(() => {
  if (!organizationStats.value || !contractDetails.value) {
    return `${0}%`;
  }
  const activeUsers = organizationStats.value.adminUsersDetail?.active || 0;
  const quantityOrdered = contractDetails.value.administrators.quantityOrdered;

  return `${Math.round((activeUsers / quantityOrdered) * 100)}%`;
});

const progressWidthStandard = computed(() => {
  if (!organizationStats.value || !contractDetails.value) {
    return `${0}%`;
  }
  const activeUsers = organizationStats.value.standardUsersDetail?.active || 0;
  const quantityOrdered = contractDetails.value.standards.quantityOrdered;

  return `${Math.round((activeUsers / quantityOrdered) * 100)}%`;
});

const progressWidthExternal = computed(() => {
  if (!organizationStats.value || !contractDetails.value) {
    return `${0}%`;
  }
  const activeUsers = organizationStats.value.outsiderUsersDetail?.active || 0;
  const quantityOrdered = contractDetails.value.outsiders.quantityOrdered;

  return `${Math.round((activeUsers / quantityOrdered) * 100)}%`;
});

onMounted(async () => {
  querying.value = true;
  try {
    if (!isDefaultOrganization(props.currentOrganization)) {
      const orgRep = await BmsAccessInstance.get().getOrganizationStats(props.currentOrganization.bmsId);
      if (!orgRep.isError && orgRep.data && orgRep.data.type === DataType.OrganizationStats) {
        organizationStats.value = orgRep.data;
      } else {
        error.value = 'clientArea.contracts.errors.noInfo';
        return;
      }
      const statusRep = await BmsAccessInstance.get().getCustomOrderStatus(props.currentOrganization);
      if (!statusRep.isError && statusRep.data && statusRep.data.type === DataType.CustomOrderStatus) {
        contractStatus.value = statusRep.data.status;
      }
      const detailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.currentOrganization);
      if (!detailsRep.isError && detailsRep.data && detailsRep.data.type === DataType.CustomOrderDetails) {
        contractDetails.value = detailsRep.data;
      } else {
        error.value = 'clientArea.contracts.errors.noInfo';
      }
    }
  } finally {
    querying.value = false;
  }
});

function getUsersPercentage(activeUsersValue: number, quantityOrderedValue: number): number {
  if (!organizationStats.value || !contractDetails.value) {
    return 0;
  }

  if (activeUsersValue > quantityOrderedValue) {
    errorExceededUsers.value = 'clientArea.contracts.user.errorExceed';
  }

  return Math.round((activeUsersValue / quantityOrderedValue) * 100);
}

function getUserClass(activeUsers: number, quantityOrdered: number): string {
  const percentage = getUsersPercentage(activeUsers, quantityOrdered);

  if (percentage > 100) {
    return 'item-active--exceeded';
  } else if (percentage > 90) {
    return 'item-active--warning';
  } else {
    return '';
  }
}

function getStorageClass(): string {
  const percentage = getStoragePercentage();

  if (percentage > 100) {
    return 'item-active--exceeded';
  } else if (percentage > 90) {
    return 'item-active--warning';
  } else {
    return '';
  }
}

function getStorageSize(value: number): number {
  if (!value) {
    return 0;
  }
  // Slice of 100Go
  return value * 100 * 1024 * 1024 * 1024;
}

function getStoragePercentage(): number {
  if (!organizationStats.value || !contractDetails.value) {
    return 0;
  }
  const ordered = getStorageSize(contractDetails.value.storage.quantityOrdered);
  const current = organizationStats.value.dataSize + organizationStats.value.metadataSize;

  if (current > ordered) {
    errorExceededStorage.value = 'clientArea.contracts.storage.errorExceed';
  }

  progressWidthStorage.value = `${Math.round((current / ordered) * 100)}%`;

  return Math.round((current / ordered) * 100);
}

async function onOrganizationSelected(org: BmsOrganization): Promise<void> {
  emits('organizationSelected', org);
}
</script>

<style scoped lang="scss">
.client-contract-page {
  display: flex;
  flex-direction: column;
  padding-bottom: ms.spacing('padding-7xl');

  &:has(.loading-container) {
    height: 100%;
  }
}

.error-message {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
  margin-bottom: 1.5rem;
  max-width: 70rem;
}

.loading-container {
  width: 100%;
  height: 100%;
  display: flex;
  gap: ms.spacing('gap-lg');
  flex-direction: column;
  justify-content: center;
  align-items: center;

  .loading-text {
    @include ms.font('body-lg-regular');
    color: ms.color('text-neutral-default');
  }
}

.contract {
  width: 100%;
  max-width: 70rem;
  height: fit-content;
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('2xl');
  overflow: hidden;
}

.contract-header {
  display: flex;
  justify-content: space-between;
  width: 100%;
  background: ms.color('surface-base-page');
  padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');

  &-title {
    display: flex;
    gap: ms.spacing('gap-3xl');
    align-items: center;

    &__text {
      @include ms.font('heading-h4');
      color: ms.color('text-base-body');
    }

    &__badge {
      color: ms.color('text-brand-default');
      background: ms.color('surface-brand-default-subtle-pressed');
      padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
      border-radius: ms.radius('6xl');
    }
  }

  &-invoice {
    display: flex;
    gap: ms.spacing('gap-3xl');
    max-width: 20rem;
    width: 100%;
    align-items: center;

    &__image {
      width: 2rem;
      height: 2rem;
    }

    &__text {
      @include ms.font('heading-h5');
      color: ms.color('text-base-body');
      display: flex;
      flex-direction: column;

      span {
        color: ms.color('text-base-description');
      }
    }

    &__button {
      margin-left: auto;
    }
  }
}

.contract-main {
  display: flex;
  flex-wrap: wrap;
  gap: ms.spacing('gap-5xl');
  padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl') ms.spacing('padding-5xl');

  &-divider {
    width: 1px;
    background: ms.color('surface-neutral-default-subtle-pressed');
  }

  &-item {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    width: 100%;
    min-width: 10rem;
    max-width: 20rem;

    &:has(.item-content-date) {
      width: fit-content;
      min-width: 15rem;
    }
  }

  .item-header {
    display: flex;
    gap: ms.spacing('gap-2xl');
    align-items: center;

    &__icon {
      color: ms.color('text-brand-default');
      background: ms.color('surface-brand-default-subtle');
      font-size: 1.5rem;
      padding: ms.spacing('padding-sm');
      border-radius: ms.radius('sm');
    }

    &__title {
      @include ms.font('heading-h5');
      color: ms.color('text-brand-default');
    }
  }

  .item-content {
    display: flex;
    flex-direction: column;

    &-row {
      display: flex;
      gap: ms.spacing('gap-5xl');
      justify-content: space-between;
      min-height: 2.5rem;
      padding-left: ms.spacing('padding-6xl');
    }

    &-column {
      display: flex;
      flex-direction: column;
      padding-left: ms.spacing('padding-6xl');
      gap: ms.spacing('gap-3xl');
    }

    &-date {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-2xl');

      &__label {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }

      &__date {
        @include ms.font('heading-h5');
        color: ms.color('text-base-body');
      }
    }

    .data-text {
      display: flex;
      flex-direction: column;
      padding: ms.spacing('padding-lg') ms.spacing('padding-none');

      &__title {
        @include ms.font('body-lg-medium');
        color: ms.color('text-neutral-default');
      }

      &__description {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }
    }

    .data-number {
      @include ms.font('heading-h3');
      padding: ms.spacing('padding-sm') ms.spacing('padding-none');
      color: ms.color('text-brand-default');

      span {
        color: ms.color('text-base-description');
      }
    }
  }
}

.organization-content {
  margin-top: 2.5rem;
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-brand-default');
  }
}

.organization-users,
.organization-storage {
  display: flex;
  background: ms.color('surface-base-page');
  border: ms.border('thin') solid ms.color('border-base-default');
  gap: 3rem;
  padding: ms.spacing('padding-4xl');
  border-radius: ms.radius('2xl');
  width: fit-content;

  &-item {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    width: 18rem;
  }

  .item-active {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    border-radius: ms.radius('lg');
    flex: 1;

    &__number {
      @include ms.font('heading-h1');
      display: flex;
      gap: ms.spacing('gap-sm');
      align-items: first baseline;
      color: ms.color('text-brand-default');
    }

    &__label {
      @include ms.font('body-lg-medium');
      color: ms.color('text-base-description');
      line-height: 0;
    }

    .progress {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-lg');
      --progressWidthStorage: v-bind(progressWidthStorage);

      &-bar {
        display: flex;
        background: ms.color('surface-disabled-default');
        border-radius: ms.radius('lg');

        &-used {
          width: var(--progressWidthStorage);
          background: ms.color('surface-brand-default');
          border-radius: ms.radius('lg');
        }

        &-unused {
          width: calc(100% - var(--progressWidthStorage));
        }
      }

      &-text {
        @include ms.font('body-md-medium');
        color: ms.color('text-base-description');
        align-self: flex-end;
      }
    }

    &--warning {
      .item-active__number {
        color: ms.color('text-warning-default');
      }

      .progress-bar-used {
        background: ms.color('surface-warning-default');
      }
    }

    &--exceeded {
      .item-active__number {
        color: ms.color('text-error-default');
      }

      .progress-bar-used {
        background: ms.color('surface-error-default');
      }
    }
  }
}

// Progress bar width
.organization-users {
  flex-wrap: wrap;
  .progress-bar {
    height: 0.25rem;
  }

  .admins {
    .progress-bar-used {
      width: v-bind(progressWidthAdmin);
    }

    .progress-bar-unused {
      width: calc(100% - v-bind(progressWidthAdmin));
    }
  }

  .members {
    .progress-bar-used {
      width: v-bind(progressWidthStandard);
    }

    .progress-bar-unused {
      width: calc(100% - v-bind(progressWidthStandard));
    }
  }

  .externals {
    .progress-bar-used {
      width: v-bind(progressWidthExternal);
    }

    .progress-bar-unused {
      width: calc(100% - v-bind(progressWidthExternal));
    }
  }
}

.organization-storage {
  .progress-bar {
    height: 0.75rem;
  }

  .progress-bar-used {
    width: v-bind(progressWidthStorage);
  }

  .progress-bar-unused {
    width: calc(100% - v-bind(progressWidthStorage));
  }
}

.organization-choice-title {
  @include ms.font('body-lg-regular');
  color: ms.color('text-neutral-default');
}

.organization-list {
  min-height: 4em;
  margin-top: 1.5rem;
  width: fit-content;
  display: flex;
  flex-direction: column;
  justify-content: left;
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-page');
  border: ms.border('thin') solid ms.color('border-base-default');
  overflow: hidden;

  &-item {
    @include ms.font('body-lg-medium');
    display: flex;
    justify-content: space-between;
    color: ms.color('text-brand-default');
    flex: 1;
    padding: ms.spacing('padding-3xl') ms.spacing('padding-3xl') ms.spacing('padding-3xl') ms.spacing('padding-4xl');
    cursor: pointer;
    min-width: 20rem;
    width: 100%;
    transition: background 0.2s;

    &__icon {
      color: ms.color('text-base-body');
    }

    &:hover {
      background: ms.color('surface-neutral-default-subtle-pressed');
    }
  }
}

.item-title {
  @include ms.font('heading-h5');
}

.contract-header-invoice__label {
  @include ms.font('body-md-regular');
}

.item-content-date__unavailable {
  @include ms.font('body-lg-medium');
}

.data-number__unit {
  @include ms.font('body-lg-medium');
}
</style>
