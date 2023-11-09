<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="drop-zone"
    :class="isActive ? 'drop-zone-active' : 'drop-zone-inactive'"
    @drop.prevent="onDrop"
    @dragenter.prevent="isActive = true"
    @dragleave.prevent="setInactive()"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const emits = defineEmits<{
  (e: 'filesDrop', entries: FileSystemEntry[]): void;
}>();

const isActive = ref(false);

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

function onDrop(event: DragEvent): void {
  isActive.value = false;
  if (event.dataTransfer) {
    const entries: FileSystemEntry[] = [];
    // May have to use event.dataTransfer.files instead of items.
    // .files gets us a FileList, the API is not as good
    // but it's the same used in <input> and it should be compatible with
    // Cypress.
    for (let i = 0; i < event.dataTransfer.items.length; i++) {
      const entry = event.dataTransfer.items[i].webkitGetAsEntry();
      if (entry) {
        entries.push(entry);
      }
    }
    if (entries.length) {
      emits('filesDrop', entries);
    }
  }
}

function preventDefaults(event: Event): void {
  event.preventDefault();
}

// Helps prevent the state not changing rapidly enough when hovering too fast
// in and out of the drop zone.
function setInactive(): void {
  setTimeout(() => {
    isActive.value = false;
  }, 50);
}
</script>

<style scoped lang="scss">
.drop-zone {
  border: 1px dashed var(--parsec-color-light-primary-200);
  border-radius: var(--parsec-radius-6);
  height: 30em;
  width: 100%;
}

.drop-zone-active {
  background-color: var(--parsec-color-light-primary-200);
}

.drop-zone-inactive {
  background-color: var(--parsec-color-light-primary-30);
}
</style>
