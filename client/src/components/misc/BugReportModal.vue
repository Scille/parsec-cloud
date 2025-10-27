<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="ReportBugModal.title"
    subtitle="ReportBugModal.subtitle"
    :close-button="{ visible: true }"
    :cancel-button="{
      disabled: false,
      label: 'ReportBugModal.form.cancel',
    }"
    :confirm-button="{
      label: 'ReportBugModal.form.submit',
      disabled: !canSend || sendingReport,
      onClick: sendBugReport,
    }"
  >
    <div class="report-bug-modal-container">
      <ms-input
        label="ReportBugModal.form.email"
        ref="emailInput"
        v-model="email"
        :validator="emailValidator"
      />
      <ms-textarea
        v-model="description"
        label="ReportBugModal.form.description"
      />
      <div class="add-file-container">
        <ion-text
          id="label"
          class="form-label"
        >
          {{ $msTranslate('ReportBugModal.form.joinFile') }}
        </ion-text>
        <file-input-list ref="listInput" />
      </div>
    </div>

    <div class="report-logs">
      <ion-toggle
        v-model="includeLogs"
        @ion-change="logToggled"
        class="report-logs__toggle"
      />
      <div class="report-logs-text">
        <ion-text class="report-logs-text__title subtitles-normal">{{ $msTranslate('ReportBugModal.log.title') }}</ion-text>
        <div class="report-logs-text-subtitles">
          <ion-text class="report-logs-text-subtitles__description body">{{ $msTranslate('ReportBugModal.log.description') }}</ion-text>
          <ion-button
            fill="clear"
            @click.stop="openLogDisplayModal"
            id="see-logs-button"
          >
            {{ $msTranslate('ReportBugModal.log.seeLogs') }}
          </ion-button>
        </div>
      </div>
    </div>

    <ms-report-text
      class="report-error"
      v-show="sendError"
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate(sendError) }}
    </ms-report-text>
  </ms-modal>
</template>

<script setup lang="ts">
import { getMimeTypeFromBuffer } from '@/common/fileTypes';
import { emailValidator } from '@/common/validators';
import { FileInputList } from '@/components/files';
import { openLogDisplayModal } from '@/components/misc';
import { isWeb, ParsecAccount } from '@/parsec';
import { BmsApi, FileData } from '@/services/bms';
import { formatLogEntry, LogEntry, WebLogger } from '@/services/webLogger';
import { IonButton, IonText, IonToggle, modalController } from '@ionic/vue';
import { MsInput, MsModal, MsModalResult, MsReportText, MsReportTheme, MsTextarea, Validity } from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef } from 'vue';

const emailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('emailInput');
const email = ref('');
const description = ref('');
const includeLogs = ref(false);
const listInputRef = useTemplateRef<InstanceType<typeof FileInputList>>('listInput');
const sendingReport = ref(false);
const sendError = ref('');
const logs = ref<Array<LogEntry>>([]);

const canSend = computed(() => {
  return emailInputRef.value && emailInputRef.value.validity === Validity.Valid && description.value.length > 0 && email.value.length > 0;
});

onMounted(async () => {
  if (ParsecAccount.isLoggedIn()) {
    const info = await ParsecAccount.getInfo();
    if (info.ok) {
      email.value = info.value.humanHandle.email;
    }
  }
});

async function logToggled(): Promise<void> {
  if (includeLogs.value) {
    if (isWeb()) {
      logs.value = await WebLogger.getEntries();
    } else {
      window.electronAPI.getLogs();
    }
  }
}

window.electronAPI.receive('parsec-log-records', async (logRecords: Array<LogEntry>) => {
  logs.value = logRecords;
});

async function readFile(file: File): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (): void => {
      const result = reader.result as ArrayBuffer;
      resolve(new Uint8Array(result));
    };
    reader.onerror = reject;
    reader.readAsArrayBuffer(file);
  });
}

async function sendBugReport(): Promise<boolean> {
  if (!canSend.value) {
    return false;
  }
  sendError.value = '';
  sendingReport.value = true;
  const files: Array<FileData> = [];
  for (const f of listInputRef.value?.getFiles() as Array<File>) {
    const fileData = await readFile(f);
    const mimeType = (await getMimeTypeFromBuffer(fileData.slice(0, 4096))) ?? 'application/octet-stream';
    files.push({
      name: f.name,
      data: fileData,
      mimeType: mimeType,
    });
  }
  const response = await BmsApi.reportBug(
    {
      email: email.value,
      description: description.value,
    },
    {
      logs: includeLogs.value ? logs.value.map((entry) => formatLogEntry(entry)) : undefined,
      files: files,
    },
  );
  sendingReport.value = false;
  if (response.isError) {
    sendError.value = 'bugReport.failed';
    return false;
  } else {
    return await modalController.dismiss({}, MsModalResult.Confirm);
  }
}
</script>

<style scoped lang="scss">
.report-bug-modal-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  position: relative;
  z-index: 3;

  .add-file-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
}

.report-logs {
  display: flex;
  gap: 0.75rem;
  margin-top: 1rem;
  padding: 1rem;
  border-radius: var(--parsec-radius-8);
  background: var(--parsec-color-light-secondary-background);
  transition: background 0.15s ease-in-out;
  position: relative;
  z-index: 3;

  &__toggle {
    --handle-width: 1rem;
    --handle-spacing: 0.125rem;

    &::part(track) {
      width: 2.25rem;
      height: 1.25rem;
    }

    &::part(handle) {
      width: 1rem;
      height: 1rem;
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    cursor: pointer;

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }

    &-subtitles {
      display: flex;
      gap: 0.5rem;

      &__description {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }
  }

  &:hover:not(#see-logs-button) {
    background: var(--parsec-color-light-secondary-premiere);
  }
}

#see-logs-button {
  --color: var(--parsec-color-light-secondary-text);
  --background: transparent;
  --background-hover: transparent;

  &::part(native) {
    text-decoration: underline;
  }

  &:hover {
    color: var(--parsec-color-light-secondary-contrast);
  }
}
</style>
