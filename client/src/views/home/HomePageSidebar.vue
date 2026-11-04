<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ion-text class="sidebar-tagline">
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
  background: linear-gradient(113.02deg, ms.color('surface-gradient-from') -1.49%, ms.color('surface-gradient-to') 100%);
  width: 100%;
  max-width: 40rem;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: end;
  position: relative;
  gap: ms.spacing('gap-4xl');
  transition: all 0.3s ease;

  body.ms-theme-dark & {
    background: linear-gradient(113deg, ms.color('surface-brand-default-subtle-pressed') -1.49%, ms.color('surface-base-page') 100%);
    border-right: ms.border('thin') solid ms.color('border-neutral-on-color');
  }

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
    gap: ms.spacing('gap-lg');
    position: relative;
    z-index: 3;
  }

  .logo-img {
    width: 8.5rem;
    position: relative;
    z-index: 3;
  }

  .sidebar-tagline {
    @include ms.font('body-xl-medium');
    color: ms.color('text-on-color-heading');
    text-align: center;
    margin-bottom: 30%;
    margin-inline: ms.spacing('padding-5xl');
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
    gap: ms.spacing('gap-lg');
  }

  .sidebar-tagline {
    @include ms.font('body-lg-medium');
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
