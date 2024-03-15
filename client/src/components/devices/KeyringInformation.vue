<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="keyring-info">
    <ms-report-text :theme="MsReportTheme.Info">
      {{ translate('Authentication.keyringInfo') }}
    </ms-report-text>
  </div>
</template>

<script setup lang="ts">
import { MsReportText } from '@/components/core/ms-text';
import { MsReportTheme } from '@/components/core/ms-types';
import { translate } from '@/services/translation';
</script>

<style scoped lang="scss">
.keyring-info {
  margin-top: 1rem;
}
</style>
