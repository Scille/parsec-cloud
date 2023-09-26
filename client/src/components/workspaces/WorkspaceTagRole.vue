<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-chip
      class="caption-caption tag"
      :class="role"
    >
      {{ translateWorkspaceRole(role) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { IonChip, IonLabel } from '@ionic/vue';
import { defineProps } from 'vue';
import { WorkspaceRole } from '@/parsec';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps<{
  role: WorkspaceRole,
}>();

function translateWorkspaceRole(role: WorkspaceRole): string {
  if (role === WorkspaceRole.Owner) {
    return t('WorkspacesPage.role.owner');
  } else if (role === WorkspaceRole.Manager) {
    return t('WorkspacesPage.role.manager');
  } else if (role === WorkspaceRole.Contributor) {
    return t('WorkspacesPage.role.contributor');
  } else if (role === WorkspaceRole.Reader) {
    return t('WorkspacesPage.role.reader');
  }
  return '';
}
</script>

<style scoped lang="scss">
</style>
