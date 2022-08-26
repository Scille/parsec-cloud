<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-header>
    <ion-toolbar>
      <ion-title>Uploading files</ion-title>
      <ion-buttons slot="end">
        {{ modalIsMaximized }}
        <ion-icon
          v-if="!modalIsMaximized"
          slot="center"
          size="large"
          :icon="chevronUp"
          class="ion-float-right"
          @click="maximizeModal()"
        >
          Expand
        </ion-icon>
        <ion-icon
          v-else
          slot="center"
          size="large"
          :icon="chevronDown"
          class="ion-float-right"
          @click="minimizeModal()"
        >
          Minimize
        </ion-icon>
        <!-- <ion-button @click="closeModal" fill="clear">Cancel all
                </ion-button> -->
        <ion-icon
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
          @click="closeModal"
        >
          Close
        </ion-icon>
      </ion-buttons>
    </ion-toolbar>
  </ion-header>
  <ion-content ref="uploadModalHeader">
    <ion-list>
      <ion-item>
        <ion-icon
          slot="start"
          size="large"
          :icon="image"
          class="file-type-icon"
        />
        <ion-label>
          <h2>Test.png</h2>
          <ion-progress-bar type="indeterminate" />
          <p>1,1 Mo / 2 Mo</p>
        </ion-label>
        <ion-icon
          @click="toDoRemove()"
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
        >
          Close
        </ion-icon>
      </ion-item>
      <ion-item>
        <ion-icon
          slot="start"
          size="large"
          :icon="image"
          class="file-type-icon"
        />
        <ion-label>
          <h2>Test with a longer name.png</h2>
          <ion-progress-bar value="0.75" />
          <p>1,5 Mo / 2 Mo</p>
        </ion-label>
        <ion-icon
          @click="toDoRemove()"
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
        >
          Close
        </ion-icon>
      </ion-item>
      <ion-item>
        <ion-icon
          slot="start"
          size="large"
          :icon="image"
          class="file-type-icon"
        />
        <ion-label>
          <h2>Test.png</h2>
          <ion-progress-bar type="indeterminate" />
          <p>1,1 Mo / 2 Mo</p>
        </ion-label>
        <ion-icon
          @click="toDoRemove()"
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
        >
          Close
        </ion-icon>
      </ion-item>
      <ion-item>
        <ion-icon
          slot="start"
          size="large"
          :icon="image"
          class="file-type-icon"
        />
        <ion-label>
          <h2>Test.png</h2>
          <ion-progress-bar type="indeterminate" />
          <p>1,1 Mo / 2 Mo</p>
        </ion-label>
        <ion-icon
          @click="toDoRemove()"
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
        >
          Close
        </ion-icon>
      </ion-item>
      <ion-item>
        <ion-icon
          slot="start"
          size="large"
          :icon="image"
          class="file-type-icon"
        />
        <ion-label>
          <h2>Test.png</h2>
          <ion-progress-bar type="indeterminate" />
          <p>1,1 Mo / 2 Mo</p>
        </ion-label>
        <ion-icon
          @click="toDoRemove()"
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
        >
          Close
        </ion-icon>
      </ion-item>
      <ion-item>
        <ion-icon
          slot="start"
          size="large"
          :icon="image"
          class="file-type-icon"
        />
        <ion-label>
          <h2>Test.png</h2>
          <ion-progress-bar type="indeterminate" />
          <p>1,1 Mo / 2 Mo</p>
        </ion-label>
        <ion-icon
          @click="toDoRemove()"
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
        >
          Close
        </ion-icon>
      </ion-item>
      <ion-item>
        <ion-icon
          slot="start"
          size="large"
          :icon="image"
          class="file-type-icon"
        />
        <ion-label>
          <h2>Test.png</h2>
          <ion-progress-bar type="indeterminate" />
          <p>1,1 Mo / 2 Mo</p>
        </ion-label>
        <ion-icon
          @click="toDoRemove()"
          slot="end"
          size="large"
          :icon="close"
          class="ion-float-right"
        >
          Close
        </ion-icon>
      </ion-item>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonTitle,
  IonToolbar,
  IonButtons,
  IonItem,
  IonList,
  IonLabel,
  modalController,
  IonIcon,
  IonProgressBar
} from '@ionic/vue';
import { close, image, chevronUp, chevronDown } from 'ionicons/icons';
import { onMounted, ref } from 'vue';

const uploadModalHeader = ref(null);
/* const props = defineProps({
    modalIsMaximized: Ref<boolean>
}); */

const modalIsMaximized = ref(false);

onMounted(() => {
  initializeModalVariables();
});

async function initializeModalVariables(): Promise<void> {
  console.log('initializeModalVariables');
  const uploadModal = uploadModalHeader.value.$el.parentNode.parentNode;
  uploadModal.addEventListener('ionBreakpointDidChange', async (ev) => {
    modalIsMaximized.value = (await uploadModal.getCurrentBreakpoint() === 1);
    console.log('breakpoint changed', ev.detail.breakpoint);
  });

  /* modalController.getTop()
        .then((uploadModal) => uploadModal.addEventListener('ionBreakpointDidChange', async (ev) => {
            modalIsMaximized.value = (await uploadModal.getCurrentBreakpoint() === 1);
            console.log('breakpoint changed', ev.detail.breakpoint);
        }))
        .catch(() => {
            console.error("uploadModal not found");
        }); */

}

async function maximizeModal(): Promise<void> {
  modalIsMaximized.value = true;
  console.log('parent is ', uploadModalHeader.value.$el.parentNode.parentNode);
  const uploadModal = await modalController.getTop();

  console.log('breakpoint is ', await uploadModal.getCurrentBreakpoint());
  uploadModal.setCurrentBreakpoint(1);
}

async function minimizeModal(): Promise<void> {
  const uploadModal = await modalController.getTop();
  console.log('breakpoint is ', await uploadModal.getCurrentBreakpoint());

  modalIsMaximized.value = false;
  uploadModalHeader.value.$el.parentNode.parentNode.setCurrentBreakpoint(0.2);
}

async function closeModal(): Promise<boolean> {
  console.log('getTop is: ', await modalController.getTop());
  return modalController.dismiss(null, 'closed');
}
function confirm(): Promise<boolean> {
  return modalController.dismiss(null, 'confirm');
}

function toDoRemove(): void {
  console.log('removed');
}
</script>

<style scoped>
.file-type-icon {
    margin-right: 16px;
}
</style>
