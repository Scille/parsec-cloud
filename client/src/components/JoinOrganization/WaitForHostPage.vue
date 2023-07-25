<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container">
    <informative-text
      :icon="caretForward"
      :text="$t('JoinOrganization.instructions.start.first')"
    />
    <informative-text
      :icon="caretForward"
      :text="$t('JoinOrganization.instructions.start.second')"
    />
  </div>
</template>

<script setup lang="ts">
import {
  caretForward,
} from 'ionicons/icons';
import InformativeText from '@/components/InformativeText.vue';

defineProps<{
  orgName: string
}>();
</script>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
</style>
