<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('WorkspacesPage.CreateWorkspaceModal.pageTitle')"
    :close-button-enabled="true"
    :confirm-button="{
      label: $t('WorkspacesPage.CreateWorkspaceModal.cancel'),
      disabled: !workspaceName,
      onClick: cancel
    }"
    :cancel-button="{
      label: $t('WorkspacesPage.CreateWorkspaceModal.create'),
      disabled: false,
      onClick: confirm
    }"
  >
    <ms-input
      :label="$t('WorkspacesPage.CreateWorkspaceModal.label')"
      :placeholder="$t('WorkspacesPage.CreateWorkspaceModal.placeholder')"
      name="createOrganization"
      v-model="workspaceName"
      type="url"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import {
  modalController,
} from '@ionic/vue';
import { ref } from 'vue';
import { ModalResultCode } from '@/common/constants';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';

const workspaceName = ref('');

function confirm(): Promise<boolean> {
  return modalController.dismiss(workspaceName.value, ModalResultCode.Confirm);
}

function cancel(): Promise<boolean> {
  return modalController.dismiss(workspaceName.value, ModalResultCode.Cancel);
}
</script>

<style scoped lang="scss">
</style>
