<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-chip
      class="caption-caption tag"
      :class="workspacerole"
      v-if="workspacerole !== undefined"
    >
      {{ translateWorkspaceRole(workspacerole) }}
    </ion-chip>
    <ion-chip
      class="caption-caption tag"
      :class="orgarole"
      v-if="orgarole !== undefined"
    >
      {{ translateOrganizationRole(orgarole) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { IonChip, IonLabel } from '@ionic/vue';
import { defineProps } from 'vue';
import { OrganizationRole, WorkspaceRole } from '@/common/mocks';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps<{
  workspacerole: WorkspaceRole,
  orgarole: OrganizationRole
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

function translateOrganizationRole(orgarole: OrganizationRole): string {
  if (orgarole === OrganizationRole.Owner) {
    return t('UsersPage.role.owner');
  } else if (orgarole === OrganizationRole.Admin) {
    return t('UsersPage.role.admin');
  } else if (orgarole === OrganizationRole.Standard) {
    return t('UsersPage.role.standard');
  } else if (orgarole === OrganizationRole.External) {
    return t('UsersPage.role.external');
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
