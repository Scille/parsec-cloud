<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="file-controls-group">
    <slot />
  </div>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss">
.file-controls-group {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1.5rem;

  @include ms.responsive-breakpoint('sm') {
    gap: 1rem;
  }
}
</style>
