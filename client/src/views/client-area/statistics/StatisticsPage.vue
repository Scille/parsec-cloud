<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-statistics">
    <template v-if="stats">
      <!-- active users -->
      <div class="users active">
        <div class="users-text">
          <ion-title class="users-active-text__title title-h2">
            {{ $msTranslate('clientArea.statistics.titles.users') }}
            <span class="subtitles-normal">
              {{ $msTranslate('clientArea.statistics.titles.active') }}
            </span>
          </ion-title>
        </div>
        <div class="users-cards-list">
          <ion-card class="users-cards-list-item">
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
              <ion-icon
                class="users-cards-list-item-icons__secondary"
                :icon="star"
              />
            </div>
            <div class="users-cards-list-item-text">
              <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.adminUsersDetail.active }}</ion-text>
              <ion-text class="users-cards-list-item-text__text subtitles-sm">
                {{ $msTranslate(stats.adminUsersDetail.active > 1 ? 'clientArea.statistics.admins' : 'clientArea.statistics.admin') }}
              </ion-text>
            </div>
          </ion-card>
          <ion-card class="users-cards-list-item">
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
              <ion-icon
                class="users-cards-list-item-icons__secondary"
                :icon="pencil"
              />
            </div>
            <div class="users-cards-list-item-text">
              <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.standardUsersDetail.active }}</ion-text>
              <ion-text class="users-cards-list-item-text__text subtitles-sm">
                {{
                  $msTranslate(stats.standardUsersDetail.active > 1 ? 'clientArea.statistics.standards' : 'clientArea.statistics.standard')
                }}
              </ion-text>
            </div>
          </ion-card>
          <ion-card class="users-cards-list-item">
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
              <ion-icon
                class="users-cards-list-item-icons__secondary"
                :icon="arrowForward"
              />
            </div>
            <div class="users-cards-list-item-text">
              <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.outsiderUsersDetail.active }}</ion-text>
              <ion-text class="users-cards-list-item-text__text subtitles-sm">
                {{
                  $msTranslate(stats.outsiderUsersDetail.active > 1 ? 'clientArea.statistics.outsiders' : 'clientArea.statistics.outsider')
                }}
              </ion-text>
            </div>
          </ion-card>
        </div>
      </div>
      <!-- revoked users -->
      <div class="users revoked">
        <div class="users-text">
          <ion-title class="users-active-text__title title-h2">
            {{ $msTranslate('clientArea.statistics.titles.users') }}
            <span class="subtitles-normal">
              {{ $msTranslate('clientArea.statistics.titles.revoked') }}
            </span>
          </ion-title>
        </div>
        <div class="users-cards-list">
          <ion-card class="users-cards-list-item">
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
              <ion-icon
                class="users-cards-list-item-icons__secondary"
                :icon="star"
              />
            </div>
            <div class="users-cards-list-item-text">
              <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.adminUsersDetail.revoked }}</ion-text>
              <ion-text class="users-cards-list-item-text__text subtitles-sm">
                {{ $msTranslate(stats.adminUsersDetail.revoked > 1 ? 'clientArea.statistics.admins' : 'clientArea.statistics.admin') }}
              </ion-text>
            </div>
          </ion-card>
          <ion-card class="users-cards-list-item">
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
              <ion-icon
                class="users-cards-list-item-icons__secondary"
                :icon="pencil"
              />
            </div>
            <div class="users-cards-list-item-text">
              <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.standardUsersDetail.revoked }}</ion-text>
              <ion-text class="users-cards-list-item-text__text subtitles-sm">
                {{
                  $msTranslate(stats.standardUsersDetail.revoked > 1 ? 'clientArea.statistics.standards' : 'clientArea.statistics.standard')
                }}
              </ion-text>
            </div>
          </ion-card>
          <ion-card class="users-cards-list-item">
            <div class="users-cards-list-item-icons">
              <ion-icon
                class="users-cards-list-item-icons__main"
                :icon="person"
              />
              <ion-icon
                class="users-cards-list-item-icons__secondary"
                :icon="arrowForward"
              />
            </div>
            <div class="users-cards-list-item-text">
              <ion-text class="users-cards-list-item-text__number title-h1">{{ stats.outsiderUsersDetail.revoked }}</ion-text>
              <ion-text class="users-cards-list-item-text__text subtitles-sm">
                {{
                  $msTranslate(stats.outsiderUsersDetail.revoked > 1 ? 'clientArea.statistics.outsiders' : 'clientArea.statistics.outsider')
                }}
              </ion-text>
            </div>
          </ion-card>
        </div>
      </div>
      <!-- storage -->
      <div class="storage">
        <ion-text class="storage__title title-h2">{{ $msTranslate('clientArea.statistics.titles.storage') }}</ion-text>
        <div class="storage-data">
          <div class="storage-data-global">
            <ion-text class="storage-data-global__total title-h1">{{ $msTranslate(formatFileSize(totalData)) }}</ion-text>
            <div class="storage-data-global-info">
              <ion-text class="storage-data-global-info__text body">{{ $msTranslate('clientArea.statistics.ofWhich') }}</ion-text>
              <ion-text class="storage-data-global-info__text">
                <span class="title-h5">{{ $msTranslate(formatFileSize(stats.dataSize)) }}</span>
                <span class="body">{{ $msTranslate('clientArea.statistics.data') }}</span>
              </ion-text>
              <ion-text class="storage-data-global-info__text">
                <span class="title-h5">{{ $msTranslate(formatFileSize(stats.metadataSize)) }}</span>
                <span class="body">{{ $msTranslate('clientArea.statistics.metadata') }}</span>
              </ion-text>
            </div>
          </div>
          <div class="storage-data-consumption">
            <ion-text class="storage-data-consumption__title title-h5">
              {{ $msTranslate('clientArea.statistics.consumptionDetail') }}
            </ion-text>
            <div class="storage-data-consumption-content">
              <div
                v-if="firstBarData"
                class="consumption-item"
              >
                <div
                  id="firstBar"
                  class="consumption"
                >
                  <div class="consumption-number">
                    <ion-text class="consumption-number__amount title-h4">{{ $msTranslate(formatFileSize(firstBarData.amount)) }}</ion-text>
                    <ion-text class="consumption-number__percentage title-h5">{{ firstBarData.percentage.toFixed(0) }}%</ion-text>
                  </div>
                  <ion-progress-bar
                    :value="firstBarData.progress"
                    class="consumption__bar"
                  />
                  <ion-text class="consumption__price subtitles-sm">
                    {{ $msTranslate('clientArea.statistics.free') }}
                  </ion-text>
                </div>
              </div>
              <!-- secondBarData -->
              <div
                v-if="secondBarData"
                class="consumption-item"
              >
                <div
                  id="secondBar"
                  class="consumption"
                >
                  <div class="consumption-number">
                    <ion-text>
                      <span class="consumption-number__amount title-h4">{{ $msTranslate(formatFileSize(secondBarData.amount)) }}</span>
                      <span class="number-multiplier">
                        {{ secondBarData.multiplier > 0 ? `<span>x${secondBarData.multiplier}</span>` : '' }}
                      </span>
                    </ion-text>
                    <ion-text class="consumption-number__percentage title-h5">{{ secondBarData.percentage.toFixed(0) }}%</ion-text>
                  </div>
                  <ion-progress-bar
                    :value="secondBarData.progress"
                    class="consumption__bar"
                  />
                </div>
                <div
                  id="thirdBar"
                  class="consumption"
                  v-if="thirdBarData"
                >
                  <div class="consumption-number">
                    <ion-text class="consumption-number__amount title-h4">
                      {{ $msTranslate(formatFileSize(thirdBarData.amount)) }}
                    </ion-text>
                    <ion-text class="consumption-number__percentage title-h5">{{ thirdBarData.percentage.toFixed(0) }}%</ion-text>
                  </div>
                  <ion-progress-bar
                    :value="thirdBarData.progress"
                    class="consumption__bar"
                  />
                </div>
                <ion-text class="consumption__price paid subtitles-sm">
                  {{ $msTranslate('clientArea.statistics.paying') }}
                </ion-text>
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
          <div class="storage-data-consumption">
            <ion-skeleton-text
              :animated="true"
              class="skeleton-loading-text"
            />
            <div class="storage-data-consumption-content">
              <div class="consumption-item">
                <div
                  id="firstBar"
                  class="consumption"
                >
                  <div class="consumption-number">
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
    <template v-else-if="!querying && !stats && !error && allOrganizations.length">
      <ion-text class="organization-choice-title body-lg">
        {{ $msTranslate('clientArea.statistics.multipleOrganizations') }}
      </ion-text>

      <div class="organization-list">
        <div
          class="organization-list-item subtitles-normal"
          v-for="org in allOrganizations"
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
    <template v-else-if="!querying && !stats && !error && allOrganizations.length === 0">
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
import { IonCard, IonProgressBar, IonText, IonIcon, IonTitle, IonSkeletonText } from '@ionic/vue';
import { formatFileSize } from '@/common/file';
import { person, star, pencil, arrowForward } from 'ionicons/icons';
import { isDefaultOrganization } from '@/views/client-area/types';

interface ProgressBarData {
  amount: number;
  percentage: number;
  progress: number;
  multiplier: number;
}

const props = defineProps<{
  organization: BmsOrganization;
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
const allOrganizations = ref<Array<BmsOrganization>>([]);

const firstBarData = ref<ProgressBarData>();
const secondBarData = ref<ProgressBarData>();
const thirdBarData = ref<ProgressBarData>();

function getFirstBarData(): ProgressBarData {
  return {
    amount: totalData.value > freeSliceSize.value ? freeSliceSize.value : totalData.value,
    percentage: totalData.value > freeSliceSize.value ? 100 : (totalData.value * 100) / freeSliceSize.value,
    progress: totalData.value > freeSliceSize.value ? 1 : totalData.value / freeSliceSize.value,
    multiplier: 0,
  };
}

function getSecondBarData(): ProgressBarData | undefined {
  if (totalData.value < freeSliceSize.value) {
    return undefined;
  }

  const truncatedAmount: number = (totalData.value - freeSliceSize.value) % payingSliceSize.value;
  const multiplier = Math.floor((totalData.value - freeSliceSize.value) / payingSliceSize.value);

  if (multiplier > 0) {
    // Second bar full, third bar will exist
    return {
      amount: payingSliceSize.value,
      percentage: 100,
      progress: 1,
      multiplier: multiplier,
    };
  }

  // Second bar not full, third bar will not be defined
  return {
    amount: truncatedAmount,
    percentage: (truncatedAmount * 100) / payingSliceSize.value,
    progress: truncatedAmount / payingSliceSize.value,
    multiplier: 0,
  };
}

function getThirdBarData(): ProgressBarData | undefined {
  if (totalData.value < freeSliceSize.value + payingSliceSize.value) {
    // First two bars are enough
    return undefined;
  }

  const truncatedAmount: number = (totalData.value - freeSliceSize.value) % payingSliceSize.value;

  return {
    amount: truncatedAmount,
    percentage: (truncatedAmount * 100) / payingSliceSize.value,
    progress: truncatedAmount / payingSliceSize.value,
    multiplier: 0,
  };
}

onMounted(async () => {
  if (isDefaultOrganization(props.organization)) {
    const listResponse = await BmsAccessInstance.get().listOrganizations();
    if (!listResponse.isError && listResponse.data && listResponse.data.type === DataType.ListOrganizations) {
      allOrganizations.value = listResponse.data.organizations;
    }
    querying.value = false;
    return;
  }
  const response = await BmsAccessInstance.get().getOrganizationStats(props.organization.bmsId);

  if (!response.isError && response.data && response.data.type === DataType.OrganizationStats) {
    stats.value = response.data;

    totalData.value = stats.value.dataSize + stats.value.metadataSize;
    freeSliceSize.value = stats.value.freeSliceSize;
    payingSliceSize.value = stats.value.payingSliceSize;

    firstBarData.value = getFirstBarData();
    secondBarData.value = getSecondBarData();
    thirdBarData.value = getThirdBarData();
  } else {
    error.value = 'clientArea.statistics.error';
  }
  querying.value = false;
});

async function onOrganizationSelected(org: BmsOrganization): Promise<void> {
  emits('organizationSelected', org);
}
</script>

<style scoped lang="scss">
.users {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-bottom: 2.5rem;

  &-text {
    display: flex;
    color: var(--parsec-color-light-primary-700);
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
      min-width: 11.625rem;

      &-icons {
        display: flex;
        position: relative;
        width: fit-content;

        &__main {
          font-size: 2rem;
          color: var(--parsec-color-light-primary-700);
          border-radius: var(--parsec-radius-circle);
          padding: 0.5rem;
        }

        &__secondary {
          position: absolute;
          bottom: 0;
          right: 0;
          border-radius: var(--parsec-radius-12);
          // should be replaced with a variable in megashark-lib
          box-shadow: 0px 1px 2px 0px rgba(14, 14, 14, 0.05);
          font-size: 0.625rem;
          color: var(--parsec-color-light-secondary-contrast);
          background: var(--parsec-color-light-secondary-white);
          padding: 0.25rem;
        }
      }

      &-text {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;

        &__number {
          color: var(--parsec-color-light-secondary-contrast);
        }

        &__text {
          color: var(--parsec-color-light-secondary-hard-grey);
        }
      }
    }
  }

  &.active {
    .users-cards-list-item {
      background: var(--parsec-color-light-primary-50);
    }

    .users-cards-list-item-icons__main {
      background: var(--parsec-color-light-primary-100);
    }
  }

  &.revoked {
    .users-cards-list-item {
      background: var(--parsec-color-light-secondary-background);
    }

    .users-cards-list-item-icons__main {
      background: var(--parsec-color-light-secondary-disabled);
      color: var(--parsec-color-light-secondary-hard-grey);
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
    color: var(--parsec-color-light-primary-700);
  }

  &-data {
    display: flex;
    gap: 3rem;

    &-consumption-content {
      display: flex;
      gap: 2rem;
    }
    &-consumption {
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

.consumption {
  display: flex;
  flex-direction: column;
  width: 100%;

  &-item {
    width: 100%;
    max-width: 20rem;
  }

  &-number {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.25rem;

    &__amount {
      color: var(--parsec-color-light-secondary-contrast);
    }

    .number-multiplier {
      color: var(--parsec-color-light-primary-500);
    }

    &__percentage {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }

  &__bar {
    --progress-background: var(--parsec-color-light-secondary-grey);
    --buffer-background: var(--parsec-color-light-secondary-disabled);
    border-radius: var(--parsec-radius-12);
  }

  &__price {
    display: flex;
    color: var(--parsec-color-light-secondary-hard-grey);
    padding-top: 0.25rem;

    &.paid {
      color: var(--parsec-color-light-primary-500);
    }
  }

  &#thirdBar {
    margin-top: 1rem;
  }

  &#secondBar,
  &#thirdBar {
    .consumption__bar {
      --progress-background: var(--parsec-color-light-primary-500);
      --buffer-background: var(--parsec-color-light-primary-100);
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

  .storage-data-consumption::before {
    content: none;
  }

  .consumption-number {
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
