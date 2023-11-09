<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('SettingsPage.pageTitle')"
    :close-button="{ visible: true }"
  >
    <settings-view />
  </ms-modal>
</template>

<script setup lang="ts">
import SettingsView from '@/views/settings/SettingsView.vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
</script>

<style lang="scss" scoped></style>
