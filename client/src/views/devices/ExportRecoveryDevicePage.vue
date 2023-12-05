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
            {{ $t('ExportRecoveryDevicePage.subtitles.newPassword') }}
          </ms-informative-text>

          <div class="file">
            <ms-informative-text>
              {{ $t('ExportRecoveryDevicePage.subtitles.twoFilesToKeep') }}
            </ms-informative-text>

            <div class="file-list">
              <div class="file-item">
                <div class="file-item__title subtitles-normal">
                  <ion-icon :icon="document" />
                  {{ $t('ExportRecoveryDevicePage.titles.recoveryFile') }}
                </div>
              </div>
              <div class="file-item">
                <div class="file-item__title subtitles-normal">
                  <ion-icon :icon="key" />
                  {{ $t('ExportRecoveryDevicePage.titles.recoveryKey') }}
                </div>
              </div>
            </div>
          </div>

          <ion-button
            @click="exportDevice()"
            id="exportDevice"
          >
            {{ $t('ExportRecoveryDevicePage.actions.understand') }}
          </ion-button>
        </div>

        <!-- STEP 2 • Download files -->
        <div
          class="recovery-container download"
          v-else-if="state === ExportDevicePageState.Download"
        >
          <ms-informative-text>
            {{ $t('ExportRecoveryDevicePage.subtitles.keepFilesSeparate') }}
          </ms-informative-text>

          <div class="file-list">
            <div class="file-item">
              <div class="file-item__title subtitles-normal">
                <ion-icon :icon="document" />
                {{ $t('ExportRecoveryDevicePage.titles.recoveryFile') }}
              </div>
              <ion-text class="file-item__description body">
                {{ $t('ExportRecoveryDevicePage.subtitles.fileExplanation') }}
              </ion-text>
              <div
                class="file-item__button"
                v-show="!recoveryFileDownloaded"
              >
                <ion-button
                  @click="downloadRecoveryFile()"
                  id="downloadButton"
                  size="default"
                >
                  <ion-icon :icon="download" />
                  {{ $t('ExportRecoveryDevicePage.actions.download') }}
                </ion-button>
              </div>
              <div
                class="file-item__downloaded body"
                v-show="recoveryFileDownloaded"
              >
                <ion-icon
                  :icon="checkmarkCircle"
                  class="checked"
                />
                {{ $t('ExportRecoveryDevicePage.subtitles.fileDownloaded') }}
              </div>
            </div>
            <div class="file-item">
              <div class="file-item__title subtitles-normal">
                <ion-icon :icon="key" />
                {{ $t('ExportRecoveryDevicePage.titles.recoveryKey') }}
              </div>
              <ion-text class="file-item__description body">
                {{ $t('ExportRecoveryDevicePage.subtitles.keyExplanation') }}
              </ion-text>
              <div
                v-show="!recoveryKeyDownloaded"
                class="file-item__button"
              >
                <ion-button
                  @click="downloadRecoveryKey()"
                  id="downloadButton"
                  size="default"
                >
                  <ion-icon :icon="download" />
                  {{ $t('ExportRecoveryDevicePage.actions.download') }}
                </ion-button>
              </div>
              <div
                class="file-item__downloaded body"
                v-show="recoveryKeyDownloaded"
              >
                <ion-icon
                  :icon="checkmarkCircle"
                  class="checked"
                />
                {{ $t('ExportRecoveryDevicePage.subtitles.fileDownloaded') }}
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
              @click="onBackToDevicesClick()"
              id="back-to-devices-button"
            >
              <ion-icon
                :icon="home"
                class="icon"
              />
              {{ $t('ExportRecoveryDevicePage.actions.backToDevices') }}
            </ion-button>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent, IonButton, IonIcon, IonText } from '@ionic/vue';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import { ref, inject, onMounted } from 'vue';
import { getPasswordFromUser } from '@/common/inputs';
import { useI18n } from 'vue-i18n';
import { exportRecoveryDevice, RecoveryDeviceErrorTag } from '@/parsec';
import { getClientInfo } from '@/parsec/login';
import { NotificationManager, Notification, NotificationKey, NotificationLevel } from '@/services/notificationManager';
import { routerNavigateTo } from '@/router';
import { home, checkmarkCircle, document, key, download } from 'ionicons/icons';

const { t } = useI18n();

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
const notificationManager: NotificationManager = inject(NotificationKey)!;
const orgId = ref('');

onMounted(async (): Promise<void> => {
  const clientInfo = await getClientInfo();
  orgId.value = clientInfo.ok ? clientInfo.value.organizationId : '';
});

async function exportDevice(): Promise<void> {
  const password = await getPasswordFromUser({
    title: t('PasswordInputModal.passwordNeeded'),
    subtitle: t('PasswordInputModal.enterPassword', { org: orgId.value }),
    inputLabel: t('PasswordInputModal.password'),
    okButtonText: t('PasswordInputModal.validate'),
  });
  if (!password) {
    return;
  }
  const result = await exportRecoveryDevice(password);
  if (!result.ok) {
    const notificationMsg =
      result.error.tag === RecoveryDeviceErrorTag.Invalid ? t('PasswordInputModal.invalid') : t('PasswordInputModal.otherError');
    // toast atm but to be changed
    notificationManager.showToast(
      new Notification({
        message: notificationMsg,
        level: NotificationLevel.Error,
      }),
    );
    return;
  }
  code = result.value.code;
  file = result.value.file;
  state.value = ExportDevicePageState.Download;
}

async function downloadRecoveryKey(): Promise<void> {
  fileDownload(code, t('ExportRecoveryDevicePage.filenames.recoveryKey', { org: orgId.value }));
  recoveryKeyDownloaded.value = true;
  notificationManager.showToast(
    new Notification({
      message: t('ExportRecoveryDevicePage.toasts.keyDownloadOk'),
      level: NotificationLevel.Success,
    }),
  );
}

async function downloadRecoveryFile(): Promise<void> {
  fileDownload(file, t('ExportRecoveryDevicePage.filenames.recoveryFile', { org: orgId.value }));
  recoveryFileDownloaded.value = true;
  notificationManager.showToast(
    new Notification({
      message: t('ExportRecoveryDevicePage.toasts.fileDownloadOk'),
      level: NotificationLevel.Success,
    }),
  );
}

async function fileDownload(data: string, fileName: string): Promise<void> {
  downloadLink.value.setAttribute('href', `data:text/plain;charset=utf-8, ${encodeURIComponent(data)}`);
  downloadLink.value.setAttribute('download', fileName);
  downloadLink.value.click();
}

// Placeholder page reset causing visual flickering
function onBackToDevicesClick(): void {
  recoveryFileDownloaded.value = false;
  recoveryKeyDownloaded.value = false;
  state.value = ExportDevicePageState.Start;
  routerNavigateTo('devices');
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

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }

      &__button {
        margin-top: 2rem;
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
        margin-top: 2rem;
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
