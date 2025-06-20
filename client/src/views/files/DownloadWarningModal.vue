<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="FoldersPage.DownloadFile.downloadWarningTitle"
    :close-button="{ visible: true }"
    :cancel-button="{
      disabled: false,
      label: 'CANCEL',
    }"
    :confirm-button="{
      label: 'DOWNLOAD',
      disabled: false,
      theme: MsReportTheme.Success,
      onClick: onConfirmClicked,
    }"
  >
    <div class="download-warning-content">
      {{ $msTranslate('FoldersPage.DownloadFile.downloadWarningText') }}
      <i18n-t
        keypath="FoldersPage.DownloadFile.downloadWarningDocMessage"
        tag="label"
        for="FoldersPage.DownloadFile.downloadWarningDoc"
      >
        <a
          class="doc-link"
          @click="openDocumentation"
        >
          {{ $msTranslate('FoldersPage.DownloadFile.downloadWarningDoc') }}
        </a>
      </i18n-t>

      <ms-checkbox
        v-model="noReminder"
        label-placement="end"
        justify="start"
        class="tos-checkbox body"
      >
        {{ $msTranslate('Do not remind me') }}
      </ms-checkbox>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { IonButton, IonIcon, modalController } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { MsReportTheme, MsModal, MsCheckbox, MsModalResult } from 'megashark-lib';
import { Env } from '@/services/environment';

const noReminder = ref(false);

onMounted(async () => {});

async function openDocumentation(): Promise<void> {
  await Env.Links.openDocumentationUserGuideLink('join_organization');
}

async function onConfirmClicked(): Promise<boolean> {
  return modalController.dismiss({ noReminder: noReminder.value }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.download-warning-content {
  margin-top: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.doc-link {
  cursor: pointer;
}
</style>
