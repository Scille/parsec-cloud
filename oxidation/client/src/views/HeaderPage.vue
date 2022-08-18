<template>
  <ion-page>
    <ion-header>
      <ion-toolbar color="primary">
        <ion-buttons slot="start">
          <ion-menu-button />
        </ion-buttons>
        <ion-buttons slot="primary">
          <ion-button
            v-if="!isPlatform('mobile')"
            slot="icon-only"
            id="trigger-help-button"
          >
            <ion-icon
              slot="icon-only"
              :icon="helpCircleOutline"
            />
          </ion-button>

          <ion-popover
            v-if="!isPlatform('mobile')"
            trigger="trigger-help-button"
            :show-backdrop="false"
            :dismiss-on-select="true"
          >
            <ion-list>
              <ion-item
                lines="none"
                :button="true"
                :detail="false"
                @click="router.push({ name: 'about' })"
              >
                <ion-label>{{ $t('HeaderPage.about') }}</ion-label>
              </ion-item>
              <ion-item
                lines="none"
                :button="true"
                :detail="false"
                @click="router.push({ name: 'support' })"
              >
                <ion-label>{{ $t('HeaderPage.support') }}</ion-label>
              </ion-item>
            </ion-list>
          </ion-popover>
          <ion-button
            v-if="!isPlatform('mobile')"
            @click="router.push({ name: 'settings' })"
          >
            <ion-icon
              slot="icon-only"
              :icon="settingsOutline"
            />
          </ion-button>
          <ion-button>
            <ion-icon
              slot="icon-only"
              :icon="personCircleOutline"
            />
          </ion-button>
        </ion-buttons>
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
  IonPage
} from '@ionic/vue';
import {
  personCircleOutline,
  settingsOutline,
  helpCircleOutline
} from 'ionicons/icons';
import { useRouter } from 'vue-router';

const router = useRouter();
</script>
