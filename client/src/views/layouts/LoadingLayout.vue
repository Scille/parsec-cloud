<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="loading-container">
        <div class="loading-content">
          <!-- prettier-ignore -->
          <ms-image
            :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
            class="logo-img"
          />
          <ms-spinner title="HomePage.organizationLogin.loading" />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { RouteBackup, Routes, getCurrentRouteQuery, navigateTo } from '@/router';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonContent, IonPage } from '@ionic/vue';
import { Base64, LogoIconGradient, MsImage, MsSpinner } from 'megashark-lib';
import { onMounted } from 'vue';

onMounted(async () => {
  // When trying to switch from one connected org to another,
  // we have trouble remounting the components because the
  // url doesn't change enough according to vue-router (/1 to /2 for example),
  // which means it doesn't remount a new component and instead insists on re-using
  // an existing one. This is a pain for us as we don't want to add a watch
  // in every component.
  // To force a reload, we first navigate to a loading page (/loading),
  // then to the connected organization.
  // If we do it too fast, it causes a blink, so we masquerade this as a feature,
  // showing the user a "please wait" message.

  const query = getCurrentRouteQuery();
  if (query.loginInfo) {
    try {
      const loginInfo = Base64.toObject(query.loginInfo) as RouteBackup;
      setTimeout(
        async () => {
          await navigateTo(loginInfo.data.route, {
            params: loginInfo.data.params,
            query: loginInfo.data.query,
            skipHandle: true,
            replace: true,
          });
        },
        // 0 can cause loading problems with the org switch (Vue does not fully unmount in some cases)
        window.isDev() ? 0 : 1500,
      );
    } catch (e: any) {
      window.electronAPI.log('error', `Invalid log in info provided: ${e}`);
      await navigateTo(Routes.Home, { skipHandle: true, replace: true });
    }
  } else {
    window.electronAPI.log('error', 'Trying to log in with no log in info provided');
    await navigateTo(Routes.Home, { skipHandle: true, replace: true });
  }
});
</script>

<style scoped lang="scss">
.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  width: 100vw;
  height: 100vh;
  user-select: none;
}

@keyframes LogoFadeIn {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}

.loading-content {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  width: fit-content;
  gap: 0.5rem;

  .logo-img {
    animation: LogoFadeIn 0.8s ease-in-out;
    width: 3.25rem;
    height: 3.25rem;
  }
}
</style>
