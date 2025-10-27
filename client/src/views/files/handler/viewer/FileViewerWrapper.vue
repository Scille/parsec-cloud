<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- file-viewer wrapper -->
  <div
    class="file-viewer-wrapper"
    v-if="!error"
  >
    <slot name="viewer" />
  </div>

  <!-- file-viewer error -->
  <div
    class="file-viewer-error"
    v-if="error"
  >
    <ms-report-text
      class="document-error"
      :theme="MsReportTheme.Error"
      v-if="error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>
  </div>

  <!-- file-viewer bottombar -->
  <div
    class="file-viewer-bottombar"
    v-if="!error"
  >
    <slot name="controls" />
  </div>
</template>

<script setup lang="ts">
import { MsReportText, MsReportTheme } from 'megashark-lib';

defineProps<{
  error?: string;
}>();
</script>

<style scoped lang="scss">
.file-viewer-wrapper {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}

.file-viewer-error {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.file-viewer-bottombar {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1.5rem;
}
</style>
