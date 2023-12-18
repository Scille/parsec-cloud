<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-icon
    slot="end"
    :icon="informationCircle"
    class="icon disabled-icon"
    @click="openInformationTooltip($event, text)"
  />
</template>

<script setup lang="ts">
import { openInformationTooltip } from '@/components/core/ms-tooltip/utils';
import { IonIcon } from '@ionic/vue';
import { informationCircle } from 'ionicons/icons';

defineProps<{
  text: string;
}>();
</script>

<style scoped lang="scss">
.icon {
  margin: 0;
  margin-left: 1em;
}

.disabled-icon {
  pointer-events: initial;
  opacity: 0.8;
  --color: var(--parsec-color-light-secondary-grey);
  position: relative;
  &:hover {
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
