<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="getStepTitle()"
      :close-button="{ visible: true }"
      :cancel-button="getPreviousButton()"
      :confirm-button="{
        label: getStepButtonLabel(),
        disabled: isConfirmButtonDisabled,
        onClick: submit,
      }"
    >
      <div
        class="modal-container"
        v-if="currentStep === UpdatePasswordStep.Password"
      >
        <div class="input-container">
          <ms-password-input
            v-model="passwordRef"
            @change="errors.password = ''"
            @on-enter-keyup="submit"
            :label="`${translationPrefix}.passwordStep.password`"
            ref="passwordInput"
          />
          <div
            class="form-error form-helperText body"
            v-if="errors.password"
          >
            {{ $msTranslate(errors.password) }}
          </div>
        </div>
      </div>
      <div
        class="modal-container"
        v-if="currentStep === UpdatePasswordStep.NewPassword"
      >
        <ms-choose-password-input
          :show-description="false"
          :inline="false"
          @keyup="onNewPasswordKeyup"
          @on-enter-keyup="submit"
          ref="choosePasswordInput"
        />

        <ms-report-text
          :theme="MsReportTheme.Error"
          v-if="errors.global"
        >
          {{ $msTranslate(errors.global) }}
        </ms-report-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { BmsAccessInstance } from '@/services/bms';
import { IonPage, modalController } from '@ionic/vue';
import {
  MsChoosePasswordInput,
  MsModal,
  MsModalResult,
  MsPasswordInput,
  MsReportText,
  MsReportTheme,
  Translatable,
  asyncComputed,
} from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const enum UpdatePasswordStep {
  Password,
  NewPassword,
}
const currentStep = ref<UpdatePasswordStep>(UpdatePasswordStep.Password);
const translationPrefix = 'clientArea.personalDataPage.modals.security';
const querying = ref(false);

const passwordRef = ref('');
const passwordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('passwordInput');
const choosePasswordInputRef = useTemplateRef<InstanceType<typeof MsChoosePasswordInput>>('choosePasswordInput');
const errors = ref({
  password: '',
  global: '',
});

const isConfirmButtonDisabled = asyncComputed(async (): Promise<boolean> => {
  return querying.value || !(await isFormValid());
});

onMounted(async () => {
  await passwordInputRef.value?.setFocus();
});

async function submit(): Promise<boolean> {
  if (!(await isFormValid())) {
    return false;
  }
  switch (currentStep.value) {
    case UpdatePasswordStep.Password:
      currentStep.value = UpdatePasswordStep.NewPassword;
      return false;
    case UpdatePasswordStep.NewPassword:
      return await updatePassword();
  }
}

async function updatePassword(): Promise<boolean> {
  try {
    querying.value = true;
    const response = await BmsAccessInstance.get().updateAuthentication(passwordRef.value, choosePasswordInputRef.value?.password ?? '');

    if (response.isError) {
      switch (response.status) {
        case 400:
          errors.value.global = `${translationPrefix}.newPasswordStep.invalidPassword`;
          break;
        case 403:
          errors.value.password = `${translationPrefix}.passwordStep.wrongPassword`;
          currentStep.value = UpdatePasswordStep.Password;
          break;
        default:
          errors.value.global = 'globalErrors.unexpected';
      }
      return false;
    }
    return await modalController.dismiss(null, MsModalResult.Confirm);
  } finally {
    querying.value = false;
  }
}

async function isFormValid(): Promise<boolean> {
  switch (currentStep.value) {
    case UpdatePasswordStep.Password:
      return !!passwordRef.value;
    case UpdatePasswordStep.NewPassword:
      return !!(choosePasswordInputRef.value && (await choosePasswordInputRef.value.areFieldsCorrect()) && arePasswordsDifferent());
  }
}

function arePasswordsDifferent(): boolean {
  return choosePasswordInputRef.value?.password !== passwordRef.value;
}

function getStepTitle(): Translatable {
  switch (currentStep.value) {
    case UpdatePasswordStep.Password:
      return `${translationPrefix}.passwordStep.title`;
    case UpdatePasswordStep.NewPassword:
      return `${translationPrefix}.newPasswordStep.title`;
  }
}

function getStepButtonLabel(): Translatable {
  switch (currentStep.value) {
    case UpdatePasswordStep.Password:
      return `${translationPrefix}.passwordStep.nextButton`;
    case UpdatePasswordStep.NewPassword:
      return `${translationPrefix}.newPasswordStep.nextButton`;
  }
}

function getPreviousButton():
  | {
      disabled: boolean;
      label?: Translatable;
      onClick?: () => Promise<boolean>;
      theme?: MsReportTheme;
    }
  | undefined {
  if (currentStep.value === UpdatePasswordStep.NewPassword) {
    return {
      disabled: false,
      label: `${translationPrefix}.newPasswordStep.previousButton`,
      onClick: onPreviousButtonClick,
    };
  }
}

async function onPreviousButtonClick(): Promise<boolean> {
  if (currentStep.value === UpdatePasswordStep.NewPassword) {
    currentStep.value = UpdatePasswordStep.Password;
  }
  return false;
}

function onNewPasswordKeyup(): void {
  errors.value.global = '';
  if (!arePasswordsDifferent()) {
    errors.value.global = `${translationPrefix}.newPasswordStep.newPasswordMustBeDifferent`;
  }
}
</script>

<style lang="scss" scoped>
.modal-container {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}
</style>
