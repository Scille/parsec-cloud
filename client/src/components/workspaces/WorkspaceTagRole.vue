<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-text class="button-small label-role-text">
      {{ $msTranslate(getWorkspaceRoleTranslationKey(role).label) }}
    </ion-text>
  </ion-label>
</template>

<script setup lang="ts">
import { WorkspaceRole } from '@/parsec';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonText, IonLabel } from '@ionic/vue';

defineProps<{
  role: WorkspaceRole | null;
}>();
</script>

<style scoped lang="scss">
.label-role {
  color: var(--parsec-color-light-secondary-text);

  &-text {
    background-color: none;
    padding: 0.25rem 0.5rem;
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
