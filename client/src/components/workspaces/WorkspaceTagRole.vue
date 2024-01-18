<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-chip
      class="caption-caption tag"
      :class="getClassForProfile(role)"
    >
      {{ translateWorkspaceRole(role).label }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { WorkspaceRole } from '@/parsec';
import { translateWorkspaceRole } from '@/services/translation';
import { IonChip, IonLabel } from '@ionic/vue';

defineProps<{
  role: WorkspaceRole;
}>();

function getClassForProfile(role: WorkspaceRole): string {
  switch (role) {
    case WorkspaceRole.Contributor:
      return 'contributor';
    case WorkspaceRole.Manager:
      return 'manager';
    case WorkspaceRole.Owner:
      return 'owner';
    case WorkspaceRole.Reader:
      return 'reader';
    default:
      return '';
  }
}
</script>

<style scoped lang="scss"></style>
