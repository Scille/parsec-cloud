<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <bms-login
        @login-success="onLoginSuccess"
        v-show="!loginInProgress"
        :hide-header="true"
      />
      <ms-spinner v-show="loginInProgress" />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent } from '@ionic/vue';
import BmsLogin from '@/views/client-area/BmsLogin.vue';
import { AuthenticationToken, PersonalInformationResultData } from '@/services/bms';
import { ref, onMounted } from 'vue';
import { navigateTo, Routes } from '@/router';
import { MsSpinner } from 'megashark-lib';

const loginInProgress = ref(false);

onMounted(async () => {});

async function onLoginSuccess(_token: AuthenticationToken, _personalInformation: PersonalInformationResultData): Promise<void> {
  await goToClientArea();
}

async function goToClientArea(): Promise<void> {
  loginInProgress.value = true;
  setTimeout(async () => {
    await navigateTo(Routes.ClientArea, { replace: true });
  }, 500);
}
</script>

<style scoped lang="scss"></style>
