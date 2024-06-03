<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="updateAvailability !== null"
    class="banner"
  >
    <!-- TODO: replace LogoIconGradient by a LogoIconWhite when it will be existing in megashark-lib -->
    <ms-image
      :image="LogoIconGradient"
      class="logo-img"
    />
    <ion-label>{{ $msTranslate('notificationCenter.newVersionAvailable') }}</ion-label>
    <ion-button @click="update()">{{ $msTranslate('notificationCenter.update') }}</ion-button>
  </div>
</template>

<script setup lang="ts">
import { EventData, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { IonButton, IonLabel } from '@ionic/vue';
import { Answer, LogoIconGradient, askQuestion, MsImage } from 'megashark-lib';
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
  .logo-img {
    width: 2.5em;
  }

  display: flex;
  width: 32em;
  padding: 1em 1.5em;
  font-weight: bold;
  color: var(--parsec-color-light-secondary-white);
  gap: 1em;
  align-items: center;
  border-radius: var(--parsec-radius-8);
  border-left: 3px solid var(--parsec-color-light-secondary-white);
  background: var(--parsec-color-light-primary-30-opacity15);
  box-shadow: var(--parsec-shadow-light);

  // TODO: improve :hover colors when mockups will be available
  ion-button {
    margin-left: auto;
    --background: var(--parsec-color-light-secondary-white);
    --color: var(--parsec-color-light-primary-600);
  }
}
</style>
