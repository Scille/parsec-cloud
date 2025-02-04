<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-processing">
    <progress-order
      :organization="props.organization"
    />
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, DataType, BmsOrganization, CustomOrderStatus } from '@/services/bms';
import { isDefaultOrganization } from '@/views/client-area/types';
import ProgressOrder from '@/components/client-area/ProgressOrder.vue';
import { ref, onMounted } from 'vue';

const customOrderStatus = ref<CustomOrderStatus>(CustomOrderStatus.Unknown);

const querying = ref(true);
const error = ref('');

const props = defineProps<{
  organization: BmsOrganization;
}>();

onMounted(async () => {
  if (isDefaultOrganization(props.organization)) {
    querying.value = true;
    return;
  }

  const orgStatusResponse = await BmsAccessInstance.get().getCustomOrderStatus(props.organization);

  if (!orgStatusResponse.isError && orgStatusResponse.data && orgStatusResponse.data.type === DataType.CustomOrderStatus) {
    customOrderStatus.value = orgStatusResponse.data.status;
  } else {
    error.value = 'clientArea.dashboard.processing.error.title';
  }

  querying.value = false;
});
</script>

<style scoped lang="scss">
.client-page-processing {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
