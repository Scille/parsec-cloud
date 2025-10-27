<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
