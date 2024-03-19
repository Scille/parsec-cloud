<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="keyring-info">
    <ms-report-text :theme="MsReportTheme.Info">
      {{ $msTranslate('Authentication.keyringInfo') }}
    </ms-report-text>
  </div>
</template>

<script setup lang="ts">
import { MsReportText } from '@/components/core/ms-text';
import { MsReportTheme } from '@/components/core/ms-types';
</script>

<style scoped lang="scss">
.keyring-info {
  margin-top: 1rem;
}
</style>
