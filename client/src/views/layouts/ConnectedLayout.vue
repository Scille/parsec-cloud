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
import { needsMocks } from '@/parsec';
import { getConnectionHandle } from '@/router';
import { EventDistributor, EventDistributorKey } from '@/services/eventDistributor';
import { FileOperationManagerKey } from '@/services/fileOperationManager';
import { InformationManagerKey } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { IonContent, IonPage, IonRouterOutlet } from '@ionic/vue';
import { inject, onMounted, provide, ref } from 'vue';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const initialized = ref(false);

onMounted(async () => {
  const handle = getConnectionHandle();
  if (!handle) {
    console.error('Could not retrieve connection handle');
    return;
  }
  if (needsMocks() && !injectionProvider.hasInjections(handle)) {
    injectionProvider.createNewInjections(handle, new EventDistributor());
  }
  const inj = injectionProvider.getInjections(handle);
  // Provide the injections to children
  provide(FileOperationManagerKey, inj.fileOperationManager);
  provide(InformationManagerKey, inj.informationManager);
  provide(EventDistributorKey, inj.eventDistributor);
  initialized.value = true;
});
</script>
