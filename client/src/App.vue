<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-app>
    <ion-router-outlet />
  </ion-app>
</template>

<script setup lang="ts">
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { translate } from '@/services/translation';
import { toggleDarkMode } from '@/states/darkMode';
import { SplashScreen } from '@capacitor/splash-screen';
import { IonApp, IonRouterOutlet } from '@ionic/vue';
import { inject, onMounted, onUnmounted } from 'vue';

const storageManager: StorageManager = inject(StorageManagerKey)!;
const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
let eventCbId: null | string = null;

onMounted(async (): Promise<void> => {
  SplashScreen.hide();

  const config = await storageManager.retrieveConfig();
  toggleDarkMode(config.theme);

  let ignoreOnline = true;

  eventCbId = await eventDistributor.registerCallback(
    Events.Offline | Events.Online | Events.IncompatibleServer,
    async (event: Events, _data: EventData) => {
      if (event === Events.Offline) {
        informationManager.present(
          new Information({
            message: translate('notification.serverOffline'),
            level: InformationLevel.Warning,
          }),
          PresentationMode.Notification,
        );
      } else if (event === Events.Online) {
        if (ignoreOnline) {
          ignoreOnline = false;
          return;
        }
        informationManager.present(
          new Information({
            message: translate('notification.serverOnline'),
            level: InformationLevel.Info,
          }),
          PresentationMode.Notification,
        );
      } else if (event === Events.IncompatibleServer) {
        informationManager.present(
          new Information({
            message: translate('notification.incompatibleServer'),
            level: InformationLevel.Error,
          }),
          PresentationMode.Notification,
        );
        await informationManager.present(
          new Information({
            message: translate('globalErrors.incompatibleServer'),
            level: InformationLevel.Error,
          }),
          PresentationMode.Modal,
        );
      }
    },
  );
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});
</script>
