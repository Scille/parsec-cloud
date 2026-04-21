<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="stepConfig.titles"
    :subtitle="stepConfig.subtitles"
    :close-button="currentStep === ImportRecoveryDeviceStep.Done ? { visible: false } : { visible: true }"
    :cancel-button="stepConfig.cancelButton"
    :confirm-button="stepConfig.confirmButton"
  >
    <div class="recovery-device-files-page">
      <template v-if="currentStep === ImportRecoveryDeviceStep.RecoveryFile">
        <ms-report-text
          :theme="MsReportTheme.Warning"
          id="warning-text"
        >
          {{ $msTranslate('ImportRecoveryDevicePage.subtitles.recoveryFilesMustExistWarning') }}
        </ms-report-text>
        <div class="recovery-list">
          <div class="recovery-item">
            <ion-text class="recovery-item__title title-h4">
              <span class="recovery-number button-large">1</span>
              {{ $msTranslate('ImportRecoveryDevicePage.modal.recoveryFile.itemRecoveryFile') }}
              <ion-icon
                class="input-validity-icon"
                v-show="recoveryFile"
                :icon="checkmarkCircle"
              />
            </ion-text>
            <div class="recovery-item-key">
              <input
                type="file"
                hidden
                ref="hiddenInput"
                accept=".psrk"
              />
              <div
                class="file-waiting"
                v-if="!recoveryFile"
              >
                <ion-button
                  class="file-waiting__button"
                  @click="importButtonClick()"
                  fill="outline"
                >
                  <ion-icon :icon="documentOutline" />
                  {{ $msTranslate('ImportRecoveryDevicePage.modal.recoveryFile.actions.addRecoveryFile') }}
                </ion-button>
              </div>

              <div
                v-else
                class="file-added"
              >
                <ion-icon :icon="documentText" />
                <ion-text class="file-added__name button-medium">{{ recoveryFile.name }}</ion-text>
                <ion-button
                  class="file-added__update"
                  @click="importButtonClick()"
                >
                  {{ $msTranslate('ImportRecoveryDevicePage.modal.recoveryFile.actions.updateRecoveryFile') }}
                </ion-button>
              </div>
            </div>
          </div>

          <div class="recovery-item">
            <ion-text class="recovery-item__title title-h4">
              <span class="recovery-number button-large">2</span>
              {{ $msTranslate('ImportRecoveryDevicePage.modal.recoveryFile.itemRecoveryKey') }}
              <ion-icon
                class="input-validity-icon"
                v-show="isSecretKeyValid"
                :icon="checkmarkCircle"
              />
            </ion-text>
            <ms-input
              class="recovery-item__input"
              id="secret-key-input"
              placeholder="ImportRecoveryDevicePage.secretKeyPlaceholder"
              v-model="secretKey"
              @change="checkSecretKeyValidity()"
            />
          </div>
        </div>
      </template>

      <template v-else-if="currentStep === ImportRecoveryDeviceStep.Authentication && importedDevice">
        <choose-authentication
          ref="chooseAuth"
          :server-config="serverConfig"
          :server-addr="importedDevice.serverAddr"
        />
      </template>

      <template v-else-if="currentStep === ImportRecoveryDeviceStep.Done">
        <div class="final-step">
          <ms-image
            :image="ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string"
            class="final-step__logo"
          />
          <ion-text class="final-step__title title-h3">
            {{ $msTranslate('ImportRecoveryDevicePage.modal.done.title') }}
          </ion-text>
          <ion-button
            fill="solid"
            size="default"
            @click="finishRecovery()"
            class="final-step__button"
          >
            {{ $msTranslate('ImportRecoveryDevicePage.modal.done.login') }}
          </ion-button>
        </div>
      </template>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { getDefaultDeviceName } from '@/common/device';
import { secretKeyValidator } from '@/common/validators';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import {
  AvailableDevice,
  DeviceAccessStrategy,
  DevicePrimaryProtectionStrategy,
  DeviceSaveStrategy,
  ImportRecoveryDeviceErrorTag,
  PrimaryProtectionStrategy,
  ServerConfig,
  constructAccessStrategy,
  constructSaveStrategy,
  getServerConfig,
  importRecoveryDevice,
  updateDeviceChangeAuthentication,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle, documentOutline, documentText } from 'ionicons/icons';
import {
  LogoIconWhite,
  MsImage,
  MsInput,
  MsModal,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  Validity,
  asyncComputed,
} from 'megashark-lib';
import { Ref, computed, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  device?: AvailableDevice;
  informationManager: InformationManager;
}>();

enum ImportRecoveryDeviceStep {
  RecoveryFile,
  Authentication,
  Done,
}

const currentStep = ref(ImportRecoveryDeviceStep.RecoveryFile);
const hiddenInputRef = useTemplateRef<HTMLInputElement>('hiddenInput');
const chooseAuthRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('chooseAuth');
const secretKey: Ref<string> = ref('');
const recoveryFile: Ref<File | null> = ref(null);
const isSecretKeyValid = ref(false);
const importedDevice = ref<AvailableDevice | undefined>(undefined);
const serverConfig = ref<ServerConfig | undefined>(undefined);
let temporaryDeviceProtection: DevicePrimaryProtectionStrategy | undefined;
let accessStrategy: DeviceAccessStrategy | undefined;
let saveStrategy: DeviceSaveStrategy | undefined;

const isFileStepValid = computed(() => !!recoveryFile.value && isSecretKeyValid.value);

const authFieldsAreValid = asyncComputed(async (): Promise<boolean> => {
  if (currentStep.value !== ImportRecoveryDeviceStep.Authentication || !chooseAuthRef.value) {
    return false;
  }
  return await chooseAuthRef.value.areFieldsCorrect();
});

const stepConfig = computed(() => {
  switch (currentStep.value) {
    case ImportRecoveryDeviceStep.RecoveryFile:
      return {
        titles: 'ImportRecoveryDevicePage.modal.recoveryFile.title',
        subtitles: 'ImportRecoveryDevicePage.modal.recoveryFile.subtitle',
        cancelButton: { disabled: false, label: 'TextInputModal.cancel' },
        confirmButton: { disabled: !isFileStepValid.value, label: 'ImportRecoveryDevicePage.modal.actions.next', onClick: advance },
      };
    case ImportRecoveryDeviceStep.Authentication:
      return {
        titles: 'ImportRecoveryDevicePage.modal.authentication.title',
        subtitles: 'ImportRecoveryDevicePage.modal.authentication.subtitle',
        cancelButton: { disabled: false, label: 'HomePage.topbar.back', onClick: goBack },
        confirmButton: { disabled: !authFieldsAreValid.value, label: 'ImportRecoveryDevicePage.actions.validateAuth', onClick: advance },
      };
    default:
      return { titles: '', subtitles: '', cancelButton: undefined, confirmButton: undefined };
  }
});

async function goBack(): Promise<boolean> {
  currentStep.value = ImportRecoveryDeviceStep.RecoveryFile;
  return false;
}

async function advance(): Promise<boolean> {
  await nextStep(currentStep.value);
  return false;
}

async function onInputChange(_event: Event): Promise<void> {
  if (hiddenInputRef.value!.files!.length === 1) {
    recoveryFile.value = hiddenInputRef.value!.files![0];
  }
  hiddenInputRef.value!.removeEventListener('change', onInputChange);
}

async function importButtonClick(): Promise<void> {
  hiddenInputRef.value!.addEventListener('change', onInputChange);
  hiddenInputRef.value!.click();
}

async function checkSecretKeyValidity(): Promise<void> {
  isSecretKeyValid.value = (await secretKeyValidator(secretKey.value)).validity === Validity.Valid;
}

async function validateInputs(): Promise<{ recoveryFile: File; secretKey: string } | undefined> {
  if (!recoveryFile.value || !secretKey.value) {
    return;
  }

  await checkSecretKeyValidity();
  if (!isSecretKeyValid.value) {
    return undefined;
  }

  return {
    recoveryFile: recoveryFile.value,
    secretKey: secretKey.value,
  };
}

async function nextStep(step: ImportRecoveryDeviceStep): Promise<void> {
  if (step === ImportRecoveryDeviceStep.RecoveryFile) {
    const filesResult = await validateInputs();
    if (!filesResult) {
      return;
    }

    const reader = filesResult.recoveryFile.stream().getReader();
    const content = new Uint8Array(filesResult.recoveryFile.size);
    let offset = 0;
    let buffer = await reader.read();
    while (!buffer.done) {
      content.set(buffer.value, offset);
      offset += buffer.value.length;
      buffer = await reader.read();
    }
    if (buffer.value) {
      content.set(buffer.value, offset);
    }

    temporaryDeviceProtection = PrimaryProtectionStrategy.usePassword(window.crypto.randomUUID());

    const importResult = await importRecoveryDevice(
      props.device ? props.device.deviceLabel : getDefaultDeviceName(),
      content,
      filesResult.secretKey.trim(),
      constructSaveStrategy(temporaryDeviceProtection),
    );

    if (!importResult.ok) {
      const notificationInfo = { message: '', level: InformationLevel.Error };
      switch (importResult.error.tag) {
        case ImportRecoveryDeviceErrorTag.InvalidPassphrase:
          notificationInfo.message = 'ImportRecoveryDevicePage.errors.keyErrorMessage';
          break;
        case ImportRecoveryDeviceErrorTag.InvalidData:
          notificationInfo.message = 'ImportRecoveryDevicePage.errors.fileErrorMessage';
          break;
        default:
          notificationInfo.message = 'ImportRecoveryDevicePage.errors.internalErrorMessage';
          break;
      }
      props.informationManager.present(new Information(notificationInfo), PresentationMode.Toast);
      return;
    }

    importedDevice.value = importResult.value;
    const serverConfigResult = await getServerConfig(importedDevice.value.serverAddr);
    if (serverConfigResult.ok) {
      serverConfig.value = serverConfigResult.value;
    }
    currentStep.value = ImportRecoveryDeviceStep.Authentication;
    return;
  }

  if (step === ImportRecoveryDeviceStep.Authentication) {
    if (!chooseAuthRef.value || !importedDevice.value || !temporaryDeviceProtection) {
      return;
    }

    saveStrategy = await chooseAuthRef.value.getSaveStrategy();
    if (!saveStrategy) {
      return;
    }

    const access = constructAccessStrategy(importedDevice.value, temporaryDeviceProtection);
    const result = await updateDeviceChangeAuthentication(access, saveStrategy);
    if (!result.ok) {
      props.informationManager.present(
        new Information({
          message: 'ImportRecoveryDevicePage.errors.internalErrorMessage',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      currentStep.value = ImportRecoveryDeviceStep.RecoveryFile;
      return;
    }

    importedDevice.value = result.value;
    accessStrategy = constructAccessStrategy(importedDevice.value, saveStrategy.primaryProtection, saveStrategy.totpProtection);
    currentStep.value = ImportRecoveryDeviceStep.Done;
  }
}

async function finishRecovery(): Promise<void> {
  if (!importedDevice.value || !accessStrategy || !saveStrategy) {
    return;
  }
  await modalController.dismiss(
    {
      device: importedDevice.value,
      access: accessStrategy,
      saveStrategy,
    },
    MsModalResult.Confirm,
  );
}
</script>

<style lang="scss" scoped>
.recovery-device-files-page {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.final-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;

  &__title {
    color: var(--parsec-color-light-secondary-white);
    text-align: center;
  }
}

.recovery-list {
  display: flex;
  flex-direction: column;
  gap: 2rem;

  .recovery-item {
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

      .input-validity-icon {
        font-size: 1.125rem;
        color: var(--parsec-color-light-success-700);
      }
    }

    .recovery-number {
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

    .file-waiting {
      display: flex;
      width: 100%;

      &__button {
        display: flex;
        align-items: center;
        gap: 1rem;
        color: var(--parsec-color-light-secondary-text);
        --background: var(--parsec-color-light-secondary-white);
        --background-hover: var(--parsec-color-light-secondary-premiere);
        width: 100%;

        &::part(native) {
          padding: 0.625rem 1rem;
          border: 1px dashed var(--parsec-color-light-secondary-light);
        }

        &:hover {
          &::part(native) {
            border-color: var(--parsec-color-light-secondary-grey);
          }
        }

        ion-icon {
          font-size: 1.25rem;
          margin-right: 0.5rem;
        }
      }
    }

    .file-added {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-white);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-12);
      padding: 0.625rem;
      min-height: 2.5rem;
      overflow: hidden;

      &__update {
        --background: var(--parsec-color-light-secondary-medium);
        --background-hover: var(--parsec-color-light-secondary-disabled);
        color: var(--parsec-color-light-secondary-text);
        margin-left: auto;
        box-shadow: var(--parsec-shadow-input);

        &::part(native) {
          padding: 0.75rem 1rem;
        }

        &:hover {
          color: var(--parsec-color-light-secondary-text);
        }
      }

      ion-icon {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-secondary-text);
        display: flex;
        align-items: center;
        padding: 0.5rem;
        border-radius: var(--parsec-radius-8);
        font-size: 1.25rem;
      }
    }

    &__input {
      width: 100%;
      border-radius: var(--parsec-radius-12);
      background: var(--parsec-color-light-secondary-white);
    }
  }
}
</style>
