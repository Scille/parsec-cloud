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
          <span>{{ $msTranslate('OrganizationRecovery.done.subtitle3') }}</span>
        </ion-text>

        <ion-button
          class="button-normal button-default restore-password-button"
          @click="goToExportRecoveryDevice()"
          fill="clear"
        >
          {{ $msTranslate('ExportRecoveryDevicePage.actions.createRecoveryButton') }}
        </ion-button>
      </div>
    </template>
    <template v-else>
      <div class="recovery-list">
        <ion-text class="restore-password__description body">
          <span>{{ $msTranslate('OrganizationRecovery.done.subtitle') }}</span>
          <span>{{ $msTranslate('OrganizationRecovery.done.subtitle2') }}</span>
          <span>{{ $msTranslate('OrganizationRecovery.done.subtitle3') }}</span>
        </ion-text>
        <!-- item -->
        <div class="recovery-item">
          <ion-text class="recovery-item-text subtitles-normal">
            <span>{{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryFile') }}</span>
            <ion-icon
              v-show="recoveryFileDownloaded"
              :icon="checkmarkCircle"
              class="checked"
            />
          </ion-text>
          <div class="recovery-item-download">
            <ion-button
              @click="downloadRecoveryFile()"
              :disabled="disableFileDownload"
              id="downloadFileButton"
              :class="{ 'file-downloaded': recoveryFileDownloaded }"
              fill="solid"
            >
              <ms-image
                class="download-icon"
                v-if="!recoveryFileDownloaded"
                :image="DownloadIcon"
              />
              <ion-icon
                v-else
                :icon="reload"
              />
              {{
                recoveryFileDownloaded
                  ? $msTranslate('ExportRecoveryDevicePage.actions.downloadAgain')
                  : $msTranslate('ExportRecoveryDevicePage.actions.download')
              }}
            </ion-button>
          </div>
        </div>

        <div class="recovery-item">
          <ion-text class="recovery-item-text subtitles-normal">
            <span>{{ $msTranslate('ExportRecoveryDevicePage.titles.recoveryKey') }}</span>
            <ion-icon
              v-show="recoveryKeyDownloaded"
              :icon="checkmarkCircle"
              class="checked"
            />
          </ion-text>

          <div class="recovery-item-download">
            <a
              ref="downloadLink"
              v-show="false"
            />
            <ion-button
              @click="downloadRecoveryKey()"
              id="downloadPassphraseButton"
              :disabled="disableKeyDownload"
              :class="{ 'file-downloaded': recoveryKeyDownloaded }"
              fill="solid"
            >
              <ms-image
                class="download-icon"
                v-if="!recoveryKeyDownloaded"
                :image="DownloadIcon"
              />
              <ion-icon
                v-else
                :icon="reload"
              />
              {{
                recoveryKeyDownloaded
                  ? $msTranslate('ExportRecoveryDevicePage.actions.downloadAgain')
                  : $msTranslate('ExportRecoveryDevicePage.actions.download')
              }}
            </ion-button>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { OwnDeviceInfo, exportRecoveryDevice, getClientInfo, listOwnDevices } from '@/parsec';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle, informationCircle, reload } from 'ionicons/icons';
import { Answer, DownloadIcon, I18n, MsImage, Translatable, askQuestion } from 'megashark-lib';
import { computed, inject, onMounted, ref, useTemplateRef } from 'vue';

let code = '';
let content: Uint8Array = new Uint8Array();
const downloadLinkRef = useTemplateRef('downloadLink');
const recoveryKeyDownloaded = ref(false);
const recoveryFileDownloaded = ref(false);
const disableFileDownload = ref(false);
const disableKeyDownload = ref(false);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const orgId = ref('');
const recoveryDeviceTemplate = ref(false);
const hasRecoveryDevice = ref(false);

const bothDownloaded = computed(() => {
  return recoveryKeyDownloaded.value && recoveryFileDownloaded.value;
});

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

async function sendRecoveryDeviceCreatedEvent(): Promise<void> {
  if (bothDownloaded.value) {
    const devicesResult = await listOwnDevices();
    if (devicesResult.ok) {
      const lastDevice = devicesResult.value.toSorted((d1, d2) => (d1.createdOn > d2.createdOn ? -1 : 1))[0];
      if (lastDevice) {
        eventDistributor.dispatchEvent(Events.DeviceCreated, { info: lastDevice });
      }
    }
  }
}

async function downloadRecoveryKey(): Promise<void> {
  disableKeyDownload.value = true;
  await downloadFile(new TextEncoder().encode(code), 'text/plain', {
    key: 'ExportRecoveryDevicePage.filenames.recoveryKey',
    data: { org: orgId.value },
  });
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
    sendRecoveryDeviceCreatedEvent();
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
    sendRecoveryDeviceCreatedEvent();
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
      width: calc(100% - 4rem);
      margin: auto;
      z-index: 2;
      box-shadow: var(--parsec-shadow-strong);
    }
  }
}

.recovery-list {
  display: flex;
  flex-wrap: wrap;
  flex-direction: column;
  gap: 1.5rem;

  .recovery-item {
    gap: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    position: relative;
    flex-wrap: wrap;

    &:last-of-type::before {
      content: '';
      position: absolute;
      top: -0.5rem;
      height: 1px;
      width: 100%;
      background: var(--parsec-color-light-secondary-medium);
    }

    @include ms.responsive-breakpoint('sm') {
      max-width: 100%;
    }

    &-text {
      color: var(--parsec-color-light-secondary-text);
      display: flex;
      align-items: center;
      gap: 0.5rem;

      span {
        flex-shrink: 0;
      }

      .checked {
        font-size: 1rem;
        color: var(--parsec-color-light-success-700);
      }
    }

    &-download {
      display: flex;
      gap: 2rem;
      margin-top: 0.5rem;
      align-items: center;

      @include ms.responsive-breakpoint('sm') {
        gap: 0.5rem;
      }

      #downloadPassphraseButton,
      #downloadFileButton {
        display: flex;
        align-items: center;
        --background: var(--parsec-color-light-secondary-text);
        --background-hover: var(--parsec-color-light-secondary-contrast);

        ion-icon,
        .download-icon {
          font-size: 0.875rem;
          margin-right: 0.625rem;
          width: 0.875rem;
        }

        .download-icon {
          --fill-color: var(--parsec-color-light-secondary-white);
        }

        ion-icon {
          color: var(--parsec-color-light-secondary-text);
        }

        &.file-downloaded {
          --background: var(--parsec-color-light-secondary-premiere);
          --background-hover: var(--parsec-color-light-secondary-medium);
          color: var(--parsec-color-light-secondary-text);

          &::part(native) {
            box-shadow: var(--parsec-shadow-soft);
            border: 1px solid var(--parsec-color-light-secondary-disabled);
          }
        }
      }
    }
  }
}
</style>
