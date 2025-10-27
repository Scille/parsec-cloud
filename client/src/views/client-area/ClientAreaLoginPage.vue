<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
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
</template>

<script setup lang="ts">
import { navigateTo, Routes } from '@/router';
import { AuthenticationToken, BmsAccessInstance, PersonalInformationResultData } from '@/services/bms';
import BmsLogin from '@/views/client-area/BmsLogin.vue';
import BmsForgotPassword from '@/views/client-area/forgot-password/BmsForgotPassword.vue';
import { onMounted, ref } from 'vue';

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
      return 'saas-login-page-container';
    case Sections.BmsForgotPassword:
      return 'saas-forgot-password-page-container';
  }
}
</script>

<style scoped lang="scss">
.saas-login-page-container,
.saas-forgot-password-page-container {
  display: flex;
  max-width: 48rem;
  margin: 0 auto;
  padding: 0.5rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem;
  }
}

.saas-login-page-container {
  width: 100%;
  overflow-y: auto;
}

.saas-login,
.saas-forgot-password {
  overflow: hidden;
  border-radius: var(--parsec-radius-12);
  height: fit-content;
  box-shadow: var(--parsec-shadow-light);

  &::before {
    z-index: 0;
  }
}
</style>
