<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="WorkspacesPage.workspaceHiddenModal.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: workspaceName === undefined,
        label: 'WorkspacesPage.workspaceHiddenModal.actionConfirm',
        onClick: confirmHidden,
      }"
      :cancel-button="{
        label: 'WorkspacesPage.workspaceHiddenModal.actionCancel',
        disabled: false,
        onClick: cancel,
      }"
    >
      <div class="workspace-hidden">
        <ion-text class="workspace-hidden__description body-lg">
          <i18n-t
            keypath="WorkspacesPage.workspaceHiddenModal.affectedWorkspaces"
            scope="global"
          >
            <template #workspace>
              <strong>{{ workspaceName ? workspaceName : '' }}</strong>
            </template>
          </i18n-t>
        </ion-text>
        <ms-report-text :theme="MsReportTheme.Info">
          {{ $msTranslate('WorkspacesPage.workspaceHiddenModal.info') }}
        </ms-report-text>
      </div>
      <ms-checkbox
        label-placement="end"
        class="workspace-hidden__checkbox body"
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
import { MsCheckbox, MsModal, MsModalResult, MsReportText, MsReportTheme } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  workspaceName: WorkspaceName;
}>();

const skipWorkspaceHiddenWarning = ref(false);

async function confirmHidden(): Promise<boolean> {
  if (!props.workspaceName) {
    return false;
  }

  return await modalController.dismiss(
    {
      workspaceName: props.workspaceName,
      skipWorkspaceHiddenWarning: skipWorkspaceHiddenWarning.value,
    },
    MsModalResult.Confirm,
  );
}

async function cancel(): Promise<boolean> {
  return await modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.workspace-hidden {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &__description {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &__checkbox {
    position: absolute;
    bottom: 2.25rem;
    color: var(--parsec-color-light-secondary-soft-text);
    padding: 0.25rem 0.25rem;
    border-radius: var(--parsec-radius-8);
    transition: background-color 0.2s ease-in-out;

    &:hover {
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-premiere);
    }
  }
}
</style>
