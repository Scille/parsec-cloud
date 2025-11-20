<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="WorkspaceHiddenModal.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: workspace === undefined,
        label: 'WorkspaceHiddenModal.actionConfirm',
        onClick: confirmHidden,
      }"
      :cancel-button="{
        label: 'WorkspaceHiddenModal.actionCancel',
        disabled: false,
        onClick: dismissModal,
      }"
    >
      <div class="workspace-hidden-content">
        <ion-text class="workspace-hidden-title body">
          {{ $msTranslate('WorkspaceHiddenModal.affectedWorkspaces') }}
        </ion-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { WorkspaceInfo } from '@/parsec';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsModal, MsModalResult } from 'megashark-lib';
import { onMounted} from 'vue';

const props = defineProps<{
  workspace: WorkspaceInfo;
}>();

onMounted(async () => {});

async function confirmHidden(): Promise<boolean> {
  if (!props.workspace) {
    return false;
  }

  return await modalController.dismiss({ workspace: props.workspace }, MsModalResult.Confirm);
}

async function dismissModal(): Promise<boolean> {
  return await modalController.dismiss(undefined, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">

</style>
