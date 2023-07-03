<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div>
    <ion-chip
      v-for="(title, index) in titles"
      :key="title"
      :color="getTitleColor(index)"
    >
      {{ title }}
    </ion-chip>
  </div>
</template>

<script setup lang="ts">
import { IonChip } from '@ionic/vue';

const props = defineProps<{
  titles: string[],
  currentIndex: number
}>();

function getTitleColor(index: number): string {
  if (index === props.currentIndex) {
    return 'primary';
  } else if (index < props.currentIndex) {
    return 'medium';
  }
  return 'secondary';
}
</script>

<style scoped lang="scss">

</style>
