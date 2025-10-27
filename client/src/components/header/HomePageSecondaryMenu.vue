<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="menu-secondary">
    <div class="menu-secondary-buttons">
      <!-- about button -->
      <ion-button
        id="trigger-version-button"
        class="menu-secondary-buttons-item"
        @click="openAboutModal"
      >
        {{ $msTranslate('MenuPage.about') }}
      </ion-button>
      <!-- doc button -->
      <ion-button
        class="menu-secondary-buttons-item"
        @click="Env.Links.openDocumentationLink"
      >
        {{ $msTranslate('MenuPage.documentation') }}
        <ion-icon :icon="open" />
      </ion-button>
      <!-- contact button -->
      <ion-button
        class="menu-secondary-buttons-item"
        @click="Env.Links.openContactLink"
      >
        {{ $msTranslate('MenuPage.contact') }}
        <ion-icon :icon="open" />
      </ion-button>
      <!-- settings button -->
      <ion-button
        id="trigger-settings-button"
        class="menu-secondary-buttons-item"
        @click="openSettings"
      >
        {{ $msTranslate('MenuPage.settings') }}
      </ion-button>
      <!-- customer area button -->
      <ion-button
        class="menu-secondary-buttons-item"
        v-show="!Env.isStripeDisabled()"
        id="trigger-customer-area-button"
        @click="$emit('customerAreaClick')"
      >
        {{ $msTranslate('HomePage.topbar.customerArea') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { openAboutModal } from '@/views/about';
import { openSettingsModal } from '@/views/settings';
import { IonButton, IonIcon } from '@ionic/vue';
import { open } from 'ionicons/icons';

async function openSettings(): Promise<void> {
  if (!Env.isAccountEnabled()) {
    return openSettingsModal();
  } else {
    emits('settingsClick');
  }
}

const emits = defineEmits<{
  (e: 'customerAreaClick'): void;
  (e: 'settingsClick'): void;
}>();
</script>

<style lang="scss" scoped>
.menu-secondary {
  display: flex;
  padding: 0 0 2rem;
  justify-content: space-between;

  @include ms.responsive-breakpoint('md') {
    flex-direction: column;
    gap: 1rem;
    padding: 0 0 1rem;
  }

  &-buttons {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;

    &-item {
      color: var(--parsec-color-light-secondary-hard-grey);
      transition: all 150ms linear;
      position: relative;
      --background-hover: none;
      display: flex;
      align-items: center;
      gap: 1rem;

      &::part(native) {
        padding: 0;
        border-radius: 0;
        background: none;
        --background-hover: none;
      }

      ion-icon {
        margin-left: 0.5rem;
        font-size: 1rem;
        color: var(--parsec-color-light-secondary-soft-grey);
      }

      &:not(:last-child)::after {
        content: '';
        position: relative;
        display: block;
        height: 1rem;
        width: 1px;
        background: var(--parsec-color-light-secondary-disabled);
        transition: all 150ms linear;

        @include ms.responsive-breakpoint('xs') {
          display: none;
        }
      }

      &:hover {
        color: var(--parsec-color-light-secondary-text);

        ion-icon {
          color: var(--parsec-color-light-secondary-soft-text);
        }
      }
    }

    #trigger-customer-area-button {
      color: var(--parsec-color-light-primary-500);
      border-radius: var(--parsec-radius-8);
      position: relative;

      &::before {
        content: '';
        position: absolute;
        bottom: -0.25rem;
        height: 1px;
        width: 0px;
        background: transparent;
        transition: all 150ms linear;
      }

      &:hover {
        color: var(--parsec-color-light-primary-600);

        &::before {
          width: 100%;
          background: var(--parsec-color-light-primary-600);
        }
      }
    }
  }
}
</style>
