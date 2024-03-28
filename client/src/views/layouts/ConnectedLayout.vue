<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- Don't load children components before we inject everything -->
  <ion-page v-if="initialized">
    <ion-content>
      <ion-router-outlet />
    </ion-content>
  </ion-page>
</template>

<script lang="ts" setup>
import { getConnectionHandle } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { ImportManagerKey } from '@/services/importManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { translate } from '@/services/translation';
import { IonContent, IonPage, IonRouterOutlet } from '@ionic/vue';
import { inject, onMounted, onUnmounted, provide, ref } from 'vue';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
let informationManager: InformationManager;
let eventDistributor: EventDistributor;
let eventCbId: null | string = null;
const initialized = ref(false);

async function processEvents(): Promise<void> {
  let ignoreOnlineEvent = true;

  eventCbId = await eventDistributor.registerCallback(
    Events.Offline | Events.Online | Events.IncompatibleServer | Events.ExpiredOrganization | Events.ClientRevoked,
    async (event: Events, _data: EventData) => {
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
    },
  );
}

onMounted(async () => {
  const handle = getConnectionHandle();
  if (!handle) {
    console.error('Could not retrieve connection handle');
    return;
  }
  const inj = injectionProvider.getInjections(handle);
  // Provide the injections to children
  provide(ImportManagerKey, inj.importManager);
  provide(InformationManagerKey, inj.informationManager);
  provide(EventDistributorKey, inj.eventDistributor);
  initialized.value = true;
  informationManager = inj.informationManager;
  eventDistributor = inj.eventDistributor;
  await processEvents();
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});
</script>
