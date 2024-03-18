<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container caption-code">
    {{ code }}
  </div>
</template>

<script setup lang="ts">
defineProps<{
  code: string;
}>();
</script>

<style scoped lang="scss">
.container {
  margin: auto;
  width: 8em;
  height: 4em;
  background-color: var(--parsec-color-light-secondary-background);
  color: var(--parsec-color-light-primary-600);
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--parsec-radius-8);
}
</style>
