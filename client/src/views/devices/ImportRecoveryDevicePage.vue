<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div v-if="state === ImportDevicePageState.Start">
    <div>
      <ion-card class="login-popup">
        <ion-card-content class="import-recovery-container">
          <ms-report-text
            :theme="MsReportTheme.Warning"
            id="warning-text"
          >
            {{ $t('ImportRecoveryDevicePage.subtitles.recoveryFilesMustExistWarning') }}
          </ms-report-text>
          <ion-text class="title-h2">
            {{ $t('ImportRecoveryDevicePage.titles.forgottenPassword') }}
          </ion-text>
          <div class="file-list">
            <div class="file-item">
              <div class="file-item__title subtitles-normal">
                <ion-icon :icon="document" />
                {{ $t('ImportRecoveryDevicePage.titles.recoveryFile') }}
              </div>
              <div class="file-item__button">
                <input
                  type="file"
                  hidden
                  ref="hiddenInput"
                />
                <div v-if="!recoveryFile">
                  {{ $t('ImportRecoveryDevicePage.subtitles.noFileSelected') }}
                </div>
                <div v-else>
                  {{ recoveryFile.name }}
                </div>
                <ion-button
                  id="browse-button"
                  @click="importButtonClick()"
                  fill="outline"
                >
                  {{ $t('ImportRecoveryDevicePage.actions.browse') }}
                </ion-button>
              </div>
            </div>
            <div class="file-item">
              <div class="file-item__title subtitles-normal">
                <ion-icon :icon="key" />
                {{ $t('ImportRecoveryDevicePage.titles.recoveryKey') }}
              </div>
              <div class="file-item__button">
                <ms-input
                  class="file-item__input"
                  id="secret-key-input"
                  :placeholder="secretKeyPlaceholder"
                  v-model="secretKey"
                  @change="checkSecretKeyValidity()"
                />
                <ion-icon
                  id="checkmark-icon"
                  v-show="isSecretKeyValid"
                  :icon="checkmarkCircle"
                />
              </div>
            </div>
          </div>
          <div class="next-button">
            <ion-button
              slot="start"
              id="to-password-change-btn"
              @click="goToPasswordChange()"
              :disabled="!isSecretKeyValid || !recoveryFile"
            >
              {{ $t('ImportRecoveryDevicePage.actions.next') }}
            </ion-button>
          </div>
        </ion-card-content>
      </ion-card>
    </div>
  </div>
  <div
    v-else-if="state === ImportDevicePageState.Password"
    class="password-input"
  >
    <ms-choose-password-input
      :password-label="$t('ImportRecoveryDevicePage.titles.setNewPassword')"
      @on-enter-keyup="createNewDevice()"
      ref="choosePasswordInput"
    />
    <ion-button
      id="validate-password-btn"
      class="validate-button"
      :disabled="!changeButtonIsEnabled"
      @click="createNewDevice()"
    >
      {{ $t('ImportRecoveryDevicePage.actions.validatePassword') }}
    </ion-button>
  </div>
  <div
    id="success-step"
    v-else-if="state === ImportDevicePageState.Done"
  >
    <ion-card class="success-card">
      <ion-card-title class="success-card__title">
        {{ $t('ImportRecoveryDevicePage.titles.passwordChanged') }}
      </ion-card-title>
      <ms-informative-text>
        {{ $t('ImportRecoveryDevicePage.subtitles.passwordModified') }}
      </ms-informative-text>
      <ion-button
        class="success-card__button"
        @click="goToHome()"
      >
        {{ $t('ImportRecoveryDevicePage.actions.goBackToLogin') }}
      </ion-button>
    </ion-card>
  </div>
</template>

<script setup lang="ts">
import { asyncComputed } from '@/common/asyncComputed';
import { Validity, secretKeyValidator } from '@/common/validators';
import { MsChoosePasswordInput, MsInformativeText, MsInput, MsReportText, MsReportTheme } from '@/components/core';
import { AvailableDevice, DeviceInfo, RecoveryImportErrorTag, SecretKey, deleteDevice, importRecoveryDevice, saveDevice } from '@/parsec';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { IonButton, IonCard, IonCardContent, IonCardTitle, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle, document, key } from 'ionicons/icons';
import { Ref, defineEmits, defineProps, inject, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

enum ImportDevicePageState {
  Start = 'start',
  Password = 'password',
  Done = 'done',
}

const state = ref(ImportDevicePageState.Start);
const choosePasswordInput: Ref<typeof MsChoosePasswordInput | null> = ref(null);
const hiddenInput = ref();
const secretKey: Ref<SecretKey> = ref('');
// cspell:disable-next-line
const secretKeyPlaceholder = 'FH3H-N3DW-RUOO-A6Q7-...';
const changeButtonIsEnabled = asyncComputed(async (): Promise<boolean> => {
  return choosePasswordInput.value && (await choosePasswordInput.value.areFieldsCorrect());
});
const recoveryFile: Ref<File | null> = ref(null);
const newDeviceInfo: Ref<DeviceInfo | null> = ref(null);
const isSecretKeyValid = ref(false);
const notificationManager: NotificationManager = inject(NotificationKey)!;

const emits = defineEmits<{
  (e: 'finished'): void;
}>();

const props = defineProps<{
  device: AvailableDevice;
}>();

onMounted(() => {
  hiddenInput.value.addEventListener('change', onInputChange);
});

async function onInputChange(_event: Event): Promise<void> {
  if (hiddenInput.value.files.length === 1) {
    recoveryFile.value = hiddenInput.value.files[0];
  }
}

async function importButtonClick(): Promise<void> {
  hiddenInput.value.click();
}

async function checkSecretKeyValidity(): Promise<void> {
  isSecretKeyValid.value = (await secretKeyValidator(secretKey.value)) === Validity.Valid;
}

async function goToPasswordChange(): Promise<void> {
  if (!recoveryFile.value) {
    return;
  }
  const result = await importRecoveryDevice(props.device.deviceLabel, recoveryFile.value, secretKey.value);
  if (result.ok) {
    newDeviceInfo.value = result.value;
    state.value = ImportDevicePageState.Password;
  } else {
    const notificationInfo = { title: '', message: '', level: NotificationLevel.Error };

    switch (result.error.tag) {
      case RecoveryImportErrorTag.KeyError:
        notificationInfo.title = t('ImportRecoveryDevicePage.errors.keyErrorTitle');
        notificationInfo.message = t('ImportRecoveryDevicePage.errors.keyErrorMessage');
        break;
      case RecoveryImportErrorTag.RecoveryFileError:
        notificationInfo.title = t('ImportRecoveryDevicePage.errors.fileErrorTitle');
        notificationInfo.message = t('ImportRecoveryDevicePage.errors.fileErrorMessage');
        break;
      case RecoveryImportErrorTag.Internal:
        notificationInfo.title = t('ImportRecoveryDevicePage.errors.internalErrorTitle');
        notificationInfo.message = t('ImportRecoveryDevicePage.errors.internalErrorMessage');
        break;
    }
    notificationManager.showToast(new Notification(notificationInfo));
  }
}

async function createNewDevice(): Promise<void> {
  // Check matching and valid passwords
  if (!choosePasswordInput.value || !(await choosePasswordInput.value.areFieldsCorrect())) {
    notificationManager.showToast(
      new Notification({
        title: t('ImportRecoveryDevicePage.errors.passwordErrorTitle'),
        message: t('ImportRecoveryDevicePage.errors.passwordErrorMessage'),
        level: NotificationLevel.Error,
      }),
    );
    return;
  }
  // Check new device info exists
  if (!newDeviceInfo.value) {
    notificationManager.showToast(
      new Notification({
        title: t('ImportRecoveryDevicePage.errors.internalErrorTitle'),
        message: t('ImportRecoveryDevicePage.errors.internalErrorMessage'),
        level: NotificationLevel.Error,
      }),
    );
    return;
  }
  // Save new device with password
  if (!(await saveDevice(newDeviceInfo.value, 'newPassword')).ok) {
    notificationManager.showToast(
      new Notification({
        title: t('ImportRecoveryDevicePage.errors.saveDeviceErrorTitle'),
        message: t('ImportRecoveryDevicePage.errors.saveDeviceErrorMessage'),
        level: NotificationLevel.Error,
      }),
    );
    return;
  }
  // Delete previous device
  await deleteDevice(props.device);

  state.value = ImportDevicePageState.Done;
}

async function goToHome(): Promise<void> {
  state.value = ImportDevicePageState.Start;
  emits('finished');
}
</script>

<style lang="scss" scoped>
.login-popup {
  box-shadow: none;
  display: flex;
  margin: 1.5rem 1.5rem;
  align-items: center;
  justify-content: left;
  flex-grow: 1;
  max-height: 80%;
  .title-h2 {
    color: var(--parsec-color-light-primary-700);
  }
}
.import-recovery-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  .file-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    .file-item {
      padding: 1.5rem;
      border-radius: var(--parsec-radius-8);
      background: var(--parsec-color-light-secondary-background);
      width: 100%;
      &__title {
        gap: 1rem;
        display: flex;
        align-items: center;
        color: var(--parsec-color-light-primary-700);
        ion-icon {
          font-size: 1.5rem;
        }
      }
      &__button {
        margin-top: 1.5rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-primary-700);
        ion-button {
          margin: 0;
        }
        ion-icon {
          font-size: 1rem;
          color: var(--parsec-color-light-success-500);
        }
      }
      &__input {
        width: 95%;
      }
    }
  }
}
.next-button {
  margin-left: auto;
}
.password-input {
  margin: 1.5rem;
}
.validate-button {
  display: flex;
  width: fit-content;
  margin-left: auto;
}
.success-card {
  box-shadow: none;
  margin: 20% 1.5rem;
  background-color: var(--parsec-color-light-primary-background);
  &__title {
    color: var(--parsec-color-light-primary-700);
    margin-bottom: 1.5rem;
  }
  &__button {
    display: flex;
    width: fit-content;
    margin-left: auto;
  }
}
</style>
