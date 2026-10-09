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
        class="update-banner-whats-new"
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
      slot="icon-only"
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
  width: 100%;
  flex-shrink: 0;
  overflow: hidden;
  z-index: 10;
  background: linear-gradient(93deg, ms.color('surface-extra-purple-default') 1%, ms.color('surface-extra-purple-default-pressed') 100%);

  &-content {
    display: flex;
    align-items: center;
    align-self: stretch;
    width: 100%;
    justify-content: center;
  }

  &-whats-new {
    white-space: nowrap;
    @include ms.font('heading-h5');
    color: ms.color('text-on-color-label');
    --background: transparent;
    --background-hover: color-mix(in srgb, var(--parsec-color-light-secondary-white) 20%, transparent);
    --border-radius: #{ms.radius('none')};
    height: 100%;

    &::part(native) {
      padding: ms.spacing('padding-2xl');
    }

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }

    &-text {
      margin-inline: ms.spacing('padding-lg');
    }

    &-icon {
      width: 1.125rem;
      height: 1.125rem;
      flex-shrink: 0;
      --fill-color: #{ms.color('icon-on-color-body')};

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
    --color: #{ms.color('icon-on-color-label')};
    --color-hover: #{ms.color('surface-base-default')};
    --background: transparent;
    --background-hover: color-mix(in srgb, var(--parsec-color-light-secondary-white) 20%, transparent);
    position: absolute;
    right: ms.spacing('padding-2xl');
    height: auto;
    margin: 0;

    &::part(native) {
      padding: ms.spacing('padding-xs');
    }

    &:hover {
      opacity: 1;
    }
  }
}
</style>
