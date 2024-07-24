<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <h1>SUMMARY</h1>
    {{ organization }}
  </div>
</template>

<script setup lang="ts">
import { BmsOrganization } from '@/services/bms';

defineProps<{
  organization: BmsOrganization;
}>();
</script>

<style scoped lang="scss"></style>
