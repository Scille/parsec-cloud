<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="clientArea.personalDataPage.modals.authentication.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: 'clientArea.personalDataPage.modals.authentication.nextButton',
        disabled: !isFormValid(),
        onClick: submit,
      }"
    >
      <div class="modal-container">
        <ms-input
          type="email"
          v-model="newEmailRef"
          @change="error = ''"
          @on-enter-keyup="submit"
          label="clientArea.personalDataPage.modals.authentication.newEmail"
          :validator="newEmailValidator"
          ref="newEmailInput"
        />
        <ms-password-input
          v-model="passwordRef"
          @change="error = ''"
          @on-enter-keyup="submit"
          label="clientArea.personalDataPage.modals.authentication.password"
        />
        <ms-report-text
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
import { MsModal, MsInput, MsModalResult, MsPasswordInput, I18n, MsReportTheme, MsReportText, Validity, IValidator } from 'megashark-lib';
import { IonPage, modalController } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { BmsAccessInstance } from '@/services/bms';
import { longLocaleCodeToShort } from '@/services/translation';
import { emailValidator } from '@/common/validators';

const props = defineProps<{
  email: string;
}>();

const newEmailRef = ref('');
const passwordRef = ref('');
const error = ref('');
const newEmailInput = ref();

const newEmailValidator: IValidator = async function (value: string) {
  const result = await emailValidator(value);
  if (result.validity === Validity.Valid && value === props.email) {
    return { validity: Validity.Invalid, reason: 'clientArea.personalDataPage.modals.authentication.newEmailMustBeDifferent' };
  }
  return result;
};

onMounted(async () => {
  await newEmailInput.value.setFocus();
});

async function submit(): Promise<boolean> {
  if (!isFormValid()) {
    return false;
  }
  const response = await BmsAccessInstance.get().updateEmail(newEmailRef.value, passwordRef.value, longLocaleCodeToShort(I18n.getLocale()));

  if (response.isError) {
    error.value = 'globalErrors.unexpected';
    switch (response.status) {
      case 400:
        error.value = 'validators.userInfo.email';
        break;
      case 403:
        error.value = 'clientArea.personalDataPage.modals.authentication.wrongPassword';
        break;
    }
    return false;
  }
  return await modalController.dismiss({}, MsModalResult.Confirm);
}

function isFormValid(): boolean {
  return !!passwordRef.value && newEmailInput.value.validity === Validity.Valid && newEmailRef.value !== props.email;
}
</script>

<style lang="scss" scoped>
.modal-container {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}
</style>
