<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      class="step-link-copy-error"
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
        <ion-text class="step-item__title">
          <span class="step-number">1</span>
          {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryKey') }}
        </ion-text>

        <div class="input-action">
          <div class="input-action-text-container">
            <ion-text class="input-action-text">{{ code }}</ion-text>
          </div>
          <div class="input-action-buttons">
            <ion-button
              @click="copyCode"
              :disabled="codeCopied !== undefined"
              class="input-action-button color-secondary"
              fill="clear"
            >
              <ion-icon
                class="button-icon button-icon-left"
                :icon="codeCopied ? checkmarkCircle : copy"
              />
              <span v-show="codeCopied === undefined">
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
              class="input-action-button color-secondary"
              fill="outline"
            >
              <ms-image
                class="button-icon button-icon-left"
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
            class="input-action-error"
          >
            {{ $msTranslate('Authentication.mfa.step2.copyError') }}
          </ion-text>
        </div>
      </div>

      <!-- Recovery file -->
      <div class="step-item recovery-file">
        <ion-text class="step-item__title">
          <span class="step-number">2</span>
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
          <ion-text class="input-action-text">{{ recoveryFileName }}</ion-text>
          <ion-button
            class="input-action-button color-secondary"
            @click="downloadRecoveryFile()"
            :disabled="fileDownloading || !content.length"
            fill="outline"
          >
            <ms-image
              class="button-icon button-icon-left"
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
        message: 'ExportRecoveryDevicePage.toasts.keyCopiedToClipboard',
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
  gap: ms.spacing('gap-3xl');

  .recovery-info {
    margin-bottom: 0.5rem;
  }
}

.step-link-copy-error,
.input-action-error {
  @include ms.font('body-sm-regular');
}

.step-list {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
}

.step-item {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
  padding: ms.spacing('padding-3xl');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-default-secondary');

  &__title {
    @include ms.font('heading-h5');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    color: ms.color('text-base-heading');
  }

  .step-number {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    @include ms.font('label-lg-medium');
    color: ms.color('text-base-body');
    border-radius: ms.radius('full');
    background: ms.color('surface-base-default');
    box-shadow: var(--parsec-shadow-input);
  }

  &:is(.recovery-key) {
    .input-action-text-container {
      background-color: ms.color('surface-neutral-default-subtle');
      padding: ms.spacing('padding-xl');
      color: ms.color('text-base-description');
      font-family: 'Courier New', Courier, monospace;
      border-radius: ms.radius('lg');
      border: ms.border('thin') solid ms.color('border-base-default');
    }

    .input-action-text {
      @include ms.font('label-md-medium');
      color: ms.color('text-base-label');
    }

    .input-action-buttons {
      display: flex;
      gap: ms.spacing('gap-2xl');
      justify-content: flex-end;
      margin-top: 1rem;
    }
  }

  &:is(.recovery-file) {
    .input-action {
      display: flex;
      gap: ms.spacing('gap-lg');
      align-items: center;

      &-text {
        @include ms.font('label-lg-medium');
        color: ms.color('text-base-label');
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        flex: 1;
      }

      &-icon {
        background: ms.color('surface-base-default');
        color: ms.color('text-base-label');
        display: flex;
        align-items: center;
        padding: ms.spacing('padding-lg');
        border-radius: ms.radius('lg');
        font-size: 1.25rem;
        flex-shrink: 0;
      }
    }
  }

  .button-icon {
    width: 1rem;
    --fill-color: #{ms.color('icon-neutral-default')};
  }
}
</style>
