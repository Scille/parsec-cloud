<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <sidebar-menu
      v-if="userInfo"
      :user-info="userInfo"
      @sidebar-width-changed="onSidebarWidthChanged"
    />
    <tab-menu
      v-if="userInfo"
      v-show="menuMode === MenuMode.Tabs"
      :user-info="userInfo"
    />
  </ion-page>
</template>

<script setup lang="ts">
import { ClientInfo, getClientInfo as parsecGetClientInfo } from '@/parsec';
import { IonPage } from '@ionic/vue';
import { Ref, onMounted, onUnmounted, ref } from 'vue';
import TabMenu from '@/views/menu/TabMenu.vue';
import SidebarMenu from '@/views/menu/SidebarMenu.vue';

enum MenuMode {
  Sidebar = 'sidebar',
  Tabs = 'tabs',
}

const sidebarWidth = ref<number>(0);
const userInfo: Ref<ClientInfo | null> = ref(null);
const menuMode = ref<MenuMode>(MenuMode.Sidebar);

function onSidebarWidthChanged(value: number): void {
  sidebarWidth.value = value;
  setToastOffset(value);
}

async function updateWindowWidth(): Promise<void> {
  if (window.innerWidth <= 768) {
    setToastOffset(0);
    menuMode.value = MenuMode.Tabs;
  } else {
    setToastOffset(sidebarWidth.value);
    menuMode.value = MenuMode.Sidebar;
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
