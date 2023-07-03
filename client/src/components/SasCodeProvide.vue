<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container">
    {{ code }}
  </div>
</template>

<script setup lang="ts">
defineProps<{
  code: string
}>();
</script>

<style scoped lang="scss">
.container {
  margin: auto;
  width: 8em;
  height: 4em;
  background-color: var(--parsec-color-light-primary-30);
  font-weight: bold;
  font-size: 1.3em;
  text-align: center;
  padding-top: auto;
  padding-bottom: auto;
  color: var(--parsec-light-primary-600);
}
</style>
