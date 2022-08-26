<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      @drop.prevent="addFile"
      @dragover="setDragging($event, true)"
      @dragenter="setDragging($event, true)"
      @dragleave="setDragging($event, false)"
    >
      Drop file here from your OS file explorer to test
      <div
        v-show="isDragging"
        class="drop-active"
      >
        <h1 class="drop-text">
          Drop files to upload
        </h1>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonPage,
  IonContent,
  modalController
} from '@ionic/vue';

import { ref } from 'vue';
import UploadModal from '@/components/ModalFilesUploading.vue';

const isDragging = ref(false);

function addFile(e): void {
  isDragging.value = false;
  console.log(e.dataTransfer.files);
  openModal();
}

function setDragging(e, bool: boolean): void {
  console.log('dragging ', isDragging.value);

  e.preventDefault();
  isDragging.value = bool;
}

async function openModal(): Promise<void> {
  const modalIsMaximized = ref(false);
  const modal = await modalController.create({
    component: UploadModal,
    initialBreakpoint: 0.5,
    breakpoints: [0.2, 0.5, 1],
    backdropDismiss: false,
    backdropBreakpoint: 0.9
    /* componentProps: {
            modalIsMaximized: modalIsMaximized
        } */

  });
  modal.present();
  /* modal.addEventListener('ionBreakpointDidChange', async(ev) => {
        modalIsMaximized.value = (await modal.getCurrentBreakpoint() === 1);
        console.log('breakpoint changed', ev.detail.breakpoint);
    }); */

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    /* message = `Hello, ${data}!`; */
  }

  console.log(modal);
}
</script>

<style lang="scss" scoped>

.drop-active {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;

    display: flex;
    justify-content: center;
    align-items: center;

    margin: auto;
    border: 3px dashed var(--ion-color-primary);
    z-index: 999;
    opacity: .8;
    background: #000;
}

.drop-text {
    opacity: 1;
    text-align: justify;
    color:cyan;
}
</style>
