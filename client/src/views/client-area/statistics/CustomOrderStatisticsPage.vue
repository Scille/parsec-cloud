<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="client-page-statistics">
    <template v-if="organizationStats && contractDetails">
      <!-- active users -->
      <div class="users-container">
        <!-- active users -->
        <div class="users active">
          <div class="users-text">
            <ion-title class="users-active-text__title">
              {{ $msTranslate('clientArea.statistics.titles.users') }}
              {{ $msTranslate('clientArea.statistics.titles.active') }}
            </ion-title>
          </div>
          <div class="users-cards-list">
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title">
                  {{ $msTranslate({ key: 'clientArea.statistics.admin', count: organizationStats.adminUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number">{{ organizationStats.adminUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title">
                  {{ $msTranslate({ key: 'clientArea.statistics.standard', count: organizationStats.standardUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number">{{ organizationStats.standardUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title">
                  {{ $msTranslate({ key: 'clientArea.statistics.outsider', count: organizationStats.outsiderUsersDetail.active }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number">{{ organizationStats.outsiderUsersDetail.active }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title">{{ $msTranslate('clientArea.statistics.total') }}</ion-text>
                <!-- eslint-disable vue/html-indent -->
                <ion-text class="users-cards-list-item-text__number">
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
            <ion-title class="users-active-text__title">
              {{ $msTranslate('clientArea.statistics.titles.users') }}
              {{ $msTranslate('clientArea.statistics.titles.revoked') }}
            </ion-title>
          </div>
          <div class="users-cards-list">
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title">
                  {{ $msTranslate({ key: 'clientArea.statistics.admin', count: organizationStats.adminUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number">{{ organizationStats.adminUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title">
                  {{ $msTranslate({ key: 'clientArea.statistics.standard', count: organizationStats.standardUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number">{{ organizationStats.standardUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
            <ion-card class="users-cards-list-item">
              <div class="users-cards-list-item-text">
                <ion-text class="users-cards-list-item-text__title">
                  {{ $msTranslate({ key: 'clientArea.statistics.outsider', count: organizationStats.outsiderUsersDetail.revoked }) }}
                </ion-text>
                <ion-text class="users-cards-list-item-text__number">{{ organizationStats.outsiderUsersDetail.revoked }}</ion-text>
              </div>
            </ion-card>
          </div>
        </div>
      </div>

      <!-- storage -->
      <div class="storage">
        <ion-text class="storage__title">{{ $msTranslate('clientArea.statistics.titles.storage') }}</ion-text>

        <div class="storage-data">
          <div class="storage-data-global">
            <ion-text
              class="storage-data-global__total"
              :class="{ overused: storagePercentage >= 100 }"
            >
              {{ $msTranslate(formatFileSize(totalData)) }}
            </ion-text>
            <div class="storage-data-global-info">
              <ion-text class="storage-data-global-info__text">
                <span class="storage-data-global-info__value">{{ $msTranslate(formatFileSize(organizationStats.dataSize)) }}</span>
                <span class="storage-data-global-info__label">{{ $msTranslate('clientArea.statistics.data') }}</span>
              </ion-text>
              <ion-text class="storage-data-global-info__text">
                <span class="storage-data-global-info__value">{{ $msTranslate(formatFileSize(organizationStats.metadataSize)) }}</span>
                <span class="storage-data-global-info__label">{{ $msTranslate('clientArea.statistics.metadata') }}</span>
              </ion-text>
            </div>
          </div>
          <div class="storage-data-usage">
            <ion-text class="storage-data-usage__title">
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
              <ion-text class="usage-number">
                {{ $msTranslate(formatFileSize(remainingStorageSize)) }}
                <span class="usage-text">{{ $msTranslate('clientArea.statistics.remaining') }}</span>
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
                  class="usage-text"
                  :class="{ overused: storagePercentage >= 100 }"
                >
                  {{ $msTranslate('clientArea.statistics.used') }}
                </span>
                <span class="usage-text">{{ $msTranslate('clientArea.statistics.remaining') }}</span>
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
      <ion-text class="organization-choice-title">
        {{ $msTranslate('clientArea.statistics.multipleOrganizations') }}
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
    <template v-else-if="!querying && !organizationStats && !contractDetails && !error">
      {{ $msTranslate('clientArea.statistics.noStats') }}
    </template>
    <ion-text
      class="statistics-error"
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
  gap: ms.spacing('gap-5xl');
  flex-direction: column;
  width: fit-content;
}

.users-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  padding: ms.spacing('padding-4xl');
  background: ms.color('surface-base-page');
  border-radius: ms.radius('2xl');
  width: fit-content;
}

.users {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;

  &-text {
    display: flex;
    color: ms.color('text-base-body');
  }

  &-cards-list {
    display: flex;
    gap: ms.spacing('gap-4xl');

    &-item {
      display: flex;
      border-radius: ms.radius('lg');
      padding: 0.875rem;
      box-shadow: none;
      gap: ms.spacing('gap-3xl');
      margin: 0;
      min-width: 9.125rem;
      box-shadow: ms.shadow('light');

      &-text {
        display: flex;
        flex-direction: column;
        gap: ms.spacing('gap-sm');

        &__number {
          @include ms.font('heading-h2');
          color: ms.color('text-base-heading');
        }

        &__title {
          @include ms.font('body-sm-regular');
          color: ms.color('text-base-description');
          font-size: 11px;
          text-transform: uppercase;
        }
      }

      &:nth-child(4) {
        background: ms.color('surface-neutral-default');

        .users-cards-list-item-text__title,
        .users-cards-list-item-text__number {
          color: ms.color('text-neutral-on-color');
        }
      }
    }
  }
}

.storage {
  display: flex;
  flex-direction: column;
  background: ms.color('surface-base-default');
  padding: ms.spacing('padding-5xl');
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('2xl');
  gap: ms.spacing('gap-4xl');
  max-width: 50rem;

  &__title {
    @include ms.font('heading-h5');
    color: ms.color('text-base-body');
  }

  &-data {
    display: flex;
    gap: 3rem;

    &-usage {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-3xl');
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
        background: ms.color('surface-disabled-default');
      }

      &__title {
        @include ms.font('heading-h6');
        color: ms.color('text-base-description');
        background: ms.color('surface-base-default');
        padding-right: ms.spacing('padding-3xl');
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
  gap: ms.spacing('gap-2xl');
  width: 100%;
  max-width: 9.5rem;

  &__total {
    @include ms.font('heading-h2');
    color: ms.color('text-brand-default');

    &.overused {
      color: ms.color('text-error-default');
    }
  }

  &-info {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');

    &__text {
      color: ms.color('text-base-description');
      display: flex;
      gap: ms.spacing('gap-md');
      align-items: center;
    }
  }
}

.usage {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: ms.spacing('gap-sm');

  &-number {
    @include ms.font('heading-h4');
    display: flex;
    align-items: baseline;
    align-self: flex-end;
    gap: ms.spacing('gap-sm');
    justify-content: space-between;
    margin-bottom: 0.25rem;
    color: ms.color('text-base-body');
  }

  &-progress {
    display: flex;
    border-radius: ms.radius('2xl');
    overflow: hidden;
    width: 100%;
    background: ms.color('surface-base-default');
    border: ms.border('thin') solid ms.color('border-base-default');

    &__bar {
      border-radius: ms.radius('2xl');
      height: 0.5rem;
    }

    #useStorageBar {
      width: v-bind(storageWidth);
      background: ms.color('surface-brand-default');

      &.overused {
        background: ms.color('surface-error-default');
      }
    }

    #remainingStorageBar {
      width: calc(100% - v-bind(storageWidth));
      background: ms.color('surface-disabled-default');
    }
  }

  &-caption {
    display: flex;
    margin-top: 0.75rem;
    width: 100%;
    gap: ms.spacing('gap-3xl');
    color: ms.color('text-base-description');

    span {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-sm');

      &::before {
        content: '';
        display: flex;
        width: 0.5rem;
        height: 0.5rem;
        border-radius: ms.radius('full');
      }

      &:nth-child(1)::before {
        background: ms.color('surface-brand-default');
      }

      &:nth-child(2)::before {
        background: ms.color('surface-disabled-default');
      }

      &.overused::before {
        background: ms.color('surface-error-default');
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
  @include ms.font('body-lg-regular');
  color: ms.color('text-neutral-default');
}

.organization-list {
  min-height: 4em;
  width: fit-content;
  display: flex;
  flex-direction: column;
  justify-content: left;
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-page');
  border: ms.border('thin') solid ms.color('border-base-default');
  overflow: auto;
  margin-bottom: 1.5rem;

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

.users-active-text__title {
  @include ms.font('heading-h5');
}

.statistics-error {
  @include ms.font('body-md-regular');
}

.storage-data-global-info__value {
  @include ms.font('heading-h6');
}

.storage-data-global-info__label {
  @include ms.font('body-md-regular');
}

.usage-text {
  @include ms.font('body-md-medium');
}
</style>
