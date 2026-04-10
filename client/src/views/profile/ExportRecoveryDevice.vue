<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
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
      <div class="step-item recovery-key">
        <ion-text class="step-item__title title-h4">
          <span class="step-number button-large">1</span>
          {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryKey') }}
        </ion-text>

        <div class="input-action">
          <ion-text class="input-action-text form-input">{{ code }}</ion-text>
          <div class="input-action-buttons">
            <ion-button
              @click="copyCode"
              :disabled="codeCopied !== undefined"
              class="input-action-button"
            >
              <ion-icon
                class="button-icon"
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
            <ion-button
              @click="downloadRecoveryKey()"
              class="input-action-button"
            >
              <ms-image
                class="button-icon"
                :image="DownloadIcon"
              />
              {{
                recoveryKeyDownloaded
                  ? $msTranslate('ExportRecoveryDevicePage.actions.downloadAgain')
                  : $msTranslate('ExportRecoveryDevicePage.actions.download')
              }}
            </ion-button>
          </div>
          <ion-text
            v-if="codeCopied === false"
            class="input-action-error body-sm"
          >
            {{ $msTranslate('Authentication.mfa.step2.copyError') }}
          </ion-text>
        </div>
      </div>

      <!-- Recovery file -->
      <div class="step-item recovery-file">
        <ion-text class="step-item__title title-h4">
          <span class="step-number button-large">2</span>
          {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryFile') }}
        </ion-text>

        <div class="input-action">
          <a
            ref="downloadLink"
            v-show="false"
          />
          <ion-icon
            class="input-action-icon"
            :icon="documentText"
          />
          <ion-text class="input-action-text button-medium">{{ recoveryFileName }}</ion-text>
          <ion-button
            class="input-action-button button-default button-medium"
            @click="downloadRecoveryFile()"
            :disabled="fileDownloading || !content.length"
            fill="solid"
          >
            <ms-image
              class="button-icon"
              :image="DownloadIcon"
            />
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
      @change="$emit('confirmed', $event)"
      class="confirmation-checkbox"
      label-placement="end"
      :disabled="!!error"
    >
      {{ $msTranslate('OrganizationRecovery.modal.confirmation') }}
    </ms-checkbox>
  </div>
</template>

<script setup lang="ts">
import { exportRecoveryDevice, OrganizationID } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle, copy, documentText } from 'ionicons/icons';
import {
  Clipboard,
  DownloadIcon,
  I18n,
  MsCheckbox,
  MsImage,
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

const emits = defineEmits<{
  (e: 'confirmed', value: boolean): void;
  (e: 'downloaded'): void;
}>();

const code = ref('');
const content = ref<Uint8Array>(new Uint8Array());

const { isLargeDisplay } = useWindowSize();
const downloadLinkRef = useTemplateRef('downloadLink');
const codeCopied = ref<boolean | undefined>(undefined);
const clipboardNotAvailable = ref(false);
const fileDownloading = ref(false);
const fileDownloaded = ref(false);
const recoveryKeyDownloaded = ref(false);
const error = ref('');

const recoveryFileName = computed(() => {
  return I18n.translate({
    key: 'ExportRecoveryDevicePage.filenames.recoveryFile',
    data: { org: props.organizationId },
  } as Translatable);
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
  emits('downloaded');
}

async function downloadRecoveryKey(): Promise<void> {
  recoveryKeyDownloaded.value = true;
  await downloadFile(new TextEncoder().encode(code.value), 'text/plain', {
    key: 'ExportRecoveryDevicePage.filenames.recoveryKey',
    data: { org: props.organizationId },
  });
  setTimeout(() => {
    props.informationManager.present(
      new Information({
        message: 'ExportRecoveryDevicePage.toasts.keyDownloadOk',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }, 500);
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

  &:is(.recovery-key) {
    .input-action-text {
      padding: 0.375rem;
    }
  }

  &:is(.recovery-file) {
    .input-action {
      &-icon {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-secondary-text);
        display: flex;
        align-items: center;
        padding: 0.5rem;
        border-radius: var(--parsec-radius-8);
        font-size: 1.25rem;
      }

      &-button {
        padding: 0.625rem 1rem;
      }
    }
  }
}
</style>
