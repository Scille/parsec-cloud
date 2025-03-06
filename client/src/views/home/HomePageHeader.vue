<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="homepage-header">
    <div class="menu-secondary">
      <ion-button
        v-if="updateAvailability !== null"
        class="update-button form-label"
        id="trigger-update-button"
        fill="clear"
        @click="update()"
      >
        {{ $msTranslate('notificationCenter.newVersionAvailable') }}
      </ion-button>
      <ion-buttons class="menu-secondary-buttons">
        <!-- about button -->
        <ion-button
          id="trigger-version-button"
          class="menu-secondary-buttons__item"
          @click="$emit('aboutClick')"
        >
          {{ $msTranslate('MenuPage.about') }}
        </ion-button>
        <!-- doc button -->
        <ion-button
          class="menu-secondary-buttons__item"
          @click="Env.Links.openDocumentationLink"
        >
          {{ $msTranslate('MenuPage.documentation') }}
          <ion-icon :icon="open" />
        </ion-button>
        <!-- contact button -->
        <ion-button
          class="menu-secondary-buttons__item"
          @click="Env.Links.openContactLink"
        >
          {{ $msTranslate('MenuPage.contact') }}
          <ion-icon :icon="open" />
        </ion-button>
        <!-- settings button -->
        <ion-button
          id="trigger-settings-button"
          class="menu-secondary-buttons__item"
          @click="$emit('settingsClick')"
        >
          {{ $msTranslate('MenuPage.settings') }}
        </ion-button>
      </ion-buttons>
    </div>
    <div class="topbar">
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
        <ion-button
          @click="emits('createOrJoinOrganizationClick', $event)"
          size="default"
          id="create-organization-button"
          class="button-default"
          v-if="displayCreateJoin"
        >
          <ion-icon :icon="add" />
          {{ $msTranslate('HomePage.noExistingOrganization.createOrJoin') }}
        </ion-button>
        <!-- customer area button -->
        <ion-button
          v-show="!Env.isStripeDisabled()"
          id="trigger-customer-area-button"
          @click="$emit('customerAreaClick')"
          v-if="!showBackButton"
        >
          {{ $msTranslate('HomePage.topbar.customerArea') }}
        </ion-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonIcon, modalController, IonText } from '@ionic/vue';
import { add, arrowBack, open } from 'ionicons/icons';
import { EventData, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { Translatable, MsModalResult } from 'megashark-lib';
import { onMounted, onUnmounted, ref, inject, Ref } from 'vue';
import { Env } from '@/services/environment';
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
  displayCreateJoin: boolean;
}>();

const emits = defineEmits<{
  (e: 'settingsClick'): void;
  (e: 'aboutClick'): void;
  (e: 'backClick'): void;
  (e: 'customerAreaClick'): void;
  (e: 'createOrJoinOrganizationClick', event: Event): void;
}>();
</script>

<style lang="scss" scoped>
.menu-secondary {
  display: flex;
  justify-content: flex-end;
  width: 100%;
  padding: 0 0 2rem;

  .update-button {
    margin-right: auto;
    min-height: 1rem;
    background: var(--parsec-color-light-primary-50);
    color: var(--parsec-color-light-primary-700);
    min-height: 1rem;
    border: 1px solid var(--parsec-color-light-primary-100);
    padding: 0 0.325rem;
    border-radius: var(--parsec-radius-32);
    transition: all 150ms linear;

    &:hover {
      --background-hover: none;
      box-shadow: var(--parsec-shadow-light);
    }
  }

  &-buttons {
    display: flex;

    &__item {
      background: none;
      color: var(--parsec-color-light-secondary-hard-grey);
      transition: all 150ms linear;
      position: relative;
      padding: 0 0.5rem;

      &::part(native) {
        --background-hover: none;
      }

      ion-icon {
        margin-left: 0.5rem;
        font-size: 1rem;
        color: var(--parsec-color-light-secondary-soft-grey);
      }

      &:hover {
        color: var(--parsec-color-light-secondary-text);

        ion-icon {
          color: var(--parsec-color-light-secondary-hard-grey);
        }
      }

      &:not(:first-child)::after {
        content: '';
        position: absolute;
        top: 50%;
        transform: translateY(-50%);
        left: 0;
        height: 80%;
        width: 1px;
        background: var(--parsec-color-light-secondary-disabled);
        transition: all 150ms linear;
      }
    }
  }
}

.topbar {
  display: flex;
  justify-content: space-between;
  width: 100%;
  padding: 0 0 2rem;
  margin-bottom: 2rem;
  position: relative;
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);

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
    gap: 1.25rem;
    justify-content: flex-end;
    width: fit-content;
    margin-left: auto;
  }
}

#create-organization-button {
  color: var(--parsec-color-light-secondary-soft-text);
  border-radius: var(--parsec-radius-32);
  transition: all 150ms linear;
  border: 1px solid transparent;

  &::part(native) {
    --background: var(--parsec-color-light-secondary-premiere);
    --background-hover: none;
    border-radius: var(--parsec-radius-32);
    padding: 0.5rem 0.825rem;
  }

  ion-icon {
    margin-right: 0.5rem;
  }

  &:hover {
    border: 1px solid var(--parsec-color-light-secondary-light);
    color: var(--parsec-color-light-secondary-text);
  }
}

#trigger-customer-area-button {
  color: var(--parsec-color-light-secondary-soft-text);
  border-radius: var(--parsec-radius-32);
  transition: all 150ms linear;
  --background: var(--parsec-color-light-primary-50);
  color: var(--parsec-color-light-primary-700);
  border: 1px solid var(--parsec-color-light-primary-100);

  &::part(native) {
    --background-hover: none;
    border-radius: var(--parsec-radius-32);
    padding: 0.5rem 0.825rem;
  }

  ion-icon {
    margin-right: 0.5rem;
  }

  &:hover {
    color: var(--parsec-color-light-primary-600);
    box-shadow: var(--parsec-shadow-light);
  }
}
</style>
