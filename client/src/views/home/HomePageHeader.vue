<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="topbar">
    <div class="topbar-left">
      <div
        class="topbar-left__logo"
        @click="$emit('backClick')"
        v-if="!showBackButton"
      >
        <ms-image
          :image="LogoRowWhite"
          class="logo-img"
        />
      </div>
      <!-- back button -->
      <ion-button
        slot="icon-only"
        id="trigger-version-button"
        class="topbar-buttons__item body"
        fill="clear"
        @click="$emit('aboutClick')"
        v-if="!showBackButton"
      >
        <ion-icon
          slot="start"
          :icon="informationCircle"
          size="small"
        />
      </ion-button>
      <!-- update button -->
      <ion-button
        v-if="updateAvailability !== null && !showBackButton"
        class="topbar-buttons__item"
        id="trigger-update-button"
        fill="clear"
        @click="update()"
      >
        <ion-icon :icon="sparkles" />
        {{ $msTranslate('notificationCenter.newVersionAvailable') }}
      </ion-button>
      <ion-button
        @click="$emit('backClick')"
        v-if="showBackButton"
        class="topbar-left__back-button"
      >
        <ion-icon
          slot="start"
          :icon="arrowBack"
        />
        {{ $msTranslate('HomePage.organizationLogin.backToList') }}
      </ion-button>
    </div>
    <div class="topbar-right">
      <ion-buttons class="topbar-buttons">
        <!-- contact button -->

        <ion-button
          class="topbar-buttons__item"
          @click="contactClicked"
        >
          <ion-icon :icon="chatbubbles" />
          {{ $msTranslate('HomePage.topbar.contactUs') }}
        </ion-button>
        <!-- settings button -->
        <ion-button
          id="trigger-settings-button"
          class="topbar-buttons__item"
          @click="$emit('settingsClick')"
        >
          <ion-icon :icon="cog" />
          {{ $msTranslate('HomePage.topbar.settings') }}
        </ion-button>
        <!-- customer area button -->
        <ion-button
          class="topbar-buttons__item"
          id="trigger-customer-area-button"
          @click="$emit('customerAreaClick')"
        >
          {{ $msTranslate('HomePage.topbar.customerArea') }}
        </ion-button>
      </ion-buttons>
    </div>
  </div>
</template>

<script setup lang="ts">
import { I18n, LogoRowWhite, MsImage } from 'megashark-lib';
import { IonButton, IonButtons, IonIcon } from '@ionic/vue';
import { arrowBack, chatbubbles, cog, informationCircle, sparkles } from 'ionicons/icons';
import { isElectron } from '@/parsec';
import { EventData, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { Answer, askQuestion } from 'megashark-lib';
import { onMounted, onUnmounted, ref, inject, Ref } from 'vue';
import { Env } from '@/services/environment';

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

defineProps<{
  showBackButton: boolean;
}>();

defineEmits<{
  (e: 'settingsClick'): void;
  (e: 'aboutClick'): void;
  (e: 'backClick'): void;
  (e: 'customerAreaClick'): void;
}>();

async function contactClicked(): Promise<void> {
  window.open(I18n.translate({ key: 'MenuPage.contactLink', data: { signUrl: Env.getSignUrl() } }), '_blank');
}
</script>

<style lang="scss" scoped>
.topbar {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  justify-content: space-between;
  width: 100%;
  max-width: var(--parsec-max-content-width);
  padding: 0;
  margin: 2rem auto 0;
}

.topbar-left {
  display: flex;
  margin: auto;
  width: 100%;
  gap: 1.5rem;

  &__logo {
    width: 8rem;
    height: 1.5rem;
    display: block;

    .logo-img {
      width: 100%;
      height: 100%;
    }
  }

  &__back-button {
    &::part(native) {
      background: none;
      --background-hover: none;
      border-radius: var(--parsec-radius-32);
      padding: 0.5rem 0.75rem;
    }

    ion-icon {
      margin-right: 0.5rem;
      transition: margin-right 150ms linear;
    }

    &:hover {
      border-color: transparent;

      ion-icon {
        margin-right: 0.75rem;
      }
    }
  }
}

.topbar-right {
  display: flex;
  justify-content: flex-end;
  width: 100%;
}

.topbar-buttons {
  display: flex;
  gap: 1rem;

  &:hover {
    border-color: var(--parsec-color-light-primary-100);
  }
}

.topbar-buttons__item {
  background: var(--parsec-color-light-primary-30-opacity15);
  color: var(--parsec-color-light-secondary-white);
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
    box-shadow: var(--parsec-shadow-strong);
  }

  ion-icon {
    font-size: 1.25rem;
    margin-right: 0.5rem;
  }

  &#trigger-customer-area-button {
    background: var(--parsec-color-light-secondary-white);
    color: var(--parsec-color-light-primary-600);
    align-self: stretch;

    &:hover {
      background: var(--parsec-color-light-secondary-premiere);
      outline: 1px solid var(--parsec-color-light-primary-100);
      outline-offset: 2px;
    }
  }

  &#trigger-version-button {
    ion-icon {
      margin: 0;
    }
  }

  &#trigger-update-button {
    background: linear-gradient(217deg, var(--parsec-color-light-primary-700), var(--parsec-color-light-primary-600)),
      linear-gradient(127deg, var(--parsec-color-light-primary-100), var(--parsec-color-light-primary-300));
    color: var(--parsec-color-light-secondary-white);
    align-self: stretch;
    transition: all 150ms linear;
    outline: 0px solid var(--parsec-color-light-primary-700);

    &:hover {
      outline-width: 1px;
      outline-offset: 2px;
    }
  }
}
</style>
