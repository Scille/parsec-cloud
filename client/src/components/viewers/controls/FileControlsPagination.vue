<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-controls-group class="file-controls-pagination">
    <file-controls-input
      class="page-number-input"
      v-model="currentPage"
      @on-submitted-value="onChange"
      :restrict-change="validatePageNumber"
      prefix="fileViewers.controls.pagination.page"
      :suffix="I18n.valueAsTranslatable(`/ ${length}`)"
      @keydown.up.prevent="next"
      @keydown.right.prevent="next"
      @keydown.down.prevent="previous"
      @keydown.left.prevent="previous"
      :maxlength="String(length).length"
      ref="inputRef"
    />
  </file-controls-group>
</template>

<script setup lang="ts">
import { I18n } from 'megashark-lib';
import { ref } from 'vue';
import { FileControlsGroup, FileControlsInput } from '@/components/viewers';

const props = defineProps<{
  length: number;
}>();

const emits = defineEmits<{
  (e: 'change', value: number): void;
}>();

defineExpose({
  getCurrentPage,
});

const currentPage = ref('1');

function getCurrentPage(): number {
  return Number(currentPage.value);
}

function setPage(pageNumber: string): void {
  currentPage.value = pageNumber;
  emits('change', Number(currentPage.value));
}

function previous(): void {
  const page = Number(currentPage.value);
  if (page > 1) {
    setPage((page - 1).toString());
  }
}

function next(): void {
  const page = Number(currentPage.value);
  if (page < props.length) {
    setPage((page + 1).toString());
  }
}

function onChange(value: string): void {
  setPage(value);
}

async function validatePageNumber(value: string): Promise<string> {
  const level = parseInt(value);
  if (level < 1 || level > props.length) {
    return '';
  }
  return value;
}
</script>

<style scoped lang="scss">
.page-number-input {
  width: 6rem;
}
</style>
