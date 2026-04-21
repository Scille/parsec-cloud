<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="stepConfig.titles"
    :subtitle="stepConfig.subtitles"
    :close-button="{ visible: true }"
    :cancel-button="stepConfig.cancelButton"
    :confirm-button="stepConfig.confirmButton"
  >
    <div class="choose-recovery-device-modal__content">
      <ion-radio-group
        v-if="currentStep === RecoveryMethodStep.MethodChoice"
        class="recovery-method-list"
        v-model="selectedMethod"
      >
        <ion-radio
          :value="RecoveryMethod.ConnectedDevice"
          class="recovery-method-item recovery-method--recommended"
          :class="{ 'recovery-method-item--selected': selectedMethod === RecoveryMethod.ConnectedDevice }"
        >
          <div class="recovery-method-item__container">
            <ms-image
              :image="RecoveryDeviceIcon"
              class="recovery-method-item__image"
            />
            <div class="recovery-method-item__content">
              <ion-text class="recovery-method-item__title title-h4">
                {{ $msTranslate('ImportRecoveryDevicePage.modal.options.otherConnectedDevice.title') }}
              </ion-text>
              <ion-text class="recovery-method-item__subtitle body-lg">
                {{ $msTranslate('ImportRecoveryDevicePage.modal.options.otherConnectedDevice.subtitle') }}
              </ion-text>
            </div>
          </div>
          <span class="recovery-method-item__recommended-badge subtitles-sm">
            {{ $msTranslate('ImportRecoveryDevicePage.modal.options.otherConnectedDevice.badge') }}
          </span>
        </ion-radio>

        <ion-radio
          :value="RecoveryMethod.RecoveryFile"
          class="recovery-method-item"
          :class="{ 'recovery-method-item--selected': selectedMethod === RecoveryMethod.RecoveryFile }"
        >
          <ms-image
            :image="RecoveryFileIcon"
            class="recovery-method-item__image"
          />
          <div class="recovery-method-item__content">
            <ion-text class="recovery-method-item__title title-h4">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.options.recoveryFile.title') }}
            </ion-text>
            <ion-text class="recovery-method-item__subtitle body-lg">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.options.recoveryFile.subtitle') }}
            </ion-text>
          </div>
        </ion-radio>

        <ion-radio
          v-if="false"
          :value="RecoveryMethod.Shamir"
          class="recovery-method-item"
          :class="{ 'recovery-method-item--selected': selectedMethod === RecoveryMethod.Shamir }"
        >
          <ms-image
            :image="RecoveryTrustIcon"
            class="recovery-method-item__image"
          />
          <div class="recovery-method-item__content">
            <ion-text class="recovery-method-item__title title-h4">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.options.shamir.title') }}
            </ion-text>
            <ion-text class="recovery-method-item__subtitle body-lg">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.options.shamir.subtitle') }}
            </ion-text>
          </div>
        </ion-radio>
      </ion-radio-group>

      <!-- step 2 - Method Recovery: Connected Device -->
      <template v-else-if="currentStep === RecoveryMethodStep.ConnectedDevice">
        <div class="method-step-list">
          <div class="method-step-list__item">
            <ion-icon
              class="method-step-list__item-icon"
              slot="start"
              :icon="logIn"
            />
            <ion-text class="method-step-list__item-description subtitles-normal">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step1') }}
            </ion-text>
          </div>
          <div class="method-step-list__item">
            <ion-icon
              class="method-step-list__item-icon"
              slot="start"
              :icon="phonePortraitOutline"
            />
            <ion-text class="method-step-list__item-description subtitles-normal">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step2') }}
            </ion-text>
          </div>
          <div class="method-step-list__item">
            <ion-icon
              class="method-step-list__item-icon"
              slot="start"
              :icon="add"
            />
            <ion-text class="method-step-list__item-description subtitles-normal">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step3') }}
            </ion-text>
          </div>
          <div class="method-step-list__item">
            <ion-icon
              class="method-step-list__item-icon"
              slot="start"
              :icon="link"
            />
            <ion-text class="method-step-list__item-description subtitles-normal">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step4') }}
            </ion-text>
          </div>
        </div>
      </template>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import RecoveryDeviceIcon from '@/assets/images/recovery-device-icon.svg?raw';
import RecoveryFileIcon from '@/assets/images/recovery-file-icon.svg?raw';
import RecoveryTrustIcon from '@/assets/images/recovery-trusted-user-icon.svg?raw';
import { RecoveryMethod } from '@/views/devices/types';
import { IonIcon, IonRadio, IonRadioGroup, IonText, modalController } from '@ionic/vue';
import { add, link, logIn, phonePortraitOutline } from 'ionicons/icons';
import { MsImage, MsModal, MsModalResult } from 'megashark-lib';
import { computed, ref } from 'vue';

enum RecoveryMethodStep {
  MethodChoice = 'method-choice',
  ConnectedDevice = 'connected-device',
}

const currentStep = ref<RecoveryMethodStep>(RecoveryMethodStep.MethodChoice);
const selectedMethod = ref<RecoveryMethod | undefined>(undefined);

const stepConfig = computed(() => {
  if (currentStep.value === RecoveryMethodStep.MethodChoice) {
    return {
      titles: 'ImportRecoveryDevicePage.modal.title',
      subtitles: 'ImportRecoveryDevicePage.modal.subtitle',
      cancelButton: { disabled: false, label: 'TextInputModal.cancel' },
      confirmButton: {
        disabled: selectedMethod.value === undefined,
        label: 'ImportRecoveryDevicePage.modal.actions.chooseMethod',
        onClick: nextStep,
      },
    };
  }
  return {
    titles: 'ImportRecoveryDevicePage.modal.connectedDevice.title',
    subtitles: 'ImportRecoveryDevicePage.modal.connectedDevice.subtitle',
    cancelButton: {
      disabled: false,
      label: 'HomePage.topbar.back',
      onClick: async () => {
        currentStep.value = RecoveryMethodStep.MethodChoice;
        return false;
      },
    },
    confirmButton: {
      disabled: false,
      label: 'ImportRecoveryDevicePage.modal.actions.next',
      onClick: nextStep,
    },
  };
});

async function nextStep(): Promise<boolean> {
  if (!selectedMethod.value) {
    return false;
  }

  if (selectedMethod.value === RecoveryMethod.ConnectedDevice && currentStep.value === RecoveryMethodStep.MethodChoice) {
    currentStep.value = RecoveryMethodStep.ConnectedDevice;
    return false;
  } else {
    await modalController.dismiss({ recoveryMethod: selectedMethod.value }, MsModalResult.Confirm);
    return true;
  }
}
</script>

<style scoped lang="scss">
.choose-recovery-device-modal__content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.recovery-method-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.recovery-method-item {
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem;
  position: relative;
  box-shadow:
    0 1px 1px 0 rgba(0, 0, 0, 0.05),
    0 1px 4px 0 rgba(0, 0, 0, 0.03),
    0 0 1px 0 rgba(0, 0, 0, 0.2);
  transition:
    transform 120ms ease,
    border-color 120ms ease,
    box-shadow 120ms ease;

  &::part(label) {
    display: flex;
    gap: 1rem;
    margin: 0;
    width: 100%;
  }

  // Hide default radio circle
  &::part(container) {
    display: none;
  }

  &:not(.recovery-method-item--selected):hover {
    outline: 1.5px solid var(--parsec-color-light-secondary-medium);
  }

  &--selected {
    outline: 1.5px solid var(--parsec-color-light-primary-400);
  }

  &__container {
    display: flex;
    gap: 1rem;
  }

  &--recommended {
    border-color: var(--parsec-color-light-primary-300);
  }

  &__image {
    width: fit-content;
    height: fit-content;
    flex-shrink: 0;
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);
    border-radius: var(--parsec-radius-8);
  }

  &__content {
    display: flex;
    width: 100%;
    text-wrap: wrap;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
  }

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-hard-grey);
    font-size: 0.9375rem;
    line-height: 1.5;
  }

  &__recommended-badge {
    padding: 0.2rem 0.6rem;
    border-radius: var(--parsec-radius-8);
    background: var(--parsec-color-light-primary-50);
    color: var(--parsec-color-light-primary-500);
    line-height: 1;
    white-space: nowrap;
    position: absolute;
    top: -0.5rem;
    right: -0.5rem;
    z-index: 100;
  }
}

.method-step-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;

  &__item {
    background: var(--parsec-color-light-secondary-premiere);
    display: flex;
    align-items: center;
    padding: 0.5rem 0.75rem;
    border-radius: var(--parsec-radius-12);
    gap: 1rem;

    &-icon {
      width: 1.125rem;
      height: 1.125rem;
      flex-shrink: 0;
      padding: 0.375rem;
      color: var(--parsec-color-light-secondary-white);
      background: var(--parsec-color-light-secondary-text);
      border-radius: var(--parsec-radius-circle);
    }

    &-description {
      color: var(--parsec-color-light-secondary-soft-text);
    }
  }
}
</style>
