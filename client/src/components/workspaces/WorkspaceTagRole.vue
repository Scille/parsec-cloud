<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-chip
      class="caption-caption tag"
      :class="role"
    >
      {{ translateWorkspaceRole($t, role) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { IonChip, IonLabel } from '@ionic/vue';
import { defineProps } from 'vue';
import { WorkspaceRole } from '@/parsec';
import { translateWorkspaceRole } from '@/common/translations';

defineProps<{
  role: WorkspaceRole,
}>();
</script>

<style scoped lang="scss">
</style>
