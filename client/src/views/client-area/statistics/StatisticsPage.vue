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
                <ion-text class="users-cards-list-item-text__title body-sm">
                  {{ $msTranslate({
                    key: 'clientArea.statistics.total',
                    count: stats.outsiderUsersDetail.active + stats.adminUsersDetail.active + stats.standardUsersDetail.active }) }}
                </ion-text>
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
        v-if="freeCircleData"
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
          <div class="storage-data-usage-content">
            <div class="usage">
              <div class="usage-data">
                <div class="usage-data-container">
                  <div class="usage-data-caption">
                    <ion-text class="usage-data-caption__number title-h5">
                      {{ $msTranslate(formatFileSize(freeSliceSize)) }}
                    </ion-text>
                    <ion-text class="usage-data-caption__text body">
                      {{ $msTranslate('clientArea.statistics.remaining') }}
                    </ion-text>
                  </div>
                  <div class="usage-data-caption">
                    <ion-text class="usage-data-caption__number title-h5">{{ $msTranslate(formatFileSize(totalData)) }}</ion-text>
                    <ion-text class="usage-data-caption__text body">{{ $msTranslate('clientArea.statistics.used') }}</ion-text>
                  </div>
                </div>
                {{ freeCircleData }}
                <progress-circle
                  :data="freeCircleData.amount"
                  :amount-value="80"
                  :text="$msTranslate('clientArea.statistics.includedTitle')"
                  :state="freeCircleData.percentage < 0.8 ? 'active' : 'success'"
                />
                <progress-circle
                  :data="payingCircleData ? payingCircleData.amount : 0"
                  :amount-value="payingCircleData ? payingCircleData.percentage : 0"
                  :text="$msTranslate('clientArea.statistics.extraTitle')"
                />
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
          class="skeleton-loading-title"
        />
        <div class="users-cards-list">
          <ion-card
            class="users-cards-list-item"
            v-for="index in 3"
            :key="index"
          >
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
            </div>
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
          class="skeleton-loading-title"
        />
        <div class="users-cards-list">
          <ion-card
            class="users-cards-list-item"
            v-for="index in 3"
            :key="index"
          >
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
            </div>
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
                  class="consumption"
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
    <template v-else-if="!querying && !stats && !error && organizations.length">
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
import { BmsAccessInstance, DataType, BmsOrganization, OrganizationStatsResultData } from '@/services/bms';
import { onMounted, ref } from 'vue';
import { IonCard, IonText, IonIcon, IonTitle, IonSkeletonText } from '@ionic/vue';
import { formatFileSize } from '@/common/file';
import { person, arrowForward } from 'ionicons/icons';
import { isDefaultOrganization } from '@/views/client-area/types';
import ProgressCircle from '@/components/client-area/ProgressCircle.vue';

interface CircleProgressData {
  amount: number;
  percentage: number;
  progress: number;
}

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

const freeCircleData = ref<CircleProgressData>();
const payingCircleData = ref<CircleProgressData>();

function getFreeCircleData(): CircleProgressData {
  return {
    amount: totalData.value > freeSliceSize.value ? freeSliceSize.value : totalData.value,
    percentage: totalData.value > freeSliceSize.value ? 100 : (totalData.value * 100) / freeSliceSize.value,
    progress: totalData.value > freeSliceSize.value ? 1 : totalData.value / freeSliceSize.value,
  };
}

function getPaidCircleData(): CircleProgressData | undefined {
  if (totalData.value < freeSliceSize.value) {
    return undefined;
  }

  const truncatedAmount: number = (totalData.value - freeSliceSize.value) % payingSliceSize.value;

  return {
    amount: truncatedAmount,
    percentage: (truncatedAmount * 100) / payingSliceSize.value,
    progress: truncatedAmount / payingSliceSize.value,
  };
}

onMounted(async () => {
  if (!isDefaultOrganization(props.currentOrganization)) {
    const response = await BmsAccessInstance.get().getOrganizationStats(props.currentOrganization.bmsId);

    if (!response.isError && response.data && response.data.type === DataType.OrganizationStats) {
      stats.value = response.data;

      totalData.value = stats.value.dataSize + stats.value.metadataSize;
      freeSliceSize.value = stats.value.freeSliceSize;
      payingSliceSize.value = stats.value.payingSliceSize;

      freeCircleData.value = getFreeCircleData();
      payingCircleData.value = getPaidCircleData();
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

        .users-cards-list-item-text__title, .users-cards-list-item-text__number {
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

  &__title {
    color: var(--parsec-color-light-secondary-text);
    width: 100%;
  }

  &-detail, &-usage {
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
  flex-direction: column;
  width: 100%;

  &-item {
    width: 100%;
    max-width: 20rem;
  }

  &-data {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--parsec-color-light-secondary-white);
    box-shadow: var(--parsec-shadow-soft);
    padding: 1.5rem;
    border-radius: var(--parsec-radius-12);
    gap: 1.5rem;

    &-container {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }

    &-caption {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      color: var(--parsec-color-light-secondary-text);
      position: relative;

      &::before {
        content: '';
        display: block;
        position: relative;
        top: 0;
        left: 0;
        width: 0.25rem;
        height: 1.25rem;
        margin-right: 0.25rem;
        border-radius: var(--parsec-radius-4);
        background: var(--parsec-color-light-secondary-text);
      }
    }

    &-caption {
      display: flex;
      gap: 0.25rem;

      &__number {
        color: var(--parsec-color-light-secondary-text);
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
