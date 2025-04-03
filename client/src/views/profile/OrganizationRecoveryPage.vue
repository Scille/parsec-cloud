<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-recovery-container">
    <ion-label
      class="button-small done"
      v-show="hasRecoveryDevice"
    >
      {{ $msTranslate('OrganizationRecovery.done.label') }}
    </ion-label>
    <ion-label
      class="body-sm danger"
      v-show="!hasRecoveryDevice"
    >
      {{ $msTranslate('OrganizationRecovery.notDone.label') }}
    </ion-label>
    <template v-if="!recoveryDeviceTemplate">
      <div class="restore-password-button">
        <ion-button
          class="button-default"
          @click="goToExportRecoveryDevice()"
        >
          {{ $msTranslate('ExportRecoveryDevicePage.actions.createRecoveryButton') }}
        </ion-button>
      </div>
    </template>
    <template v-else>
      <div class="recovery-list">
        <!-- item -->
        <div class="recovery-item">
          <div class="recovery-item-text">
            <div class="recovery-item-text__title subtitles-normal">
              <ion-icon :icon="documentIcon" />
              <span>{{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryFile') }}</span>
            </div>
            <ion-text class="recovery-item-text__description body">
              {{ $msTranslate('ExportRecoveryDevicePage.subtitles.fileExplanation') }}
            </ion-text>
          </div>
          <div class="recovery-item-download">
            <div class="recovery-item-download__button">
              <ion-button
                @click="downloadRecoveryFile()"
                :disabled="disableFileDownload"
                id="downloadFileButton"
                size="default"
                :fill="recoveryFileDownloaded ? 'outline' : 'solid'"
              >
                <ion-icon :icon="recoveryFileDownloaded ? reload : download" />
                {{
                  recoveryFileDownloaded
                    ? $msTranslate('ExportRecoveryDevicePage.actions.downloadAgain')
                    : $msTranslate('ExportRecoveryDevicePage.actions.download')
                }}
              </ion-button>
            </div>
            <ion-text
              class="recovery-item-download__downloaded body"
              v-show="recoveryFileDownloaded"
            >
              <ion-icon
                :icon="checkmarkCircle"
                class="checked"
              />
              {{ $msTranslate('ExportRecoveryDevicePage.subtitles.fileDownloaded') }}
            </ion-text>
          </div>
        </div>

        <div class="recovery-item">
          <div class="recovery-item-text">
            <div class="recovery-item-text__title subtitles-normal">
              <ion-icon :icon="documentLock" />
              <span>{{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryKey') }}</span>
            </div>
            <ion-text class="recovery-item-text__description body">
              {{ $msTranslate('ExportRecoveryDevicePage.subtitles.keyExplanation') }}
            </ion-text>
          </div>

          <div class="recovery-item-download">
            <a
              ref="downloadLink"
              v-show="false"
            />
            <div class="recovery-item-download__button">
              <ion-button
                @click="downloadRecoveryKey()"
                id="downloadPassphraseButton"
                :disabled="disableKeyDownload"
                size="default"
                :fill="recoveryKeyDownloaded ? 'outline' : 'solid'"
              >
                <ion-icon :icon="recoveryKeyDownloaded ? reload : download" />
                {{
                  recoveryKeyDownloaded
                    ? $msTranslate('ExportRecoveryDevicePage.actions.downloadAgain')
                    : $msTranslate('ExportRecoveryDevicePage.actions.download')
                }}
              </ion-button>
            </div>
            <ion-text
              class="recovery-item-download__downloaded body"
              v-show="recoveryKeyDownloaded"
            >
              <ion-icon
                :icon="checkmarkCircle"
                class="checked"
              />
              {{ $msTranslate('ExportRecoveryDevicePage.subtitles.fileDownloaded') }}
            </ion-text>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { listOwnDevices, OwnDeviceInfo, getClientInfo, exportRecoveryDevice } from '@/parsec';
import { IonButton, IonIcon, IonText, IonLabel } from '@ionic/vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { I18n, Translatable, askQuestion, Answer } from 'megashark-lib';
import { checkmarkCircle, document as documentIcon, download, documentLock, reload } from 'ionicons/icons';
import { inject, onMounted, ref } from 'vue';

let code = '';
let content: Uint8Array = new Uint8Array();
const downloadLink = ref();
const recoveryKeyDownloaded = ref(false);
const recoveryFileDownloaded = ref(false);
const disableFileDownload = ref(false);
const disableKeyDownload = ref(false);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const orgId = ref('');
const recoveryDeviceTemplate = ref(false);
const hasRecoveryDevice = ref(false);

onMounted(async (): Promise<void> => {
  const result = await getClientInfo();
  if (!result.ok) {
    return;
  }
  orgId.value = result.value.organizationId;

  const ownDevicesResult = await listOwnDevices();
  if (ownDevicesResult.ok) {
    const ownDevices = ownDevicesResult.value;
    hasRecoveryDevice.value = ownDevices.find((d: OwnDeviceInfo) => d.isRecovery) !== undefined;
  }
});

async function goToExportRecoveryDevice(): Promise<void> {
  if (hasRecoveryDevice.value) {
    const answer = await askQuestion(
      'OrganizationRecovery.done.recreateQuestionTitle',
      'OrganizationRecovery.done.recreateQuestionMessage',
      { yesText: 'OrganizationRecovery.done.recreateYes', noText: 'OrganizationRecovery.done.recreateNo' },
    );
    if (answer === Answer.No) {
      return;
    }
  }

  const exportResult = await exportRecoveryDevice();
  if (!exportResult.ok) {
    informationManager.present(
      new Information({
        message: 'ExportRecoveryDevicePage.errors.exportFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  code = exportResult.value[0];
  content = exportResult.value[1];
  recoveryDeviceTemplate.value = true;
}

async function downloadRecoveryKey(): Promise<void> {
  disableKeyDownload.value = true;
  await downloadFile(code, 'text/plain', { key: 'ExportRecoveryDevicePage.filenames.recoveryKey', data: { org: orgId.value } });
  setTimeout(() => {
    disableKeyDownload.value = false;
    recoveryKeyDownloaded.value = true;
    informationManager.present(
      new Information({
        message: 'ExportRecoveryDevicePage.toasts.keyDownloadOk',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }, 500);
}

async function downloadRecoveryFile(): Promise<void> {
  disableFileDownload.value = true;
  await downloadFile(content, 'application/octet-stream', {
    key: 'ExportRecoveryDevicePage.filenames.recoveryFile',
    data: { org: orgId.value },
  });
  setTimeout(() => {
    disableFileDownload.value = false;
    recoveryFileDownloaded.value = true;
    informationManager.present(
      new Information({
        message: 'ExportRecoveryDevicePage.toasts.fileDownloadOk',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }, 500);
}

async function downloadFile(
  data: Uint8Array | string,
  contentType: 'application/octet-stream' | 'text/plain',
  fileName: Translatable,
): Promise<void> {
  const blob = new Blob([data], { type: contentType });
  const url = window.URL.createObjectURL(blob);

  downloadLink.value.setAttribute('href', url);
  downloadLink.value.setAttribute('download', I18n.translate(fileName));
  downloadLink.value.click();
}
</script>

<style scoped lang="scss">
ion-label {
  width: max-content;
  position: absolute;
  top: 1.5rem;
  right: 1.5rem;
  padding: 0.125rem 1rem;
  border-radius: var(--parsec-radius-12);

  &.done {
    background: var(--parsec-color-light-success-100);
    color: var(--parsec-color-light-success-700);
  }

  &.danger {
    background: var(--parsec-color-light-danger-100);
    color: var(--parsec-color-light-danger-700);
  }
}
.recovery-list {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5rem;

  .recovery-item {
    padding: 1.5rem;
    border-radius: var(--parsec-radius-8);
    background: var(--parsec-color-light-secondary-background);
    max-width: 26rem;
    gap: 0.5rem;
    display: flex;
    flex-direction: column;

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__title {
        gap: 0.5rem;
        display: flex;
        align-items: center;
        color: var(--parsec-color-light-primary-700);

        ion-icon {
          font-size: 1.5rem;
        }
      }

      &__description {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }

    &-download {
      display: flex;
      gap: 1rem;
      margin-top: 0.5rem;

      &__button {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-primary-700);

        &::part(native) {
          padding: 0.75rem 1.125rem;
        }

        ion-button {
          margin: 0;
        }

        #downloadPassphraseButton,
        #downloadFileButton {
          display: flex;
          align-items: center;

          ion-icon {
            font-size: 1rem;
            margin-right: 0.625rem;
          }
        }
      }

      &__downloaded {
        display: flex;
        align-items: center;
        padding: 0.375rem 0;
        gap: 0.5rem;
        color: var(--parsec-color-light-secondary-grey);

        .checked {
          font-size: 1.125rem;
          color: var(--parsec-color-light-success-700);
        }
      }
    }
  }
}
</style>
