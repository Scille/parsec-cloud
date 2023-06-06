<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-chip
      class="caption-caption tag"
      :class="role"
    >
      {{ translateRole(role) }}
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
  role: WorkspaceRole
}>();

function translateRole(role: WorkspaceRole): string {
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
.tag {
  padding: .25rem .5rem;
  height: auto;
}

.Owner {
  background-color: var(--parsec-color-light-primary-50);
  color: var(--parsec-color-light-primary-700);
}

.Manager {
  background-color: var(--parsec-color-tags-indigo100);
  color: var(--parsec-color-tags-indigo500);
}

.Contributor {
  background-color: var(--parsec-color-tags-green100);
  color: var(--parsec-color-tags-green500);
}

.Reader {
  background-color: var(--parsec-color-tags-orange100);
  color: var(--parsec-color-tags-orange500);
}
</style>
