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
  if (Number(pageNumber) < 1 || Number(pageNumber) > props.length) {
    return;
  }
  currentPage.value = pageNumber;
  emits('change', Number(currentPage.value));
}

function previous(): void {
  setPage((Number(currentPage.value) - 1).toString());
}

function next(): void {
  setPage((Number(currentPage.value) + 1).toString());
}

function onChange(value: string): void {
  setPage(value);
}

async function validatePageNumber(value: string): Promise<string> {
  const pageNumber = parseInt(value);
  if (pageNumber < 1 || pageNumber > props.length) {
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
