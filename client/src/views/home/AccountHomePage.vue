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
          <account-login-page
            @login-success="onLoginSuccess"
            @skip-click="onSkipClicked"
            :disabled="true"
          />
          <div>
            <ion-button
              @click="goToCustomerArea"
              :disabled="disableGoTo"
            >
              GO TO CUSTOMER AREA
            </ion-button>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import HomePageSidebar from '@/views/home/HomePageSidebar.vue';
import AccountLoginPage from '@/views/account/AccountLoginPage.vue';
import HomePageHeader from '@/views/home/HomePageHeader.vue';
import { getCurrentRouteParams, getCurrentRouteQuery, navigateTo, Routes } from '@/router';
import { IonContent, IonPage, IonButton } from '@ionic/vue';
import { ParsecAccount } from '@/parsec';
import { onMounted, ref } from 'vue';

const disableGoTo = ref(false);

onMounted(async () => {
  if (ParsecAccount.isLoggedIn()) {
    await onLoginSuccess();
  }
});

async function onLoginSuccess(): Promise<void> {
  console.log('LOGIN');
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
}

async function onSkipClicked(): Promise<void> {
  console.log('SKIP');
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
}

async function goToCustomerArea(): Promise<void> {
  disableGoTo.value = true;
  const query = getCurrentRouteQuery();
  query.bmsLogin = true;
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: query });
}
</script>

<style lang="scss" scoped>
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
  }

  // Should be edited later with responsive
  .homepage-header {
    @include ms.responsive-breakpoint('lg') {
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

    @include ms.responsive-breakpoint('lg') {
      padding: 4.26rem 3rem 0;
    }

    @include ms.responsive-breakpoint('sm') {
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
