<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="OrganizationRecovery.modal.title"
    subtitle="ExportRecoveryDevicePage.subtitles.aboutRecoveryFiles"
    :close-button="{ visible: true }"
    :confirm-button="{
      disabled: !canConfirm,
      label: 'ExportRecoveryDevicePage.actions.backToDevices',
      onClick: onConfirm,
    }"
  >
    <div class="export-recovery-modal-content">
      <ms-report-text
        v-if="error"
        :theme="MsReportTheme.Error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>

      <ion-text
        v-if="clipboardNotAvailable"
        class="step-link-copy-error body-sm"
      >
        {{ $msTranslate('DevicesPage.greet.linkNotCopiedToClipboard') }}
      </ion-text>

      <div
        v-if="!error"
        class="step-list"
      >
        <ms-report-text
          :theme="MsReportTheme.Info"
          class="recovery-info"
        >
          {{ $msTranslate('OrganizationRecovery.file.info') }}
        </ms-report-text>

        <!-- Recovery key -->
        <div class="step-item">
          <ion-text class="step-item__title title-h4">
            <span class="step-number button-large">1</span>
            {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryKey') }}
          </ion-text>

          <div class="input-copy">
            <ion-text class="input-copy-text form-input">{{ code }}</ion-text>
            <ion-button
              @click="copyCode"
              :disabled="codeCopied !== undefined"
              class="input-copy-button"
            >
              <ion-icon
                class="copy-icon"
                :icon="codeCopied ? checkmarkCircle : copy"
              />
              <span
                v-show="codeCopied === undefined"
                v-if="isLargeDisplay"
              >
                {{ $msTranslate('Authentication.mfa.step2.buttonCopy') }}
              </span>
              <span
                v-show="codeCopied === true"
                v-if="isLargeDisplay"
              >
                {{ $msTranslate('Authentication.mfa.step2.buttonCopied') }}
              </span>
            </ion-button>
            <ion-text
              v-if="codeCopied === false"
              class="input-copy-error body-sm"
            >
              {{ $msTranslate('Authentication.mfa.step2.copyError') }}
            </ion-text>
          </div>
        </div>

        <!-- Recovery file -->
        <div class="step-item">
          <ion-text class="step-item__title title-h4">
            <span class="step-number button-large">2</span>
            {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryFile') }}
          </ion-text>

          <div class="file-download">
            <a
              ref="downloadLink"
              v-show="false"
            />
            <ion-icon
              class="file-download__icon"
              :icon="documentText"
            />
            <ion-text class="file-download__name button-medium">{{ recoveryFileName }}</ion-text>
            <ion-button
              class="file-download__button button-default button-medium"
              @click="downloadRecoveryFile()"
              :disabled="fileDownloading || !content.length"
              fill="solid"
            >
              {{
                !fileDownloaded
                  ? $msTranslate('ExportRecoveryDevicePage.actions.download')
                  : $msTranslate('ExportRecoveryDevicePage.actions.downloadAgain')
              }}
            </ion-button>
          </div>
        </div>
      </div>

      <ms-checkbox
        v-model="confirmFilesDownloaded"
        class="confirmation-checkbox"
        label-placement="end"
        :disabled="!!error"
      >
        {{ $msTranslate('OrganizationRecovery.modal.confirmation') }}
      </ms-checkbox>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { exportRecoveryDevice, OrganizationID } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle, copy, documentText } from 'ionicons/icons';
import {
  Clipboard,
  I18n,
  MsCheckbox,
  MsModal,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  Translatable,
  useWindowSize,
} from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  organizationId: OrganizationID;
  informationManager: InformationManager;
}>();

const code = ref('');
const content = ref<Uint8Array>(new Uint8Array());

const { isLargeDisplay } = useWindowSize();
const downloadLinkRef = useTemplateRef('downloadLink');
const codeCopied = ref<boolean | undefined>(undefined);
const clipboardNotAvailable = ref(false);
const fileDownloading = ref(false);
const fileDownloaded = ref(false);
const confirmFilesDownloaded = ref(false);
const error = ref('');

const recoveryFileName = computed(() => {
  return I18n.translate({
    key: 'ExportRecoveryDevicePage.filenames.recoveryFile',
    data: { org: props.organizationId },
  } as Translatable);
});

const downloadsCompleted = computed(() => {
  return fileDownloaded.value;
});

const canConfirm = computed(() => {
  return downloadsCompleted.value && confirmFilesDownloaded.value;
});

onMounted(async () => {
  const exportResult = await exportRecoveryDevice();

  if (!exportResult.ok) {
    error.value = 'ExportRecoveryDevicePage.errors.exportFailed';
    return;
  }

  code.value = exportResult.value[0];
  content.value = exportResult.value[1];
});

async function copyCode(): Promise<void> {
  const copyResult = await Clipboard.writeText(code.value);
  codeCopied.value = copyResult;

  if (!codeCopied.value) {
    clipboardNotAvailable.value = true;

    codeCopied.value = undefined;
  } else {
    clipboardNotAvailable.value = false;
    setTimeout(() => {
      codeCopied.value = undefined;
    }, 2000);
    props.informationManager.present(
      new Information({
        message: 'DevicesPage.greet.linkCopiedToClipboard',
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  }
}

async function downloadRecoveryFile(): Promise<void> {
  if (!content.value.length) {
    return;
  }

  fileDownloading.value = true;
  await downloadFile(content.value, 'application/octet-stream', {
    key: 'ExportRecoveryDevicePage.filenames.recoveryFile',
    data: { org: props.organizationId },
  });
  fileDownloaded.value = true;
  fileDownloading.value = false;
}

async function onConfirm(): Promise<boolean> {
  return modalController.dismiss({ recoveryActionsDone: downloadsCompleted.value }, MsModalResult.Confirm);
}

async function downloadFile(
  data: Uint8Array,
  contentType: 'application/octet-stream' | 'text/plain',
  fileName: Translatable,
): Promise<void> {
  const blob = new Blob([data.buffer as ArrayBuffer], { type: contentType });
  const url = window.URL.createObjectURL(blob);

  downloadLinkRef.value?.setAttribute('href', url);
  downloadLinkRef.value?.setAttribute('download', I18n.translate(fileName));
  downloadLinkRef.value?.click();
}
</script>

<style scoped lang="scss">
.export-recovery-modal-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  .recovery-info {
    margin-bottom: 0.5rem;
  }
}

.step-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.step-item {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding: 1rem;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-medium);

  &__title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-text);
  }

  .step-number {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    color: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-circle);
    background: var(--parsec-color-light-secondary-white);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    box-shadow: var(--parsec-shadow-input);
  }

  .input-copy {
    padding: 0.75rem 0.5rem 0.75rem 0.75rem;
  }
}

.file-download {
  display: flex;
  gap: 0.5rem;
  width: 100%;
  align-items: center;

  &__icon {
    background: var(--parsec-color-light-secondary-white);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    color: var(--parsec-color-light-secondary-text);
    display: flex;
    align-items: center;
    padding: 0.5rem 0.5rem;
    border-radius: var(--parsec-radius-8);
    font-size: 1.25rem;
  }

  &__name {
    color: var(--parsec-color-light-secondary-text);
  }

  &__button {
    color: var(--parsec-color-light-secondary-white);
    --background: var(--parsec-color-light-secondary-text) !important;
    --background-hover: var(--parsec-color-light-secondary-contrast) !important;
    margin-left: auto;

    &:hover {
      background: var(--parsec-color-light-primary-50);
    }
  }
}
</style>
