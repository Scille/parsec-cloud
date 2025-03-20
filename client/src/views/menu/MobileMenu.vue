<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-tabs
    class="menu-mobile"
    tabs="5"
    :when="false"
  >
    <ion-tab-bar
      slot="bottom"
      class="tab-bar"
    >
      <ion-tab-button
        tab="workspaces"
        class="button-container"
        :class="currentRouteIs(Routes.Workspaces) ? 'active' : ''"
        @click="navigateTo(Routes.Workspaces)"
      >
        <div class="tab-bar-list-button">
          <ion-icon
            :icon="business"
            class="tab-bar-list-button__icon"
          />
          <ion-text class="tab-bar-list-button__text button-medium">
            {{ $msTranslate('SideMenu.tabbar.workspaces') }}
          </ion-text>
        </div>
      </ion-tab-button>

      <ion-tab-button
        tab="documents"
        :disabled="!hasVisited(Routes.Documents)"
        class="button-container"
        :class="currentRouteIs(Routes.Documents) ? 'active' : ''"
        @click="goToLastDocumentsPage"
      >
        <div class="tab-bar-list-button">
          <ion-icon
            :icon="folder"
            class="tab-bar-list-button__icon"
          />
          <ion-text class="tab-bar-list-button__text button-medium">
            {{ $msTranslate('SideMenu.tabbar.files') }}
          </ion-text>
        </div>
      </ion-tab-button>

      <!-- fab button hidden for now, waiting a responsive modal -->
      <ion-tab-button
        tab="add-icon"
        class="button-container fab-button-container"
      >
        <ion-fab
          class="fab-content"
          size="small"
          v-show="false"
        >
          <ion-fab-button
            class="fab-button"
            id="open-modal"
          >
            <ms-image
              :image="AddIcon"
              class="fab-icon"
            />
          </ion-fab-button>
        </ion-fab>
      </ion-tab-button>

      <ion-tab-button
        tab="organization"
        class="button-container"
        :class="currentRouteIs(Routes.Organization) ? 'active' : ''"
        @click="navigateTo(Routes.Organization)"
      >
        <div class="tab-bar-list-button">
          <ion-icon
            :icon="prism"
            class="tab-bar-list-button__icon"
          />
          <ion-text class="tab-bar-list-button__text button-medium">
            {{ $msTranslate('SideMenu.tabbar.organization') }}
          </ion-text>
        </div>
      </ion-tab-button>

      <ion-tab-button
        tab="profile"
        class="button-container"
        :class="currentRouteIs(Routes.MyProfile) ? 'active' : ''"
        @click="navigateTo(Routes.MyProfile)"
      >
        <div class="tab-bar-list-button">
          <ion-icon
            :icon="personCircle"
            class="tab-bar-list-button__icon"
          />
          <ion-text class="tab-bar-list-button__text button-medium">
            {{ userInfo ? userInfo.humanHandle.label : '' }}
          </ion-text>
        </div>
      </ion-tab-button>
    </ion-tab-bar>
    <ion-router-outlet id="main" />
  </ion-tabs>
</template>

<script setup lang="ts">
import { MsImage, AddIcon } from 'megashark-lib';
import { IonTabs, IonTabBar, IonTabButton, IonIcon, IonFab, IonFabButton, IonText, IonRouterOutlet } from '@ionic/vue';
import { business, prism, personCircle, folder } from 'ionicons/icons';
import { Routes, currentRouteIs, navigateTo, hasVisited, getLastVisited } from '@/router';
import { ClientInfo } from '@/parsec';

defineProps<{
  userInfo: ClientInfo;
}>();

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
</script>

<style lang="scss" scoped>
.menu-mobile {
  .tab-bar {
    background: var(--parsec-color-light-secondary-inversed-contrast);
    padding: 0.5rem 1rem 1.5rem;
    justify-content: space-between;
    border-top: 1px solid var(--parsec-color-light-secondary-premiere);
    display: flex;
    overflow: visible;
    contain: none;
  }

  .button-container {
    --background: none;
    --background-hover: var(--parsec-color-light-primary-30-opacity15);
    --color: var(--parsec-color-light-secondary-text);
    --color-hover: none;
    --min-height: 0;
    margin: auto;
    width: 100%;

    &::part(native) {
      padding: 0;
      overflow: visible;
    }

    &.active {
      --color: var(--parsec-color-light-primary-600);
    }
  }

  .tab-bar-list-button {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    overflow: hidden;

    &__icon {
      font-size: 1.5rem;
    }

    &__text {
      font-size: 0.75rem;
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      max-width: 4.75rem;
      width: 100%;
    }
  }

  .fab-button-container {
    max-width: 3.75rem;

    .fab-button {
      width: 2.375rem;
      height: 2.375rem;
      --background: var(--parsec-color-light-primary-600);
      --background-activated: var(--parsec-color-light-secondary-white);
      --border-radius: var(--parsec-radius-12);
      --box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.3), 0px 1px 3px 1px rgba(0, 0, 0, 0.15);
      --color: var(--parsec-color-light-secondary-inversed-contrast);

      .fab-icon {
        --fill-color: var(--parsec-color-light-secondary-inversed-contrast);
      }
    }
  }
}
</style>
