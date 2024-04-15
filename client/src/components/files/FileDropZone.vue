<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="drop-zone"
    :class="isActive ? 'drop-zone-active' : 'drop-zone-inactive'"
    @drop.prevent="onDrop"
    @dragenter.prevent="onDragEnter()"
    @dragleave.prevent="onDragLeave()"
  >
    <slot />

    <div
      v-show="isActive && props.showDropMessage"
      class="drop-message"
    >
      <ms-image
        :image="AddDocument"
        class="restore-password-header-img"
      />
      <ion-label class="subtitles-normal">
        {{ $msTranslate('FoldersPage.ImportFile.dropInstructions') }}
      </ion-label>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AddDocument, MsImage } from '@/components/core';
import { FileImportTuple, getFilesFromDrop } from '@/components/files/utils';
import { FsPath } from '@/parsec';
import { IonLabel } from '@ionic/vue';
import { computed, onMounted, onUnmounted, ref } from 'vue';

defineExpose({
  reset,
});

const props = defineProps<{
  currentPath: FsPath;
  disabled?: boolean;
  showDropMessage?: boolean;
}>();

const emits = defineEmits<{
  (e: 'filesAdded', imports: FileImportTuple[]): void;
}>();

const dragEnterCount = ref(0);

const isActive = computed(() => {
  return !props.disabled && dragEnterCount.value > 0;
});

onMounted(() => {
  // Prevent the browser from handling those events itself
  document.body.addEventListener('dragenter', preventDefaults);
  document.body.addEventListener('dragover', preventDefaults);
  document.body.addEventListener('dragleave', preventDefaults);
  document.body.addEventListener('drop', preventDefaults);
});

onUnmounted(() => {
  // Restore the browser's event handling
  document.body.removeEventListener('dragenter', preventDefaults);
  document.body.removeEventListener('dragover', preventDefaults);
  document.body.removeEventListener('dragleave', preventDefaults);
  document.body.removeEventListener('drop', preventDefaults);
});

async function onDrop(event: DragEvent): Promise<void> {
  event.stopImmediatePropagation();
  dragEnterCount.value = 0;
  const imports = await getFilesFromDrop(event, props.currentPath);
  if (imports.length) {
    emits('filesAdded', imports);
  }
}

function preventDefaults(event: Event): void {
  event.preventDefault();
}

function onDragLeave(): void {
  if (dragEnterCount.value > 0) {
    dragEnterCount.value -= 1;
  }
}

function onDragEnter(): void {
  dragEnterCount.value += 1;
  props.showDropMessage === true;
}

function reset(): void {
  dragEnterCount.value = 0;
}
</script>

<style scoped lang="scss">
.drop-zone {
  width: 100%;
  height: 100%;
}

.drop-zone-active {
  position: relative;

  &::before {
    content: '';
    width: calc(100% + 2rem);
    height: calc(100% - 2.5rem);
    border-radius: var(--parsec-radius-6);
    outline: 2px dashed var(--parsec-color-light-primary-400);
    outline-offset: -2px;
    position: absolute;
    top: 1rem;
    left: -1rem;
    z-index: 2;
  }
}

.drop-message {
  position: absolute;
  left: 50%;
  bottom: 2em;
  transform: translate(-50%, -50%);
  width: fit-content;
  background-color: var(--parsec-color-light-primary-50);
  border-radius: var(--parsec-radius-6);
  padding: 0.75rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--parsec-color-light-secondary-text);
  z-index: 10;
}
</style>
