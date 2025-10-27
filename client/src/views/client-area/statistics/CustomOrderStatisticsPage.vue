<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-statistics">
    <template v-if="organizationStats && contractDetails">
      <!-- active users -->
      <div class="users-container">
        <!-- active users -->
        <div class="users active">
          <div class="users-text">
            <ion-title class="users-active-text__title title-h4">
              {{ $msTranslate('clientArea.statistics.titles.users') }}
              {{ $msTranslate('clientArea.statistics.titles.active') }}
            </ion-title>
          </div>
          <div class="users-cards-list">
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.admin', count: organizationStats.adminUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ organizationStats.adminUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.standard', count: organizationStats.standardUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ organizationStats.standardUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.outsider', count: organizationStats.outsiderUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ organizationStats.outsiderUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">{{ $msTranslate('clientArea.statistics.total') }}</ion-text>
                <!-- eslint-disable vue/html-indent -->
                <ion-text class="users-cards-list-item-text__number title-h1">
                  {{
                    organizationStats.outsiderUsersDetail.active +
                    organizationStats.adminUsersDetail.active +
                    organizationStats.standardUsersDetail.active
                  }}
                </ion-text>
              </div>
            </ion-card>
          </div>
        </div>
        <!-- revoked users -->
        <div class="users revoked">
          <div class="users-text">
            <ion-title class="users-active-text__title title-h4">
              {{ $msTranslate('clientArea.statistics.titles.users') }}
              {{ $msTranslate('clientArea.statistics.titles.revoked') }}
            </ion-title>
          </div>
          <div class="users-cards-list">
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.admin', count: organizationStats.adminUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ organizationStats.adminUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.standard', count: organizationStats.standardUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ organizationStats.standardUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.outsider', count: organizationStats.outsiderUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ organizationStats.outsiderUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
          </div>
        </div>
      </div>

      <!-- storage -->
      <div class="storage">
        <ion-text class="storage__title title-h4">{{ $msTranslate('clientArea.statistics.titles.storage') }}</ion-text>

        <div class="storage-data">
          <div class="storage-data-global">
            <ion-text
              class="storage-data-global__total title-h1"
              :class="{ overused: storagePercentage >= 100 }"
            >
              {{ $msTranslate(formatFileSize(totalData)) }}
            </ion-text>
            <div class="storage-data-global-info">
              <ion-text class="storage-data-global-info__text">
                <span class="title-h5">{{ $msTranslate(formatFileSize(organizationStats.dataSize)) }}</span>
                <span class="body">{{ $msTranslate('clientArea.statistics.data') }}</span>
              </ion-text>
              <ion-text class="storage-data-global-info__text">
                <span class="title-h5">{{ $msTranslate(formatFileSize(organizationStats.metadataSize)) }}</span>
                <span class="body">{{ $msTranslate('clientArea.statistics.metadata') }}</span>
              </ion-text>
            </div>
          </div>
          <div class="storage-data-usage">
            <ion-text class="storage-data-usage__title title-h5">
              {{ $msTranslate('clientArea.statistics.detail') }}
            </ion-text>

            <div class="usage">
              <ms-report-text
                :theme="MsReportTheme.Warning"
                v-if="storagePercentage === 100"
              >
                {{ $msTranslate('clientArea.statistics.fullUsed') }}
              </ms-report-text>
              <ms-report-text
                :theme="MsReportTheme.Error"
                v-if="storagePercentage > 90 && storagePercentage < 100"
              >
                {{ $msTranslate('clientArea.statistics.almostFullUsed') }}
              </ms-report-text>
              <ion-text class="usage-number title-h3">
                {{ $msTranslate(formatFileSize(remainingStorageSize)) }}
                <span class="subtitles-sm">{{ $msTranslate('clientArea.statistics.remaining') }}</span>
              </ion-text>
              <div class="usage-progress">
                <div
                  class="usage-progress__bar"
                  :class="{ overused: storagePercentage >= 100 }"
                  id="useStorageBar"
                />
                <div
                  class="usage-progress__bar"
                  id="remainingStorageBar"
                />
              </div>
              <div class="usage-caption">
                <span
                  class="subtitles-sm"
                  :class="{ overused: storagePercentage >= 100 }"
                >
                  {{ $msTranslate('clientArea.statistics.used') }}
                </span>
                <span class="subtitles-sm">{{ $msTranslate('clientArea.statistics.remaining') }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
    <template v-else-if="querying">
      <!-- active users -->
      <div class="users active skeleton">
        <ion-skeleton-text
          :animated="true"
          class="skeleton-title"
        />
        <div class="users-cards-list">
          <ion-card
            class="users-cards-list-item"
            v-for="index in 3"
            :key="index"
          >
            <div class="users-cards-list-item-text">
              <ion-skeleton-text
                :animated="true"
                class="skeleton-loading-number"
              />
              <ion-skeleton-text
                :animated="true"
                class="skeleton-loading-text"
              />
            </div>
          </ion-card>
        </div>
      </div>
      <!-- revoked users -->
      <div class="users revoked skeleton">
        <ion-skeleton-text
          :animated="true"
          class="skeleton-title"
        />
        <div class="users-cards-list">
          <ion-card
            class="users-cards-list-item"
            v-for="index in 3"
            :key="index"
          >
            <div class="users-cards-list-item-text">
              <ion-skeleton-text
                :animated="true"
                class="skeleton-loading-number"
              />
              <ion-skeleton-text
                :animated="true"
                class="skeleton-loading-text"
              />
            </div>
          </ion-card>
        </div>
      </div>
      <!-- storage -->
      <div class="storage skeleton">
        <ion-skeleton-text
          :animated="true"
          class="skeleton-loading-title"
        />
        <div class="storage-data">
          <div class="storage-data-global">
            <ion-skeleton-text
              :animated="true"
              class="skeleton-loading-number"
            />
            <div class="storage-data-global-info">
              <ion-skeleton-text
                :animated="true"
                class="skeleton-loading-text"
              />
              <ion-skeleton-text
                :animated="true"
                class="skeleton-loading-text"
              />
              <ion-skeleton-text
                :animated="true"
                class="skeleton-loading-text"
              />
            </div>
          </div>
          <div class="storage-data-usage">
            <ion-skeleton-text
              :animated="true"
              class="skeleton-loading-text"
            />
            <div class="storage-data-usage-content">
              <div class="usage-item">
                <div
                  id="firstBar"
                  class="usage"
                >
                  <div class="usage-number">
                    <ion-skeleton-text
                      :animated="true"
                      class="skeleton-loading-text"
                    />
                    <ion-skeleton-text
                      :animated="true"
                      class="skeleton-loading-text"
                    />
                  </div>
                  <ion-skeleton-text
                    :animated="true"
                    class="skeleton-loading-text"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
    <template v-else-if="!querying && !organizationStats && !contractDetails && !error && organizations.length">
      <ion-text class="organization-choice-title body-lg">
        {{ $msTranslate('clientArea.statistics.multipleOrganizations') }}
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
    <template v-else-if="!querying && !organizationStats && !contractDetails && !error">
      {{ $msTranslate('clientArea.statistics.noStats') }}
    </template>
    <ion-text
      class="statistics-error body"
      v-show="error"
    >
      {{ $msTranslate(error) }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { BmsAccessInstance, BmsOrganization, CustomOrderDetailsResultData, DataType, OrganizationStatsResultData } from '@/services/bms';
import { isDefaultOrganization } from '@/views/client-area/types';
import { IonCard, IonIcon, IonSkeletonText, IonText, IonTitle } from '@ionic/vue';
import { arrowForward } from 'ionicons/icons';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  currentOrganization: BmsOrganization;
  organizations: Array<BmsOrganization>;
}>();

const emits = defineEmits<{
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

const organizationStats = ref<OrganizationStatsResultData | undefined>(undefined);
const contractDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);
const storageSize = computed(() => (contractDetails.value ? contractDetails.value.storage.quantityOrdered * 100 * 1024 * 1024 * 1024 : 0));
const storagePercentage = computed(() =>
  organizationStats.value ? Math.min(Math.round((totalData.value / storageSize.value) * 100), 100) : 0,
);
const storageWidth = computed(() => `${storagePercentage.value}%`);
const remainingStorageSize = computed(() => (contractDetails.value ? Math.max(storageSize.value - totalData.value, 0) : 0));
const totalData = ref<number>(0);
const querying = ref(true);
const error = ref('');

onMounted(async () => {
  try {
    if (isDefaultOrganization(props.currentOrganization)) {
      return;
    }

    const organizationStatusResp = await BmsAccessInstance.get().getOrganizationStatus(props.currentOrganization.bmsId);
    if (organizationStatusResp.isError || organizationStatusResp.data?.type !== DataType.OrganizationStatus) {
      error.value = 'clientArea.statistics.error';
      return;
    }

    const isBootstrapped = organizationStatusResp.data.isBootstrapped ?? false;

    if (!isBootstrapped) {
      error.value = 'clientArea.statistics.notBootstrapped';
      return;
    }

    const orgStatsResponse = await BmsAccessInstance.get().getOrganizationStats(props.currentOrganization.bmsId);

    if (!orgStatsResponse.isError && orgStatsResponse.data?.type === DataType.OrganizationStats) {
      organizationStats.value = orgStatsResponse.data;
      totalData.value = organizationStats.value.dataSize + organizationStats.value.metadataSize;
    } else {
      error.value = 'clientArea.statistics.error';
    }

    const contractDetailsResponse = await BmsAccessInstance.get().getCustomOrderDetails(props.currentOrganization);
    if (
      !contractDetailsResponse.isError &&
      contractDetailsResponse.data &&
      contractDetailsResponse.data.type === DataType.CustomOrderDetails
    ) {
      contractDetails.value = contractDetailsResponse.data;
    } else {
      error.value = 'clientArea.statistics.error';
    }
  } finally {
    querying.value = false;
  }
});

async function onOrganizationSelected(org: BmsOrganization): Promise<void> {
  emits('organizationSelected', org);
}
</script>

<style scoped lang="scss">
.client-page-statistics {
  display: flex;
  gap: 2rem;
  flex-direction: column;
  width: fit-content;
}

.users-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  padding: 1.5rem;
  background: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-12);
  width: fit-content;
}

.users {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;

  &-text {
    display: flex;
    color: var(--parsec-color-light-secondary-text);
  }

  &-cards-list {
    display: flex;
    gap: 1.5rem;

    &-item {
      display: flex;
      border-radius: var(--parsec-radius-8);
      padding: 0.875rem;
      box-shadow: none;
      gap: 1rem;
      margin: 0;
      min-width: 9.125rem;
      box-shadow: var(--parsec-shadow-soft);

      &-text {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;

        &__number {
          color: var(--parsec-color-light-secondary-contrast);
        }

        &__title {
          color: var(--parsec-color-light-secondary-hard-grey);
          font-size: 11px;
          text-transform: uppercase;
        }
      }

      &:nth-child(4) {
        background: var(--parsec-color-light-secondary-text);

        .users-cards-list-item-text__title,
        .users-cards-list-item-text__number {
          color: var(--parsec-color-light-secondary-white);
        }
      }
    }
  }
}

.storage {
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 2rem;
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  border-radius: var(--parsec-radius-12);
  gap: 1.5rem;
  max-width: 50rem;

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-data {
    display: flex;
    gap: 3rem;

    &-usage {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      position: relative;
      width: 100%;

      &::before {
        content: '';
        display: block;
        position: absolute;
        top: 0.5rem;
        right: 0;
        width: 100%;
        height: 1px;
        background: var(--parsec-color-light-secondary-disabled);
      }

      &__title {
        color: var(--parsec-color-light-secondary-hard-grey);
        background: var(--parsec-color-light-secondary-inversed-contrast);
        padding-right: 1rem;
        width: fit-content;
        position: relative;
        z-index: 2;
      }
    }
  }
}

.storage-data-global {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  width: 100%;
  max-width: 9.5rem;

  &__total {
    color: var(--parsec-color-light-primary-600);

    &.overused {
      color: var(--parsec-color-light-danger-500);
    }
  }

  &-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    &__text {
      color: var(--parsec-color-light-secondary-hard-grey);
      display: flex;
      gap: 0.375rem;
      align-items: center;
    }
  }
}

.usage {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 0.25rem;

  &-number {
    display: flex;
    align-items: baseline;
    align-self: flex-end;
    gap: 0.25rem;
    justify-content: space-between;
    margin-bottom: 0.25rem;
    color: var(--parsec-color-light-secondary-text);
  }

  &-progress {
    display: flex;
    border-radius: var(--parsec-radius-12);
    overflow: hidden;
    width: 100%;
    background: var(--parsec-color-light-secondary-inversed-contrast);
    border: 1px solid var(--parsec-color-light-secondary-medium);

    &__bar {
      border-radius: var(--parsec-radius-12);
      height: 0.5rem;
    }

    #useStorageBar {
      width: v-bind(storageWidth);
      background: var(--parsec-color-light-primary-600);

      &.overused {
        background: var(--parsec-color-light-danger-500);
      }
    }

    #remainingStorageBar {
      width: calc(100% - v-bind(storageWidth));
      background: var(--parsec-color-light-secondary-disabled);
    }
  }

  &-caption {
    display: flex;
    margin-top: 0.75rem;
    width: 100%;
    gap: 1rem;
    color: var(--parsec-color-light-secondary-hard-grey);

    span {
      display: flex;
      align-items: center;
      gap: 0.25rem;

      &::before {
        content: '';
        display: flex;
        width: 0.5rem;
        height: 0.5rem;
        border-radius: var(--parsec-radius-circle);
      }

      &:nth-child(1)::before {
        background: var(--parsec-color-light-primary-600);
      }

      &:nth-child(2)::before {
        background: var(--parsec-color-light-secondary-disabled);
      }

      &.overused::before {
        background: var(--parsec-color-light-danger-500);
      }
    }
  }
}

.skeleton {
  &-loading-number {
    width: 6rem;
    height: 1.5rem;
  }

  &-loading-title {
    width: 8rem;
    height: 1rem;
  }

  .storage-data-usage::before {
    content: none;
  }

  .usage-number {
    .skeleton-loading-text {
      width: 2rem;
      height: 1rem;
    }
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
