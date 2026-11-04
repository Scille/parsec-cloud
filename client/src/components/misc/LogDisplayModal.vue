<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal
    title="LogDisplayModal.title"
    :close-button="{ visible: true }"
  >
    <log-display />
  </ms-modal>
</template>

<script setup lang="ts">
import LogDisplay from '@/components/misc/LogDisplay.vue';
import { MsModal } from 'megashark-lib';
</script>

<style scoped lang="scss"></style>
