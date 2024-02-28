<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-status status">
    <ion-chip
      class="caption-caption tag"
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
.revoked {
  background-color: red;
}

.active {
  background-color: blue;
}
</style>
