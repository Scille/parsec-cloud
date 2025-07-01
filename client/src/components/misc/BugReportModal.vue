<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="REPORT BUG"
    subtitle="If you encounted a problem in the app, report it so we can investigate."
    :close-button="{ visible: true }"
    :cancel-button="{
      disabled: false,
      label: 'CANCEL',
    }"
    :confirm-button="{
      label: 'REPORT BUG',
      disabled: !canSend || sendingReport,
      onClick: sendBugReport,
    }"
  >
    <ms-input
      ref="emailInputRef"
      v-model="email"
      :validator="emailValidator"
    /><br />
    <ms-textarea v-model="description" /><br />
    <file-input-list ref="listInputRef" /><br />
    <ion-toggle
      v-model="includeLogs"
      class="include-logs"
    />
    INCLUDE LOGS

    <br />
    <ion-button
      fill="clear"
      @click="seeLogs"
    >
      {{ 'SEE THE LOGS' }}
    </ion-button>
    <ms-report-text
      v-show="sendError"
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate(sendError) }}
    </ms-report-text>
  </ms-modal>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { MsModal, MsInput, MsTextarea, Validity, MsModalResult, MsReportTheme, MsReportText } from 'megashark-lib';
import { ParsecAccount } from '@/parsec';
import { FileInputList } from '@/components/files';
import { emailValidator } from '@/common/validators';
import { BmsApi, FileData } from '@/services/bms';
import { openLogDisplayModal } from '@/components/misc';
import { modalController, IonToggle, IonButton } from '@ionic/vue';
import { wait } from '@/parsec/internals';

const emailInputRef = ref<typeof MsInput>();
const email = ref('');
const description = ref('');
const includeLogs = ref(false);
const listInputRef = ref<typeof FileInputList>();
const sendingReport = ref(false);
const sendError = ref('');

const canSend = computed(() => {
  return emailInputRef.value && emailInputRef.value.validity === Validity.Valid && description.value.length > 0 && email.value.length > 0;
});

onMounted(async () => {
  if (ParsecAccount.isLoggedIn()) {
    const info = await ParsecAccount.getInfo();
    if (info.ok) {
      email.value = info.value.email;
    }
  }
});

async function seeLogs(): Promise<void> {
  const top = await modalController.getTop();
  if (top) {
    top.classList.add('overlapped-modal');
  }
  await openLogDisplayModal();
  if (top) {
    top.classList.remove('overlapped-modal');
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
    files.push({
      name: f.name,
      data: await readFile(f),
      mimeType: 'application/octet-stream',
    });
  }
  // // Used for tests, remove this
  await wait(1000);
  const response = await BmsApi.reportBug(
    {
      email: email.value,
      description: description.value,
    },
    {
      includeLogs: includeLogs.value,
      files: files,
    },
  );
  console.log(response);
  sendingReport.value = false;
  if (response.isError) {
    sendError.value = 'Failed to send the report';
    return true;
  } else {
    console.log('DISMISS');
    return await modalController.dismiss({}, MsModalResult.Confirm);
  }
}
</script>

<style scoped lang="scss"></style>
