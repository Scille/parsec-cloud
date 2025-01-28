<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="sidebar-homepage">
    <ms-image
      :image="LogoRowWhite"
      class="logo-img"
    />
    <ion-text class="sidebar-homepage__tagline subtitles-lg">{{ $msTranslate('HomePage.sidebar.tagline') }}</ion-text>
  </div>
</template>

<script setup lang="ts">
import { MsImage, LogoRowWhite } from 'megashark-lib';
import { IonText } from '@ionic/vue';
</script>

<style lang="scss" scoped>
.sidebar-homepage {
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

  // Should be edited later with responsive
  @media screen and (max-width: 1500px) {
    max-width: 30rem;
  }

  &::before {
    content: '';
    position: absolute;
    height: 560px;
    width: 100%;
    max-height: 60vh;
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

  &__tagline {
    color: var(--parsec-color-light-secondary-white);
    text-align: center;
    margin-bottom: 25%;
  }
}
</style>
