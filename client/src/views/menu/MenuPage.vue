<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <main-menu
      v-if="userInfo"
      :user-info="userInfo"
      @sidebar-width-changed="onSidebarWidthChanged"
      @window-width-changed="updateToastOffset"
    />
  </ion-page>
</template>

<script setup lang="ts">
import { ClientInfo, getClientInfo as parsecGetClientInfo } from '@/parsec';
import { IonPage } from '@ionic/vue';
import { Ref, onMounted, onUnmounted, ref } from 'vue';

import MainMenu from '@/views/menu/MainMenu.vue';
import { useWindowSize } from 'megashark-lib';

const { isSmallDisplay } = useWindowSize();
const sidebarWidth = ref<number>(0);
const userInfo: Ref<ClientInfo | null> = ref(null);

function onSidebarWidthChanged(value: number): void {
  sidebarWidth.value = value;
  updateToastOffset();
}

function updateToastOffset(): void {
  if (isSmallDisplay.value) {
    setToastOffset(0);
  } else {
    setToastOffset(sidebarWidth.value);
  }
}

onMounted(async () => {
  const infoResult = await parsecGetClientInfo();

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(infoResult.error)}`);
  }
  updateToastOffset();
});

onUnmounted(async () => {
  setToastOffset(0);
});

function setToastOffset(width: number): void {
  window.document.documentElement.style.setProperty('--ms-toast-offset', `${width}px`);
}
</script>

<style lang="scss" scoped>
* {
  transition: all 100ms ease-in-out;
}
</style>
