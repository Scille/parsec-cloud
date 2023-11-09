<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="card">
    <ion-icon
      class="icon"
      :icon="desktopOutline"
    />
    <ion-text class="text">
      {{ label }}
    </ion-text>

    <ion-chip
      class="badge"
      v-show="isCurrent"
      :outline="true"
      color="success"
    >
      {{ $t('DevicesPage.activeDeviceBadge') }}
    </ion-chip>
  </div>
</template>

<script setup lang="ts">
import { desktopOutline } from 'ionicons/icons';
import { IonChip, IonIcon, IonText } from '@ionic/vue';

defineProps<{
  label: string;
  isCurrent: boolean;
}>();
</script>

<style scoped lang="scss">
.card {
  background-color: var(--parsec-color-light-primary-30);
  height: 5em;
  padding: 1em;
  width: 100%;
  border-radius: 6px;
  display: flex;
  margin-top: 1em;
}

.text {
  padding-left: 1em;
  font-size: 120%;
  margin-top: auto;
  margin-bottom: auto;
}

.icon {
  height: 100%;
  width: 3em;
  color: var(--parsec-color-light-primary-800);
}

.badge {
  margin-left: auto;
  margin-top: auto;
  margin-bottom: auto;
}
</style>
