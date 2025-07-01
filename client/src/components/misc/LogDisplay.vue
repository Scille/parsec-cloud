<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container">
    <div
      class="logs"
      v-if="!loading"
    >
      <ms-report-text
        v-for="(record, index) of logs"
        :key="index"
        :theme="LevelThemeMapping[record.level]"
      >
        {{ record.timestamp }} {{ record.message }}
      </ms-report-text>
      <ms-informative-text v-show="logs.length === 0">
        {{ 'NO LOGS' }}
      </ms-informative-text>
    </div>
    <div
      class="loading"
      v-show="loading"
    >
      <ion-skeleton-text :animated="true" />
      <ion-skeleton-text :animated="true" />
      <ion-skeleton-text :animated="true" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { isWeb } from '@/parsec';
import { LogEntry, WebLogger } from '@/services/webLogger';
import { IonSkeletonText } from '@ionic/vue';
import { MsInformativeText, MsReportTheme, MsReportText } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const LevelThemeMapping = {
  ['debug']: MsReportTheme.Info,
  ['info']: MsReportTheme.Info,
  ['warn']: MsReportTheme.Warning,
  ['error']: MsReportTheme.Error,
  ['critical']: MsReportTheme.Error,
};

const logs = ref<Array<LogEntry>>([]);
const loading = ref(true);

onMounted(async () => {
  loading.value = true;
  if (isWeb()) {
    logs.value = (await WebLogger.getEntries()).reverse();
    loading.value = false;
  } else {
    await window.electronAPI.getLogs();
  }
});

window.electronAPI.receive('parsec-log-records', async (logRecords: Array<LogEntry>) => {
  logs.value = logRecords.reverse();
  loading.value = false;
});
</script>

<style scoped lang="scss">
.container {
  overflow-y: auto;
  height: 32em;
}
</style>
