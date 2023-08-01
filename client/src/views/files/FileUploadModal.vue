<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ion-buttons
      slot="end"
      class="closeBtn-container"
    >
      <ion-button
        slot="icon-only"
        @click="closeModal()"
        class="closeBtn"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
    <ion-header class="modal-header">
      <ion-icon
        class="modal-header__icon"
        :icon="folder"
        size="medium"
      />
      <ion-title
        class="modal-header__title title-h2"
      >
        {{ $t('FoldersPage.importModal.title') }}
      </ion-title>
    </ion-header>
    <div class="modal-content inner-content">
      <file-drop-zone
        @files-drop="onFilesDrop"
      >
        <file-import />
      </file-drop-zone>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  IonIcon,
  IonButton,
  IonButtons,
  IonHeader,
  IonTitle,
  modalController,
} from '@ionic/vue';
import {
  close,
  folder,
} from 'ionicons/icons';
import { ModalResultCode } from '@/common/constants';
import FileImport from '@/components/files/FileImport.vue';
import FileDropZone from '@/components/files/FileDropZone.vue';
import { join as joinPath } from '@/common/path';

const props = defineProps<{
  currentPath: string
}>();

let fd = 0;

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Cancel);
}

async function mockParsecUploadFile(currentPath: string, fsEntry: FileSystemEntry): Promise<void> {

  async function mockParsecOpenFile(path: string, mode: string): Promise<number> {
    fd += 1;
    console.log('Opening', path, 'with mode', mode, 'as', fd);
    return fd;
  }

  async function mockParsecCloseFile(fd: number): Promise<void> {
    console.log('Closing', fd);
  }

  async function mockParsecWriteFile(fd: number, chunk: Uint8Array): Promise<number> {
    console.log('Writing to', fd, 'content of size', chunk.length);
    return chunk.length;
  }

  async function mockParsecMkdir(path: string): Promise<void> {
    console.log('Makedir', path);
  }

  const parsecPath = joinPath(currentPath, fsEntry.name);

  if (fsEntry.isDirectory) {
    console.log('Uploading folder', fsEntry.name, 'to', currentPath);
    mockParsecMkdir(fsEntry.name);
    const reader = (fsEntry as FileSystemDirectoryEntry).createReader();
    reader.readEntries((entries) => {
      entries.forEach(async (entry) => {
        await mockParsecUploadFile(parsecPath, entry);
      });
    });
  } else {
    console.log('Uploading file', fsEntry.name, 'to', currentPath);
    (fsEntry as FileSystemFileEntry).file(async (file) => {
      const reader = file.stream().getReader();
      const fd = await mockParsecOpenFile(parsecPath, 'w+');
      // eslint-disable-next-line no-constant-condition
      while (true) {
        const data = await reader.read();
        if (data.done) {
          break;
        }
        await mockParsecWriteFile(fd, data.value);
      }
      await mockParsecCloseFile(fd);
    });
  }
}

async function onFilesDrop(entries: FileSystemEntry[]): Promise<void> {
  for (const entry of entries) {
    await mockParsecUploadFile(props.currentPath, entry);
  }
}
</script>

<style scoped lang="scss">
.modal {
  padding: 3.5rem;
  justify-content: start;
}

.closeBtn-container {
    position: absolute;
    top: 2rem;
    right: 2rem;
  }

.closeBtn-container, .closeBtn {
  margin: 0;
  --padding-start: 0;
  --padding-end: 0;
}

.closeBtn {
  width: fit-content;
  height: fit-content;
  --border-radius: var(--parsec-radius-4);
  --background-hover: var(--parsec-color-light-primary-50);
  border-radius: var(--parsec-radius-4);

  &__icon {
    padding: 4px;
    color: var(--parsec-color-light-primary-500);

    &:hover {
      --background-hover: var(--parsec-color-light-primary-50);
    }
  }

  &:active {
    border-radius: var(--parsec-radius-4);
    background: var(--parsec-color-light-primary-100);
  }
}

.modal-header {
  margin-bottom: 2.5rem;
  display: flex;
  align-items: center;
  gap: 1rem;

  &__title {
    padding: 0;
    color: var(--parsec-color-light-primary-600);
  }

  &__icon {
    color: var(--parsec-color-light-primary-600);
    font-size: 1.5rem;
  }
}
</style>
