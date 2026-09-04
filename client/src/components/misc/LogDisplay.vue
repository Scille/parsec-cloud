<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        <div class="toggle-view-container">
          <div
            class="toggle-view"
            role="tablist"
          >
            <ion-button
              class="toggle-view-button"
              role="tab"
              :aria-selected="logSource === 'GUI'"
              :class="{ active: logSource === 'GUI' }"
              @click="setSource('GUI')"
              :disabled="logSource === 'GUI'"
            >
              <ion-text class="toggle-view-button__label">
                {{ $msTranslate('SettingsModal.seeLogs.sourceGUI') }}
              </ion-text>
            </ion-button>
            <ion-button
              class="toggle-view-button"
              role="tab"
              :aria-selected="logSource === 'libparsec'"
              :class="{ active: logSource === 'libparsec' }"
              @click="setSource('libparsec')"
              :disabled="logSource === 'libparsec'"
            >
              <ion-text class="toggle-view-button__label">
                {{ $msTranslate('SettingsModal.seeLogs.sourceLibparsec') }}
              </ion-text>
            </ion-button>
          </div>
        </div>
        <ms-feedback-button
          fill="outline"
          size="small"
          class="logs-buttons__item color-secondary"
          id="log-copy-button"
          :callback="copyLogs"
          :normal-state="{ text: 'LogDisplayModal.copy', icon: copy }"
          :success-state="{ text: 'LogDisplayModal.copySuccess' }"
          :failure-state="{ text: 'LogDisplayModal.copyFailed' }"
        />
        <ms-feedback-button
          fill="outline"
          size="small"
          class="logs-buttons__item color-secondary"
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
import { IonButton, IonSkeletonText, IonText, modalController } from '@ionic/vue';
import { copy, download } from 'ionicons/icons';
import { DateTime } from 'luxon';
import { Clipboard, MsFeedbackButton, MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const downloadLinkRef = useTemplateRef<HTMLAnchorElement>('downloadLink');
const logs = ref<string>('');
const loading = ref(true);
const logSource = ref<'GUI' | 'libparsec'>('GUI');

onMounted(async () => {
  await setSource('GUI');
});

async function setSource(source: 'GUI' | 'libparsec'): Promise<void> {
  logSource.value = source;
  loading.value = true;
  logs.value = await getLogs(logSource.value);
  loading.value = false;
}

async function copyLogs(): Promise<boolean> {
  return await Clipboard.writeText(logs.value);
}

async function downloadLogs(): Promise<boolean> {
  try {
    const blob = new Blob([logs.value], { type: 'text/plain' });
    const url = window.URL.createObjectURL(blob);
    downloadLinkRef.value?.setAttribute('href', url);
    downloadLinkRef.value?.setAttribute('download', `parsec_${logSource.value}_${DateTime.now().toFormat('yyyy-MM-dd_HH-mm-ss')}.log`);
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
}

.log-container {
  display: flex;
  width: 100%;
  height: 100%;
  flex-direction: column;
  background-color: ms.color('surface-base-page-secondary');
  position: relative;
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  overflow: hidden;
  border-radius: ms.radius('lg');
}

.log-area {
  padding: ms.spacing('padding-3xl');
  scrollbar-width: thin;
  scrollbar-gutter: both-edges;
  display: flex;
  width: 100%;
  height: 100%;
  resize: none;
  background-color: ms.color('surface-base-default-secondary');
  color: ms.color('text-base-body');
  border: none;
  padding-top: ms.spacing('padding-8xl');
}

.logs-buttons {
  width: calc(100% - 1rem);
  position: absolute;
  top: 0;
  left: 0;
  padding: ms.spacing('padding-2xl') ms.spacing('padding-2xl') ms.spacing('padding-2xl');
  display: flex;
  justify-content: flex-end;
  align-items: center;
  background-color: ms.color('surface-base-page');
  gap: ms.spacing('gap-3xl');
  border-bottom: ms.border('thin') solid ms.color('border-base-default');

  &__item {
    height: fit-content;
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

.toggle-view-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: ms.spacing('gap-3xl');
  margin-right: auto;

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: flex-start;
    gap: ms.spacing('gap-none');
  }

  .toggle-view {
    display: flex;
    align-items: center;
    width: fit-content;
    padding: ms.spacing('padding-none');
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    gap: ms.spacing('gap-xs');
    background: ms.color('surface-base-page-secondary');
    border-radius: ms.radius('lg');
    flex-shrink: 0;
    overflow: hidden;

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }

    &-button {
      --color: #{ms.color('text-neutral-default')};
      --color-hover: #{ms.color('text-neutral-default-hover')};
      --background: none;
      --background-hover: #{ms.color('surface-base-default')};
      --background-hover-opacity: #{ms.opacity('10')};
      --background-activated: #{ms.color('surface-base-default')};
      --border-radius: #{ms.radius('md')};
      margin: 0;
      opacity: ms.opacity('disabled');

      @include ms.responsive-breakpoint('sm') {
        flex: 1;
      }

      &::part(native) {
        display: flex;
        padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
      }

      &__label {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      &.active {
        --background: #{ms.color('surface-base-default')};
        border-radius: ms.radius('md');
        box-shadow: ms.shadow('toggle');
        cursor: default;
        opacity: ms.opacity('10');
      }
    }
  }
}
</style>
