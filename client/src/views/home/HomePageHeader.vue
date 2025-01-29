<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="topbar">
    <ion-button
      v-if="updateAvailability !== null"
      class="update-button form-label"
      id="trigger-update-button"
      fill="clear"
      @click="update()"
    >
      {{ $msTranslate('notificationCenter.newVersionAvailable') }}
    </ion-button>
    <div class="topbar-left">
      <ion-text
        class="topbar-left__title title-h1"
        v-if="!showBackButton"
      >
        {{ $msTranslate('HomePage.topbar.welcome') }}
      </ion-text>
      <!-- back button -->
      <ion-button
        @click="$emit('backClick')"
        v-if="showBackButton"
        class="topbar-left__back-button"
      >
        <ion-icon
          slot="start"
          :icon="arrowBack"
        />
        {{ $msTranslate(backButtonTitle ?? 'HomePage.topbar.backToList') }}
      </ion-button>
    </div>
    <div class="topbar-right">
      <ion-buttons class="topbar-buttons">
        <!-- about button -->
        <ion-button
          slot="icon-only"
          id="trigger-version-button"
          class="topbar-buttons__item body"
          fill="clear"
          @click="$emit('aboutClick')"
        >
          <ion-icon :icon="informationCircle" />
        </ion-button>
        <!-- doc button -->
        <ion-button
          class="topbar-buttons__item"
          @click="Env.Links.openDocumentationLink"
        >
          <ion-icon :icon="documentText" />
        </ion-button>
        <!-- contact button -->
        <ion-button
          class="topbar-buttons__item"
          @click="Env.Links.openContactLink"
        >
          <ion-icon :icon="chatbubbles" />
        </ion-button>
        <!-- settings button -->
        <ion-button
          id="trigger-settings-button"
          class="topbar-buttons__item"
          @click="$emit('settingsClick')"
        >
          <ion-icon :icon="cog" />
        </ion-button>
        <!-- customer area button -->
        <ion-button
          class="topbar-buttons__item"
          id="trigger-customer-area-button"
          @click="$emit('customerAreaClick')"
          v-if="!showBackButton"
        >
          <ion-icon :icon="personCircle" />
          {{ $msTranslate('HomePage.topbar.customerArea') }}
        </ion-button>
      </ion-buttons>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonIcon, modalController, IonText } from '@ionic/vue';
import { arrowBack, chatbubbles, cog, informationCircle, documentText, personCircle } from 'ionicons/icons';
import { EventData, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { Translatable, MsModalResult } from 'megashark-lib';
import { onMounted, onUnmounted, ref, inject, Ref } from 'vue';
import { Env } from '@/services/environment';
import { needsMocks } from '@/parsec';
import UpdateAppModal from '@/views/about/UpdateAppModal.vue';
import { APP_VERSION } from '@/services/environment';

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
  if (needsMocks()) {
    // Dispatch dummy update event to be able to debug the UpdateAppModal
    eventDistributor.dispatchEvent(Events.UpdateAvailability, { updateAvailable: true, version: 'v3.1.0' });
  }
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

async function update(): Promise<void> {
  if (!updateAvailability.value) {
    window.electronAPI.log('error', 'Missing update data when trying to update');
    return;
  }
  if (!updateAvailability.value.version) {
    window.electronAPI.log('error', 'Version missing from update data');
    return;
  }

  const modal = await modalController.create({
    component: UpdateAppModal,
    canDismiss: true,
    cssClass: 'update-app-modal',
    backdropDismiss: false,
    componentProps: {
      currentVersion: APP_VERSION,
      targetVersion: updateAvailability.value.version,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role === MsModalResult.Confirm) {
    window.electronAPI.prepareUpdate();
  }
}

defineProps<{
  showBackButton: boolean;
  backButtonTitle?: Translatable;
}>();

defineEmits<{
  (e: 'settingsClick'): void;
  (e: 'aboutClick'): void;
  (e: 'backClick'): void;
  (e: 'customerAreaClick'): void;
}>();
</script>

<style lang="scss" scoped>
.topbar {
  display: flex;
  justify-content: space-between;
  width: 100%;
  padding: 0 0 2rem;
  position: relative;
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
}

.update-button {
  background: var(--parsec-color-light-primary-50);
  color: var(--parsec-color-light-primary-700);
  min-height: 1rem;
  border: 1px solid var(--parsec-color-light-primary-100);
  padding: 0 0.825rem;
  border-radius: var(--parsec-radius-32);
  position: absolute;
  top: -3.5rem;
  left: 50%;
  transform: translate(-50%, 0);
  transition: all 150ms linear;

  &:hover {
    --background-hover: none;
    box-shadow: var(--parsec-shadow-light);
  }
}

.topbar-left {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 1.5rem;

  &__title {
    background: var(--parsec-color-light-gradient-background);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    margin: 0.25rem 0;
  }

  &__back-button {
    color: var(--parsec-color-light-secondary-soft-text);
    position: relative;
    --overflow: visible;
    margin: 1px 0;

    &::part(native) {
      background: none;
      --background-hover: none;
      border-radius: var(--parsec-radius-32);
      padding: 0.5rem 0.75rem;
    }

    ion-icon {
      margin-right: 0.5rem;
      position: absolute;
      left: -1.75rem;
      transition: left 150ms linear;
    }

    &:hover {
      border-color: transparent;

      ion-icon {
        left: -2.25rem;
      }
    }
  }
}

.topbar-right {
  display: flex;
  justify-content: flex-end;
  width: fit-content;
  margin-left: auto;
}

.topbar-buttons {
  display: flex;
  gap: 1rem;

  &:hover {
    border-color: var(--parsec-color-light-primary-100);
  }
}

.topbar-buttons__item {
  background: var(--parsec-color-light-secondary-white);
  color: var(--parsec-color-light-secondary-soft-text);
  border-radius: var(--parsec-radius-32);
  transition: all 150ms linear;

  &::part(native) {
    --background-hover: none;
    border-radius: var(--parsec-radius-32);
    padding: 0.5rem 0.75rem;
  }

  &:hover {
    background: var(--parsec-color-light-primary-100);
    color: var(--parsec-color-light-primary-600);
    box-shadow: var(--parsec-shadow-light);
  }

  ion-icon {
    font-size: 1.25rem;
  }

  &#trigger-customer-area-button {
    background: var(--parsec-color-light-primary-50);
    color: var(--parsec-color-light-primary-700);
    border: 1px solid var(--parsec-color-light-primary-100);

    ion-icon {
      margin-right: 0.5rem;
    }

    &:hover {
      color: var(--parsec-color-light-primary-600);
    }
  }
}
</style>
