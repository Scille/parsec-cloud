<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :confirm-button="{
      label: 'CHOOSE AUTH',
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
import { asyncComputed, MsModal, MsModalResult, MsSpinner, Translatable } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  title: Translatable;
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
</script>

<style scoped lang="scss"></style>
