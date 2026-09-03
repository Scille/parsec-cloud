<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-spinner v-show="!initialized" />
  <ms-password-input
    v-show="initialized"
    v-if="device.ty.tag === AvailableDeviceTypeTag.Password"
    v-model="password"
    @change="onPasswordChange"
    label="Password.currentPassword"
    @on-enter-keyup="onEnterPressed"
    ref="passwordInput"
  />
  <div
    v-show="initialized"
    v-if="device.ty.tag === AvailableDeviceTypeTag.OpenBao"
    class="container"
  >
    <connect-sso
      v-if="serverConfig"
      :server-config="serverConfig"
      @open-bao-connected="onSSOConnected"
    />
    <ms-report-text
      v-else
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate('Authentication.method.sso.serverConfigUnavailable') }}
    </ms-report-text>
  </div>
</template>

<script setup lang="ts">
import ConnectSso from '@/components/devices/ConnectSso.vue';
import {
  AvailableDevice,
  AvailableDeviceTypePKI,
  AvailableDeviceTypeTag,
  DevicePrimaryProtectionStrategy,
  openCertificate,
  PrimaryProtectionStrategy,
  ServerConfig,
} from '@/parsec';
import { OpenBaoClient } from '@/services/openBao';
import { MsPasswordInput, MsReportText, MsReportTheme, MsSpinner } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  device: AvailableDevice;
  serverConfig?: ServerConfig;
}>();

const emits = defineEmits<{
  (e: 'authenticationSelected', protection?: DevicePrimaryProtectionStrategy): void;
  (e: 'enterPressed'): void;
}>();

const initialized = ref(false);
const password = ref('');
const passwordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('passwordInput');

onMounted(async () => {
  initialized.value = false;
  try {
    switch (props.device.ty.tag) {
      case AvailableDeviceTypeTag.Keyring:
        emits('authenticationSelected', PrimaryProtectionStrategy.useKeyring());
        return;
      case AvailableDeviceTypeTag.PKI: {
        const certResult = await openCertificate((props.device.ty as AvailableDeviceTypePKI).certificateRef);
        if (!certResult.ok) {
          break;
        }
        emits('authenticationSelected', PrimaryProtectionStrategy.useSmartcard(certResult.value));
        return;
      }
      case AvailableDeviceTypeTag.Password:
        if (passwordInputRef.value) {
          await passwordInputRef.value.setFocus();
        }
        break;
      default:
        break;
    }
  } finally {
    initialized.value = true;
  }
});

async function onPasswordChange(): Promise<void> {
  emits('authenticationSelected', password.value ? PrimaryProtectionStrategy.usePassword(password.value) : undefined);
}

async function onEnterPressed(): Promise<void> {
  emits('enterPressed');
}

async function onSSOConnected(client: OpenBaoClient): Promise<void> {
  emits('authenticationSelected', PrimaryProtectionStrategy.useOpenBao(client.getConnectionInfo()));
  emits('enterPressed');
}
</script>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
