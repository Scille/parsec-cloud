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
        :disabled="aggregatingLogs"
        class="report-logs__toggle"
      />
      <div class="report-logs-text">
        <ion-text class="report-logs-text__title">{{ $msTranslate('ReportBugModal.log.title') }}</ion-text>
        <div class="report-logs-text-subtitles">
          <ion-text class="report-logs-text-subtitles__description">{{ $msTranslate('ReportBugModal.log.description') }}</ion-text>
          <ion-button
            fill="clear"
            @click.stop="openLogDisplayModal()"
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
import { getLogs, openLogDisplayModal } from '@/components/misc';
import { ParsecAccount } from '@/parsec';
import { BmsApi, FileData } from '@/services/bms';
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
const logs = ref<string>('');
const aggregatingLogs = ref(false);

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
    aggregatingLogs.value = true;
    logs.value = await getLogs();
    aggregatingLogs.value = false;
  }
}

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
      logs: includeLogs.value ? logs.value : undefined,
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
  gap: ms.spacing('gap-3xl');
  position: relative;
  z-index: 3;

  .add-file-container {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
  }
}

.report-logs {
  display: flex;
  gap: ms.spacing('gap-2xl');
  margin-top: 1rem;
  padding: ms.spacing('padding-3xl');
  border-radius: ms.radius('lg');
  background: ms.color('surface-base-default-secondary');
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
    gap: ms.spacing('gap-lg');
    cursor: pointer;

    &__title {
      @include ms.font('body-lg-medium');
      color: ms.color('text-base-body');
    }

    &-subtitles {
      display: flex;
      gap: ms.spacing('gap-lg');

      &__description {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }
    }
  }

  &:hover:not(#see-logs-button) {
    background: ms.color('surface-base-page-secondary');
  }
}

#see-logs-button {
  --color: #{ms.color('text-base-body')};
  --background: transparent;
  --background-hover: transparent;

  &::part(native) {
    text-decoration: underline;
  }

  &:hover {
    color: ms.color('text-base-heading');
  }
}
</style>
