<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <h1>{{ $msTranslate('clientArea.statistics.titles.stats') }}</h1>
    <div v-if="stats">
      <div>
        <div class="subtitle">
          <h2>{{ $msTranslate('clientArea.statistics.titles.users') }}</h2>
          <ion-text>{{ $msTranslate('clientArea.statistics.titles.active') }}</ion-text>
        </div>
        <div class="card-row">
          <ion-card class="card card-active">
            {{ stats.adminUsersDetail.active }}
            {{ $msTranslate(stats.adminUsersDetail.active > 1 ? 'clientArea.statistics.admins' : 'clientArea.statistics.admin') }}
          </ion-card>
          <ion-card class="card card-active">
            {{ stats.standardUsersDetail.active }}
            {{ $msTranslate(stats.standardUsersDetail.active > 1 ? 'clientArea.statistics.standards' : 'clientArea.statistics.standard') }}
          </ion-card>
          <ion-card class="card card-active">
            {{ stats.outsiderUsersDetail.active }}
            {{ $msTranslate(stats.outsiderUsersDetail.active > 1 ? 'clientArea.statistics.outsiders' : 'clientArea.statistics.outsider') }}
          </ion-card>
        </div>
      </div>
      <div>
        <div class="subtitle">
          <h2>{{ $msTranslate('clientArea.statistics.titles.users') }}</h2>
          <ion-text>{{ $msTranslate('clientArea.statistics.titles.revoked') }}</ion-text>
        </div>
        <div class="card-row">
          <ion-card class="card card-revoked">
            {{ stats.adminUsersDetail.revoked }}
            {{ $msTranslate(stats.adminUsersDetail.revoked > 1 ? 'clientArea.statistics.admins' : 'clientArea.statistics.admin') }}
          </ion-card>
          <ion-card class="card card-revoked">
            {{ stats.standardUsersDetail.revoked }}
            {{ $msTranslate(stats.standardUsersDetail.revoked > 1 ? 'clientArea.statistics.standards' : 'clientArea.statistics.standard') }}
          </ion-card>
          <ion-card class="card card-revoked">
            {{ stats.outsiderUsersDetail.revoked }}
            {{ $msTranslate(stats.outsiderUsersDetail.revoked > 1 ? 'clientArea.statistics.outsiders' : 'clientArea.statistics.outsider') }}
          </ion-card>
        </div>
      </div>
      <div class="bottom-part">
        <h2>{{ $msTranslate('clientArea.statistics.titles.storage') }}</h2>
        <div class="bottom-part-data">
          <div>
            <h2>{{ $msTranslate(formatFileSize(totalData)) }}</h2>
            {{ $msTranslate('clientArea.statistics.ofWhich') }}
            {{ $msTranslate(formatFileSize(stats.dataSize)) }} {{ $msTranslate('clientArea.statistics.data') }}
            {{ $msTranslate(formatFileSize(stats.metadataSize)) }} {{ $msTranslate('clientArea.statistics.metadata') }}
          </div>
          <div>
            {{ $msTranslate('clientArea.statistics.consumptionDetail') }}
            <div v-if="firstBarData">
              <div id="firstBar">
                <ion-progress-bar :value="firstBarData.progress" />
                {{ $msTranslate(formatFileSize(firstBarData.amount)) }}
                {{ firstBarData.percentage.toFixed(0) }}%
              </div>
              {{ $msTranslate('clientArea.statistics.free') }}
            </div>
            <div v-if="secondBarData">
              <div id="secondBar">
                {{ $msTranslate(formatFileSize(secondBarData.amount)) }}
                {{ secondBarData.multiplier > 0 ? `x${secondBarData.multiplier}` : '' }}
                {{ secondBarData.percentage.toFixed(0) }}%
                <ion-progress-bar :value="secondBarData.progress" />
              </div>
              <div
                v-if="thirdBarData"
                id="thirdBar"
              >
                {{ $msTranslate(formatFileSize(thirdBarData.amount)) }}
                {{ thirdBarData.percentage.toFixed(0) }}%
                <ion-progress-bar :value="thirdBarData.progress" />
              </div>
              {{ $msTranslate('clientArea.statistics.paying') }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, DataType, BmsOrganization, OrganizationStatsResultData } from '@/services/bms';
import { onMounted, ref } from 'vue';
import { IonCard, IonProgressBar, IonText } from '@ionic/vue';
import { formatFileSize } from '@/common/file';

interface ProgressBarData {
  amount: number;
  percentage: number;
  progress: number;
  multiplier: number;
}

const props = defineProps<{
  organization: BmsOrganization;
}>();

const stats = ref<OrganizationStatsResultData>();
const totalData = ref<number>(0);
const freeSliceSize = ref<number>(0);
const payingSliceSize = ref<number>(0);

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
  const response = await BmsAccessInstance.get().getOrganizationStats(props.organization.bmsId);

  if (!response.isError && response.data && response.data.type === DataType.OrganizationStats) {
    stats.value = response.data;

    totalData.value = stats.value.dataSize + stats.value.metadataSize;
    freeSliceSize.value = stats.value.freeSliceSize;
    payingSliceSize.value = stats.value.payingSliceSize;

    firstBarData.value = getFirstBarData();
    secondBarData.value = getSecondBarData();
    thirdBarData.value = getThirdBarData();
  }
});
</script>

<style scoped lang="scss">
.subtitle {
  color: var(--parsec-color-light-primary-700);
}

.bottom-part {
  background-color: var(--parsec-color-light-secondary-premiere);
  display: flex;
  flex-direction: column;

  &-data {
    display: flex;
    flex-direction: row;
  }
}

.card {
  width: 186px;
  height: 76.1px;

  &-active {
    background-color: var(--parsec-color-light-primary-30);
  }

  &-revoked {
    background-color: var(--parsec-color-light-secondary-background);
  }

  &-row {
    display: flex;
    flex-direction: row;
  }
}
</style>
