<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <h1>CONTRACTS</h1>
  </div>
</template>

<script setup lang="ts">
import { BmsOrganization } from '@/services/bms';
import { onMounted } from 'vue';

defineProps<{
  organization: BmsOrganization;
}>();

onMounted(async () => {});
</script>

<style scoped lang="scss"></style>
