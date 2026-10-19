<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
          class="recovery-method-item"
          :class="{ 'recovery-method-item--selected': selectedMethod === RecoveryMethod.ConnectedDevice }"
        >
          <div class="recovery-method-item__container">
            <ms-image
              :image="isDarkTheme ? RecoveryDeviceIconDark : RecoveryDeviceIcon"
              class="recovery-method-item__image"
            />
            <div class="recovery-method-item__content">
              <div class="recovery-method-item__header">
                <ion-text class="recovery-method-item__title">
                  {{ $msTranslate('ImportRecoveryDevicePage.modal.options.otherConnectedDevice.title') }}
                </ion-text>
                <span class="recovery-method-item__recommended-badge">
                  {{ $msTranslate('ImportRecoveryDevicePage.modal.options.otherConnectedDevice.badge') }}
                </span>
              </div>
              <ion-text class="recovery-method-item__subtitle">
                {{ $msTranslate('ImportRecoveryDevicePage.modal.options.otherConnectedDevice.subtitle') }}
              </ion-text>
            </div>
          </div>
        </ion-radio>

        <ion-radio
          :value="RecoveryMethod.RecoveryFile"
          class="recovery-method-item"
          :class="{ 'recovery-method-item--selected': selectedMethod === RecoveryMethod.RecoveryFile }"
        >
          <ms-image
            :image="isDarkTheme ? RecoveryFileIconDark : RecoveryFileIcon"
            class="recovery-method-item__image"
          />
          <div class="recovery-method-item__content">
            <ion-text class="recovery-method-item__title">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.options.recoveryFile.title') }}
            </ion-text>
            <ion-text class="recovery-method-item__subtitle">
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
            :image="isDarkTheme ? RecoveryTrustIconDark : RecoveryTrustIcon"
            class="recovery-method-item__image"
          />
          <div class="recovery-method-item__content">
            <ion-text class="recovery-method-item__title">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.options.shamir.title') }}
            </ion-text>
            <ion-text class="recovery-method-item__subtitle">
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
            <ion-text class="method-step-list__item-description">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step1') }}
            </ion-text>
          </div>
          <div class="method-step-list__item">
            <ion-icon
              class="method-step-list__item-icon"
              slot="start"
              :icon="phonePortraitOutline"
            />
            <ion-text class="method-step-list__item-description">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step2') }}
            </ion-text>
          </div>
          <div class="method-step-list__item">
            <ion-icon
              class="method-step-list__item-icon"
              slot="start"
              :icon="add"
            />
            <ion-text class="method-step-list__item-description">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step3') }}
            </ion-text>
          </div>
          <div class="method-step-list__item">
            <ion-icon
              class="method-step-list__item-icon"
              slot="start"
              :icon="link"
            />
            <ion-text class="method-step-list__item-description">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.connectedDevice.steps.step4') }}
            </ion-text>
          </div>
        </div>
      </template>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import RecoveryDeviceIconDark from '@/assets/images/recovery-device-icon-dark.svg?raw';
import RecoveryDeviceIcon from '@/assets/images/recovery-device-icon.svg?raw';
import RecoveryFileIconDark from '@/assets/images/recovery-file-icon-dark.svg?raw';
import RecoveryFileIcon from '@/assets/images/recovery-file-icon.svg?raw';
import RecoveryTrustIconDark from '@/assets/images/recovery-trusted-user-icon-dark.svg?raw';
import RecoveryTrustIcon from '@/assets/images/recovery-trusted-user-icon.svg?raw';
import { RecoveryMethod } from '@/views/devices/types';
import { IonIcon, IonRadio, IonRadioGroup, IonText, modalController } from '@ionic/vue';
import { add, link, logIn, phonePortraitOutline } from 'ionicons/icons';
import { MsImage, MsModal, MsModalResult, useTheme } from 'megashark-lib';
import { computed, ref } from 'vue';

enum RecoveryMethodStep {
  MethodChoice = 'method-choice',
  ConnectedDevice = 'connected-device',
}

const { isDarkTheme } = useTheme();
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
  gap: ms.spacing('gap-3xl');
}

.recovery-method-list {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
}

.recovery-method-item {
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-page');
  border: ms.border('thin') solid transparent;
  padding: ms.spacing('padding-3xl');
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
    gap: ms.spacing('gap-3xl');
    margin: 0;
    width: 100%;
  }

  // Hide default radio circle
  &::part(container) {
    display: none;
  }

  &:not(.recovery-method-item--selected):hover {
    border-color: ms.color('border-neutral-default-subtle-hover');
  }

  &--selected {
    border-color: ms.color('border-brand-default');
  }

  &__container {
    display: flex;
    gap: ms.spacing('gap-3xl');
  }

  &__image {
    width: fit-content;
    height: fit-content;
    flex-shrink: 0;
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);
    border-radius: ms.radius('md');
  }

  &__content {
    display: flex;
    width: 100%;
    text-wrap: wrap;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    flex: 1;
  }

  &__header {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
  }

  &__title {
    @include ms.font('heading-h5');
    color: ms.color('text-base-heading');
  }

  &__subtitle {
    @include ms.font('body-md-medium');
    color: ms.color('text-base-description');
  }

  &__recommended-badge {
    @include ms.font('label-sm-emphasis');
    padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
    border-radius: ms.radius('2xl');
    background: ms.color('surface-information-default-subtle');
    color: ms.color('text-information-default');
    box-shadow: ms.shadow('elevation-xs');
    white-space: nowrap;
  }
}

.method-step-list {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');

  &__item {
    background: ms.color('surface-base-page-secondary');
    display: flex;
    align-items: center;
    padding: ms.spacing('padding-3xl');
    border-radius: ms.radius('lg');
    gap: ms.spacing('gap-2xl');

    &-icon {
      width: 1.125rem;
      height: 1.125rem;
      flex-shrink: 0;
      padding: ms.spacing('padding-sm');
      color: ms.color('text-on-color-heading');
      background: ms.color('surface-neutral-default');
      border-radius: ms.radius('full');
    }

    &-description {
      @include ms.font('body-lg-medium');
      color: ms.color('text-neutral-default');
    }
  }
}
</style>
