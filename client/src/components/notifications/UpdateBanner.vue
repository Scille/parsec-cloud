<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="updateAvailability !== null"
    class="banner"
  >
    <ion-label>{{ $msTranslate('notificationCenter.newVersionAvailable') }}</ion-label>
    <ion-button @click="update()">{{ $msTranslate('notificationCenter.update') }}</ion-button>
  </div>
</template>

<script setup lang="ts">
import { EventData, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { IonButton, IonLabel } from '@ionic/vue';
import { Answer, askQuestion } from 'megashark-lib';
import { onMounted, onUnmounted, ref, inject, Ref } from 'vue';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const eventDistributor = injectionProvider.getDefault().eventDistributor;
let eventCbId: string | null = null;
const updateAvailability: Ref<UpdateAvailabilityData | null> = ref(null);

onMounted(async () => {
  eventCbId = await eventDistributor.registerCallback(Events.UpdateAvailability, async (event: Events, data?: EventData) => {
    if (event === Events.UpdateAvailability) {
      updateAvailability.value = data as UpdateAvailabilityData;
    }
  });
  window.electronAPI.getUpdateAvailability();
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

async function update(): Promise<void> {
  const answer = await askQuestion('HomePage.topbar.updateConfirmTitle', 'HomePage.topbar.updateConfirmQuestion', {
    yesText: 'HomePage.topbar.updateYes',
    noText: 'HomePage.topbar.updateNo',
  });
  if (answer === Answer.Yes) {
    window.electronAPI.prepareUpdate();
  }
}
</script>

<style scoped lang="scss">
.banner {
  width: 600px;
  height: 64px;
  color: green;
  border: 10px dotted purple;
  background-color: pink;
  display: flex;
  font-weight: bold;
  animation: blinker 0.1s linear infinite;
}

@keyframes blinker {
  50% {
    opacity: 0;
    background-color: orange;
    border: 3px dotted red;
    width: 650px;
    height: 128px;
  }
}
</style>
