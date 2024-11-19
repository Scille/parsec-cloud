<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- file-viewer wrapper -->
  <div class="file-viewer-wrapper">
    <slot name="viewer" />
  </div>

  <!-- file-viewer bottombar -->
  <div class="file-viewer-bottombar">
    <slot name="controls" />
  </div>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss">
.file-viewer-wrapper {
  height: 100%;
  height: -webkit-fill-available;
  height: -moz-available;
  height: stretch;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}

.file-viewer-bottombar {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1.5rem;
}
</style>
