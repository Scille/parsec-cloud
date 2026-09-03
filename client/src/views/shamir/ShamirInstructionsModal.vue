<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal
    title="ImportRecoveryDevicePage.modal.shamir.instructions.title"
    :close-button="{ visible: true }"
  >
    <div class="shamir-instructions">
      <div class="shamir-instructions-progress">
        <div
          v-for="barStep in [Steps.GetLinks, Steps.ExchangeCodes, Steps.RecoverAccount]"
          :key="barStep"
          class="shamir-instructions-progress__bar"
          :class="{ 'shamir-instructions-progress__bar--done': barStep <= step }"
        />
      </div>

      <div class="instruction-step">
        <div class="instruction-step__icon-container">
          <ion-icon
            class="instruction-step__icon"
            :icon="currentStep.icon"
          />
          <div class="instruction-step__divider" />
        </div>
        <div class="instruction-step__content">
          <ion-text class="instruction-step__title">
            {{ $msTranslate(currentStep.title) }}
          </ion-text>
          <ion-text class="instruction-step__description">
            {{ $msTranslate(currentStep.description) }}
          </ion-text>
        </div>
      </div>

      <div class="shamir-instructions-footer">
        <div class="shamir-instructions-footer__actions">
          <ion-button
            class="shamir-instructions-footer__previous color-secondary"
            fill="clear"
            @click="onPreviousClicked"
          >
            {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.instructions.previous') }}
          </ion-button>
          <ion-button
            class="shamir-instructions-footer__next"
            @click="onNextClicked"
          >
            {{ $msTranslate(nextButtonLabel) }}
          </ion-button>
        </div>
      </div>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle, hourglass, paperPlane } from 'ionicons/icons';
import { MsModal, MsModalResult, Translatable } from 'megashark-lib';
import { computed, ref } from 'vue';

enum Steps {
  GetLinks = 0,
  ExchangeCodes = 1,
  RecoverAccount = 2,
}

interface InstructionStep {
  icon: string;
  title: Translatable;
  description: Translatable;
}

const step = ref(Steps.GetLinks);
const currentStep = computed<InstructionStep>(() => {
  switch (step.value) {
    case Steps.GetLinks:
      return {
        icon: paperPlane,
        title: 'ImportRecoveryDevicePage.modal.shamir.instructions.steps.getLinks.title',
        description: 'ImportRecoveryDevicePage.modal.shamir.instructions.steps.getLinks.description',
      };
    case Steps.ExchangeCodes:
      return {
        icon: hourglass,
        title: 'ImportRecoveryDevicePage.modal.shamir.instructions.steps.exchangeCodes.title',
        description: 'ImportRecoveryDevicePage.modal.shamir.instructions.steps.exchangeCodes.description',
      };
    case Steps.RecoverAccount:
      return {
        icon: checkmarkCircle,
        title: 'ImportRecoveryDevicePage.modal.shamir.instructions.steps.recoverAccount.title',
        description: 'ImportRecoveryDevicePage.modal.shamir.instructions.steps.recoverAccount.description',
      };
    default:
      return {
        icon: '',
        title: '',
        description: '',
      };
  }
});
const isLastStep = computed(() => step.value === Steps.RecoverAccount);
const nextButtonLabel = computed(() =>
  isLastStep.value
    ? 'ImportRecoveryDevicePage.modal.shamir.instructions.understood'
    : 'ImportRecoveryDevicePage.modal.shamir.instructions.next',
);

async function onPreviousClicked(): Promise<void> {
  if (step.value === Steps.GetLinks) {
    await modalController.dismiss(undefined, MsModalResult.Cancel);
  } else {
    step.value -= 1;
  }
}

async function onNextClicked(): Promise<void> {
  if (isLastStep.value) {
    await modalController.dismiss(undefined, MsModalResult.Confirm);
  } else {
    step.value += 1;
  }
}
</script>

<style scoped lang="scss">
.shamir-instructions {
  display: flex;
  flex-direction: column;
}

.shamir-instructions-progress {
  display: flex;
  gap: ms.spacing('gap-lg');
  margin-bottom: 1.5rem;

  &__bar {
    flex: 1;
    height: 0.25rem;
    border-radius: ms.radius('6xl');
    background: ms.color('border-base-default');
    transition: background 150ms ease-in-out;

    &--done {
      background: ms.color('surface-brand-default');
    }
  }
}

.instruction-step {
  display: flex;
  gap: 0.9375rem;
  padding: ms.spacing('padding-3xl');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-default-secondary');

  &__icon-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: ms.spacing('gap-lg');
    flex-shrink: 0;
  }

  &__icon {
    width: 1.125rem;
    height: 1.125rem;
    padding: 0.4375rem;
    border-radius: ms.radius('full');
    color: ms.color('icon-brand-default');
    background: ms.color('surface-brand-default-subtle-hover');
  }

  &__divider {
    width: 1px;
    height: 2.0625rem;
    background: ms.color('border-base-default');
  }

  &__content {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    padding-top: ms.spacing('padding-md');
  }

  &__title {
    @include ms.font('heading-h5');
    color: ms.color('text-base-heading');
  }

  &__description {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-description');
  }
}

.shamir-instructions-footer {
  display: flex;
  justify-content: flex-end;
  margin-top: 2rem;

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    margin-bottom: 1.5rem;
  }

  &__actions {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');

    @include ms.responsive-breakpoint('sm') {
      flex-direction: column;
      align-items: stretch;

      ion-button {
        margin: 0;
      }
    }
  }
}
</style>
