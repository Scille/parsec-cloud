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
let ignoreOnlineEvent = true;

async function handleEvent(event: Events, _data: EventData): Promise<void> {
  switch (event) {
    case Events.Offline: {
      informationManager.present(
        new Information({
          message: translate('notification.serverOffline'),
          level: InformationLevel.Warning,
        }),
        PresentationMode.Notification,
      );
      break;
    }
    case Events.Online: {
      if (ignoreOnlineEvent) {
        ignoreOnlineEvent = false;
        return;
      }
      informationManager.present(
        new Information({
          message: translate('notification.serverOnline'),
          level: InformationLevel.Info,
        }),
        PresentationMode.Notification,
      );
      break;
    }
    case Events.IncompatibleServer: {
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
      break;
    }
    case Events.ExpiredOrganization: {
      informationManager.present(
        new Information({
          message: translate('notification.expiredOrganization'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Notification,
      );
      await informationManager.present(
        new Information({
          message: translate('globalErrors.expiredOrganization'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      break;
    }
    case Events.ClientRevoked: {
      informationManager.present(
        new Information({
          message: translate('notification.clientRevoked'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Notification,
      );
      await informationManager.present(
        new Information({
          message: translate('globalErrors.clientRevoked'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      break;
    }
  }
}

onMounted(async (): Promise<void> => {
  SplashScreen.hide();

  const config = await storageManager.retrieveConfig();
  toggleDarkMode(config.theme);

  eventCbId = await eventDistributor.registerCallback(
    Events.Offline | Events.Online | Events.IncompatibleServer | Events.ExpiredOrganization | Events.ClientRevoked,
    async (event: Events, data: EventData) => {
      await handleEvent(event, data);
    },
  );
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});
</script>
