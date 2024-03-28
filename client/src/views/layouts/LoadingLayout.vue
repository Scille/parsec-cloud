<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="loading-container">
        <div class="loading-content">
          <ms-image
            :image="LogoIconGradient"
            class="logo-img"
          />
          <ms-spinner :title="$t('HomePage.organizationLogin.loading')" />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { Base64 } from '@/common/base64';
import { MsImage, LogoIconGradient } from '@/components/core/ms-image';
import { MsSpinner } from '@/components/core';
import { getCurrentRouteQuery, navigateTo, RouteBackup, Routes } from '@/router';
import { IonContent, IonPage } from '@ionic/vue';
import { onMounted } from 'vue';

onMounted(async () => {
  setTimeout(async () => {
    const query = getCurrentRouteQuery();
    if (query.loginInfo) {
      const loginInfo = Base64.toObject(query.loginInfo) as RouteBackup;
      await navigateTo(loginInfo.data.route, {
        params: loginInfo.data.params,
        query: loginInfo.data.query,
        skipHandle: true,
        replace: true,
      });
    } else {
      await navigateTo(Routes.Home, { skipHandle: true, replace: true });
    }
  }, 1000);
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
