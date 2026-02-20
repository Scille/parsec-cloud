<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="state === ImportDevicePageState.Start"
    class="recovery-content"
  >
    <!-- step 1: recovery file -->
    <div class="recovery-header">
      <ion-text class="recovery-header__title title-h1">
        {{ $msTranslate('ImportRecoveryDevicePage.titles.forgottenPassword') }}
      </ion-text>
    </div>
    <ion-card class="recovery-card">
      <ion-card-content class="card-container">
        <organization-card
          v-if="device"
          :device="device"
          :org-name-only="true"
        />
        <ms-report-text
          :theme="MsReportTheme.Warning"
          id="warning-text"
        >
          {{ $msTranslate('ImportRecoveryDevicePage.subtitles.recoveryFilesMustExistWarning') }}
        </ms-report-text>
        <div class="recovery-list">
          <!-- recovery item -->
          <div class="recovery-list-item">
            <div class="recovery-list-item__title subtitles-normal">
              <span class="number subtitles-normal">1</span>
              {{ $msTranslate('ImportRecoveryDevicePage.titles.recoveryFile') }}
            </div>
            <div class="recovery-list-item__button">
              <input
                type="file"
                hidden
                ref="hiddenInput"
                accept=".psrk"
              />
              <div
                v-if="!recoveryFile"
                class="body"
              >
                {{ $msTranslate('ImportRecoveryDevicePage.subtitles.noFileSelected') }}
              </div>
              <div
                v-else
                class="body file-added"
                @click="importButtonClick()"
              >
                {{ recoveryFile.name }}
              </div>
              <ion-button
                id="browse-button"
                @click="importButtonClick()"
                fill="outline"
              >
                {{ $msTranslate('ImportRecoveryDevicePage.actions.browse') }}
              </ion-button>
            </div>
          </div>

          <!-- ----- -->
          <div class="recovery-divider" />

          <!-- recovery item -->
          <div
            class="recovery-list-item"
            :class="{ disabled: !recoveryFile }"
          >
            <div class="recovery-list-item__title subtitles-normal">
              <span class="number">2</span>
              {{ $msTranslate('ImportRecoveryDevicePage.titles.recoveryKey') }}
            </div>
            <div class="recovery-list-item__button">
              <ms-input
                class="recovery-list-item__input"
                id="secret-key-input"
                placeholder="ImportRecoveryDevicePage.secretKeyPlaceholder"
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
            size="large"
            id="to-password-change-btn"
            @click="goToPasswordChange()"
            :disabled="!isSecretKeyValid || !recoveryFile"
          >
            {{ $msTranslate('ImportRecoveryDevicePage.actions.next') }}
          </ion-button>
        </div>
      </ion-card-content>
    </ion-card>
  </div>
  <!-- step 2: new authentication -->
  <div
    v-else-if="state === ImportDevicePageState.Authentication"
    class="recovery-content password-input"
  >
    <div class="recovery-header">
      <ion-text class="recovery-header__title title-h1">
        <span v-if="isWeb()">{{ $msTranslate('ImportRecoveryDevicePage.titles.setNewPassword') }}</span>
        <span v-else>{{ $msTranslate('ImportRecoveryDevicePage.titles.setNewAuthentication') }}</span>
      </ion-text>
    </div>
    <ion-card class="recovery-card">
      <choose-authentication ref="chooseAuth" />
      <ion-button
        id="validate-password-btn"
        class="validate-button"
        :disabled="!changeButtonIsEnabled"
        @click="createNewDevice"
      >
        {{ $msTranslate('ImportRecoveryDevicePage.actions.validateAuth') }}
      </ion-button>
    </ion-card>
  </div>
  <!-- step 3: done -->
  <div
    v-else-if="state === ImportDevicePageState.Done"
    id="success-step"
    class="recovery-content"
  >
    <ion-card class="recovery-card success-card">
      <ion-card-title class="success-card__title title-h2">
        {{ $msTranslate('ImportRecoveryDevicePage.titles.passwordChanged') }}
      </ion-card-title>
      <ms-informative-text>
        {{ $msTranslate('ImportRecoveryDevicePage.subtitles.passwordModified') }}
      </ms-informative-text>
      <ion-button
        class="success-card__button"
        @click="onLoginClick"
      >
        {{ $msTranslate('ImportRecoveryDevicePage.actions.login') }}
      </ion-button>
    </ion-card>
  </div>
</template>

<script setup lang="ts">
import { getDefaultDeviceName } from '@/common/device';
import { secretKeyValidator } from '@/common/validators';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import {
  AvailableDevice,
  constructAccessStrategy,
  DeviceAccessStrategy,
  importRecoveryDevice,
  ImportRecoveryDeviceErrorTag,
  isWeb,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { IonButton, IonCard, IonCardContent, IonCardTitle, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { asyncComputed, MsInformativeText, MsInput, MsReportText, MsReportTheme, Validity } from 'megashark-lib';
import { inject, Ref, ref, useTemplateRef } from 'vue';

enum ImportDevicePageState {
  Start = 'start',
  Authentication = 'authentication',
  Done = 'done',
}

const state = ref(ImportDevicePageState.Start);
const chooseAuthRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('chooseAuth');
const hiddenInputRef = useTemplateRef<HTMLInputElement>('hiddenInput');
const secretKey: Ref<string> = ref('');
const recoveryFile: Ref<File | null> = ref(null);
const isSecretKeyValid = ref(false);
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const informationManager: InformationManager = injectionProvider.getDefault().informationManager;
let accessStrategy: DeviceAccessStrategy | undefined;
let newDevice: AvailableDevice;

const changeButtonIsEnabled = asyncComputed(async (): Promise<boolean> => {
  if (!chooseAuthRef.value) {
    return false;
  }
  return await chooseAuthRef.value.areFieldsCorrect();
});

const emits = defineEmits<{
  (e: 'organizationSelected', device: AvailableDevice, access: DeviceAccessStrategy): void;
}>();

const props = defineProps<{
  device?: AvailableDevice;
}>();

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

async function goToPasswordChange(): Promise<void> {
  state.value = ImportDevicePageState.Authentication;
}

async function createNewDevice(): Promise<void> {
  if (!recoveryFile.value) {
    return;
  }
  if (!(await chooseAuthRef.value!.areFieldsCorrect())) {
    return;
  }

  const reader = recoveryFile.value.stream().getReader();
  const content = new Uint8Array(recoveryFile.value.size);
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

  if (!chooseAuthRef.value) {
    informationManager.present(
      new Information({
        message: 'ImportRecoveryDevicePage.errors.internalErrorMessage',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    state.value = ImportDevicePageState.Start;
    return;
  }
  const saveStrategy = chooseAuthRef.value.getSaveStrategy();
  if (!saveStrategy) {
    return;
  }
  const result = await importRecoveryDevice(
    props.device ? props.device.deviceLabel : getDefaultDeviceName(),
    content,
    secretKey.value.trim(),
    saveStrategy,
  );
  if (result.ok) {
    newDevice = result.value;
    accessStrategy = constructAccessStrategy(newDevice, saveStrategy.primaryProtection, saveStrategy.totpProtection);
    state.value = ImportDevicePageState.Done;
  } else {
    const notificationInfo = { message: '', level: InformationLevel.Error };

    switch (result.error.tag) {
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
    informationManager.present(new Information(notificationInfo), PresentationMode.Toast);
    state.value = ImportDevicePageState.Start;
  }
}

async function onLoginClick(): Promise<void> {
  emits('organizationSelected', newDevice, accessStrategy as DeviceAccessStrategy);
}
</script>

<style lang="scss" scoped>
.recovery-content {
  height: auto;
  width: 60vw;
  max-width: 40rem;
  display: flex;
  margin: 2rem auto;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  box-shadow: none;

  @include ms.responsive-breakpoint('sm') {
    width: 100%;
    max-width: none;
    margin: 0 auto;
    gap: 1.5rem;
  }
}

.recovery-header {
  &__title {
    background: var(--parsec-color-light-gradient-background);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }
}

.recovery-card {
  height: auto;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  padding: 2rem;
  margin: 0;
  border-radius: var(--parsec-radius-12);
  box-shadow: var(--parsec-shadow-light);
  background: var(--parsec-color-light-secondary-white);

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem;
  }
}

.card-container {
  display: flex;
  flex-direction: column;
  padding: 0;
  gap: 2rem;
  overflow: auto;

  .recovery-list {
    display: flex;
    flex-direction: column;
    gap: 2rem;

    &-item {
      border-radius: var(--parsec-radius-8);
      width: 100%;
      position: relative;
      display: flex;
      flex-direction: column;
      gap: 1.5rem;

      &__title {
        color: var(--parsec-color-light-primary-700);
        display: flex;
        align-items: center;
        gap: 0.5rem;

        .number {
          display: flex;
          justify-content: center;
          align-items: center;
          border-radius: var(--parsec-radius-32);
          width: 1.25rem;
          height: 1.25rem;
          color: var(--parsec-color-light-secondary-white);
          background: var(--parsec-color-light-primary-700);
        }
      }

      &__button {
        display: flex;
        align-items: center;
        gap: 1rem;
        color: var(--parsec-color-light-secondary-grey);

        .file-added {
          color: var(--parsec-color-light-secondary-text);

          &:hover {
            cursor: pointer;
            text-decoration: underline;
          }
        }

        ion-button {
          margin: 0;
        }
        ion-icon {
          font-size: 1.25rem;
          color: var(--parsec-color-light-success-500);
        }
      }

      &__input {
        width: 100%;
      }

      &.disabled {
        opacity: 0.3;
        pointer-events: none;
        user-select: none;
      }
    }
  }

  .recovery-divider {
    width: 100%;
    height: 1px;
    background-color: var(--parsec-color-light-secondary-medium);
  }
}

.validate-button,
.next-button {
  display: flex;
  width: fit-content;
  margin-left: auto;

  @include ms.responsive-breakpoint('sm') {
    width: 100%;
  }

  ion-button {
    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }
  }
}

#validate-password-btn {
  margin-top: 2rem;
}

#success-step {
  margin-inline: auto;
  transform: translate(0, 20%);

  .success-card {
    &__title {
      color: var(--parsec-color-light-primary-700);
      margin-bottom: 1.5rem;
    }

    &__button {
      margin-top: 2rem;
      display: flex;
      width: fit-content;
      margin-left: auto;

      @include ms.responsive-breakpoint('sm') {
        width: 100%;
      }
    }
  }
}
</style>
