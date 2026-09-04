<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-label class="label-role">
    <ion-text class="label-role-text">
      {{ $msTranslate(getWorkspaceRoleTranslationKey(role).label) }}
    </ion-text>
  </ion-label>
</template>

<script setup lang="ts">
import { WorkspaceRole } from '@/parsec';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonLabel, IonText } from '@ionic/vue';

defineProps<{
  role: WorkspaceRole | null;
}>();
</script>

<style scoped lang="scss">
.label-role {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-sm');
  width: fit-content;
  padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
  background: ms.color('surface-base-default');
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  border-radius: ms.radius('2xl');
  box-shadow: ms.shadow('elevation-xs');

  &-text {
    @include ms.font('label-sm-emphasis');
    color: ms.color('text-base-label');
  }
}
</style>
