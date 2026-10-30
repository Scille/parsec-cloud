<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="login-popup">
    <!-- login -->
    <div class="login-header">
      <ion-text class="login-header__title title-h1">
        {{ $msTranslate('HomePage.organizationLogin.login') }}
      </ion-text>
    </div>
    <ion-card class="login-card">
      <ion-card-header class="login-card-header">
        <organization-card
          :device="device"
          :org-name-only="true"
        />
      </ion-card-header>
      <ion-card-content class="login-card-content">
        <ms-input
          :label="'HomePage.organizationLogin.emailLabel'"
          v-model="email"
          id="ms-input"
          :disabled="true"
        />
        <div
          class="login-card-content__password"
          v-if="device.ty.tag === AvailableDeviceTypeTag.Password"
        >
          <ms-password-input
            :label="'HomePage.organizationLogin.passwordLabel'"
            ref="passwordInput"
            v-model="password"
            @on-enter-keyup="onLoginClick()"
            id="password-input"
            @change="onPasswordChange"
            :password-is-invalid="passwordIsInvalid"
          />
          <ion-button
            fill="clear"
            @click="$emit('forgottenPasswordClick', device)"
            id="forgotten-password-button"
            class="button-medium"
          >
            {{ $msTranslate('HomePage.organizationLogin.forgottenPassword') }}
          </ion-button>
        </div>
      </ion-card-content>
      <ion-footer class="login-card-footer">
        <ms-report-text
          :theme="MsReportTheme.Error"
          v-if="errorMessage"
          class="error"
        >
          {{ $msTranslate(errorMessage) }}
        </ms-report-text>
        <div v-if="device.ty.tag === AvailableDeviceTypeTag.OpenBao">
          <div
            v-if="!loading && serverConfig?.openbao && serverConfig.openbao.auths.length > 0"
            class="sso-provider-list"
          >
            <sso-provider-card
              v-for="provider of serverConfig.openbao.auths"
              :key="provider.id"
              :provider="provider"
              @sso-selected="onLoginSSOClick"
            />
          </div>
          <div v-else-if="loading">
            <ms-spinner />
          </div>
          <div v-else-if="!loading && !serverConfig">
            {{ $msTranslate('Authentication.method.sso.serverConfigUnavailable') }}
            <ion-button @click="refreshServerConfig">{{ $msTranslate('Authentication.method.sso.retry') }}</ion-button>
          </div>
          <div v-else>
            {{ $msTranslate('Authentication.method.sso.unavailable') }}
          </div>
        </div>
        <ion-button
          v-else
          @click="onLoginClick"
          size="large"
          :disabled="password.length == 0 || loginInProgress === true"
          class="login-button"
        >
          <span v-show="!loginInProgress">{{ $msTranslate('HomePage.organizationLogin.loginButton') }}</span>
          <ms-spinner
            v-show="loginInProgress === true"
            :size="14"
            :speed="2"
          />
        </ion-button>
      </ion-footer>
    </ion-card>
    <!-- end of login -->
  </div>
</template>

<script setup lang="ts">
import { SsoProviderCard } from '@/components/devices';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import {
  AvailableDevice,
  AvailableDeviceTypeTag,
  ClientStartError,
  ClientStartErrorTag,
  constructAccessStrategy,
  DeviceAccessStrategy,
  getServerConfig,
  OpenBaoAuthConfig,
  PrimaryProtectionStrategy,
  ServerConfig,
} from '@/parsec';
import { openBaoConnect, OpenBaoErrorType } from '@/services/openBao';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonFooter, IonText } from '@ionic/vue';
import { MsInput, MsPasswordInput, MsReportText, MsReportTheme, MsSpinner } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  device: AvailableDevice;
  loginInProgress?: boolean;
}>();

const emits = defineEmits<{
  (e: 'loginClick', device: AvailableDevice, access: DeviceAccessStrategy): void;
  (e: 'forgottenPasswordClick', device: AvailableDevice): void;
}>();

const passwordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('passwordInput');
const password = ref('');
const loading = ref(true);
const errorMessage = ref('');
const passwordIsInvalid = ref(false);
const email = props.device.humanHandle.email;
const isConnectWithSSO = ref(false);
const serverConfig = ref<ServerConfig | undefined>(undefined);

onMounted(async () => {
  await passwordInputRef.value?.setFocus();
  if (props.device.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    await refreshServerConfig();
  }
});

async function refreshServerConfig(): Promise<void> {
  loading.value = true;
  const result = await getServerConfig(props.device.serverAddr);
  if (result.ok) {
    serverConfig.value = result.value;
  }
  loading.value = false;
}

async function onLoginSSOClick(provider: OpenBaoAuthConfig): Promise<void> {
  if (props.loginInProgress || !serverConfig.value?.openbao) {
    return;
  }

  const connResult = await openBaoConnect(
    serverConfig.value.openbao.serverUrl,
    provider.id,
    provider.mountPath,
    serverConfig.value.openbao.secret.mountPath,
    serverConfig.value.openbao.transitMountPath,
  );

  if (!connResult.ok) {
    if (connResult.error.type === OpenBaoErrorType.PopupFailed) {
      errorMessage.value = 'Authentication.popupBlocked';
    } else {
      errorMessage.value = 'Authentication.invalidOpenBaoData';
    }
    window.nativeAPI.log('error', `Failed to log in to openbao: ${JSON.stringify(connResult.error)}`);
    return;
  } else {
    isConnectWithSSO.value = true;
  }
  emits(
    'loginClick',
    props.device,
    constructAccessStrategy(props.device, PrimaryProtectionStrategy.useOpenBao(connResult.value.getConnectionInfo())),
  );
}

async function onLoginClick(): Promise<void> {
  if (!props.loginInProgress) {
    emits('loginClick', props.device, constructAccessStrategy(props.device, PrimaryProtectionStrategy.usePassword(password.value)));
  }
}

async function setLoginError(error: ClientStartError): Promise<void> {
  switch (error.tag) {
    case ClientStartErrorTag.LoadDeviceDecryptionFailed:
      errorMessage.value = 'HomePage.organizationLogin.passwordError';
      break;
    case ClientStartErrorTag.LoadDeviceInvalidPath:
      window.nativeAPI.log('warn', error.error);
      errorMessage.value = 'HomePage.organizationLogin.deviceNotFound';
      break;
    case ClientStartErrorTag.LoadDeviceInvalidData:
      window.nativeAPI.log('warn', error.error);
      errorMessage.value = 'HomePage.organizationLogin.deviceInvalidData';
      break;
    case ClientStartErrorTag.DeviceUsedByAnotherProcess:
      errorMessage.value = 'HomePage.organizationLogin.deviceAlreadyUsed';
      break;
    case ClientStartErrorTag.Internal:
      window.nativeAPI.log('warn', error.error);
      errorMessage.value = 'HomePage.organizationLogin.unknownError';
      break;
  }
  passwordIsInvalid.value = true;
}

async function onPasswordChange(value: string): Promise<void> {
  if (value.length === 0) {
    passwordIsInvalid.value = false;
  }
  errorMessage.value = '';
}

defineExpose({
  setLoginError,
});
</script>

<style lang="scss" scoped>
.sso-provider-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  width: 100%;
}

.login-popup {
  height: auto;
  width: 100%;
  max-width: 28rem;
  margin: 2rem auto;
  display: flex;
  gap: 1.5rem;
  flex-direction: column;
  align-items: center;
  box-shadow: none;

  @include ms.responsive-breakpoint('xl') {
    margin: 0 auto;
  }

  @include ms.responsive-breakpoint('sm') {
    margin: 0 auto;
  }
}

.login-header__title {
  background: var(--parsec-color-light-gradient-background);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.login-card {
  background: var(--parsec-color-light-secondary-white);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  padding: 2em;
  margin: 0;
  border-radius: var(--parsec-radius-12);
  box-shadow: none;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  width: 100%;
  transition: box-shadow 150ms ease-in-out;
  overflow-y: auto;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem;
  }

  &:has(.has-focus) {
    box-shadow: var(--parsec-shadow-light);
  }

  &-header {
    padding: 0;
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 0;

    &__password {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      margin: 0;
    }

    #password-input {
      margin: 0;
    }

    #forgotten-password-button {
      margin: 0;
      position: relative;
      width: fit-content;
      color: var(--parsec-color-light-secondary-grey);

      &::part(native) {
        --background-hover: none;
        padding: 0 0 0 2px;
      }

      &:hover {
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }

  &-footer {
    padding: 0;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    align-items: center;

    .login-button {
      width: 100%;
    }
  }
  .error {
    width: 100%;
    padding: 0.5rem 0.75rem 0.5rem 0.625rem;
  }
}
</style>
