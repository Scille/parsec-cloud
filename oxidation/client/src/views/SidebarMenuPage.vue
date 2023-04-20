<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <div
      class="divider"
      ref="divider"
    >
    </div>
    <ion-split-pane
      content-id="main"
      ref="splitPane"
    >
      <ion-menu
        content-id="main"
        class="sidebar"
      >
        <ion-header class="sidebar__header">
          <!-- active oragnization -->
          <ion-card class="organization-card">
            <ion-card-header class="organization-card__header">
              <div class="organization-card__container">
                <ion-avatar class="orga-avatar">
                  <span>{{ device.organizationId?.substring(0, 2) }}</span>
                </ion-avatar>
                <div class="orga-text">
                  <ion-card-subtitle class="caption-info">
                    {{ $t('HomePage.organizationActionSheet.header') }}
                  </ion-card-subtitle>
                  <ion-card-title class="title-h4">
                    {{ device.organizationId }}
                  </ion-card-title>
                </div>
              </div>
              <!-- new icon to provide -->
              <div class="organization-card__icon">
                <svg
                  width="32"
                  height="32"
                  viewBox="0 0 32 32"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path d="M10.3495 20.4231L15.6049 26.795C15.6896 26.8977 15.7947 26.9801 15.9129 27.0366C16.0311 27.0931 16.1597 27.1223 16.2899 27.1223C16.42 27.1223 16.5486 27.0931 16.6669 27.0366C16.7851 26.9801 16.8902 26.8977 16.9749 26.795L22.2303 20.4231C22.7319 19.8149 22.316 18.8755 21.5453 18.8755L11.033 18.8755C10.2622 18.8755 9.84641 19.8149 10.3495 20.4231Z" fill="#F9F9FB"/>
                  <path d="M22.2326 13.4558L16.9772 7.08389C16.8925 6.98124 16.7874 6.89884 16.6691 6.84234C16.5509 6.78585 16.4223 6.7566 16.2921 6.7566C16.162 6.7566 16.0334 6.78585 15.9151 6.84234C15.7969 6.89884 15.6918 6.98124 15.6071 7.08389L10.3517 13.4558C9.85015 14.064 10.266 15.0034 11.0367 15.0034L21.549 15.0034C22.3198 15.0034 22.7356 14.064 22.2326 13.4558Z" fill="#F9F9FB"/>
                </svg>
              </div>
            </ion-card-header>

            <div class="organization-card__manageBtn">
              <ion-icon
                :icon="cog"
                slot="start"
                size="small"
              />
              <ion-text
                class="subtitles-sm"
                button
                @click="navigateToPage('settings')"
              >
                Gérer mon organisation
              </ion-text>
            </div>
          </ion-card>
          <!-- end of active organzation -->
        </ion-header>

        <ion-content class="ion-padding ">
          <!-- list of workspaces -->
          <ion-list class="list-workspaces">
            <ion-header
              lines="none"
              button
              @click="navigateToPage('workspacesPages')"
              class="list-workspaces__header title-h5"
            >
              All {{ $t('OrganizationPage.workspaces') }}
            </ion-header>

            <ion-item
              lines="none"
              button
              @click="navigateToPage('workspaces')"
              v-for="workspace in workspacesExampleData"
              :key="workspace.id"
            >
              <ion-icon
                :icon="business"
                slot="start"
              />
              <ion-label>{{ workspace.name }}</ion-label>
            </ion-item>
          </ion-list>
          <!-- list of workspaces -->
        </ion-content>
      </ion-menu>

      <ion-router-outlet id="main" />
    </ion-split-pane>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonMenu,
  IonSplitPane,
  IonTitle,
  IonToolbar,
  IonIcon,
  IonText,
  IonButton,
  IonList,
  IonCard,
  IonCardTitle,
  IonCardSubtitle,
  IonCardHeader,
  IonLabel,
  IonPage,
  IonAvatar,
  IonItem,
  IonRouterOutlet,
  menuController,
  GestureDetail
} from '@ionic/vue';
import {
  business,
  cog
} from 'ionicons/icons';
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { createGesture } from '@ionic/vue';

import { useRouter, useRoute } from 'vue-router';
import { AvailableDevice } from '../plugins/libparsec/definitions';
import useSidebarMenu from '@/services/sidebarMenu';

let device: any = {};

// fake data
const workspacesExampleData = [
  {
    id: 1234,
    name: 'Product Design',
    userRole: 'Owner',
    userCount: 3
  },
  {
    id: 2345,
    name: 'Marketing',
    userRole: 'Contributor',
    userCount: 1
  },
  {
    id: 3456,
    name: 'Engineering',
    userRole: 'Contributor',
    userCount: 4
  },
  {
    id: 4567,
    name: 'Research',
    userRole: 'Reader',
    userCount: 3
  }
];
const router = useRouter();
const currentRoute = useRoute();
const { t, d } = useI18n();

const splitPane = ref();
const divider = ref();
const { defaultWidth, initialWidth, computedWidth, wasReset } = useSidebarMenu();

const unwatch = watch(wasReset, (value) => {
  if (value) {
    resizeMenu(defaultWidth);
    wasReset.value = false;
  }
});

function navigateToPage(pageName: string): void {
  router.push({ name: pageName });
  menuController.close();
}

// code au chargement de la page
onMounted(() => {
  device = JSON.parse(currentRoute.query.device as string);

  if (divider.value) {
    const gesture = createGesture({
      gestureName: 'resize-menu',
      el: divider.value,
      onEnd,
      onMove
    });
    gesture.enable();
  }
});

onUnmounted(() => {
  unwatch();
});

function onMove(detail: GestureDetail): void {
  requestAnimationFrame(() => {
    let currentWidth = initialWidth.value + detail.deltaX;
    if (currentWidth >= 4 && currentWidth <= 500) {
      console.log('test');
      if (currentWidth <= 150) {
        currentWidth = 4;
      }
      resizeMenu(currentWidth);
      computedWidth.value = currentWidth;
    }
  });
}

function onEnd(): void {
  initialWidth.value = computedWidth.value;
}

function resizeMenu(newWidth: number): void {
  if (splitPane.value) {
    splitPane.value.$el.style.setProperty('--side-width', `${newWidth}px`);
  }
  if (divider.value) {
    divider.value.style.setProperty('left', `${newWidth - 4}px`);
  }
}

</script>

<style lang="scss" scoped>

.divider {
  width: 8px;
  height: 100%;
  position: absolute;
  left: 296px;
  top: 0;
  z-index: 10000;
  cursor:ew-resize;
  display: flex;
  justify-content: center;
  &::after {
    content: '';
    width: 4px;
    height: 100%;
    padding: 20rem 0;
  }
  &:hover::after, &:active::after{
    background: var(--parsec-color-light-primary-200);
  }
}

.sidebar, .sidebar ion-content {
  --background: var(--parsec-color-light-primary-800);
}
.sidebar{
  border: none;
  user-select: none;
  border-radius: 0 .5rem 0;
  // logo parsec
  &::after{
    content: url('../assets/images/logo/logo_icon_white.svg');
    opacity: .03;
    width: 100%;
    max-width: 270px;
    max-height: 170px;
    display: block;
    position: fixed;
    bottom: 16px;
    right: -60px;
  }
}
.organization-card{
  --background: var(--parsec-color-light-primary-30-opacity15);
  box-shadow: none;
  margin: 0.5rem;
  &__header{
    display: flex;
    justify-content: space-between;
  }
  &__container{
    box-shadow: none;
    display: flex;
    align-items: center;
    justify-content: left;
    gap: 0.75em;
    position: relative;
    z-index: 2;
    min-width: 0;
    .orga-avatar{
      background-color: var(--parsec-color-light-primary-800);
      color: var(--parsec-color-light-primary-50);
      width: 42px;
      height: 42px;
      border-radius: 50%;
      text-align: center;
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      position: relative;
      z-index: 1;
    }
    .orga-text{
      display: flex;
      flex-direction: column;
      white-space: nowrap;
      overflow: hidden;
      ion-card-subtitle{
      --color: var(--parsec-color-light-secondary-light);
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      }
      ion-card-title{
        padding: 0.1875em 0;
        margin: 0;
        --color: var(--parsec-color-light-primary-50);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }
    }
  }
  &__icon{
    white-space: nowrap;
    display: flex;
    align-items: center;
    background-color: var(--parsec-color-light-primary-800);
    position: relative;
    z-index: 2;
    &::before{
      content: '';
      height: 100%;
      width: 100%;
      position: absolute;
      z-index: -1;
      background-color: var(--parsec-color-light-primary-30-opacity15);
    }
  }
  &__manageBtn{
    padding:0.625em 1em;
    display: flex;
    align-items: center;
    gap: 1em;
    color: var(--parsec-color-light-secondary-light);
    border-top: 1px solid var(--parsec-color-light-primary-30-opacity15);
    &:hover{
      background: var(--parsec-color-light-primary-30-opacity15);
    }
    ion-text{
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
    ion-icon{
      min-width: 18px;
    }
  }
}
.list-md{
  background: none;
}

ion-split-pane {
  --side-min-width: 0px;
  --side-max-width: 500px;
  --side-width: 300px;
}

ion-menu {
  user-select: none;
}

.list-workspaces{
  display: flex;
  flex-direction: column;
  flex: 1;
  &__header{
    opacity: 0.6;
    color: var(--parsec-color-light-primary-100);
    margin-bottom: 1rem;
    width: fit-content;
    transition: border 0.2s ease-in-out;
    cursor: pointer;
    &:hover::after{
      background: var(--parsec-color-light-primary-100);
      height: 2px;
      width: 100%;
      border-radius: 5px;
    }
  }
  .item-label{
    --background:none;
    border-radius: 4px;
    border: solid 1px var(--parsec-color-light-primary-800);
    &:hover{
      border: solid 1px var(--parsec-color-light-primary-30-opacity15);
    }
    &:active{
      --background: var(--parsec-color-light-primary-30-opacity15);
    }
    ion-label{
      --color: var(--parsec-color-light-primary-100);
    }
    ion-icon{
      color: var(--parsec-color-light-primary-100);
      font-size: 1.25em;
      margin-inline-end: 12px;
    }
  }
}
</style>
