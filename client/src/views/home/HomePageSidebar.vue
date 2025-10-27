<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- prettier-ignore -->
  <div
    class="sidebar-container"
    :class="{
      'sidebar-container--custom-logo': customLogo,
      'sidebar-container--custom-image': ResourcesManager.instance().get(Resources.HomeSidebar) !== undefined
    }"
  >
    <ms-image
      :image="(ResourcesManager.instance().get(Resources.LogoFull, LogoRowWhite) as string)"
      class="logo-img"
    />
    <div class="sidebar-bottom">
      <ion-text
        class="sidebar-tagline "
        :class="customLogo ? 'subtitles-normal' : 'subtitles-lg'"
      >
        {{ $msTranslate(customLogo ? 'HomePage.sidebar.poweredBy' : 'HomePage.sidebar.tagline') }}
      </ion-text>
      <ms-image
        @click="Env.Links.openDeveloperLink()"
        class="logo-icon"
        v-if="customLogo"
        :image="LogoRowWhite"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonText } from '@ionic/vue';
import { LogoRowWhite, MsImage } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const backgroundImage = ref();
const customLogo = ref(ResourcesManager.instance().get(Resources.LogoFull) !== undefined);

onMounted(() => {
  const sidebarImage = ResourcesManager.instance().get(Resources.HomeSidebar) as Uint8Array;
  if (sidebarImage) {
    const blob = new Blob([sidebarImage.buffer as ArrayBuffer], { type: 'image/png' });
    backgroundImage.value = `url("${window.URL.createObjectURL(blob)}")`;
  }
});
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

  @include ms.responsive-breakpoint('xxl') {
    max-width: 35rem;
  }

  @include ms.responsive-breakpoint('xl') {
    max-width: 30rem;

    &:before {
      height: 560px;
      max-height: 50vh;
    }
  }

  @include ms.responsive-breakpoint('lg') {
    max-width: 22rem;
  }

  @include ms.responsive-breakpoint('md') {
    max-width: 17rem;
  }

  @include ms.responsive-breakpoint('sm') {
    display: none;
  }

  &::before {
    content: '';
    position: absolute;
    height: 580px;
    width: 100%;
    max-height: 60vh;
    top: 0;
    right: 0;
    background-image: url('@/assets/images/background/shapes-circles.svg');
    background-size: cover;
    background-repeat: no-repeat;
    background-position: top left;
  }

  .sidebar-bottom {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    position: relative;
    z-index: 3;
  }

  .logo-img {
    width: 8.5rem;
    position: relative;
    z-index: 3;
  }

  .sidebar-tagline {
    color: var(--parsec-color-light-secondary-white);
    text-align: center;
    margin-bottom: 30%;
    margin-inline: 2rem;
  }
}

.sidebar-container--custom-image {
  background-image: v-bind(backgroundImage);
  background-size: cover;
  background-repeat: no-repeat;
  background-position: top left;

  &::before {
    content: '';
    opacity: 0.5;
    position: absolute;
    height: 100%;
    max-height: 100%;
    z-index: 0;
  }

  &::after {
    content: '';
    opacity: 0.5;
    background: linear-gradient(180deg, rgba(27, 27, 40, 0) 0%, var(--parsec-color-light-secondary-black) 100%);
    position: absolute;
    width: 100%;
    height: 15rem;
    flex-shrink: 0;
    bottom: 0;
    right: 0;
    z-index: 0;
  }
}

.sidebar-container--custom-logo {
  justify-content: end;
  padding-bottom: 5%;

  &::after {
    content: '';
    opacity: 0.5;
    background: linear-gradient(180deg, rgba(27, 27, 40, 0) 0%, var(--parsec-color-light-secondary-black) 100%);
    position: absolute;
    width: 100%;
    height: 15rem;
    flex-shrink: 0;
    bottom: 0;
    right: 0;
    z-index: 0;
  }

  .sidebar-bottom {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .sidebar-tagline {
    position: relative;
    z-index: 1;
    margin-bottom: 0;
    margin-inline: 0;
  }

  .logo-img {
    max-width: 7rem;
    max-height: 10rem;
    width: 100%;
    height: fit-content;
    z-index: 2;
  }

  .logo-icon {
    width: 6rem;
    position: relative;
    z-index: 2;
    cursor: pointer;
  }
}
</style>
