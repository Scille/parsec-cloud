<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div
    v-if="appUpdated"
    id="app-updated-banner"
    class="update-banner"
  >
    <div class="update-banner-content">
      <ion-button
        id="app-updated-whats-new-button"
        class="update-banner-whats-new title-h4"
        fill="clear"
        @click="openChanges"
      >
        <ms-image
          :image="Sparkles"
          class="update-banner-whats-new-icon"
        />
        <span class="update-banner-whats-new-text">
          {{ $msTranslate({ key: 'HomePage.appUpdated.title', data: { version: APP_VERSION } }) }}
        </span>
        <ms-image
          :image="OpenIcon"
          class="update-banner-whats-new-icon update-banner-whats-new-icon--open"
        />
      </ion-button>
    </div>
    <ion-button
      id="app-updated-close-button"
      class="update-banner-close"
      fill="clear"
      :aria-label="$msTranslate('HomePage.appUpdated.close')"
      @click="closeUpdateBanner"
    >
      <ion-icon
        :icon="close"
        class="update-banner-close__icon"
      />
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import Sparkles from '@/assets/images/sparkles.svg?raw';
import { APP_VERSION, Env } from '@/services/environment';
import { useUpdateManager } from '@/services/updateManager';
import { IonButton, IonIcon } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { MsImage, OpenIcon } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const updateManager = useUpdateManager();
const appUpdated = ref<boolean>(false);

onMounted(async () => {
  appUpdated.value = await updateManager.hasAppBeenUpdated();
});

async function closeUpdateBanner(): Promise<void> {
  await updateManager.storeAppVersion();
  appUpdated.value = false;
}

async function openChanges(): Promise<void> {
  Env.Links.openChangelogLink();
}
</script>

<style lang="scss" scoped>
.update-banner {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: fit-content;
  flex-shrink: 0;
  border-radius: var(--parsec-radius-8);
  z-index: 10;
  background: linear-gradient(
    93deg,
    var(--parsec-color-surface-extra-purple-default, #6600cc) 1%,
    var(--parsec-color-surface-extra-purple-default-pressed, #330066) 100%
  );

  &-content {
    display: flex;
    align-items: center;
    align-self: stretch;
    width: 100%;
    justify-content: center;
  }

  &-whats-new {
    white-space: nowrap;
    color: var(--parsec-color-text-on-color-label, #f6f6f9);
    --background: transparent;
    --background-hover: color-mix(in srgb, var(--parsec-color-light-secondary-white) 20%, transparent);
    --border-radius: var(--parsec-radius-8);
    height: 100%;
    margin: 0;

    &::part(native) {
      padding: 0.75rem;
    }

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }

    &-text {
      margin-inline: 0.5rem;
    }

    &-icon {
      width: 1.125rem;
      height: 1.125rem;
      flex-shrink: 0;
      --fill-color: var(--parsec-color-light-secondary-white);

      &--open {
        position: relative;
        top: 0;
        right: 0;
        transition: all 0.2s ease-in-out;
      }
    }

    &:hover .update-banner-whats-new-icon--open {
      top: -0.125rem;
      right: -0.125rem;
    }
  }

  &-close {
    --color: var(--parsec-color-light-secondary-text);
    --color-hover: var(--parsec-color-light-secondary-text);
    --background: var(--parsec-color-light-secondary-white);
    --background-hover: var(--parsec-color-light-secondary-white);
    --border-radius: var(--parsec-radius-circle);
    --box-shadow: var(--parsec-shadow-light);
    position: absolute;
    top: -0.5rem;
    right: -0.5rem;
    min-height: 0;
    height: auto;
    margin: 0;

    &::part(native) {
      padding: 0.125rem;
    }

    &__icon {
      font-size: 1rem;
    }
  }
}
</style>
