<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="authentication-container">
    <template v-if="querying">
      <ms-spinner />
    </template>
    <template v-else-if="currentDevice">
      <div class="authentication-content">
        <div class="authentication-item">
          <div class="authentication-item-header">
            <ion-text class="authentication-item-header__label">
              {{ $msTranslate('Authentication.label') }}
            </ion-text>
          </div>
          <div class="authentication-item-content">
            <ion-text class="authentication-item-content__description method">
              {{ $msTranslate(authMethodLabel) }}
            </ion-text>
            <ion-button
              id="change-authentication-button"
              class="authentication-item-button color-secondary"
              fill="outline"
              @click="openChangeAuthentication()"
            >
              <ion-text class="authentication-item-button__label">
                {{ $msTranslate('Authentication.changeAuthenticationButton') }}
              </ion-text>
            </ion-button>
          </div>
        </div>

        <!-- MFA -->
        <div
          class="authentication-item mfa"
          :class="currentDevice.totpOpaqueKeyId ? 'mfa-active' : 'mfa-inactive'"
        >
          <div
            class="authentication-item-header"
            v-if="totpStatus"
          >
            <ion-text class="authentication-item-header__label">
              {{ $msTranslate('Authentication.mfa.title') }}
              <span
                v-if="currentDevice.totpOpaqueKeyId"
                class="tag"
              >
                {{ $msTranslate('Authentication.mfa.active') }}
              </span>
              <span
                v-if="!currentDevice.totpOpaqueKeyId"
                class="tag"
              >
                {{ $msTranslate('Authentication.mfa.inactive') }}
              </span>
            </ion-text>
            <ms-report-text
              class="authentication-item-header__report"
              v-show="totpStatus?.tag === TOTPSetupStatusTag.Confirmed && !currentDevice.totpOpaqueKeyId"
              :theme="MsReportTheme.Info"
            >
              {{ $msTranslate('Authentication.mfa.inactiveInfo') }}
            </ms-report-text>
          </div>
          <div
            class="authentication-item-header mfa-error"
            v-else
          >
            <ion-text class="authentication-item-header__label">{{ $msTranslate('Authentication.mfa.title') }}</ion-text>
            <ms-report-text :theme="MsReportTheme.Error">{{ $msTranslate('Authentication.mfa.error.emptyStatus') }}</ms-report-text>
          </div>
          <ms-report-text
            class="authentication-item--mandatory"
            v-if="mandatoryTotp"
            :theme="MsReportTheme.Warning"
          >
            {{ $msTranslate('Authentication.mfa.info') }}
          </ms-report-text>
          <div class="authentication-item-content">
            <ion-text class="authentication-item-content__description">
              {{ $msTranslate('Authentication.mfa.description') }}
            </ion-text>
            <ion-button
              v-if="totpStatus && totpStatus.tag === TOTPSetupStatusTag.Unconfirmed"
              id="totp-setup-button"
              class="authentication-item-button color-secondary"
              @click="setupTotp"
            >
              <ion-text class="authentication-item-button__label">
                {{ $msTranslate('Authentication.mfa.buttons.configure') }}
              </ion-text>
            </ion-button>
            <ion-toggle
              :disabled="totpActivated && mandatoryTotp"
              v-model="totpActivated"
              @ion-change="toggleTotp"
              v-if="totpStatus && totpStatus.tag === TOTPSetupStatusTag.Confirmed"
            />
          </div>
        </div>
      </div>
    </template>
    <template v-if="error">
      <div class="device-not-found">
        <ion-icon :icon="warning" />
        <ion-text class="item-container__text">
          {{ $msTranslate(error) }}
        </ion-text>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {
  AvailableDevice,
  AvailableDeviceTypeTag,
  getCurrentAvailableDevice,
  getCurrentServerConfig,
  getPrimaryProtectionTypeForDeviceType,
  getTotpStatus,
  ServerConfig,
  TOTPSetupStatus,
  TOTPSetupStatusTag,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import UpdateAuthenticationModal from '@/views/authentication/UpdateAuthenticationModal.vue';
import ActivateTotpModal from '@/views/totp/ActivateTotpModal.vue';
import DeleteTotpModal from '@/views/totp/DeleteTotpModal.vue';
import { IonButton, IonIcon, IonText, IonToggle, modalController, ToggleCustomEvent } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { Answer, askQuestion, MsModalResult, MsReportText, MsReportTheme, MsSpinner, Translatable } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const currentDevice: Ref<AvailableDevice | null> = ref(null);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const error = ref('');
const querying = ref(true);
const totpStatus = ref<TOTPSetupStatus | undefined>(undefined);
const totpActivated = ref(false);
const serverConfig = ref<ServerConfig | undefined>(undefined);
const mandatoryTotp = computed(() => {
  if (!serverConfig.value || !currentDevice.value) {
    return false;
  }
  const protectionType = getPrimaryProtectionTypeForDeviceType(currentDevice.value.ty.tag);
  if (!protectionType) {
    return false;
  }
  return serverConfig.value.doesAuthMethodRequireTotp(protectionType);
});
const authMethodLabel = computed(() => {
  if (!currentDevice.value) return '';

  const methodLabels: Record<AvailableDeviceTypeTag, Translatable> = {
    [AvailableDeviceTypeTag.Keyring]: 'Authentication.useKeyring',
    [AvailableDeviceTypeTag.Password]: 'Authentication.usePassword',
    [AvailableDeviceTypeTag.PKI]: 'Authentication.useSmartcard',
    [AvailableDeviceTypeTag.OpenBao]: 'Authentication.useSso',
    [AvailableDeviceTypeTag.AccountVault]: '',
    [AvailableDeviceTypeTag.Recovery]: '',
  };

  return methodLabels[currentDevice.value.ty.tag] || '';
});

async function openChangeAuthentication(): Promise<void> {
  if (!currentDevice.value) {
    return;
  }

  if (currentDevice.value.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    if (!serverConfig.value?.openbao) {
      error.value = 'Authentication.invalidOpenBaoData';
      return;
    }
  }

  if (currentDevice.value.ty.tag === AvailableDeviceTypeTag.PKI) {
    const answer = await askQuestion('Authentication.method.smartcard.warn.title', 'Authentication.method.smartcard.warn.subtitle', {
      yesText: 'Authentication.method.smartcard.warn.yes',
      noText: 'Authentication.method.smartcard.warn.no',
    });

    if (answer === Answer.No) {
      return;
    }
  }

  const modal = await modalController.create({
    component: UpdateAuthenticationModal,
    cssClass: 'change-authentication-modal',
    componentProps: {
      currentDevice: currentDevice.value,
      informationManager: informationManager.value,
      serverConfig: serverConfig.value,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();
  if (role === MsModalResult.Confirm) {
    const result = await getCurrentAvailableDevice();
    if (result.ok) {
      currentDevice.value = result.value;
      error.value = '';
    } else {
      error.value = 'MyProfilePage.errors.failedToRetrieveInformation';
    }
  }
}

onMounted(async () => {
  await refresh();
});

async function refresh(): Promise<void> {
  try {
    querying.value = true;
    const totpStatusResult = await getTotpStatus();

    totpStatus.value = totpStatusResult.ok ? totpStatusResult.value : undefined;

    const serverConfigResult = await getCurrentServerConfig();
    if (serverConfigResult.ok) {
      serverConfig.value = serverConfigResult.value;
    }

    const deviceResult = await getCurrentAvailableDevice();

    if (!deviceResult.ok) {
      informationManager.value.present(
        new Information({
          message: 'MyProfilePage.errors.failedToRetrieveInformation',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      error.value = 'MyProfilePage.errors.failedToRetrieveInformation';
    } else {
      error.value = '';
      currentDevice.value = deviceResult.value;
      totpActivated.value = currentDevice.value.totpOpaqueKeyId !== null;
    }
  } finally {
    querying.value = false;
  }
}

async function toggleTotp(event: ToggleCustomEvent): Promise<void> {
  // The event has the new value of the toggle: when checked is false, it means
  // it was true and was just switched to false.
  if (!event.detail.checked) {
    await deleteTotp();
  } else {
    await activateTotp();
  }
  await refresh();
}

async function activateTotp(): Promise<void> {
  if (!currentDevice.value || !totpStatus.value) {
    return;
  }
  const modal = await modalController.create({
    component: ActivateTotpModal,
    cssClass: 'activate-totp-modal',
    componentProps: {
      params: {
        mode: 'activate',
        device: currentDevice.value,
      },
    },
    canDismiss: true,
    backdropDismiss: true,
    showBackdrop: true,
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm) {
    return;
  }
  await refresh();
  informationManager.value.present(
    new Information({
      message: 'Authentication.mfa.mfaSuccess.description',
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
}

async function setupTotp(): Promise<void> {
  if (!currentDevice.value) {
    return;
  }
  const modal = await modalController.create({
    component: ActivateTotpModal,
    cssClass: 'activate-totp-modal',
    componentProps: {
      params: {
        mode: 'setup',
        device: currentDevice.value,
      },
    },
    canDismiss: true,
    backdropDismiss: true,
    showBackdrop: true,
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm) {
    return;
  }
  await refresh();
  informationManager.value.present(
    new Information({
      message: 'Authentication.mfa.mfaSuccess.description',
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
}

async function deleteTotp(): Promise<void> {
  if (!currentDevice.value) {
    return;
  }
  const modal = await modalController.create({
    component: DeleteTotpModal,
    cssClass: 'delete-totp',
    componentProps: {
      device: currentDevice.value,
    },
    canDismiss: true,
    backdropDismiss: true,
    showBackdrop: true,
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm) {
    return;
  }
  await refresh();
  informationManager.value.present(
    new Information({
      message: 'Authentication.mfa.mfaSuccess.deleteOk',
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
}
</script>

<style scoped lang="scss">
.authentication-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-page');
  padding: ms.spacing('padding-4xl');
  box-shadow:
    0 1px 1px 0 rgba(0, 0, 0, 0.05),
    0 1px 4px 0 rgba(0, 0, 0, 0.03),
    0 0 1px 0 rgba(0, 0, 0, 0.2);
}

.authentication-content {
  display: flex;
  flex-direction: column;
}

.authentication-item {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');

  &:not(:last-child) {
    padding-bottom: ms.spacing('padding-4xl');
    border-bottom: ms.border('thin') solid ms.color('border-base-default');
    margin-bottom: 1.5rem;
  }

  @include ms.responsive-breakpoint('lg') {
    flex-direction: column;
    align-items: stretch;
  }

  @include ms.responsive-breakpoint('sm') {
    align-items: stretch;
    flex-direction: column;
  }

  &-header {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    flex-grow: 1;

    &__label {
      @include ms.font('heading-h5');
      color: ms.color('text-base-heading');
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-2xl');
    }

    &__report {
      padding: ms.spacing('padding-lg') ms.spacing('padding-3xl') ms.spacing('padding-lg') ms.spacing('padding-xl') !important;
    }
  }

  &-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: ms.spacing('gap-3xl');

    @include ms.responsive-breakpoint('md') {
      flex-direction: column;
      align-items: start;
      gap: ms.spacing('gap-2xl');
    }

    &__description {
      @include ms.font('body-lg-regular');
      color: ms.color('text-base-description');

      &.method {
        font-style: italic;
        color: ms.color('text-base-body');
      }
    }
  }

  &-button {
    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }
  }

  &.mfa {
    .authentication-item-text__label {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-2xl');
    }

    &-active .tag,
    &-inactive .tag {
      @include ms.font('label-md-medium');
      flex-shrink: 0;
      padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
      border-radius: ms.radius('2xl');
      white-space: nowrap;
      margin-left: auto;
    }

    &-active .tag {
      color: ms.color('text-success-default');
      background: ms.color('surface-success-default-subtle-hover');
    }

    &-inactive .tag {
      color: ms.color('text-error-default');
      background: ms.color('surface-error-default-subtle-hover');
    }

    .authentication-item-button {
      @include ms.responsive-breakpoint('sm') {
        position: static;
        transform: none;
      }
    }
  }
}

.device-not-found {
  display: flex;
  align-items: center;
  background: ms.color('surface-error-default-subtle-hover');
  color: ms.color('text-error-default');
  padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
  border-radius: ms.radius('lg');
  gap: ms.spacing('gap-lg');

  ion-text {
    @include ms.font('body-md-regular');
    padding: ms.spacing('padding-sm') ms.spacing('padding-none');
  }

  ion-icon {
    font-size: 1.25rem;
  }
}

.mfa-error {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
}
</style>
