<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="'SettingsModal.pageTitle'"
    :close-button="{ visible: true }"
  >
    <settings-list />
  </ms-modal>
</template>

<script setup lang="ts">
import SettingsList from '@/components/settings/SettingsList.vue';
import { MsModal } from 'megashark-lib';
</script>

<style lang="scss" scoped></style>
