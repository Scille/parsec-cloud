<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="FoldersPage.DownloadFile.downloadWarningTitle"
    :close-button="{ visible: true }"
    :cancel-button="{
      disabled: false,
      label: 'FoldersPage.DownloadFile.cancel',
    }"
    :confirm-button="{
      label: 'FoldersPage.DownloadFile.downloadButton',
      disabled: false,
      theme: MsReportTheme.Success,
      onClick: onConfirmClicked,
    }"
  >
    <div class="download-warning">
      <ms-report-text :theme="MsReportTheme.Warning">
        {{ $msTranslate('FoldersPage.DownloadFile.downloadWarningText') }}
      </ms-report-text>
      <ms-image
        :image="DownloadWarningImage"
        class="download-warning-image"
      />
      <i18n-t
        keypath="FoldersPage.DownloadFile.downloadWarningDocMessage"
        tag="label"
        for="FoldersPage.DownloadFile.downloadWarningDoc"
        class="download-warning-documentation body"
      >
        <a
          class="doc-link"
          @click="openDocumentation"
        >
          {{ $msTranslate('FoldersPage.DownloadFile.downloadWarningDoc') }}
        </a>
      </i18n-t>
    </div>
    <ms-checkbox
      v-model="noReminder"
      label-placement="end"
      justify="start"
      class="download-warning-checkbox"
    >
      {{ $msTranslate('FoldersPage.DownloadFile.notRemindMe') }}
    </ms-checkbox>
  </ms-modal>
</template>

<script setup lang="ts">
import DownloadWarningImage from '@/assets/images/warning-download.svg';
import { Env } from '@/services/environment';
import { modalController } from '@ionic/vue';
import { MsCheckbox, MsImage, MsModal, MsModalResult, MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

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
.download-warning {
  margin-top: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  position: relative;

  &-image {
    width: 100%;
    max-width: 100%;
    height: auto;
    margin: 0 auto;
  }

  &-documentation {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-checkbox {
    position: absolute;
    z-index: 200;
    bottom: 1.75rem;

    @include ms.responsive-breakpoint('sm') {
    }

    @include ms.responsive-breakpoint('xs') {
      position: relative;
      bottom: 0;
      margin-top: 1rem;
    }
  }
}
.doc-link {
  cursor: pointer;
  text-decoration: underline;
  font-weight: 500;

  &:hover {
    color: var(--parsec-color-light-primary-600);
  }
}
</style>
