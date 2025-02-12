<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="recovery">
        <div class="recovery-container download">
          <ms-informative-text>
            {{ $msTranslate('ExportRecoveryDevicePage.subtitles.aboutRecoveryFiles') }}
          </ms-informative-text>
          <ms-informative-text>
            {{ $msTranslate('ExportRecoveryDevicePage.subtitles.keepFilesSeparate') }}
          </ms-informative-text>
          <ms-informative-text>
            {{ $msTranslate('ExportRecoveryDevicePage.subtitles.warningUniqueness') }}
          </ms-informative-text>

          <div class="file-list">
            <div class="file-item">
              <div class="file-item__title subtitles-normal">
                <ion-icon :icon="documentIcon" />
                {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryFile') }}
              </div>
              <div class="file-item__subtitle">
                <ion-text class="file-item__description body">
                  {{ $msTranslate('ExportRecoveryDevicePage.subtitles.fileExplanation') }}
                </ion-text>
              </div>
              <div
                class="file-item__downloaded body"
                v-show="recoveryFileDownloaded"
              >
                <ion-icon
                  :icon="checkmarkCircle"
                  class="checked"
                />
                {{ $msTranslate('ExportRecoveryDevicePage.subtitles.fileDownloaded') }}
              </div>
              <div class="file-item__button">
                <ion-button
                  @click="downloadRecoveryFile()"
                  :disabled="disableFileDownload"
                  id="downloadButton"
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
            </div>
            <div class="file-item">
              <div class="file-item__title subtitles-normal">
                <ion-icon :icon="key" />
                {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryKey') }}
              </div>
              <div class="file-item__subtitle">
                <ion-text class="file-item__description body">
                  {{ $msTranslate('ExportRecoveryDevicePage.subtitles.keyExplanation') }}
                </ion-text>
              </div>
              <!-- Used to trigger the download -->
              <a
                ref="downloadLink"
                v-show="false"
              />
              <div
                class="file-item__downloaded body"
                v-show="recoveryKeyDownloaded"
              >
                <ion-icon
                  :icon="checkmarkCircle"
                  class="checked"
                />
                {{ $msTranslate('ExportRecoveryDevicePage.subtitles.fileDownloaded') }}
              </div>
              <div class="file-item__button">
                <ion-button
                  @click="downloadRecoveryKey()"
                  id="downloadButton"
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
            </div>
          </div>
          <div v-show="recoveryKeyDownloaded && recoveryFileDownloaded">
            <ion-button
              class="return-btn button-outline"
              fill="outline"
              @click="goToDevicesPage"
              id="back-to-devices-button"
            >
              <ion-icon
                :icon="home"
                class="icon"
              />
              {{ $msTranslate('ExportRecoveryDevicePage.actions.backToDevices') }}
            </ion-button>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { exportRecoveryDevice } from '@/parsec';
import { getClientInfo } from '@/parsec/login';
import { navigateTo, Routes } from '@/router';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { MsInformativeText, I18n, Translatable } from 'megashark-lib';
import { IonButton, IonContent, IonIcon, IonPage, IonText } from '@ionic/vue';
import { checkmarkCircle, document as documentIcon, download, home, key, reload } from 'ionicons/icons';
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

onMounted(async (): Promise<void> => {
  const result = await getClientInfo();

  if (!result.ok) {
    return;
  }
  orgId.value = result.value.organizationId;
  const exportResult = await exportRecoveryDevice();
  if (!exportResult.ok) {
    const notificationMsg = 'ExportRecoveryDevicePage.errors.exportFailed';
    // toast atm but to be changed
    informationManager.present(
      new Information({
        message: notificationMsg,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  code = exportResult.value[0];
  content = exportResult.value[1];
});

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

async function goToDevicesPage(): Promise<void> {
  await navigateTo(Routes.MyProfile, { replace: true });
}
</script>

<style scoped lang="scss">
.recovery {
  display: flex;
  max-width: 70rem;
  margin: 2.5em 2rem 0;
}

.recovery-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;

  .file-list {
    display: flex;
    gap: 1.5rem;

    .file-item {
      padding: 1.5rem;
      border-radius: var(--parsec-radius-8);
      background: var(--parsec-color-light-primary-30);
      max-width: 24rem;

      &__title {
        gap: 1rem;
        display: flex;
        align-items: center;
        color: var(--parsec-color-light-primary-700);

        ion-icon {
          font-size: 1.5rem;
        }
      }

      &__subtitle {
        margin-bottom: 1.5rem;
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }

      &__button {
        margin-top: 0.5rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-primary-700);

        ion-button {
          margin: 0;
        }

        #downloadButton {
          display: flex;
          align-items: center;

          ion-icon {
            font-size: 1.25rem;
            margin-right: 0.625rem;
          }
        }
      }

      &__downloaded {
        display: flex;
        align-items: center;
        margin-top: 0.5rem;
        padding: 0.375rem 0;
        gap: 0.5rem;
        color: var(--parsec-color-light-secondary-grey);

        .checked {
          font-size: 1.25rem;
          color: var(--parsec-color-light-success-500);
        }
      }
    }
  }
}

.download {
  .file-item__title {
    margin-bottom: 1rem;
  }
}

.return-btn {
  .icon {
    font-size: 1rem;
    margin-right: 0.625rem;
  }
}
</style>
