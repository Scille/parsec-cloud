<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    SHAMIR OTHERS
    <div v-if="shamirInfo.length === 0">T'APPARTIENS A AUCUN SHAMIR TAS PAS D'AMIS</div>

    <div v-if="shamirInfo.length > 0">
      <ion-list>
        <ion-item
          v-for="info in shamirInfo"
          :key="info.userId"
        >
          {{ info.owner.humanHandle.label }} {{ info.owner.humanHandle.email }}
        </ion-item>
      </ion-list>
    </div>

    <div v-if="error">
      <ms-report-text :theme="MsReportTheme.Error">
        {{ $msTranslate(error) }}
      </ms-report-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { OtherShamirRecoveryInfo, getOthersShamirRecovery } from '@/parsec';
import { IonItem, IonList } from '@ionic/vue';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const shamirInfo = ref<Array<OtherShamirRecoveryInfo>>([]);
const error = ref('');

onMounted(async () => {
  await refreshShamirStatus();
});

async function refreshShamirStatus(): Promise<void> {
  const result = await getOthersShamirRecovery();

  if (!result.ok) {
    error.value = 'ERROR';
  } else {
    shamirInfo.value = result.value;
  }
}
</script>

<style scoped lang="scss"></style>
