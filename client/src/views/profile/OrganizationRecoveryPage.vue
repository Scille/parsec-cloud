<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-recovery-container">
    <template v-if="!recoveryDeviceTemplate">
      <div class="restore-password">
        <div
          class="restore-password__advice form-label"
          :class="hasRecoveryDevice ? 'restore-password__advice--done' : ''"
        >
          <ion-icon
            :icon="hasRecoveryDevice ? checkmarkCircle : informationCircle"
            class="advice-icon"
          />
          <ion-text class="advice-text">
            <span v-if="hasRecoveryDevice">
              {{ $msTranslate('OrganizationRecovery.adviceDone') }}
            </span>
            <span v-else>
              {{ $msTranslate('OrganizationRecovery.advice') }}
            </span>
          </ion-text>
        </div>

        <ion-text class="restore-password__description body">
          <span>{{ $msTranslate('OrganizationRecovery.done.subtitle') }}</span>
          <span>{{ $msTranslate('OrganizationRecovery.done.subtitle2') }}</span>
        </ion-text>

        <ion-button
          class="button-normal restore-password-button"
          @click="goToExportRecoveryDevice()"
          fill="clear"
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
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { I18n, Translatable, askQuestion, Answer } from 'megashark-lib';
import { checkmarkCircle, document as documentIcon, download, documentLock, reload, informationCircle } from 'ionicons/icons';
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
.organization-recovery-container {
  display: flex;
  flex-direction: column;
}

.restore-password {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &__description {
    margin-top: -0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &__advice {
    display: flex;
    width: 100%;
    align-items: center;
    background: var(--parsec-color-light-info-50);
    padding: 0.625rem 0.75rem;
    gap: 0.5rem;
    border-radius: var(--parsec-radius-8);
    color: var(--parsec-color-light-info-500);

    .advice-icon {
      font-size: 1rem;
      color: var(--parsec-color-light-info-500);
      flex-shrink: 0;
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &--done {
      background: var(--parsec-color-light-success-50);
      color: var(--parsec-color-light-success-700);

      .advice-icon {
        color: var(--parsec-color-light-success-700);
      }

      .advice-text {
        color: var(--parsec-color-light-success-700);
      }
    }
  }

  &-button {
    --background: var(--parsec-color-light-secondary-text);
    --background-hover: var(--parsec-color-light-secondary-contrast);
    color: var(--parsec-color-light-secondary-white);
    width: fit-content;

    @include ms.responsive-breakpoint('xs') {
      position: fixed;
      bottom: 2rem;
      left: 2rem;
      transform: translateX(50%, 50%);
      width: calc(100% - 4rem);
      margin: auto;
      z-index: 2;
      box-shadow: var(--parsec-shadow-strong);
      --overflow: visible;
      overflow: visible;
    }
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
