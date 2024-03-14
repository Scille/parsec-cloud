<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-status status">
    <ion-chip
      class="button-small tag-status"
      :class="{ revoked: revoked, active: !revoked }"
    >
      {{ revoked ? $t('UsersPage.status.revoked') : $t('UsersPage.status.active') }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { IonChip, IonLabel } from '@ionic/vue';

defineProps<{
  revoked: boolean;
}>();
</script>

<style scoped lang="scss">
.label-status {
  margin: 0;
}

.tag-status {
  padding: 0.125rem 0.5rem;
  margin: 0;
  height: fit-content;
  border-radius: var(--parsec-radius-6);
  min-height: 0;
  width: 5.5rem;
  display: flex;
  justify-content: center;
}

.revoked {
  background-color: var(--parsec-color-light-danger-100);
  color: var(--parsec-color-light-danger-500);
}

.active {
  background-color: var(--parsec-color-light-info-100);
  color: var(--parsec-color-light-info-500);
}
</style>
