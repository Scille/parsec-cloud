<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        <ms-image
          class="menu-secondary-buttons-item__open-icon button-icon-right"
          :image="OpenIcon"
        />
      </ion-button>
      <!-- contact button -->
      <ion-button
        class="menu-secondary-buttons-item link"
        @click="Env.Links.openContactLink"
      >
        {{ $msTranslate('MenuPage.contact') }}
        <ms-image
          class="menu-secondary-buttons-item__open-icon button-icon-right"
          :image="OpenIcon"
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
import { IonButton } from '@ionic/vue';
import { MsImage, OpenIcon } from 'megashark-lib';

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
  align-items: center;

  &-buttons {
    --spacing-between: #{ms.spacing('gap-4xl')};
    display: flex;
    align-items: center;
    gap: var(--spacing-between);
    flex-wrap: wrap;
    -webkit-app-region: no-drag;

    &-item {
      @include ms.font('label-md-medium');
      color: ms.color('text-neutral-default');
      min-height: auto;
      margin: 0;
      transition: all 150ms linear;
      position: relative;
      --color-hover: #{ms.color('text-brand-default')};

      &::part(native) {
        padding: ms.spacing('padding-none') !important;
        border-radius: ms.radius('none');
        background: none;
      }

      &__open-icon {
        width: 1rem;
        height: 1rem;
        --fill-color: #{ms.color('icon-neutral-default')};
        flex-shrink: 0;
        position: relative;
        top: 0;
        right: 0;
        transition: all 150ms linear;
      }

      &:hover &__open-icon {
        top: -0.125rem;
        right: -0.125rem;
      }

      &:not(:last-child)::after {
        content: '';
        position: absolute;
        right: calc(var(--spacing-between) / (-2));
        top: 50%;
        transform: translateY(-50%);
        height: 1rem;
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
