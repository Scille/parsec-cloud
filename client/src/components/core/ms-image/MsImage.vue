<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template class="ms-image">
  <div
    v-if="isSvg(image)"
    class="svg-container"
    v-html="image"
  />
  <img
    v-else
    :src="image"
  />
</template>

<script setup lang="ts">
defineProps<{
  image: string;
}>();

function isSvg(imageData: string): boolean {
  return imageData.startsWith('<');
}
</script>

<style scoped lang="scss">
.svg-container {
  color: var(--fill-color, var(--parsec-color-light-primary-700));
  display: flex;
}
</style>
