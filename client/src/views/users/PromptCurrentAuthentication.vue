<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-password-input
    v-if="device.ty.tag === AvailableDeviceTypeTag.Password"
    v-model="password"
    @change="onPasswordChange"
    label="Password.currentPassword"
    @on-enter-keyup="onPasswordChange"
    ref="passwordInput"
  />
  <div
    class="provider-card"
    v-if="device.ty.tag === AvailableDeviceTypeTag.OpenBao"
  >
    <sso-provider-card
      :provider="(device.ty as AvailableDeviceTypeOpenBao).openbaoPreferredAuthId as OpenBaoAuthConfigTag"
      :is-connected="false"
      @sso-selected="onSSOLoginClicked"
    />
    <ms-spinner
      v-if="querying"
      class="provider-card-spinner"
    />
  </div>
  {{ $msTranslate(errorMessage) }}
</template>

<script setup lang="ts">
import { AvailableDevice, DevicePrimaryProtectionStrategy, AvailableDeviceTypeTag, PrimaryProtectionStrategy, AvailableDeviceTypeOpenBao, OpenBaoAuthConfigTag } from '@/parsec';
import {
} from '@ionic/vue';
import { onMounted, ref, useTemplateRef } from 'vue';
import { MsSpinner, MsPasswordInput } from 'megashark-lib';
import SsoProviderCard from '@/components/devices/SsoProviderCard.vue';
import { AvailableDeviceTypePKI, ServerConfig } from '@/plugins/libparsec';
import { openBaoConnect, OpenBaoErrorType } from '@/services/openBao';

const props = defineProps<{
  device: AvailableDevice;
  serverConfig?: ServerConfig;
}>();

const emits = defineEmits<{
  (e: 'authenticationSelected', protection?: DevicePrimaryProtectionStrategy): void;
}>();

const querying = ref(false);
const initialized = ref(false);
const errorMessage = ref('');
const password = ref('');
const passwordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('passwordInput');

onMounted(async () => {
  initialized.value = false;
  switch (props.device.ty.tag) {
    case AvailableDeviceTypeTag.Keyring:
      emits('authenticationSelected', PrimaryProtectionStrategy.useKeyring());
      return;
    case AvailableDeviceTypeTag.PKI:
      emits('authenticationSelected', PrimaryProtectionStrategy.useSmartcard((props.device.ty as AvailableDeviceTypePKI).certificateRef));
      return;
    case AvailableDeviceTypeTag.Password:
      if (passwordInputRef.value) {
        await passwordInputRef.value.setFocus();
      }
      break;
    default:
      break;
  }
  initialized.value = true;
});

async function onPasswordChange(): Promise<void> {
  emits('authenticationSelected', password.value ? PrimaryProtectionStrategy.usePassword(password.value) : undefined);
}

async function onSSOLoginClicked(): Promise<void> {
  if (querying.value) {
    window.electronAPI.log('warn', 'Clicked on SSO login while already login in');
    return;
  }
  if (!props.serverConfig || !props.serverConfig.openbao) {
    window.electronAPI.log('error', 'Server config or current device not found');
    return;
  }
  if (props.device.ty.tag !== AvailableDeviceTypeTag.OpenBao) {
    window.electronAPI.log('error', 'Device is not OpenBao device');
    return;
  }
  const provider = (props.device.ty as AvailableDeviceTypeOpenBao).openbaoPreferredAuthId;
  const auth = props.serverConfig.openbao.auths.find((v) => v.tag === provider);
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
        errorMessage.value = 'Authentication.popupBlocked';
      } else {
        errorMessage.value = 'Authentication.invalidOpenBaoData';
      }
      window.electronAPI.log('error', `Error while connecting with SSO: ${JSON.stringify(result.error)}`);
    } else {
      emits('authenticationSelected', PrimaryProtectionStrategy.useOpenBao(result.value.getConnectionInfo()));
    }
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">

</style>
