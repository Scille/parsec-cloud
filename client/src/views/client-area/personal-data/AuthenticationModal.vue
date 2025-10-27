<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="labels.title"
      :subtitle="labels.subtitle"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: labels.button,
        disabled: !canProgress || querying,
        onClick: submit,
      }"
    >
      <div class="modal-container">
        <div
          class="email-container"
          v-show="step === Steps.NewEmail"
        >
          <ms-input
            type="email"
            v-model="newEmailRef"
            @change="error = ''"
            @on-enter-keyup="submit"
            :label="`${translationPrefix}.newEmail`"
            :validator="newEmailValidator"
            ref="newEmailInput"
          />
        </div>
        <div
          class="password-container"
          v-show="step === Steps.Password"
        >
          <ms-password-input
            v-model="passwordRef"
            @change="error = ''"
            @on-enter-keyup="submit"
            :label="`${translationPrefix}.password`"
            ref="passwordInput"
          />
        </div>
        <div
          class="code-container"
          v-show="step === Steps.Code"
        >
          <ms-code-validation-input
            :code-length="6"
            @code-complete="onCodeComplete"
            ref="codeValidationInput"
          />
          <div class="bottomtext">
            <ion-text
              button
              @click="resendCode"
              :disabled="resendDisabled"
              class="send-code subtitles-sm"
            >
              {{ $msTranslate(`${translationPrefix}.resend`) }}
            </ion-text>
            <ion-icon
              :icon="checkmark"
              v-show="resendDisabled && resendOk"
            />
          </div>
        </div>
        <ms-report-text
          class="change-password-error"
          :theme="MsReportTheme.Error"
          v-if="error"
        >
          {{ $msTranslate(error) }}
        </ms-report-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import { BmsAccessInstance, BmsLang } from '@/services/bms';
import { longLocaleCodeToShort } from '@/services/translation';
import { IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import { checkmark } from 'ionicons/icons';
import {
  asyncComputed,
  I18n,
  IValidator,
  MsCodeValidationInput,
  MsInput,
  MsModal,
  MsModalResult,
  MsPasswordInput,
  MsReportText,
  MsReportTheme,
  Validity,
} from 'megashark-lib';
import { onMounted, Ref, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  email: string;
}>();

enum Steps {
  NewEmail = 'new-email',
  Password = 'password',
  Code = 'code',
}

const step = ref(Steps.NewEmail);
const translationPrefix = 'clientArea.personalDataPage.modals.authentication';
const newEmailRef = ref('');
const passwordRef = ref('');
const error = ref('');
const newEmailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('newEmailInput');
const passwordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('passwordInput');
const resendDisabled = ref(false);
const resendOk = ref(false);
const validationCode: Ref<Array<string>> = ref([]);
const querying = ref(false);
const isInit = ref(false);
const labels = ref({ title: '', subtitle: '', button: '' });
const codeSent = ref(false);
const codeValidationInputRef = useTemplateRef<InstanceType<typeof MsCodeValidationInput>>('codeValidationInput');

const canProgress = asyncComputed(async () => {
  if (!isInit.value) {
    return false;
  }
  switch (step.value) {
    case Steps.NewEmail:
      return newEmailInputRef.value?.validity === Validity.Valid;
    case Steps.Password:
      return passwordRef.value.length > 0;
    case Steps.Code:
      return validationCode.value.length > 0;
  }
});

const newEmailValidator: IValidator = async function (value: string) {
  const result = await emailValidator(value);
  if (result.validity === Validity.Valid && value === props.email) {
    return { validity: Validity.Invalid, reason: `${translationPrefix}.newEmailMustBeDifferent` };
  }
  return result;
};

onMounted(async () => {
  await switchStep(Steps.NewEmail);
  isInit.value = true;
});

async function submit(): Promise<boolean> {
  querying.value = true;
  if (step.value === Steps.NewEmail) {
    await switchStep(Steps.Password);
  } else if (step.value === Steps.Password) {
    if (!codeSent.value) {
      const response = await BmsAccessInstance.get().updateEmailSendCode(
        newEmailRef.value,
        longLocaleCodeToShort(I18n.getLocale()) as BmsLang,
      );
      if (response.isError) {
        error.value = 'globalErrors.unexpected';
      } else {
        codeSent.value = true;
      }
    }
    if (codeSent.value) {
      await switchStep(Steps.Code);
    }
  } else if (step.value === Steps.Code) {
    const response = await BmsAccessInstance.get().updateEmail(
      newEmailRef.value,
      passwordRef.value,
      validationCode.value.join(''),
      longLocaleCodeToShort(I18n.getLocale()) as BmsLang,
    );
    if (response.isError) {
      error.value = 'globalErrors.unexpected';
      switch (response.status) {
        case 400:
          if (response.errors && response.errors.length) {
            switch (response.errors[0].code) {
              case 'EMAIL_ALREADY_VALIDATED':
                error.value = `${translationPrefix}.errors.emailAlreadyUsed`;
                break;
              case 'EMAIL_VALIDATION_CODE_TRIES_EXCEEDED':
                error.value = `${translationPrefix}.errors.tooManyTries`;
                break;
              case 'EMAIL_VALIDATION_INVALID_CODE':
                error.value = `${translationPrefix}.errors.invalidCode`;
                break;
            }
          }
          break;
        case 403:
          error.value = `${translationPrefix}.errors.wrongPassword`;
          switchStep(Steps.Password);
          break;
        default:
          error.value = 'globalErrors.unexpected';
      }
    } else {
      await modalController.dismiss(null, MsModalResult.Confirm);
    }
  }
  querying.value = false;
  return true;
}

async function switchStep(newStep: Steps): Promise<void> {
  step.value = newStep;
  switch (newStep) {
    case Steps.NewEmail:
      await newEmailInputRef.value?.setFocus();
      labels.value = {
        title: `${translationPrefix}.emailTitle`,
        subtitle: `${translationPrefix}.emailSubtitle`,
        button: `${translationPrefix}.continue`,
      };
      break;
    case Steps.Password:
      await passwordInputRef.value?.setFocus();
      labels.value = {
        title: `${translationPrefix}.passwordTitle`,
        subtitle: '',
        button: `${translationPrefix}.continue`,
      };
      break;
    case Steps.Code:
      // Set focus doesn't work, no idea why,
      // keeping it anyway in case it's fixed in megashark-lib
      await codeValidationInputRef.value?.setFocus();
      labels.value = {
        title: `${translationPrefix}.validateTitle`,
        subtitle: `${translationPrefix}.validateSubtitle`,
        button: `${translationPrefix}.validate`,
      };
      break;
  }
}

async function resendCode(): Promise<void> {
  resendDisabled.value = true;

  const response = await BmsAccessInstance.get().updateEmailSendCode(newEmailRef.value, longLocaleCodeToShort(I18n.getLocale()) as BmsLang);
  if (response.isError) {
    error.value = `${translationPrefix}.errors.resendFailed`;
  }
  resendOk.value = !response.isError;

  setTimeout(() => {
    resendDisabled.value = false;
  }, 5000);
}

async function onCodeComplete(code: Array<string>): Promise<void> {
  validationCode.value = code;
}
</script>

<style lang="scss" scoped>
.modal-container {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;

  .send-code {
    margin-left: auto;
    color: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-6);
    padding: 0.125rem 0.5rem;

    &:hover {
      cursor: pointer;
      background: var(--parsec-color-light-secondary-premiere);
    }

    &[disabled='true'] {
      pointer-events: none;
      color: var(--parsec-color-light-secondary-light);
    }
  }
}
</style>
