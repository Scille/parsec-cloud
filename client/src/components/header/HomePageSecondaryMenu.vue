<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="menu-secondary">
    <div class="menu-secondary-buttons">
      <!-- about button -->
      <ion-button
        id="trigger-version-button"
        class="menu-secondary-buttons-item link"
        @click="openAboutModal"
      >
        {{ $msTranslate('MenuPage.about') }}
      </ion-button>
      <!-- doc button -->
      <ion-button
        class="menu-secondary-buttons-item link"
        @click="Env.Links.openDocumentationLink"
      >
        {{ $msTranslate('MenuPage.documentation') }}
        <ion-icon
          :icon="open"
          class="button-icon-right"
        />
      </ion-button>
      <!-- contact button -->
      <ion-button
        class="menu-secondary-buttons-item link"
        @click="Env.Links.openContactLink"
      >
        {{ $msTranslate('MenuPage.contact') }}
        <ion-icon
          :icon="open"
          class="button-icon-right"
        />
      </ion-button>
      <!-- settings button -->
      <ion-button
        id="trigger-settings-button"
        class="menu-secondary-buttons-item link"
        @click="openSettings"
      >
        {{ $msTranslate('MenuPage.settings') }}
      </ion-button>
      <!-- bug report button -->
      <ion-button
        id="bug-report-button"
        class="menu-secondary-buttons-item link"
        @click="$emit('reportBugClick')"
      >
        {{ $msTranslate('MenuPage.reportBug') }}
      </ion-button>
      <!-- customer area button -->
      <ion-button
        class="menu-secondary-buttons-item link"
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
  (e: 'reportBugClick'): void;
}>();
</script>

<style lang="scss" scoped>
.menu-secondary {
  display: flex;
  padding: 0 0 ms.spacing('padding-5xl');
  justify-content: space-between;

  @include ms.responsive-breakpoint('md') {
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    padding: 0 0 ms.spacing('padding-3xl');
  }

  &-buttons {
    display: flex;
    gap: ms.spacing('gap-2xl');
    flex-wrap: wrap;
    -webkit-app-region: no-drag;

    &-item {
      color: ms.color('text-base-description');
      transition: all 150ms linear;
      position: relative;
      margin-right: ms.spacing('padding-lg');

      &::part(native) {
        padding: ms.spacing('padding-none');
        border-radius: ms.radius('none');
        background: none;
      }

      &:not(:last-child)::after {
        content: '';
        position: absolute;
        right: -0.5rem;
        top: 0;
        height: 1.5rem;
        width: ms.border('thin');
        background: ms.color('border-base-default');
        transition: all 150ms linear;

        @include ms.responsive-breakpoint('xs') {
          display: none;
        }
      }
    }

    #trigger-customer-area-button {
      color: ms.color('text-brand-default');
      position: relative;

      &::before {
        content: '';
        position: absolute;
        bottom: -0.25rem;
        left: 0;
        height: ms.border('thin');
        width: 0px;
        background: transparent;
        transition: all 150ms linear;
      }

      &:hover {
        color: ms.color('text-brand-default-hover');

        &::before {
          width: 100%;
          background: ms.color('surface-brand-default-hover');
        }
      }
    }
  }
}
</style>
