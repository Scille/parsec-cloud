<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-statistics">
    <template v-if="stats">
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
                  {{ $msTranslate({ key: 'clientArea.statistics.admin', count: stats.adminUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.adminUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.standard', count: stats.standardUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.standardUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.outsider', count: stats.outsiderUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.outsiderUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">{{ $msTranslate('clientArea.statistics.total') }}</ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">
                  {{ stats.outsiderUsersDetail.active + stats.adminUsersDetail.active + stats.standardUsersDetail.active }}
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
                  {{ $msTranslate({ key: 'clientArea.statistics.admin', count: stats.adminUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.adminUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.standard', count: stats.standardUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.standardUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({ key: 'clientArea.statistics.outsider', count: stats.outsiderUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.outsiderUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
          </div>
        </div>
      </div>
      <!-- storage -->
      <div
        class="storage"
        v-if="includedData"
      >
        <ion-text class="storage__title title-h4">{{ $msTranslate('clientArea.statistics.titles.storage') }}</ion-text>

        <div class="storage-detail">
          <ion-text class="storage-detail__title title-h5">
            <span>{{ $msTranslate('clientArea.statistics.detail') }}</span>
          </ion-text>
          <div class="storage-detail-data">
            <ion-text class="storage-detail-data__total title-h1">{{ $msTranslate(formatFileSize(totalData)) }}</ion-text>
            <div class="storage-detail-data-info">
              <ion-text class="storage-detail-data-info__text">
                <span class="body">{{ $msTranslate('clientArea.statistics.data') }}</span>
                <span class="title-h4">{{ $msTranslate(formatFileSize(stats.dataSize)) }}</span>
              </ion-text>
              <ion-text class="storage-detail-data-info__text">
                <span class="body">{{ $msTranslate('clientArea.statistics.metadata') }}</span>
                <span class="title-h4">{{ $msTranslate(formatFileSize(stats.metadataSize)) }}</span>
              </ion-text>
            </div>
          </div>
        </div>

        <div class="storage-usage">
          <ion-text class="storage-usage__title title-h5">
            <span>{{ $msTranslate('clientArea.statistics.usage') }}</span>
          </ion-text>
          <div class="usage">
            <div class="usage-data">
              <ion-text class="usage-data__title title-h5">{{ $msTranslate('clientArea.statistics.included') }}</ion-text>

              <div class="usage-data-content">
                <div class="usage-data-caption">
                  <ion-text class="usage-data-caption__title title-h2">
                    {{
                      totalData > INCLUDED_STORAGE
                        ? $msTranslate(formatFileSize(INCLUDED_STORAGE))
                        : $msTranslate(formatFileSize(totalData))
                    }}
                  </ion-text>
                  <ion-text class="usage-data-caption__description body">
                    {{ $msTranslate(formatFileSize(INCLUDED_STORAGE)) }}
                    <span>{{ $msTranslate('clientArea.statistics.included') }}</span>
                  </ion-text>
                </div>
                <progress-circle
                  :amount-value="Math.trunc(includedData.percentage)"
                  :state="'default'"
                />
              </div>
            </div>

            <div class="usage-data extra">
              <ion-text class="usage-data__title title-h5">{{ $msTranslate('clientArea.statistics.extra') }}</ion-text>
              <div class="usage-data-content">
                <div class="usage-data-caption">
                  <ion-text class="usage-data-caption__title title-h2">
                    {{ $msTranslate(formatFileSize(payingData ? payingData.amount : 0)) }}
                  </ion-text>
                  <ion-text class="usage-data-caption__description body">
                    {{ $msTranslate('clientArea.statistics.additionalCost') }}
                  </ion-text>
                </div>
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
          class="skeleton-title"
        />
        <div class="storage-data">
          <div class="storage-data-global">
            <ion-skeleton-text
              :animated="true"
              class="storage-data-global-title"
            />
            <div class="storage-data-global-info">
              <ion-skeleton-text
                :animated="true"
                class="skeleton-number"
              />
              <ion-skeleton-text
                :animated="true"
                class="skeleton-text"
              />
              <ion-skeleton-text
                :animated="true"
                class="skeleton-text"
              />
            </div>
          </div>
          <div class="storage-data-usage">
            <ion-skeleton-text
              :animated="true"
              class="storage-data-usage-title"
            />
            <div class="storage-data-usage-info">
              <div class="usage-info-container">
                <div class="usage-info-data">
                  <ion-skeleton-text
                    :animated="true"
                    class="skeleton-number"
                  />
                  <ion-skeleton-text
                    :animated="true"
                    class="skeleton-text"
                  />
                  <ion-skeleton-text
                    :animated="true"
                    class="skeleton-text"
                  />
                </div>
                <ion-skeleton-text
                  :animated="true"
                  class="skeleton-circle"
                />
              </div>
              <div class="usage-info-container">
                <div class="usage-info-data">
                  <ion-skeleton-text
                    :animated="true"
                    class="skeleton-number"
                  />
                  <ion-skeleton-text
                    :animated="true"
                    class="skeleton-text"
                  />
                  <ion-skeleton-text
                    :animated="true"
                    class="skeleton-text"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
    <template v-else-if="!querying && !stats && !error && organizations.length">
      <ion-text class="organization-choice-title body-lg">
        {{ $msTranslate('clientArea.statistics.multipleOrganizations') }}
      </ion-text>

      <div class="organization-list">
        <ms-search-input
          placeholder="HomePage.organizationList.search"
          v-model="searchQuery"
          id="search-input-organization"
          v-if="organizations.length > 4"
        />
        <div
          class="organization-list-item subtitles-normal"
          v-for="org in filteredOrganizations"
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
    <template v-else-if="!querying && !stats && !error && organizations.length === 0">
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
import ProgressCircle from '@/components/client-area/ProgressCircle.vue';
import { BmsAccessInstance, BmsOrganization, DataType, OrganizationStatsResultData } from '@/services/bms';
import { isDefaultOrganization } from '@/views/client-area/types';
import { IonCard, IonIcon, IonSkeletonText, IonText, IonTitle } from '@ionic/vue';
import { arrowForward } from 'ionicons/icons';
import { MsSearchInput } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

interface CircleProgressData {
  amount: number;
  percentage: number;
  progress: number;
}

const INCLUDED_STORAGE = 1024 * 1024 * 1024 * 100; // 100GB

const props = defineProps<{
  currentOrganization: BmsOrganization;
  organizations: Array<BmsOrganization>;
}>();

const emits = defineEmits<{
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

const stats = ref<OrganizationStatsResultData>();
const totalData = ref<number>(0);
const freeSliceSize = ref<number>(0);
const payingSliceSize = ref<number>(0);
const querying = ref(true);
const error = ref('');
const searchQuery = ref('');

const includedData = ref<CircleProgressData>();
const payingData = ref<CircleProgressData>();

function getIncludedData(): CircleProgressData {
  return {
    amount: totalData.value > freeSliceSize.value ? freeSliceSize.value : totalData.value,
    percentage: totalData.value > INCLUDED_STORAGE ? 100 : (freeSliceSize.value * 100) / INCLUDED_STORAGE,
    progress: totalData.value > INCLUDED_STORAGE ? 1 : freeSliceSize.value / INCLUDED_STORAGE,
  };
}

const filteredOrganizations = computed(() => {
  const query = searchQuery.value.trim();
  if (query) {
    return props.organizations.filter((org) => org.name.toLowerCase().includes(query.toLowerCase()));
  } else {
    return props.organizations;
  }
});

function getExtraData(): CircleProgressData | undefined {
  if (totalData.value < freeSliceSize.value) {
    return undefined;
  }

  const truncatedAmount: number = totalData.value - freeSliceSize.value;

  return {
    amount: totalData.value - freeSliceSize.value,
    percentage: (truncatedAmount * 100) / payingSliceSize.value,
    progress: truncatedAmount / payingSliceSize.value,
  };
}

onMounted(async () => {
  if (!isDefaultOrganization(props.currentOrganization)) {
    // TODO: If the organization is not bootstrapped (needs to request status),
    // we should redirect directly to dashboard page or show an error message like the CustomOrderStatisticsPage.
    // https://github.com/Scille/parsec-cloud/issues/10417
    const response = await BmsAccessInstance.get().getOrganizationStats(props.currentOrganization.bmsId);

    if (!response.isError && response.data && response.data.type === DataType.OrganizationStats) {
      stats.value = response.data;

      totalData.value = stats.value.dataSize + stats.value.metadataSize;
      freeSliceSize.value = totalData.value > INCLUDED_STORAGE ? INCLUDED_STORAGE : totalData.value;
      payingSliceSize.value = stats.value.payingSliceSize;

      includedData.value = getIncludedData();
      payingData.value = getExtraData();
    } else {
      error.value = 'clientArea.statistics.error';
    }
  }
  querying.value = false;
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
    flex-wrap: wrap;

    &-item {
      display: flex;
      border-radius: var(--parsec-radius-8);
      padding: 0.875rem;
      box-shadow: none;
      gap: 1rem;
      margin: 0;
      min-width: 9.125rem;
      box-shadow: var(--parsec-shadow-soft);

      @include ms.responsive-breakpoint('sm') {
        flex: 1 1 calc(50% - 0.75rem);
        max-width: calc(50% - 0.75rem);
      }

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
  flex-wrap: wrap;
  background: var(--parsec-color-light-secondary-background);
  padding: 1.5rem;
  margin-bottom: 2.5rem;
  border-radius: var(--parsec-radius-12);
  gap: 2rem;
  width: 100%;

  @include ms.responsive-breakpoint('lg') {
    flex-direction: column;
  }

  &__title {
    color: var(--parsec-color-light-secondary-text);
    width: 100%;
  }

  &-detail,
  &-usage {
    display: flex;
    gap: 1.5rem;
    flex-direction: column;

    &__title {
      position: relative;
      display: flex;
      color: var(--parsec-color-light-secondary-hard-grey);

      span {
        position: relative;
        background: var(--parsec-color-light-secondary-background);
        padding-right: 0.5rem;
        z-index: 2;
      }

      &::after {
        z-index: 1;
        content: '';
        display: block;
        position: absolute;
        top: 0.5rem;
        right: 0;
        width: 100%;
        height: 1px;
        background: var(--parsec-color-light-secondary-disabled);
      }
    }
  }

  &-detail {
    flex: 1;

    @include ms.responsive-breakpoint('lg') {
      flex-direction: row;
    }
  }

  &-usage {
    flex: 2;
  }
}

.storage-detail-data {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  width: 100%;
  max-width: 9.5rem;

  @include ms.responsive-breakpoint('lg') {
    gap: 1.25rem;
    flex-direction: row;
    max-width: 100%;
  }

  &__total {
    color: var(--parsec-color-light-secondary-text);
  }

  &-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    max-width: 9.5rem;

    &__text {
      color: var(--parsec-color-light-secondary-hard-grey);
      display: flex;
      gap: 0.375rem;
      align-items: center;
      justify-content: space-between;
    }
  }
}

.usage {
  display: flex;
  width: 100%;
  gap: 1.5rem;

  @include ms.responsive-breakpoint('lg') {
    flex-direction: column;
  }

  &-data {
    display: flex;
    flex-direction: column;
    background: var(--parsec-color-light-secondary-white);
    width: 100%;
    box-shadow: var(--parsec-shadow-soft);
    padding: 1.5rem;
    border-radius: var(--parsec-radius-12);
    min-width: 18rem;

    &__title {
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &-content {
      display: flex;
      align-items: center;
      gap: 1.5rem;
      height: 100%;
    }

    &-caption {
      display: flex;
      flex-direction: column;
      gap: 0.375rem;
      width: 100%;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}

.skeleton {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &-title {
    width: 8rem;
    height: 1rem;
  }

  .storage-data-global {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 100%;
    max-width: 9.5rem;

    .skeleton-number {
      width: 8rem;
      height: 2rem;
    }

    .skeleton-text {
      max-width: 9.5rem;
      height: 1rem;
    }

    &-info {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
      max-width: 9.5rem;
    }
  }

  &-loading-text {
    width: 4rem;
    height: 1rem;
  }

  .storage-data {
    display: flex;
    gap: 2rem;
  }

  .storage-data-usage {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 100%;
  }

  .storage-data-usage-info {
    display: flex;
    gap: 1.5rem;
    width: 100%;

    .usage-info-container {
      display: flex;
      background: var(--parsec-color-light-secondary-white);
      border-radius: var(--parsec-radius-12);
      gap: 1rem;
      padding: 1.5rem;
      width: 100%;

      .usage-info-data {
        display: flex;
        flex-direction: column;
        gap: 0.375rem;
        width: 100%;
      }

      .skeleton-number {
        width: 8rem;
        height: 2rem;
      }

      .skeleton-text {
        max-width: 9.5rem;
        height: 1rem;
      }

      .skeleton-circle {
        width: 5rem;
        height: 5rem;
        flex-shrink: 0;
        align-self: flex-end;
        border-radius: var(--parsec-radius-circle);
      }
    }
  }
}

.organization-choice-title {
  color: var(--parsec-color-light-secondary-soft-text);
}

.organization-list {
  min-height: 4em;
  width: fit-content;
  display: flex;
  flex-direction: column;
  justify-content: left;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  overflow: auto;

  #search-input-organization {
    border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
    flex-shrink: 0;
    padding: 0.5rem 0.5rem 0.5rem 1.25rem;
    border: none;
    border-radius: 0;
    position: sticky;
    top: 0rem;
    background: var(--parsec-color-light-secondary-white);
    z-index: 10;

    &:focus-within {
      outline: none;
    }
  }

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
