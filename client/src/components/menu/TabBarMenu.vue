<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="tab-bar-menu">
    <div
      class="tab-bar-menu-button"
      :class="currentRouteIs(Routes.Workspaces) ? 'active' : ''"
      @click="navigateTo(Routes.Workspaces)"
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

    <div class="tab-bar-menu fab-button-container">
      <ion-fab
        class="fab-content"
        size="small"
        @click="isMenuOpen ? dismissModal() : openAddMenuModal()"
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
      @click="navigateTo(Routes.Organization)"
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
      @click="navigateTo(Routes.MyProfile)"
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
import { IonFab, IonFabButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { folder, prism, personCircle, business } from 'ionicons/icons';
import { hasVisited, Routes, navigateTo, currentRouteIs, getLastVisited } from '@/router';
import { ClientInfo, getClientInfo as parsecGetClientInfo } from '@/parsec';
import { ref, Ref, onMounted } from 'vue';
import { AddIcon, MsImage } from 'megashark-lib';
import AddMenuModal from '@/views/menu/AddMenuModal.vue';

const isMenuOpen = ref(false);

const userInfo: Ref<ClientInfo | null> = ref(null);

const emit = defineEmits<{
  (e: 'actionClicked', data: any): void;
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
  await navigateTo(Routes.Documents, {
    params: routeBackup.data.params,
    query: { workspaceHandle: routeBackup.data.query.workspaceHandle, documentPath: routeBackup.data.query.documentPath },
  });
}

async function dismissModal(): Promise<void> {
  isMenuOpen.value = false;

  const modal = await modalController.getTop();
  if (modal) {
    await modal.dismiss();
  }
}

async function openAddMenuModal(): Promise<void> {
  isMenuOpen.value = true;

  const modal = await modalController.create({
    component: AddMenuModal,
    cssClass: 'tab-menu-modal',
    showBackdrop: true,
    handle: true,
    backdropDismiss: true,
    breakpoints: [0, 1],
    // https://ionicframework.com/docs/api/modal#scrolling-content-at-all-breakpoints
    // expandToScroll: false, should be added to scroll with Ionic 8
    initialBreakpoint: 1,
  });
  await modal.present();
  const { data } = await modal.onDidDismiss();

  if (data) {
    emit('actionClicked', data);
  }
  await dismissModal();
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
      width: 2.375rem;
      height: 2.375rem;
      --background: var(--parsec-color-light-primary-600);
      --background-activated: var(--parsec-color-light-secondary-white);
      --border-radius: var(--parsec-radius-12);
      --box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.3), 0px 1px 3px 1px rgba(0, 0, 0, 0.15);
      --color: var(--parsec-color-light-secondary-inversed-contrast);

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
