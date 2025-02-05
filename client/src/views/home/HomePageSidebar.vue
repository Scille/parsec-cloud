<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="sidebar-container">
    <ms-image
      :image="LogoRowWhite"
      class="logo-img"
    />
    <ion-text class="sidebar-tagline subtitles-lg">{{ $msTranslate('HomePage.sidebar.tagline') }}</ion-text>
  </div>
</template>

<script setup lang="ts">
import { MsImage, LogoRowWhite } from 'megashark-lib';
import { IonText } from '@ionic/vue';
</script>

<style lang="scss" scoped>
.sidebar-container {
  background: var(--parsec-color-light-gradient);
  width: 100%;
  max-width: 40rem;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: end;
  position: relative;
  gap: 1.5rem;
  transition: all 0.3s ease;

  &::before {
    content: '';
    position: absolute;
    height: 580px;
    width: 100%;
    max-height: 70vh;
    top: 0;
    right: 0;
    background-image: url('@/assets/images/background/shapes-circles.svg');
    background-size: cover;
    background-repeat: no-repeat;
    background-position: top left;
  }

  .logo-img {
    width: 8.5rem;
  }

  .sidebar-tagline {
    color: var(--parsec-color-light-secondary-white);
    text-align: center;
    margin-bottom: 20%;
  }
}
</style>
