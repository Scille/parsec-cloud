<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="provider-card"
    v-if="serverConfig.openbao"
  >
    <sso-provider-card
      v-for="auth in serverConfig.openbao.auths.filter((auth) => isSSOProviderHandled(auth.tag))"
      :key="auth.tag"
      :provider="auth.tag"
      :is-connected="openBaoClient !== undefined && openBaoClient.getConnectionInfo().provider === auth.tag"
      @sso-selected="onSSOLoginClicked"
    />
    <ms-spinner
      v-if="querying"
      class="provider-card-spinner"
    />
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
import { OpenBaoAuthConfigTag, ServerConfig } from '@/parsec';
import { isSSOProviderHandled, OpenBaoClient, openBaoConnect, OpenBaoError, OpenBaoErrorType } from '@/services/openBao';
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

async function onSSOLoginClicked(provider: OpenBaoAuthConfigTag): Promise<void> {
  if (querying.value) {
    window.electronAPI.log('warn', 'Clicked on SSO login while already login in');
    return;
  }
  if (!props.serverConfig?.openbao) {
    window.electronAPI.log('error', 'OpenBao not enabled on this server');
    return;
  }
  const auth = props.serverConfig.openbao.auths.find((auth) => auth.tag === provider);
  if (!auth) {
    window.electronAPI.log('error', `Provider '${provider}' selected but is not available in server config`);
    return;
  }
  try {
    querying.value = true;
    const result = await openBaoConnect(
      props.serverConfig.openbao.serverUrl,
      auth.tag,
      auth.mountPath,
      props.serverConfig.openbao.secret.mountPath,
      props.serverConfig.openbao.transitMountPath,
    );
    if (!result.ok) {
      if (result.error.type === OpenBaoErrorType.PopupFailed) {
        error.value = 'Authentication.popupBlocked';
      } else {
        error.value = 'Authentication.invalidOpenBaoData';
      }
      window.electronAPI.log('error', `Error while connecting with SSO: ${JSON.stringify(result.error)}`);
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
.provider-card {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  position: relative;

  &-spinner {
    position: absolute;
    top: 1.125rem;
    right: 2rem;
  }
}
</style>
