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
import { EventDistributorKey } from '@/services/eventDistributor';
import { ImportManagerKey } from '@/services/importManager';
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
  const inj = injectionProvider.getInjections(handle);
  // Provide the injections to children
  provide(ImportManagerKey, inj.importManager);
  provide(InformationManagerKey, inj.informationManager);
  provide(EventDistributorKey, inj.eventDistributor);
  initialized.value = true;
});
</script>
