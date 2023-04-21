<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header>
      <ion-toolbar class="topbar">
        <!-- icon visible when menu is hidden -->
        <ion-buttons slot="start">
          <ion-button
            v-if="!isPlatform('mobile') && !isSidebarMenuVisible()"
            slot="icon-only"
            id="trigger-toggle-menu-button"
            class="ion-hide-lg-down topbar-button__item"
            @click="resetSidebarMenu()"
          >
            <ion-icon
              slot="icon-only"
              :icon="menu"
            />
          </ion-button>
        </ion-buttons>
        <!-- end of icon visible when menu is hidden -->

        <!-- icon menu on mobile -->
        <ion-buttons slot="start">
          <ion-menu-button />
        </ion-buttons>
        <!-- end of icon menu on mobile -->
        <!-- voir pour composant -->
        <ion-breadcrumbs class="breadcrumb">
          <ion-breadcrumb href="#home">
            <ion-icon
              slot="icon-only"
              :icon="home"
            />
            Mes espaces
          </ion-breadcrumb>
          <ion-breadcrumb href="#electronics">breadcrumb item</ion-breadcrumb>
        </ion-breadcrumbs>

        <!-- top right icon + profil -->
        <ion-buttons slot="primary" class="topbar-right">
          <div class="topbar-button__list">
            <ion-button
              v-if="!isPlatform('mobile')"
              slot="icon-only"
              id="trigger-search-button"
              class="topbar-button__item"
            >
              <ion-icon
                slot="icon-only"
                :icon="search"
              />
            </ion-button>
            <ion-button
              v-if="!isPlatform('mobile')"
              slot="icon-only"
              id="trigger-notifications-button"
              class="topbar-button__item"
            >
              <ion-icon
                slot="icon-only"
                :icon="notifications"
              />
            </ion-button>
          </div>

          <profile-header
            :firstname="'toto'"
            :lastname="'toto'"
            class="profile-header"
          />
        </ion-buttons>
        <!-- top right icon + profil -->
      </ion-toolbar>
    </ion-header>

    <ion-content>
      <ion-router-outlet />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonHeader,
  IonToolbar,
  IonButton,
  IonIcon,
  IonMenuButton,
  IonButtons,
  isPlatform,
  IonList,
  IonLabel,
  IonItem,
  IonContent,
  IonPopover,
  IonRouterOutlet,
  IonPage,
  IonBreadcrumb,
  IonBreadcrumbs
} from '@ionic/vue';
import {
  personCircle,
  settings,
  helpCircle,
  menu,
  home,
  search,
  notifications
} from 'ionicons/icons';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import ProfileHeader from '@/components/ProfileHeader.vue';
import { ref } from 'vue';
import useSidebarMenu from '@/services/sidebarMenu';

const router = useRouter();
const { t } = useI18n();
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu } = useSidebarMenu();

</script>

<style scoped lang="scss">

.topbar{
  --background: var(--parsec-color-light-secondary-background);
  display: flex;
  padding: 2em;
}
.topbar-right{
  display: flex;
  gap: 2.5em;
}
.topbar-button__list{
  display: flex;
  gap: 1.25em;
  align-items: center;
  &::after{
    content: '';
    display: block;
    width: 1px;
    height: 1.5em;
    margin-left: 1em;
    background: var(--parsec-color-light-secondary-light);
  }
}
.topbar-button__item, .sc-ion-buttons-md-s .button{
  border: 1px solid var(--parsec-color-light-secondary-light);
  color: var(--parsec-color-light-primary-700);
  border-radius: 50%;
  --padding-top: 0;
  --padding-end: 0;
  --padding-bottom: 0;
  --padding-start: 0;
  &:hover{
    --background-hover: var(--parsec-color-light-primary-50);
    background: var(--parsec-color-light-primary-50);
    border: var(--parsec-color-light-primary-50);
  }

  .button-native{
    --padding-top: 0;
    --padding-end: 0;
    --padding-bottom: 0;
    --padding-start: 0;
  }
  ion-icon{
    font-size: 1.25rem;
  }
}

.breadcrumb{
  padding: 0;
  color: var(--parsec-color-light-secondary-grey);
  &-active{
    color: var(--parsec-color-light-primary-700)
  }
}

</style>
