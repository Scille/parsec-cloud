<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="WorkspacesPage.workspaceHiddenModal.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: false,
        label: 'WorkspacesPage.workspaceHiddenModal.actionConfirm',
        onClick: confirmHidden,
      }"
      :cancel-button="{
        label: 'WorkspacesPage.workspaceHiddenModal.actionCancel',
        disabled: false,
      }"
    >
      <div class="workspace-hidden">
        <ms-rich-text
          class="workspace-hidden__description"
          :text="{ key: 'WorkspacesPage.workspaceHiddenModal.affectedWorkspaces', data: { workspace: workspaceName } }"
        />
      </div>
      <ms-checkbox
        label-placement="end"
        class="workspace-hidden__checkbox"
        v-model="skipWorkspaceHiddenWarning"
      >
        <ion-text>
          {{ $msTranslate('WorkspacesPage.workspaceHiddenModal.noReminder') }}
        </ion-text>
      </ms-checkbox>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { WorkspaceName } from '@/parsec';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsCheckbox, MsModal, MsModalResult, MsRichText } from 'megashark-lib';
import { ref } from 'vue';

defineProps<{
  workspaceName: WorkspaceName;
}>();

const skipWorkspaceHiddenWarning = ref(false);

async function confirmHidden(): Promise<boolean> {
  return await modalController.dismiss(
    {
      skipWorkspaceHiddenWarning: skipWorkspaceHiddenWarning.value,
    },
    MsModalResult.Confirm,
  );
}
</script>

<style scoped lang="scss">
.workspace-hidden {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  &__description {
    @include ms.font('body-lg-regular');

    color: ms.color('text-base-description');
  }

  &__checkbox {
    @include ms.font('body-md-regular');

    position: absolute;
    z-index: 30;
    bottom: 1.625rem;
    color: ms.color('text-neutral-default');
    padding: ms.spacing('padding-sm');
    border-radius: ms.radius('lg');
    transition: background-color 0.2s ease-in-out;

    &:hover {
      color: ms.color('text-neutral-default-hover');
      background-color: ms.color('surface-neutral-default-subtle-hover');
    }
  }
}
</style>
