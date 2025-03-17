<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div id="page">
        <!-- sidebar -->
        <home-page-sidebar class="homepage-sidebar" />
        <!-- main content -->
        <div class="homepage-content">
          <home-page-header
            class="homepage-header"
            :show-back-button="false"
            :display-create-join="false"
          />
          <div :class="getCurrentSectionClass()">
            <bms-login
              v-if="section === Sections.BmsLogin"
              @login-success="onLoginSuccess"
              :hide-header="true"
              class="saas-login"
              @forgotten-password-clicked="switchSection(Sections.BmsForgotPassword)"
            />
            <bms-forgot-password
              v-if="section === Sections.BmsForgotPassword"
              class="saas-forgot-password"
              :hide-header="true"
              @cancel="switchSection(Sections.BmsLogin)"
              @login-requested="switchSection(Sections.BmsLogin)"
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import BmsLogin from '@/views/client-area/BmsLogin.vue';
import BmsForgotPassword from '@/views/client-area/forgot-password/BmsForgotPassword.vue';
import { AuthenticationToken, BmsAccessInstance, PersonalInformationResultData } from '@/services/bms';
import { onMounted, ref } from 'vue';
import { navigateTo, Routes } from '@/router';
import HomePageSidebar from '@/views/home/HomePageSidebar.vue';
import HomePageHeader from '@/views/home/HomePageHeader.vue';
import { IonContent, IonPage } from '@ionic/vue';

const enum Sections {
  BmsLogin,
  BmsForgotPassword,
}

const section = ref<Sections>(Sections.BmsLogin);

onMounted(() => {
  section.value = Sections.BmsLogin;
});

async function onLoginSuccess(_token: AuthenticationToken, _personalInformation: PersonalInformationResultData): Promise<void> {
  await goToClientArea();
}

async function switchSection(newSection: Sections): Promise<void> {
  section.value = newSection;
}

async function goToClientArea(): Promise<void> {
  BmsAccessInstance.get().reloadKey += 1;
  await navigateTo(Routes.ClientArea, { replace: true, skipHandle: true });
}

function getCurrentSectionClass(): string {
  switch (section.value) {
    case Sections.BmsLogin:
      return 'saas-login-container';
    case Sections.BmsForgotPassword:
      return 'saas-forgot-password-container';
  }
}
</script>

<style scoped lang="scss">
.saas-login-container,
.saas-forgot-password-container {
  display: flex;
  max-width: 48rem;
  margin: 0 auto;
}

.saas-login-container {
  width: 100%;
  overflow-y: auto;
}

.saas-login,
.saas-forgot-password {
  overflow: hidden;
  border-radius: var(--parsec-radius-12);
  height: fit-content;

  &::before {
    z-index: 0;
  }
}

@import '@/theme/responsive-mixin';

#page {
  position: relative;
  height: 100vh;
  display: flex;
  overflow: hidden;
  align-items: self-start;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  z-index: -10;

  // Should be edited later with responsive
  .homepage-sidebar {
    @include breakpoint('ultra-wide') {
      max-width: 35rem;
    }

    @include breakpoint('wide') {
      max-width: 30rem;

      &:before {
        height: 560px;
        max-height: 50vh;
      }
    }

    @include breakpoint('lg') {
      max-width: 22rem;
    }

    @include breakpoint('md') {
      max-width: 17rem;
    }

    @include breakpoint('sm') {
      display: none;
    }
  }

  // Should be edited later with responsive
  .homepage-header {
    @include breakpoint('lg') {
      flex-direction: column-reverse;
      gap: 1rem;
    }
  }

  .homepage-content {
    width: 100%;
    height: 100%;
    position: relative;
    max-width: var(--parsec-max-content-width);
    padding: 2rem 5rem 0;
    display: flex;
    flex-direction: column;

    @include breakpoint('lg') {
      padding: 4.26rem 3rem 0;
    }

    @include breakpoint('sm') {
      padding: 1.5rem 1.5rem 0;
    }
  }

  &::before {
    content: '';
    position: absolute;
    height: 100%;
    width: 100%;
    max-width: 500px;
    max-height: 500px;
    bottom: 0;
    right: 0;
    background-image: url('@/assets/images/background/blob-shape.svg');
    background-size: contain;
    background-repeat: no-repeat;
    background-position: top center;
    opacity: 0.1;
    filter: blur(600px);
  }
}
</style>
