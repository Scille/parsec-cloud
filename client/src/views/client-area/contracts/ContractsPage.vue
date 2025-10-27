<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-contract-page">
    <span
      class="form-error body"
      v-show="error"
    >
      {{ $msTranslate(error) }}
    </span>

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
            <ion-text class="contract-header-title__text title-h3">
              {{ $msTranslate('clientArea.invoicesCustomOrder.contract') }}{{ contractDetails.number }}
            </ion-text>
          </div>
          <div class="contract-header-invoice">
            <ms-image
              class="contract-header-invoice__image"
              alt="pdf"
              :image="File.Pdf"
            />
            <ion-text class="contract-header-invoice__text title-h4">
              <span class="body">{{ $msTranslate('clientArea.invoicesCustomOrder.number') }}</span>
              {{ contractDetails.number }}
            </ion-text>
            <div class="contract-header-invoice__button">
              <a
                @click="Env.Links.openUrl(contractDetails.link)"
                class="custom-button custom-button-fill button-medium"
                download
              >
                {{ $msTranslate('clientArea.invoicesCustomOrder.download') }}
              </a>
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
              <ion-title class="item-header__title title-h4">
                {{ $msTranslate('clientArea.invoicesCustomOrder.information') }}
              </ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-column">
                <div class="item-content-date">
                  <ion-text class="item-content-date__label body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.startingDate') }}
                  </ion-text>
                  <ion-text
                    class="item-content-date__date title-h4"
                    v-if="contractDetails.licenseStart"
                  >
                    {{ $msTranslate(I18n.formatDate(contractDetails.licenseStart, 'short')) }}
                  </ion-text>
                  <ion-text
                    v-else
                    class="subtitles-normal"
                  >
                    {{ $msTranslate('clientArea.contracts.errors.unavailable') }}
                  </ion-text>
                </div>
                <div class="item-content-date">
                  <ion-text class="item-content-date__label body">{{ $msTranslate('clientArea.invoicesCustomOrder.endingDate') }}</ion-text>
                  <ion-text
                    class="item-content-date__date title-h4"
                    v-if="contractDetails.licenseEnd"
                  >
                    {{ $msTranslate(I18n.formatDate(contractDetails.licenseEnd, 'short')) }}
                  </ion-text>
                  <ion-text
                    v-else
                    class="subtitles-normal"
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
              <ion-title class="item-header__title title-h4">
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
                  <ion-text class="data-text__title subtitles-normal">
                    {{
                      $msTranslate({
                        key: 'clientArea.invoicesCustomOrder.users.admin',
                        count: contractDetails.administrators.quantityOrdered,
                      })
                    }}
                  </ion-text>
                  <ion-text class="data-text__description body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.users.adminDescription') }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">{{ contractDetails.administrators.quantityOrdered }}</ion-text>
              </div>
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title subtitles-normal">
                    {{
                      $msTranslate({
                        key: 'clientArea.invoicesCustomOrder.users.standard',
                        count: contractDetails.standards.quantityOrdered,
                      })
                    }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">{{ contractDetails.standards.quantityOrdered }}</ion-text>
              </div>
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title subtitles-normal">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.users.external') }}
                  </ion-text>
                  <ion-text class="data-text__description body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.users.externalDescription') }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">{{ contractDetails.outsiders.quantityOrdered }}</ion-text>
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
              <ion-title class="item-header__title title-h4">
                {{ $msTranslate('clientArea.invoicesCustomOrder.storage.title') }}
              </ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title subtitles-normal">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.storage.storage') }}
                  </ion-text>
                  <ion-text class="data-text__description body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.storage.stack') }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">
                  {{ contractDetails.storage.quantityOrdered }}
                  <span class="subtitles-normal">
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
        <ion-title class="organization-content__title title-h3">{{ $msTranslate('clientArea.contracts.previewOrganization') }}</ion-title>
        <div
          class="organization-users"
          v-if="organizationStats.adminUsersDetail || organizationStats.standardUsersDetail || organizationStats.outsiderUsersDetail"
        >
          <!-- admins-->
          <div
            class="organization-users-item admins"
            v-if="organizationStats.adminUsersDetail"
          >
            <ion-text class="item-title title-h4">
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
              <ion-text class="item-active__number title-h1-xl">
                {{ organizationStats.adminUsersDetail.active }}
                <span class="item-active__label subtitles-normal">
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
                <ion-text class="progress-text subtitles-sm">
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
            <ion-text class="item-title title-h4">
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
              <ion-text class="item-active__number title-h1-xl">
                {{ organizationStats.standardUsersDetail.active }}
                <span class="item-active__label subtitles-normal">
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
                <ion-text class="progress-text subtitles-sm">
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
            <ion-text class="item-title title-h4">
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
              <ion-text class="item-active__number title-h1-xl">
                {{ organizationStats.outsiderUsersDetail.active }}
                <span class="item-active__label subtitles-normal">
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
                <ion-text class="progress-text subtitles-sm">
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
            <ion-text class="item-title title-h4">{{ $msTranslate('clientArea.contracts.storage.title') }}</ion-text>
            <div
              class="item-active"
              :class="getStorageClass()"
            >
              <ion-text class="item-active__number title-h1-xl">
                {{ $msTranslate(formatFileSize(organizationStats.metadataSize + organizationStats.dataSize)) }}
                <span class="item-active__label subtitles-normal">
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
                <ion-text class="progress-text subtitles-sm">
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
        <ion-text class="body-lg loading-text">{{ $msTranslate('clientArea.contracts.loading') }}</ion-text>
      </div>
    </template>
    <template v-if="isDefaultOrganization(currentOrganization)">
      <ion-text class="organization-choice-title body-lg">
        {{ $msTranslate('clientArea.contracts.multipleOrganizations') }}
      </ion-text>

      <div class="organization-list">
        <div
          class="organization-list-item subtitles-normal"
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
import { IonIcon, IonText, IonTitle } from '@ionic/vue';
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
  padding-bottom: 3rem;

  &:has(.loading-container) {
    height: 100%;
  }
}

.error-message {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.loading-container {
  width: 100%;
  height: 100%;
  display: flex;
  gap: 0.5rem;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  .loading-text {
    color: var(--parsec-color-light-secondary-soft-text);
  }
}

.contract {
  width: 100%;
  max-width: 70rem;
  height: fit-content;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  overflow: hidden;
}

.contract-header {
  display: flex;
  justify-content: space-between;
  width: 100%;
  background: var(--parsec-color-light-secondary-background);
  padding: 0.75rem 1rem;

  &-title {
    display: flex;
    gap: 1rem;
    align-items: center;

    &__text {
      color: var(--parsec-color-light-secondary-text);
    }

    &__badge {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-100);
      padding: 0.25rem 0.5rem;
      border-radius: var(--parsec-radius-32);
    }
  }

  &-invoice {
    display: flex;
    gap: 1rem;
    max-width: 20rem;
    width: 100%;
    align-items: center;

    &__image {
      width: 2rem;
      height: 2rem;
    }

    &__text {
      color: var(--parsec-color-light-secondary-text);
      display: flex;
      flex-direction: column;

      span {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    &__button {
      margin-left: auto;

      .custom-button {
        background: var(--parsec-color-light-secondary-text);
        border: none;
        color: var(--parsec-color-light-secondary-white);

        &:hover {
          background: var(--parsec-color-light-secondary-contrast);
        }
      }
    }
  }
}

.contract-main {
  display: flex;
  flex-wrap: wrap;
  gap: 2rem;
  padding: 1.5rem 2rem 2rem;

  &-divider {
    width: 1px;
    background: var(--parsec-color-light-secondary-medium);
  }

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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
    gap: 0.75rem;
    align-items: center;

    &__icon {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-30);
      font-size: 1.5rem;
      padding: 0.25rem;
      border-radius: var(--parsec-radius-4);
    }

    &__title {
      color: var(--parsec-color-light-primary-900);
    }
  }

  .item-content {
    display: flex;
    flex-direction: column;

    &-row {
      display: flex;
      gap: 2rem;
      justify-content: space-between;
      min-height: 2.5rem;
      padding-left: 2.5rem;
    }

    &-column {
      display: flex;
      flex-direction: column;
      padding-left: 2.5rem;
      gap: 1rem;
    }

    &-date {
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &__label {
        color: var(--parsec-color-light-secondary-grey);
      }

      &__date {
        color: var(--parsec-color-light-secondary-text);
      }
    }

    .data-text {
      display: flex;
      flex-direction: column;
      padding: 0.5rem 0;

      &__title {
        color: var(--parsec-color-light-secondary-soft-text);
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    .data-number {
      padding: 0.25rem 0;
      color: var(--parsec-color-light-primary-600);

      span {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}

.organization-content {
  margin-top: 2.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &__title {
    color: var(--parsec-color-light-primary-700);
  }
}

.organization-users,
.organization-storage {
  display: flex;
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  gap: 3rem;
  padding: 1.5rem;
  border-radius: var(--parsec-radius-12);
  width: fit-content;

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 18rem;
  }

  .item-active {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    border-radius: var(--parsec-radius-8);
    flex: 1;

    &__number {
      display: flex;
      gap: 0.25rem;
      align-items: first baseline;
      color: var(--parsec-color-light-primary-600);
    }

    &__label {
      color: var(--parsec-color-light-secondary-grey);
      line-height: 0;
    }

    .progress {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      --progressWidthStorage: v-bind(progressWidthStorage);

      &-bar {
        display: flex;
        background: var(--parsec-color-light-secondary-disabled);
        border-radius: var(--parsec-radius-8);

        &-used {
          width: var(--progressWidthStorage);
          background: var(--parsec-color-light-primary-500);
          border-radius: var(--parsec-radius-8);
        }

        &-unused {
          width: calc(100% - var(--progressWidthStorage));
        }
      }

      &-text {
        color: var(--parsec-color-light-secondary-hard-grey);
        align-self: flex-end;
      }
    }

    &--warning {
      .item-active__number {
        color: var(--parsec-color-light-warning-500);
      }

      .progress-bar-used {
        background: var(--parsec-color-light-warning-500);
      }
    }

    &--exceeded {
      .item-active__number {
        color: var(--parsec-color-light-danger-500);
      }

      .progress-bar-used {
        background: var(--parsec-color-light-danger-500);
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
  color: var(--parsec-color-light-secondary-soft-text);
}

.organization-list {
  min-height: 4em;
  margin-top: 1.5rem;
  width: fit-content;
  display: flex;
  flex-direction: column;
  justify-content: left;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  overflow: hidden;

  &-item {
    display: flex;
    justify-content: space-between;
    color: var(--parsec-color-light-primary-700);
    flex: 1;
    padding: 1em 1rem 1rem 1.5rem;
    cursor: pointer;
    min-width: 20rem;
    width: 100%;
    transition: background 0.2s;

    &__icon {
      color: var(--parsec-color-light-secondary-text);
    }

    &:hover {
      background: var(--parsec-color-light-secondary-medium);
    }
  }
}
</style>
