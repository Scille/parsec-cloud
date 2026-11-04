<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal
    title="HeaderPage.titles.about"
    :close-button="{ visible: true }"
  >
    <about-view />
  </ms-modal>
</template>

<script setup lang="ts">
import AboutView from '@/views/about/AboutView.vue';
import { MsModal } from 'megashark-lib';
</script>

<style lang="scss" scoped></style>
