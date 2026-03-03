<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div v-show="active">
    <ms-spinner />
  </div>
  <div
    v-for="result in searchResults"
    :key="result.stats.id"
  >
    {{ result.name }}
  </div>
</template>

<script setup lang="ts">
import { SearchResult } from '@/parsec';
import { MsSpinner } from 'megashark-lib';

defineProps<{
  searchResults: Array<SearchResult>;
  active: boolean;
}>();
</script>

<style scoped lang="scss"></style>
