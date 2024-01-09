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
      <div
        class="topbar-left__version body"
        @click="$emit('aboutClick')"
        v-if="!showBackButton"
      >
        <ion-icon
          slot="start"
          :icon="informationCircle"
          size="small"
        />
        <span class="version-text">
          {{ getAppVersion() }}
        </span>
      </div>
      <ion-button
        @click="$emit('backClick')"
        v-if="showBackButton"
        class="topbar-left__back-button"
      >
        <ion-icon
          slot="start"
          :icon="arrowBack"
        />
        {{ $t('HomePage.organizationLogin.backToList') }}
      </ion-button>
    </div>
    <div class="topbar-right">
      <ion-buttons class="topbar-right-buttons">
        <ion-button
          class="topbar-right-buttons__item"
          :disabled="true"
        >
          <ion-icon :icon="chatbubbles" />
          {{ $t('HomePage.topbar.contactUs') }}
        </ion-button>
        <ion-button
          slot="icon-only"
          id="trigger-settings-button"
          class="topbar-right-buttons__item"
          @click="$emit('settingsClick')"
        >
          <ion-icon :icon="cog" />
          {{ $t('HomePage.topbar.settings') }}
        </ion-button>
      </ion-buttons>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getAppVersion } from '@/common/mocks';
import { LogoRowWhite, MsImage } from '@/components/core';
import { IonButton, IonButtons, IonIcon } from '@ionic/vue';
import { arrowBack, chatbubbles, cog, informationCircle } from 'ionicons/icons';

defineProps<{
  showBackButton: boolean;
}>();

defineEmits<{
  (e: 'settingsClick'): void;
  (e: 'aboutClick'): void;
  (e: 'backClick'): void;
}>();
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
  align-items: center;
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

  &__version {
    cursor: pointer;
    color: var(--parsec-color-light-secondary-premiere);
    padding: 0.5rem 0.75rem;
    border-radius: var(--parsec-radius-32);
    border: 1px solid var(--parsec-color-light-primary-30-opacity15);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 150ms linear;

    .version-text {
      line-height: 0;
    }

    ion-icon {
      font-size: 1.375rem;
    }

    &:hover {
      border-color: var(--parsec-color-light-primary-100);
    }
  }
}

.topbar-right {
  display: flex;
  justify-content: flex-end;
  width: 100%;

  &-buttons {
    display: flex;
    gap: 1rem;

    &__item {
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
    }
  }
}
</style>
