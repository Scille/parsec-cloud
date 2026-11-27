<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      v-if="isActive && props.showDropMessage"
      class="drop-message"
    >
      <ms-image
        :image="ImportFile"
        class="drop-message__image"
      />
      <ion-text class="drop-message__text">
        {{ $msTranslate('FoldersPage.ImportFile.dropInstructions') }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import ImportFile from '@/assets/images/import-file.svg?raw';
import { getFilesFromDrop } from '@/components/files/utils';
import { IonText } from '@ionic/vue';
import { MsImage } from 'megashark-lib';
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';

defineExpose({
  reset,
});

const props = defineProps<{
  disabled?: boolean;
  showDropMessage?: boolean;
  isReader?: boolean;
}>();

const emits = defineEmits<{
  (e: 'filesAdded', gen: AsyncGenerator<File[]>): void;
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
  emits('filesAdded', getFilesFromDrop(event));
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
      left: 0.5rem;
      right: 0.5rem;
    }

    &.drop-active {
      outline: 1px dashed ms.color('border-brand-default');
      border-radius: ms.radius('lg');

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        border-radius: inherit;
        background: ms.color('surface-brand-default');
        opacity: ms.opacity('1');
      }
    }
  }
}

.drop-zone .drop-zone > .drop-zone-dashed::before,
.drop-zone:has(.drop-zone-active) > .drop-zone-dashed::before {
  display: none;
}

.drop-message {
  position: absolute;
  left: 50%;
  bottom: 2em;
  transform: translate(-50%, -50%);
  width: fit-content;
  background-color: ms.color('surface-base-default');
  border: ms.border('thin') solid ms.color('border-brand-default-subtle');
  border-radius: ms.radius('xl');
  box-shadow: ms.shadow('elevation-2xl');
  padding: ms.spacing('padding-4xl') ms.spacing('padding-4xl');
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: ms.spacing('gap-lg');
  color: ms.color('text-base-description');
  z-index: 1101;

  @include ms.responsive-breakpoint('sm') {
    position: fixed;
  }

  &__image {
    width: 2.5rem;
    height: 2.5rem;
  }

  &__text {
    @include ms.font('body-lg-medium');
  }
}
</style>
