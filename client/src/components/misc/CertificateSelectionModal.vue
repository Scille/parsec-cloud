<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="DOES NOT MATTER WILL GET DELETED"
    :close-button="{ visible: true }"
  >
    <div>
      <certificate-selection />
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import CertificateSelection from '@/components/misc/CertificateSelection.vue';
import { MsModal } from 'megashark-lib';
</script>

<style scoped lang="scss"></style>
