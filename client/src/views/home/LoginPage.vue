<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="login-popup">
    <organization-card
      :device="device"
      class="login-popup__organization"
    />
    <ion-card class="login-card">
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
            class="link"
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
            v-if="!globalLoading && serverConfig?.openbao && serverConfig.openbao.auths.length > 0"
            class="sso-provider-list"
          >
            <sso-provider-card
              v-for="provider of serverConfig.openbao.auths"
              :key="provider.id"
              :provider="provider"
              @sso-selected="onLoginSSOClick"
              :disabled="connectingToProvider !== undefined || loginInProgress"
              :loading="connectingToProvider === provider.id || loginInProgress"
              :is-connected="connectedToProvider === provider.id"
            />
          </div>
          <div v-else-if="globalLoading">
            <ms-spinner />
          </div>
          <div v-else-if="!globalLoading && !serverConfig">
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
import { IonButton, IonCard, IonCardContent, IonFooter } from '@ionic/vue';
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
const globalLoading = ref(true);
const errorMessage = ref('');
const passwordIsInvalid = ref(false);
const email = props.device.humanHandle.email;
const serverConfig = ref<ServerConfig | undefined>(undefined);
const connectingToProvider = ref<string | undefined>(undefined);
const connectedToProvider = ref<string | undefined>(undefined);

onMounted(async () => {
  await passwordInputRef.value?.setFocus();
  if (props.device.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    await refreshServerConfig();
  }
});

async function refreshServerConfig(): Promise<void> {
  globalLoading.value = true;
  const result = await getServerConfig(props.device.serverAddr);
  if (result.ok) {
    serverConfig.value = result.value;
  }
  globalLoading.value = false;
}

async function onLoginSSOClick(provider: OpenBaoAuthConfig): Promise<void> {
  if (props.loginInProgress || !serverConfig.value?.openbao || connectingToProvider.value) {
    return;
  }

  try {
    connectingToProvider.value = provider.id;
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
      } else if (connResult.error.type !== OpenBaoErrorType.Closed) {
        errorMessage.value = 'Authentication.invalidOpenBaoData';
      }
      window.nativeAPI.log('error', `Failed to log in to openbao: ${JSON.stringify(connResult.error)}`);
      connectedToProvider.value = undefined;
    } else {
      connectedToProvider.value = provider.id;
      emits(
        'loginClick',
        props.device,
        constructAccessStrategy(props.device, PrimaryProtectionStrategy.useOpenBao(connResult.value.getConnectionInfo())),
      );
    }
  } finally {
    connectingToProvider.value = undefined;
  }
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
  width: 100%;
}

.login-popup {
  height: auto;
  width: 100%;
  max-width: 28rem;
  margin: ms.spacing('padding-5xl') auto;
  display: flex;
  gap: ms.spacing('gap-4xl');
  flex-direction: column;
  align-items: center;
  box-shadow: none;

  @include ms.responsive-breakpoint('xl') {
    margin: 0 auto;
  }

  @include ms.responsive-breakpoint('sm') {
    margin: 3rem auto;
  }

  &__organization {
    max-width: 100%;
  }
}

.login-card {
  background: ms.color('surface-base-default');
  border: ms.border('thin') solid ms.color('border-base-default');
  padding: ms.spacing('padding-5xl');
  margin: 0;
  border-radius: ms.radius('4xl');
  box-shadow: none;
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  width: 100%;
  transition: box-shadow 150ms ease-in-out;
  overflow-y: auto;

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl');
  }

  &:has(.has-focus) {
    box-shadow: ms.shadow('elevation-sm');
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    padding: ms.spacing('padding-none');

    &__password {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-lg');
      margin: 0;
    }

    #password-input {
      margin: 0;
    }

    #forgotten-password-button {
      @include ms.font('label-md-medium');
      color: ms.color('text-base-description');

      &::part(native) {
        --background-hover: none;
      }
    }
  }

  &-footer {
    padding: ms.spacing('padding-none');
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-4xl');
    align-items: center;

    .login-button {
      width: 100%;
    }
  }
  .error {
    width: 100%;
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl') ms.spacing('padding-lg') ms.spacing('padding-xl');
  }
}
</style>
