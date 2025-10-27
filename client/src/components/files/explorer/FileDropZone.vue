<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="drop-zone"
    :class="isActive ? 'drop-zone-active' : 'drop-zone-inactive'"
    @drop.prevent="onDrop"
    @dragenter.prevent="onDragEnter()"
    @dragleave.prevent="onDragLeave()"
    @contextmenu="onContextMenu"
  >
    <slot />
    <div
      class="drop-zone-dashed"
      :class="isActive ? 'drop-active' : ''"
    />

    <div
      v-show="isActive && props.showDropMessage"
      class="drop-message"
    >
      <ms-image
        :image="DocumentImport"
        class="restore-password-header-img"
      />
      <ion-text class="subtitles-sm">
        {{ $msTranslate('FoldersPage.ImportFile.dropInstructions') }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FileImportTuple, getFilesFromDrop } from '@/components/files/utils';
import { FsPath } from '@/parsec';
import { IonText } from '@ionic/vue';
import { DocumentImport, MsImage } from 'megashark-lib';
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';

defineExpose({
  reset,
});

const props = defineProps<{
  currentPath: FsPath;
  disabled?: boolean;
  showDropMessage?: boolean;
  isReader?: boolean;
}>();

const emits = defineEmits<{
  (e: 'filesAdded', imports: FileImportTuple[]): void;
  (e: 'dropAsReader'): void;
  (e: 'globalMenuClick', event: Event): void;
}>();

const dragEnterCount = ref(0);

const isActive = computed(() => {
  return !props.disabled && !props.isReader && dragEnterCount.value > 0;
});

onMounted(() => {
  if (window.document) {
    // Prevent the browser from handling those events itself
    window.document.body.addEventListener('dragenter', preventDefaults);
    window.document.body.addEventListener('dragover', preventDefaults);
    window.document.body.addEventListener('dragleave', preventDefaults);
    window.document.body.addEventListener('drop', preventDefaults);
  }
});

onBeforeUnmount(() => {
  if (window.document) {
    // Restore the browser's event handling
    window.document.body.removeEventListener('dragenter', preventDefaults);
    window.document.body.removeEventListener('dragover', preventDefaults);
    window.document.body.removeEventListener('dragleave', preventDefaults);
    window.document.body.removeEventListener('drop', preventDefaults);
  }
});

async function onContextMenu(event: Event): Promise<void> {
  event.preventDefault();
  emits('globalMenuClick', event);
}

async function onDrop(event: DragEvent): Promise<void> {
  if (props.isReader) {
    event.stopPropagation();
    emits('dropAsReader');
    return;
  }
  if (props.disabled) {
    return;
  }
  event.stopPropagation();
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
  if (!props.disabled && !props.isReader) {
    dragEnterCount.value += 1;
    props.showDropMessage === true;
  }
}

function reset(): void {
  dragEnterCount.value = 0;
}
</script>

<style scoped lang="scss">
.drop-zone {
  width: 100%;
  height: 100%;
  position: relative;

  &-dashed {
    display: flex;
    flex-direction: column;
    flex-grow: 0;
    pointer-events: none;
    position: absolute;
    left: 0.125rem;
    right: 1rem;
    top: 0.5rem;
    bottom: 0.5rem;
    z-index: 1100;

    @include ms.responsive-breakpoint('sm') {
      top: 0.75rem;
      bottom: 0.75rem;
    }

    &.drop-active {
      outline: 1px dashed var(--parsec-color-light-primary-400);
      border-radius: var(--parsec-radius-8);
    }
  }
}

.drop-message {
  position: absolute;
  left: 50%;
  bottom: 2em;
  transform: translate(-50%, -50%);
  width: fit-content;
  background-color: var(--parsec-color-light-secondary-premiere);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-8);
  box-shadow: var(--parsec-shadow-strong);
  padding: 0.75rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--parsec-color-light-secondary-text);
  z-index: 10;
}
</style>
