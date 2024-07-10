<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <bms-login
      v-if="step === Steps.BmsLogin"
      @login-success="onLoginSuccess"
    />
  </ion-page>
</template>

<script setup lang="ts">
// import { OrganizationID } from '@/parsec';
import { AuthenticationToken } from '@/services/bmsApi';
import { IonPage } from '@ionic/vue';
import { ref } from 'vue';
import BmsLogin from '@/views/bms/BmsLogin.vue';

enum Steps {
  BmsLogin,
  OrganizationName,
  PersonalInformation,
  Authentication,
  Summary,
  Creation,
}

defineProps<{
  bootstrapLink?: string;
}>();

// const organizationName = ref<OrganizationID | undefined>(undefined);
// const personalInformation = ref<PersonalInformationResultData | undefined>(undefined);
const authenticationToken = ref<AuthenticationToken | undefined>(undefined);
const step = ref<Steps>(Steps.BmsLogin);

async function onLoginSuccess(token: AuthenticationToken): Promise<void> {
  console.log('SUCCESS');
  authenticationToken.value = token;
  step.value = Steps.OrganizationName;
}
</script>

<style scoped lang="scss"></style>
