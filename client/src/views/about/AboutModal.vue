<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="HeaderPage.titles.about"
    :close-button="{ visible: true }"
  >
    <about-view />
  </ms-modal>
</template>

<script setup lang="ts">
import { MsModal } from 'megashark-lib';
import AboutView from '@/views/about/AboutView.vue';
</script>

<style lang="scss" scoped></style>
