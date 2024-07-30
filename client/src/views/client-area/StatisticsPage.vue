<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <h1>STATISTICS</h1>
    <div v-if="stats">
      {{ stats }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, DataType, BmsOrganization, OrganizationStatsResultData } from '@/services/bms';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  organization: BmsOrganization;
}>();

const stats = ref<OrganizationStatsResultData | undefined>(undefined);

onMounted(async () => {
  const response = await BmsAccessInstance.get().getOrganizationStats(props.organization.bmsId);

  if (!response.isError && response.data && response.data.type === DataType.OrganizationStats) {
    stats.value = response.data;
  }
});
</script>

<style scoped lang="scss"></style>
