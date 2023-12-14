<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="sidebar">
    <div class="sidebar-content">
      <div class="sidebar-content__titles">
        <h1 class="title-h1-lg">
          {{ $t('HomePage.organizationList.title') }}
        </h1>
        <ion-text class="subtitles-normal">
          {{ $t('HomePage.organizationList.subtitle') }}
        </ion-text>
      </div>
      <div class="sidebar-footer">
        <div class="sidebar-footer__logo">
          <ms-image
            :image="LogoRowWhite"
            class="logo-img"
          />
        </div>
        <div
          class="sidebar-footer__version body"
          @click="$emit('aboutClick')"
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
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon } from '@ionic/vue';
import { informationCircle } from 'ionicons/icons';
import { getAppVersion } from '@/common/mocks';
import { MsImage, LogoRowWhite } from '@/components/core';

defineEmits<{
  (e: 'aboutClick'): void;
}>();
</script>

<style lang="scss" scoped>
.sidebar {
  display: flex;
  height: 100vh;
  width: 40vw;
  padding: 2rem;
  background: var(--parsec-color-light-gradient);
  position: relative;
  justify-content: flex-end;
  z-index: -3;

  &::before {
    content: '';
    position: absolute;
    z-index: -2;
    top: 0;
    right: 0;
    width: 100vw;
    height: 100vh;
    background: url('@/assets/images/background/homepage-rectangle.svg') repeat center;
    background-size: cover;
  }

  .sidebar-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-left: 2rem;

    &__titles {
      display: flex;
      flex-direction: column;
      justify-content: center;
      flex-grow: 2;
      max-width: var(--parsec-max-title-width);
      position: relative;
      color: var(--parsec-color-light-secondary-premiere);
      gap: 1rem;
    }
  }

  .sidebar-footer {
    display: flex;
    width: 100%;
    justify-content: space-between;

    &__logo {
      display: flex;
      width: 100%;

      .logo-img {
        max-height: 3em;
        width: 30%;
        height: 100%;
      }
    }

    &__version {
      cursor: pointer;
      color: var(--parsec-color-light-secondary-premiere);
      padding: 0.3rem 0.5rem;
      border-radius: var(--parsec-radius-8);
      background-color: var(--parsec-color-light-primary-30-opacity15);
      border: 1px solid var(--parsec-color-light-primary-30-opacity15);
      display: flex;
      align-items: center;
      gap: 0.5rem;
      transition: all 150ms linear;

      .version-text {
        line-height: 0;
        padding: 0.8rem 0;
      }

      ion-icon {
        font-size: 1.375rem;
        color: var(--parsec-color-light-primary-100);
      }

      &:hover {
        border-color: var(--parsec-color-light-primary-100);
      }
    }
  }
}
</style>
