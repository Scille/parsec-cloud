<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div
    class="connect-sso"
    v-if="serverConfig.openbao"
  >
    <div class="sso-provider-list">
      <sso-provider-card
        v-for="provider in serverConfig.openbao.auths"
        :key="provider.id"
        :provider="provider"
        :is-connected="openBaoClient?.getConnectionInfo()?.provider === provider.id"
        @sso-selected="onSSOLoginClicked"
        :class="{ disabled: querying }"
      />
      <ms-spinner
        v-if="querying"
        class="spinner"
      />
    </div>
    <ms-report-text
      v-if="error"
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>
  </div>
</template>

<script setup lang="ts">
import SsoProviderCard from '@/components/devices/SsoProviderCard.vue';
import { OpenBaoAuthConfig, ServerConfig } from '@/parsec';
import { OpenBaoClient, openBaoConnect, OpenBaoError, OpenBaoErrorType } from '@/services/openBao';
import { MsReportText, MsReportTheme, MsSpinner, Translatable } from 'megashark-lib';
import { ref } from 'vue';

const openBaoClient = ref<OpenBaoClient | undefined>(undefined);
const querying = ref(false);
const error = ref<Translatable | undefined>(undefined);

const props = defineProps<{
  serverConfig: ServerConfig;
}>();

const emits = defineEmits<{
  (e: 'openBaoConnected', client: OpenBaoClient): void;
  (e: 'openBaoConnectError', error: OpenBaoError): void;
}>();

defineExpose({
  getOpenBaoClient,
});

function getOpenBaoClient(): OpenBaoClient | undefined {
  return openBaoClient.value;
}

async function onSSOLoginClicked(provider: OpenBaoAuthConfig): Promise<void> {
  if (querying.value) {
    window.nativeAPI.log('warn', 'Clicked on SSO login while already login in');
    return;
  }
  if (!props.serverConfig?.openbao) {
    window.nativeAPI.log('error', 'OpenBao not enabled on this server');
    return;
  }
  try {
    error.value = '';
    querying.value = true;
    const result = await openBaoConnect(
      props.serverConfig.openbao.serverUrl,
      provider.id,
      provider.mountPath,
      props.serverConfig.openbao.secret.mountPath,
      props.serverConfig.openbao.transitMountPath,
    );
    if (!result.ok) {
      if (result.error.type === OpenBaoErrorType.PopupFailed) {
        error.value = 'Authentication.popupBlocked';
      } else {
        error.value = 'Authentication.invalidOpenBaoData';
      }
      window.nativeAPI.log('error', `Error while connecting with SSO: ${JSON.stringify(result.error)}`);
      emits('openBaoConnectError', result.error);
      return;
    }
    openBaoClient.value = result.value;
    emits('openBaoConnected', openBaoClient.value);
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.connect-sso {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  .sso-provider-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    position: relative;
    align-items: center;

    .disabled {
      pointer-events: none;
      opacity: 0.2;
    }
  }

  .spinner {
    position: absolute;
    top: 1.75rem;
    left: 50%;
    transform: translate(-50%, -50%);
    background: var(--parsec-color-light-secondary-white);
    border-radius: 50%;
    padding: 0.125rem;
  }
}
</style>
