<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('UsersPage.CreateUserInvitationModal.pageTitle')"
    :close-button-enabled="true"
    :cancel-button="{
      label: $t('UsersPage.CreateUserInvitationModal.cancel'),
      disabled: false,
      onClick: cancel
    }"
    :confirm-button="{
      label: $t('UsersPage.CreateUserInvitationModal.create'),
      disabled: emailValidator(email) !== Validity.Valid,
      onClick: confirm
    }"
  >
    <ms-input
      :label="$t('UsersPage.CreateUserInvitationModal.label')"
      :placeholder="$t('UsersPage.CreateUserInvitationModal.placeholder')"
      v-model="email"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import {
  modalController,
} from '@ionic/vue';
import { ref } from 'vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { MsModalResult } from '@/components/core/ms-modal/MsModal.vue';
import { emailValidator, Validity } from '@/common/validators';

const email = ref('');

function confirm(): Promise<boolean> {
  return modalController.dismiss(email.value, MsModalResult.Confirm);
}

function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
</style>
