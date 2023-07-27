<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-chip
      class="caption-caption tag"
      :class="workspacerole"
    >
      {{ translateWorkspaceRole(workspacerole) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { IonChip, IonLabel } from '@ionic/vue';
import { defineProps } from 'vue';
import { WorkspaceRole } from '@/common/mocks';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps<{
  workspacerole: WorkspaceRole,
}>();

function translateWorkspaceRole(workspacerole: WorkspaceRole): string {
  if (workspacerole === WorkspaceRole.Owner) {
    return t('WorkspacesPage.role.owner');
  } else if (workspacerole === WorkspaceRole.Manager) {
    return t('WorkspacesPage.role.manager');
  } else if (workspacerole === WorkspaceRole.Contributor) {
    return t('WorkspacesPage.role.contributor');
  } else if (workspacerole === WorkspaceRole.Reader) {
    return t('WorkspacesPage.role.reader');
  }
  return '';
}
</script>

<style scoped lang="scss">
.tag {
  padding: .25rem .5rem;
  height: auto;
}
</style>
