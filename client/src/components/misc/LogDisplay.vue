<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="log-display-modal-container">
    <div
      class="logs"
      v-if="!loading"
    >
      <ms-report-text
        v-for="(record, index) of logs"
        :key="index"
        :theme="LevelThemeMapping[record.level]"
        class="log-entry"
      >
        <div class="log-entry-text">
          <ion-text class="log-entry-text__message body">{{ record.message }}</ion-text>
          <ion-text class="log-entry-text__timestamp body-sm">{{ record.timestamp }}</ion-text>
        </div>
      </ms-report-text>
      <ms-report-text
        v-show="logs.length === 0"
        :theme="MsReportTheme.Info"
      >
        {{ $msTranslate('LogDisplayModal.noLog') }}
      </ms-report-text>
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
import { IonSkeletonText, IonText } from '@ionic/vue';
import { MsReportText, MsReportTheme } from 'megashark-lib';
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
    window.electronAPI.getLogs();
  }
});

window.electronAPI.receive('parsec-log-records', async (logRecords: Array<LogEntry>) => {
  logs.value = logRecords.reverse();
  loading.value = false;
});
</script>

<style scoped lang="scss">
.log-display-modal-container {
  overflow-y: auto;
}

.log-entry {
  border-radius: var(--parsec-radius-8);
  background: none;

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.25em;

    &__message {
      color: var(--parsec-color-light-primary-text);
    }

    &__timestamp {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &:nth-child(odd) {
    background-color: var(--parsec-color-light-secondary-background);
  }
}
</style>
