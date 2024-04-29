<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="recovery">
        <!-- STEP 1 • Information -->
        <div
          class="recovery-container information"
          v-if="state === ExportDevicePageState.Start"
        >
          <ms-informative-text>
            {{ $msTranslate('ExportRecoveryDevicePage.subtitles.newPassword') }}
          </ms-informative-text>

          <div class="file">
            <ms-informative-text>
              {{ $msTranslate('ExportRecoveryDevicePage.subtitles.twoFilesToKeep') }}
            </ms-informative-text>

            <div class="file-list">
              <div class="file-item">
                <div class="file-item__title subtitles-normal">
                  <ion-icon :icon="document" />
                  {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryFile') }}
                </div>
              </div>
              <div class="file-item">
                <div class="file-item__title subtitles-normal">
                  <ion-icon :icon="key" />
                  {{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryKey') }}
                </div>
              </div>
            </div>
          </div>

          <ion-button
            @click="exportDevice()"
            id="exportDevice"
          >
            {{ $msTranslate('ExportRecoveryDevicePage.actions.understand') }}
          </ion-button>
        </div>

        <!-- STEP 2 • Download files -->
        <div
          class="recovery-container download"
          v-else-if="state === ExportDevicePageState.Download"
        >
          <ms-informative-text>
            {{ $msTranslate('ExportRecoveryDevicePage.subtitles.keepFilesSeparate') }}
          </ms-informative-text>

          <div class="file-list">
            <div class="file-item">
              <div class="file-item__title subtitles-normal">
                <ion-icon :icon="document" />
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
          <a
            ref="downloadLink"
            v-show="false"
          />
          <div v-show="recoveryKeyDownloaded && recoveryFileDownloaded">
            <ion-button
              class="return-btn button-outline"
              fill="outline"
              @click="routerGoBack()"
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
import { RecoveryDeviceErrorTag, exportRecoveryDevice } from '@/parsec';
import { getClientInfo } from '@/parsec/login';
import { routerGoBack } from '@/router';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { MsInformativeText, Translatable, I18n, getPasswordFromUser } from 'megashark-lib';
import { IonButton, IonContent, IonIcon, IonPage, IonText } from '@ionic/vue';
import { checkmarkCircle, document, download, home, key, reload } from 'ionicons/icons';
import { inject, onMounted, ref } from 'vue';

enum ExportDevicePageState {
  Start = 'start',
  Download = 'download',
}

const state = ref(ExportDevicePageState.Start);
let code = '';
let file = '';
const downloadLink = ref();
const recoveryKeyDownloaded = ref(false);
const recoveryFileDownloaded = ref(false);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const orgId = ref('');

onMounted(async (): Promise<void> => {
  const clientInfo = await getClientInfo();
  orgId.value = clientInfo.ok ? clientInfo.value.organizationId : '';
});

async function exportDevice(): Promise<void> {
  const password = await getPasswordFromUser({
    title: 'PasswordInputModal.passwordNeeded',
    subtitle: { key: 'PasswordInputModal.enterPassword', data: { org: orgId.value } },
    inputLabel: 'PasswordInputModal.password',
    okButtonText: 'PasswordInputModal.validate',
  });
  if (!password) {
    return;
  }
  const result = await exportRecoveryDevice(password);
  if (!result.ok) {
    const notificationMsg =
      result.error.tag === RecoveryDeviceErrorTag.Invalid ? 'PasswordInputModal.invalid' : 'PasswordInputModal.otherError';
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
  code = result.value.code;
  file = result.value.file;
  state.value = ExportDevicePageState.Download;
}

async function downloadRecoveryKey(): Promise<void> {
  fileDownload(code, { key: 'ExportRecoveryDevicePage.filenames.recoveryKey', data: { org: orgId.value } });
  setTimeout(() => {
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
  fileDownload(file, { key: 'ExportRecoveryDevicePage.filenames.recoveryFile', data: { org: orgId.value } });
  setTimeout(() => {
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

async function fileDownload(data: string, fileName: Translatable): Promise<void> {
  downloadLink.value.setAttribute('href', `data:text/plain;charset=utf-8, ${encodeURIComponent(data)}`);
  downloadLink.value.setAttribute('download', I18n.translate(fileName));
  downloadLink.value.click();
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

  .file {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

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

.information {
  #exportDevice {
    margin-top: 0.5rem;
    width: fit-content;
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
