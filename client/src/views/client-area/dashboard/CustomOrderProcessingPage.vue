<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <progress-order
    :organization="props.organization"
    class="client-page-processing"
  />
</template>

<script setup lang="ts">
import { BmsOrganization } from '@/services/bms';
import ProgressOrder from '@/components/client-area/ProgressOrder.vue';

const props = defineProps<{
  organization: BmsOrganization;
}>();
</script>

<style scoped lang="scss">
// eslint-disable-next-line vue-scoped-css/no-unused-selector
.client-page-processing.process-container {
  box-shadow: var(--parsec-shadow-light);
}
</style>
