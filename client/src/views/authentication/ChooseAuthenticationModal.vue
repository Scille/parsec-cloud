<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="subtitle"
    :close-button="{ visible: true, onClick: onCloseClicked }"
    :confirm-button="{
      label: 'Authentication.chooseModal.continue',
      disabled: !authChosen,
      onClick: onAuthenticationChosen,
    }"
  >
    <div v-if="querying">
      <ms-spinner />
    </div>
    <choose-authentication
      ref="authChoice"
      v-if="!querying"
      :server-config="serverConfig"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import { getServerConfig, ParsecAddr, ServerConfig } from '@/parsec';
import { modalController } from '@ionic/core';
import { Answer, askQuestion, asyncComputed, MsModal, MsModalResult, MsSpinner, Translatable } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  title: Translatable;
  subtitle: Translatable;
  serverAddr: ParsecAddr;
}>();

const serverConfig = ref<ServerConfig | undefined>(undefined);
const querying = ref(true);
const authChoiceRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('authChoice');

const authChosen = asyncComputed(async () => {
  return Boolean(await authChoiceRef.value?.areFieldsCorrect());
});

onMounted(async () => {
  querying.value = true;
  const result = await getServerConfig(props.serverAddr);
  if (result.ok) {
    serverConfig.value = result.value;
  }
  querying.value = false;
});

async function onAuthenticationChosen(): Promise<boolean> {
  const strategy = await authChoiceRef.value?.getSaveStrategy();
  if (!strategy) {
    return false;
  }
  return await modalController.dismiss({ saveStrategy: strategy }, MsModalResult.Confirm);
}

async function onCloseClicked(): Promise<boolean> {
  const answer = await askQuestion('Authentication.chooseModal.interrupt.title', 'Authentication.chooseModal.interrupt.message', {
    yesText: 'Authentication.chooseModal.interrupt.yes',
    noText: 'Authentication.chooseModal.interrupt.no',
  });
  if (answer !== Answer.Yes) {
    return false;
  }
  return await modalController.dismiss(undefined, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss"></style>
