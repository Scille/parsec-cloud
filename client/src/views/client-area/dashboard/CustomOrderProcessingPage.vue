<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-processing">
    <ms-report-text :theme="MsReportTheme.Info">
      {{ $msTranslate('clientArea.dashboard.processing') }}
    </ms-report-text>
  </div>
</template>

<script setup lang="ts">
import { MsReportText, MsReportTheme } from 'megashark-lib';
</script>

<style scoped lang="scss">
.client-page-processing {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
