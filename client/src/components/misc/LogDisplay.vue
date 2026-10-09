<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="log-modal-container">
    <div
      class="log-container"
      v-if="!loading"
    >
      <textarea
        v-show="logs.length > 0"
        class="log-area"
        readonly
        :value="logs"
      />
      <ms-report-text
        v-show="logs.length === 0"
        :theme="MsReportTheme.Info"
      >
        {{ $msTranslate('LogDisplayModal.noLog') }}
      </ms-report-text>
      <div class="logs-buttons">
        <ms-feedback-button
          fill="clear"
          size="small"
          class="logs-buttons__item"
          id="log-copy-button"
          :callback="copyLogs"
          :normal-state="{ text: 'LogDisplayModal.copy', icon: copy }"
          :success-state="{ text: 'LogDisplayModal.copySuccess' }"
          :failure-state="{ text: 'LogDisplayModal.copyFailed' }"
        />
        <ms-feedback-button
          fill="clear"
          size="small"
          class="logs-buttons__item"
          id="log-download-button"
          :callback="downloadLogs"
          :normal-state="{ text: 'LogDisplayModal.download', icon: download }"
          :success-state="{ text: 'LogDisplayModal.downloadSuccess' }"
          :failure-state="{ text: 'LogDisplayModal.downloadFailed' }"
        />
      </div>

      <a
        ref="downloadLink"
        class="hidden-download-link"
      />
    </div>

    <div
      class="loading"
      v-show="loading"
    >
      <ion-skeleton-text
        :animated="true"
        class="skeleton"
      />
      <ion-skeleton-text
        :animated="true"
        class="skeleton"
      />
      <ion-skeleton-text
        :animated="true"
        class="skeleton"
      />
    </div>

    <ion-button
      id="log-close-button"
      @click="close"
    >
      {{ $msTranslate('LogDisplayModal.close') }}
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { getLogs } from '@/components/misc/utils';
import { IonButton, IonSkeletonText, modalController } from '@ionic/vue';
import { copy, download } from 'ionicons/icons';
import { DateTime } from 'luxon';
import { Clipboard, MsFeedbackButton, MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const downloadLinkRef = useTemplateRef<HTMLAnchorElement>('downloadLink');
const logs = ref<string>('');
const loading = ref(true);

onMounted(async () => {
  loading.value = true;
  logs.value = await getLogs();
  loading.value = false;
});

async function copyLogs(): Promise<boolean> {
  return await Clipboard.writeText(logs.value);
}

async function downloadLogs(): Promise<boolean> {
  try {
    const blob = new Blob([logs.value], { type: 'text/plain' });
    const url = window.URL.createObjectURL(blob);
    downloadLinkRef.value?.setAttribute('href', url);
    downloadLinkRef.value?.setAttribute('download', `parsec_${DateTime.now().toFormat('yyyy-MM-dd_HH-mm-ss')}.log`);
    downloadLinkRef.value?.click();
    window.URL.revokeObjectURL(url);
    return true;
  } catch (err: unknown) {
    window.nativeAPI.log('error', `Error when downloading logs: ${String(err)}`);
    return false;
  }
}

async function close(): Promise<void> {
  await modalController.dismiss();
}
</script>

<style scoped lang="scss">
.log-modal-container {
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  border-radius: ms.radius('lg');
}

.log-container {
  display: flex;
  width: 100%;
  height: 100%;
  flex-direction: column;
  background-color: ms.color('surface-base-page-secondary');
  position: relative;
}

.log-area {
  padding: ms.spacing('padding-3xl');
  scrollbar-width: thin;
  scrollbar-gutter: both-edges;
  display: flex;
  width: 100%;
  height: 100%;
  resize: none;
  background-color: ms.color('surface-base-page-secondary');
  color: ms.color('text-base-body');
  border: none;
  padding-top: ms.spacing('padding-8xl');
}

.logs-buttons {
  width: calc(100% - 1rem);
  position: absolute;
  top: 0;
  left: 0;
  padding: ms.spacing('padding-2xl') ms.spacing('padding-2xl') ms.spacing('padding-2xl') ms.spacing('padding-none');
  display: flex;
  justify-content: flex-end;
  background-color: ms.color('surface-base-page-secondary');
  gap: ms.spacing('gap-3xl');
  border-bottom: ms.border('thin') solid ms.color('border-base-default');
}

#log-close-button {
  width: fit-content;
  margin-top: 2rem;
  margin-left: auto;
}

.skeleton {
  height: 70px;
}
</style>
