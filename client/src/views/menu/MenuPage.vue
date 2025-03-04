<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <desktop-menu
      v-if="userInfo && !isMobile()"
      :user-info="userInfo"
      @sidebar-width-changed="onSidebarWidthChanged"
    />
    <mobile-menu
      v-if="userInfo && isMobile()"
      :user-info="userInfo"
    />
  </ion-page>
</template>

<script setup lang="ts">
import { ClientInfo, getClientInfo as parsecGetClientInfo, isMobile } from '@/parsec';
import { IonPage } from '@ionic/vue';
import { Ref, onMounted, onUnmounted, ref } from 'vue';
import MobileMenu from '@/views/menu/MobileMenu.vue';
import DesktopMenu from '@/views/menu/DesktopMenu.vue';

const sidebarWidth = ref<number>(0);
const userInfo: Ref<ClientInfo | null> = ref(null);

function onSidebarWidthChanged(value: number): void {
  sidebarWidth.value = value;
  setToastOffset(value);
}

async function updateWindowWidth(): Promise<void> {
  if (window.innerWidth <= 768) {
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

  window.addEventListener('resize', updateWindowWidth);
  updateWindowWidth();
});

onUnmounted(async () => {
  setToastOffset(0);
  window.removeEventListener('resize', updateWindowWidth);
});

function setToastOffset(width: number): void {
  window.document.documentElement.style.setProperty('--ms-toast-offset', `${width}px`);
}
</script>

<style lang="scss" scoped>
@import '@/theme/responsive-mixin';

* {
  transition: all 100ms ease-in-out;
}
</style>
