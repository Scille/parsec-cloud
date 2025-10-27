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
      ref="input"
    />
  </file-controls-group>
</template>

<script setup lang="ts">
import { FileControlsGroup, FileControlsInput } from '@/components/files/handler/viewer';
import { I18n } from 'megashark-lib';
import { computed, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  length: number;
  page?: number;
}>();

const emits = defineEmits<{
  (e: 'change', value: number): void;
  (e: 'update:page', value: number): void;
}>();

defineExpose({
  getCurrentPage,
});

const inputRef = useTemplateRef<InstanceType<typeof FileControlsInput>>('input');
const currentPage = ref('1');
const inputLengthStyle = computed(() => {
  return `${6 + props.length.toString().length * 1}rem`;
});

const pageWatchCancel = watch(
  () => props.page,
  (value) => {
    if (value === undefined || value === null || value < 1 || value > props.length || inputRef.value?.isEditing()) {
      return;
    }
    currentPage.value = value.toString();
  },
);

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

onUnmounted(() => {
  pageWatchCancel();
});
</script>

<style scoped lang="scss">
.page-number-input {
  width: v-bind(inputLengthStyle);
}
</style>
