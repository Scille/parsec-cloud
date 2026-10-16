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
  padding: 0.75rem;
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

.toggle-view-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  margin-right: auto;

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: flex-start;
    gap: 0;
  }

  .toggle-view {
    display: flex;
    width: fit-content;
    background: var(--parsec-color-light-secondary-white);
    padding: 3px;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-10);
    box-shadow: var(--parsec-shadow-input);

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }

    &-button {
      --color: var(--parsec-color-light-secondary-hard-grey);
      --background: none;
      --background-hover: var(--parsec-color-light-secondary-medium);
      margin: 0;

      @include ms.responsive-breakpoint('sm') {
        flex: 1;
      }

      &:hover {
        --color: var(--parsec-color-light-secondary-soft-text);
      }

      &::part(native) {
        display: flex;
        padding: 0.5rem 0.75rem;
      }

      &__icon {
        flex-shrink: 0;
        font-size: 1.125rem;
        margin-right: 0.5rem;
      }

      &__label {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      &.active {
        --background: var(--parsec-color-light-secondary-medium);
        --border-radius: var(--parsec-radius-6);
        --color: var(--parsec-color-light-primary-700);
        cursor: default;
        opacity: 1;
      }
    }
  }
}
</style>
