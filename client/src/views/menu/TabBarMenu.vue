<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="tab-bar-menu"
    v-if="userInfo"
    :class="{
      'tab-bar-menu--outsider': userInfo.currentProfile === UserProfile.Outsider,
      'tab-bar-menu--outsider-file': userInfo.currentProfile === UserProfile.Outsider && currentRouteIs(Routes.Documents),
    }"
    id="tab-bar"
  >
    <div
      class="tab-bar-menu-button"
      :class="currentRouteIs(Routes.Workspaces) ? 'active' : ''"
      @click="switchTab(Routes.Workspaces)"
    >
      <ion-icon
        :icon="business"
        class="tab-bar-menu-button__icon"
      />
      <ion-text class="tab-bar-menu-button__text button-medium">
        {{ $msTranslate('SideMenu.tabbar.workspaces') }}
      </ion-text>
    </div>

    <div
      class="tab-bar-menu-button"
      :disabled="!hasVisited(Routes.Documents)"
      :class="{
        active: currentRouteIs(Routes.Documents),
        disabled: !hasVisited(Routes.Documents),
      }"
      @click="goToLastDocumentsPage"
    >
      <ion-icon
        :icon="folder"
        class="tab-bar-menu-button__icon"
      />
      <ion-text class="tab-bar-menu-button__text button-medium">
        {{ $msTranslate('SideMenu.tabbar.files') }}
      </ion-text>
    </div>

    <div
      class="tab-bar-menu fab-button-container"
      v-show="actions.length > 0"
    >
      <ion-fab
        class="fab-content"
        size="small"
        id="add-menu-fab-button"
        @click="!isMenuOpen ? openMenuModal() : closeMenuModal()"
      >
        <ion-fab-button
          class="fab-button"
          :class="isMenuOpen ? 'active' : ''"
          id="open-modal"
        >
          <ms-image
            :image="AddIcon"
            class="fab-icon"
          />
        </ion-fab-button>
      </ion-fab>
    </div>

    <div
      class="tab-bar-menu-button"
      :class="currentRouteIs(Routes.Organization) ? 'active' : ''"
      @click="switchTab(Routes.Organization)"
      v-show="userInfo && userInfo.currentProfile !== UserProfile.Outsider"
    >
      <ion-icon
        :icon="prism"
        class="tab-bar-menu-button__icon"
      />
      <ion-text class="tab-bar-menu-button__text button-medium">
        {{ $msTranslate('SideMenu.tabbar.organization') }}
      </ion-text>
    </div>
    <div
      class="tab-bar-menu-button"
      :class="currentRouteIs(Routes.MyProfile) ? 'active' : ''"
      @click="switchTab(Routes.MyProfile)"
    >
      <ion-icon
        :icon="personCircle"
        class="tab-bar-menu-button__icon"
      />
      <ion-text class="tab-bar-menu-button__text button-medium">
        {{ userInfo ? userInfo.humanHandle.label : '' }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ClientInfo, getClientInfo as parsecGetClientInfo, UserProfile } from '@/parsec';
import { currentRouteIs, getLastVisited, hasVisited, navigateTo, Routes } from '@/router';
import TabBarMenuModal from '@/views/menu/TabBarMenuModal.vue';
import { MenuAction } from '@/views/menu/types';
import { IonFab, IonFabButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { business, folder, personCircle, prism } from 'ionicons/icons';
import { AddIcon, MsImage, MsModalResult } from 'megashark-lib';
import { onMounted, ref, Ref } from 'vue';

const isMenuOpen = ref(false);
const userInfo: Ref<ClientInfo | null> = ref(null);

const props = defineProps<{
  actions: Array<Array<MenuAction>>;
}>();

const emits = defineEmits<{
  (e: 'actionClicked', action: MenuAction): void;
}>();

onMounted(async () => {
  const infoResult = await parsecGetClientInfo();

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(infoResult.error)}`);
  }
});

async function goToLastDocumentsPage(): Promise<void> {
  const routeBackup = getLastVisited(Routes.Documents);

  if (!routeBackup) {
    return;
  }
  await closeMenuModal();
  await navigateTo(Routes.Documents, {
    params: routeBackup.data.params,
    query: { workspaceHandle: routeBackup.data.query.workspaceHandle, documentPath: routeBackup.data.query.documentPath },
  });
}

async function switchTab(route: Routes): Promise<void> {
  if (currentRouteIs(route)) {
    return;
  }
  await closeMenuModal();
  await navigateTo(route);
}

async function closeMenuModal(): Promise<void> {
  if (!isMenuOpen.value) {
    return;
  }
  isMenuOpen.value = false;

  const modal = await modalController.getTop();
  if (modal) {
    await modal.dismiss();
  }
}

async function openMenuModal(): Promise<void> {
  if (isMenuOpen.value) {
    return;
  }
  isMenuOpen.value = true;

  const modal = await modalController.create({
    component: TabBarMenuModal,
    cssClass: 'tab-menu-modal',
    showBackdrop: true,
    handle: true,
    backdropDismiss: true,
    breakpoints: [0, 1],
    expandToScroll: false,
    initialBreakpoint: 1,
    componentProps: {
      actions: props.actions,
    },
  });
  await modal.present();
  const { data, role } = await modal.onDidDismiss();

  if (role === MsModalResult.Confirm && data?.action) {
    emits('actionClicked', data.action);
  }
  await modal.dismiss();
  isMenuOpen.value = false;
}
</script>

<style lang="scss" scoped>
.tab-bar-menu {
  background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 0.5rem 1rem 1.5rem;
  justify-content: space-between;
  border: none;
  border-top: 1px solid var(--parsec-color-light-secondary-premiere);
  position: relative;
  z-index: 20;
  width: 100%;
  display: grid;
  grid-template-columns: repeat(5, 1fr);

  &--outsider {
    grid-template-columns: repeat(3, 1fr);
  }

  &--outsider-file {
    grid-template-columns: repeat(4, 1fr);
  }

  &-button {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    cursor: pointer;
    color: var(--parsec-color-light-secondary-text);

    &.active {
      color: var(--parsec-color-light-primary-600);
      position: relative;

      &::after {
        content: '';
        position: absolute;
        bottom: -0.5rem;
        left: 50%;
        transform: translateX(-50%);
        width: 0.25rem;
        height: 0.25rem;
        border-radius: var(--parsec-radius-circle);
        background: var(--parsec-color-light-primary-600);
      }
    }

    &.disabled {
      opacity: 0.4;
      cursor: not-allowed;
    }

    &__icon {
      font-size: 1.5rem;
    }

    &__text {
      font-size: 0.75rem;
      text-align: center;
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      max-width: 4.75rem;
      width: 100%;
    }
  }

  .fab-button-container {
    max-width: 3.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    margin: auto;
    border: none;

    .fab-content {
      position: relative;
    }

    .fab-button {
      width: 2.5rem;
      height: 2.5rem;
      --background: var(--parsec-color-light-gradient-background);
      --background-focused: var(--parsec-color-light-primary-700);
      --background-activated: var(--parsec-color-light-secondary-white);
      --border-radius: var(--parsec-radius-12);
      --box-shadow: var(--parsec-shadow-soft);
      --color: var(--parsec-color-light-secondary-inversed-contrast);
      --transition: all 0.2s ease-in-out;

      .fab-icon {
        transition: all 0.2s ease-in-out;
        transform: rotate(0deg);
        --fill-color: var(--parsec-color-light-secondary-inversed-contrast);
      }

      &.active {
        --background: var(--parsec-color-light-secondary-inversed-contrast);
        --background-activated: var(--parsec-color-light-primary-600);
        --color: var(--parsec-color-light-primary-600);

        .fab-icon {
          --fill-color: var(--parsec-color-light-primary-600);
          transform: rotate(45deg);
        }
      }
    }
  }
}
</style>
