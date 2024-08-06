<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-login-container">
    <bms-login
      @login-success="onLoginSuccess"
      v-show="!loginInProgress"
      :hide-header="true"
      class="saas-login"
    />
    <ms-spinner v-show="loginInProgress" />
  </div>
</template>

<script setup lang="ts">
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

<style scoped lang="scss">
.saas-login-container {
  display: flex;
  height: 100vh;
  width: 100%;
  max-width: 48rem;
  max-height: 32rem;
  margin: auto;
}

.saas-login {
  overflow: hidden;

  border-radius: var(--parsec-radius-12);
  overflow: hidden;

  &::before {
    z-index: 0;
  }
}
</style>
