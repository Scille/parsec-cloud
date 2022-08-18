<template>
  <ion-page>
    <ion-split-pane
      content-id="main"
    >
      <ion-menu content-id="main">
        <ion-header>
          <ion-toolbar color="primary">
            <ion-title>Parsec</ion-title>
          </ion-toolbar>
        </ion-header>
        <ion-content>
          <ion-list>
            <ion-item
              lines="none"
              button
              @click="navigateToPage('home')"
            >
              <ion-icon
                :icon="folderOutline"
                slot="start"
              />
              <ion-label>{{ $t('MenuPage.documents') }}</ion-label>
            </ion-item>
            <ion-item
              lines="none"
              button
              @click="navigateToPage('organization')"
            >
              <ion-icon
                :icon="peopleOutline"
                slot="start"
              />
              <ion-label>{{ $t('MenuPage.myOrganization') }}</ion-label>
            </ion-item>
            <ion-item-divider
              v-if="isPlatform('mobile')"
            />
            <ion-item
              lines="none"
              v-if="isPlatform('mobile')"
              button
              @click="navigateToPage('settings')"
            >
              <ion-icon
                :icon="settingsOutline"
                slot="start"
              />
              <ion-label>{{ $t('MenuPage.settings') }}</ion-label>
            </ion-item>
            <ion-item
              lines="none"
              v-if="isPlatform('mobile')"
              button
              @click="navigateToPage('support')"
            >
              <ion-icon
                :icon="helpCircleOutline"
                slot="start"
              />
              <ion-label>{{ $t('MenuPage.support') }}</ion-label>
            </ion-item>
            <ion-item
              lines="none"
              v-if="isPlatform('mobile')"
              button
              @click="navigateToPage('about')"
            >
              <ion-icon
                :icon="informationCircleOutline"
                slot="start"
              />
              <ion-label>{{ $t('MenuPage.about') }}</ion-label>
            </ion-item>
          </ion-list>
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
  IonPage,
  IonTitle,
  IonToolbar,
  IonIcon,
  IonItem,
  IonList,
  IonMenu,
  IonLabel,
  IonSplitPane,
  IonRouterOutlet,
  isPlatform,
  menuController,
  IonItemDivider
} from '@ionic/vue';
import {
  helpCircleOutline,
  informationCircleOutline,
  folderOutline,
  settingsOutline,
  peopleOutline
} from 'ionicons/icons';
import { useRouter } from 'vue-router';

function navigateToPage(pageName: string): void {
  router.push({ name: pageName });
  menuController.close();
}

const router = useRouter();
</script>
