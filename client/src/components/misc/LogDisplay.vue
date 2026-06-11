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
          class="logs-buttons__item button-medium"
          id="log-copy-button"
          :callback="copyLogs"
          :normal-state="{ text: 'LogDisplayModal.copy', icon: copy }"
          :success-state="{ text: 'LogDisplayModal.copySuccess' }"
          :failure-state="{ text: 'LogDisplayModal.copyFailed' }"
        />
        <ms-feedback-button
          fill="clear"
          class="logs-buttons__item button-secondary"
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
  } catch {
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
  border-radius: var(--parsec-radius-8);
}

.log-container {
  display: flex;
  width: 100%;
  height: 100%;
  flex-direction: column;
  background-color: var(--parsec-color-light-secondary-premiere);
  position: relative;
}

.log-area {
  padding: 1rem;
  scrollbar-width: thin;
  scrollbar-gutter: both-edges;
  display: flex;
  width: 100%;
  height: 100%;
  resize: none;
  background-color: var(--parsec-color-light-secondary-premiere);
  color: var(--parsec-color-light-primary-text);
  border: none;
  padding-top: 4rem;
}

.logs-buttons {
  width: calc(100% - 1rem);
  position: absolute;
  top: 0;
  left: 0;
  padding: 0.75rem 0.75rem 0.75rem 0;
  display: flex;
  justify-content: flex-end;
  background-color: var(--parsec-color-light-secondary-premiere);
  gap: 1rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);

  &__item {
    color: var(--parsec-color-light-secondary-text);

    &::part(native) {
      box-shadow: var(--parsec-shadow-card);
      padding: 0.5rem 1rem;
      background-color: var(--parsec-color-light-secondary-white);
      --background-hover: var(--parsec-color-light-secondary-disabled);
      border-radius: var(--parsec-radius-8);
      transition: background-color 0.15s ease-in-out;
    }

    &-icon {
      font-size: 1rem;
      margin-right: 0.5rem;
    }
  }
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
