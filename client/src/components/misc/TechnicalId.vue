<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-text class="label-id body-sm">
    {{ $msTranslate('common.technicalId') }}
    <span class="label-id__info">{{ id }}</span>
  </ion-text>
</template>

<script setup lang="ts">
import { DeviceID, FileID, UserID, WorkspaceID } from '@/parsec';
import { IonText } from '@ionic/vue';

defineProps<{
  id: UserID | FileID | WorkspaceID | DeviceID;
}>();
</script>

<style scoped lang="scss">
.label-id {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
