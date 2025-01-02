<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-controls-group>
    <file-controls-button :icon="scan" />
  </file-controls-group>
</template>

<script setup lang="ts">
import { scan } from 'ionicons/icons';
import { FileControlsButton, FileControlsGroup } from '@/components/viewers';
</script>

<style lang="scss" scoped></style>
