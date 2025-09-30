<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="UsersPage.CreateUserInvitationModal.pageTitle"
    subtitle="CAN INSERT MULTIPLE EMAILS BY SEPARATING THEM WITH ;"
    :close-button="{ visible: true }"
    :cancel-button="{
      disabled: false,
      label: 'CANCEL',
    }"
    :confirm-button="{
      label: emails.length === 0 ? 'UsersPage.CreateUserInvitationModal.create' : 'INVITE ALL',
      disabled: !hasValidEmail,
      theme: MsReportTheme.Success,
      onClick: onConfirmClicked,
    }"
  >
    <ms-input
      @on-enter-keyup="onEnterPressed"
      @change="onInputChange"
      v-model="textModel"
      placeholder="UsersPage.CreateUserInvitationModal.placeholder"
      label="UsersPage.CreateUserInvitationModal.label"
    />
    <div
      v-for="email in emails"
      :key="email.address"
      class="email"
      :class="email.valid ? 'email-ok' : 'email-ko'"
    >
      {{ email }}
      <span
        class="delete-email"
        @click="removeEmail(email.address)"
      >
        DELETE
      </span>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import { modalController } from '@ionic/vue';
import { MsModalResult, MsInput, MsReportTheme, Validity, MsModal } from 'megashark-lib';
import { computed, ref } from 'vue';

const textModel = ref('');
const modelIsValid = ref(false);
const emails = ref<Array<{ address: string; valid: boolean }>>([]);
const hasValidEmail = computed(() => {
  return modelIsValid.value || emails.value.some((e) => e.valid);
});

async function onInputChange(): Promise<void> {
  modelIsValid.value = (await emailValidator(textModel.value)).validity === Validity.Valid;
}

async function onEnterPressed(): Promise<void> {
  const texts = textModel.value.split(';');

  for (const text of texts) {
    const addr = text.trim();
    const isValid = (await emailValidator(addr)).validity === Validity.Valid;
    if (emails.value.find((e) => e.address === addr) === undefined) {
      emails.value.push({ address: addr, valid: isValid });
    }
  }
  textModel.value = '';
}

async function removeEmail(addr: string): Promise<void> {
  const idx = emails.value.findIndex((e) => e.address === addr);
  if (idx !== -1) {
    emails.value.splice(idx, 1);
  }
}

async function onConfirmClicked(): Promise<boolean> {
  if (!hasValidEmail.value) {
    return false;
  }
  const addresses = new Array<string>(...emails.value.map((e) => (e.valid ? e.address : undefined)).filter((e) => e !== undefined));
  if (modelIsValid.value && addresses.find((e) => e === textModel.value) === undefined) {
    addresses.push(textModel.value);
  }
  return await modalController.dismiss({ emails: addresses }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.email {
  border: 2px solid salmon;
}

.email-ok {
  background-color: green;
}

.email-ko {
  background-color: red;
}
</style>
