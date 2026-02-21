<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="authentication-container">
    <template v-if="currentDevice">
      <div class="authentication-content">
        <div class="authentication-item">
          <div class="authentication-item-text">
            <ion-text class="authentication-item-text__label title-h4">
              {{ $msTranslate('Authentication.label') }}
            </ion-text>
            <ion-text class="authentication-item-text__value method body-lg">
              {{ $msTranslate(authMethodLabel) }}
            </ion-text>
          </div>
          <ion-button
            id="change-authentication-button"
            class="authentication-item-button button-default button-large secondary-button"
            fill="clear"
            @click="openChangeAuthentication()"
          >
            <ion-text class="authentication-item-button__label">
              {{ $msTranslate('Authentication.changeAuthenticationButton') }}
            </ion-text>
          </ion-button>
        </div>

        <!-- MFA -->
        <div
          class="authentication-item mfa"
          :class="totpStatus && totpStatus.tag === TOTPSetupStatusTag.AlreadySetup ? 'mfa-active' : 'mfa-inactive'"
        >
          <div
            class="authentication-item-text"
            v-if="totpStatus"
          >
            <ion-text class="authentication-item-text__label title-h4">
              {{ $msTranslate('Authentication.mfa.title') }}
              <span
                v-if="totpStatus.tag === TOTPSetupStatusTag.AlreadySetup"
                class="tag"
              >
                {{ $msTranslate('Authentication.mfa.active') }}
              </span>
              <span
                v-if="totpStatus.tag === TOTPSetupStatusTag.Stalled"
                class="tag"
              >
                {{ $msTranslate('Authentication.mfa.inactive') }}
              </span>
            </ion-text>
            <ion-text class="authentication-item-text__value body-lg">
              {{ $msTranslate('Authentication.mfa.description') }}
            </ion-text>
          </div>
          <ms-report-text
            v-else
            :theme="MsReportTheme.Error"
          >
            {{ $msTranslate('Authentication.mfa.error.emptyStatus') }}
          </ms-report-text>
          <ion-button
            v-if="totpStatus && totpStatus.tag === TOTPSetupStatusTag.Stalled"
            id="change-authentication-button"
            class="authentication-item-button button-default button-large primary-button"
            fill="clear"
            @click="setupTotp"
          >
            <ion-text class="authentication-item-button__label">
              {{ $msTranslate('Authentication.mfa.buttons.configure') }}
            </ion-text>
          </ion-button>
          <ion-button
            v-if="totpStatus && totpStatus.tag === TOTPSetupStatusTag.AlreadySetup && false"
            id="change-authentication-button"
            class="authentication-item-button button-default button-large secondary-button"
            fill="clear"
            @click="deleteTotp"
          >
            <ion-text class="authentication-item-button__label">{{ $msTranslate('Authentication.mfa.buttons.delete') }}</ion-text>
          </ion-button>
        </div>
      </div>
    </template>
    <template v-if="error">
      <div class="device-not-found">
        <ion-icon :icon="warning" />
        <ion-text class="item-container__text body">
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
  getServerConfig,
  getTotpStatus,
  TOTPSetupStatus,
  TOTPSetupStatusTag,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import ActivateTotpModal from '@/views/totp/ActivateTotpModal.vue';
import DeleteTotpModal from '@/views/totp/DeleteTotpModal.vue';
import UpdateAuthenticationModal from '@/views/users/UpdateAuthenticationModal.vue';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { Answer, askQuestion, MsModalResult, MsReportText, MsReportTheme, Translatable } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const currentDevice: Ref<AvailableDevice | null> = ref(null);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const error = ref('');
const totpStatus = ref<TOTPSetupStatus | undefined>(undefined);

const authMethodLabel = computed(() => {
  if (!currentDevice.value) return '';

  const methodLabels: Record<AvailableDeviceTypeTag, Translatable> = {
    [AvailableDeviceTypeTag.Keyring]: 'Authentication.method.keyring',
    [AvailableDeviceTypeTag.Password]: 'Authentication.method.password',
    [AvailableDeviceTypeTag.PKI]: 'Authentication.method.smartCard',
    [AvailableDeviceTypeTag.OpenBao]: 'Authentication.method.openBao',
    [AvailableDeviceTypeTag.AccountVault]: '',
    [AvailableDeviceTypeTag.Recovery]: '',
  };

  return methodLabels[currentDevice.value.ty.tag] || '';
});

async function openChangeAuthentication(): Promise<void> {
  if (!currentDevice.value) {
    return;
  }

  const configResult = await getServerConfig(currentDevice.value.serverAddr);

  if (currentDevice.value.ty.tag === AvailableDeviceTypeTag.OpenBao) {
    if (!configResult.ok || !configResult.value.openbao) {
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
      serverConfig: configResult.ok ? configResult.value : undefined,
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
  const totpStatusResult = await getTotpStatus();

  if (totpStatusResult.ok) {
    totpStatus.value = totpStatusResult.value;
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
  }
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
    cssClass: 'delete-totp-modal',
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
  gap: 1.5rem;
}

.authentication-content {
  display: flex;
  flex-direction: column;
}

.authentication-item {
  display: flex;
  align-items: center;
  gap: 1.25rem;
  padding: 1.5rem 0;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);

  @include ms.responsive-breakpoint('lg') {
    flex-direction: column;
    align-items: stretch;
  }

  @include ms.responsive-breakpoint('sm') {
    flex-direction: row;
    align-items: center;
  }

  @include ms.responsive-breakpoint('sm') {
    align-items: stretch;
    flex-direction: column;
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex-grow: 1;

    &__label {
      color: var(--parsec-color-light-secondary-text);
    }

    &__value {
      color: var(--parsec-color-light-secondary-text);

      &.method {
        font-style: italic;
      }
    }
  }

  &-button {
    width: fit-content;
    height: fit-content;
    box-shadow: var(--parsec-shadow-input);

    &.primary-button {
      --background: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-contrast);
      color: var(--parsec-color-light-secondary-white);
    }

    &.secondary-button {
      --background: var(--parsec-color-light-secondary-white);
      --background-hover: var(--parsec-color-light-secondary-medium);
      border: 1px solid var(--parsec-color-light-secondary-disabled);
      color: var(--parsec-color-light-secondary-text);
      border-radius: var(--parsec-radius-8);
    }

    &::part(native) {
      font-size: 0.9375rem;
      padding: 0.75rem 0.825rem;
    }

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }
  }

  &.mfa {
    .authentication-item-text__label {
      display: flex;
      align-items: center;
      gap: 0.75rem;
    }

    &-active .tag,
    &-inactive .tag {
      padding: 0.125rem 0.375rem;
      border-radius: var(--parsec-radius-8);
      font-size: 0.825rem;
      font-weight: 500;
    }

    &-active .tag {
      color: var(--parsec-color-light-success-700);
      background: var(--parsec-color-light-success-100);
    }

    &-inactive .tag {
      color: var(--parsec-color-light-danger-700);
      background: var(--parsec-color-light-danger-100);
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
  background: var(--parsec-color-light-danger-50);
  color: var(--parsec-color-light-danger-700);
  padding: 0.5rem 1rem;
  border-radius: var(--parsec-radius-8);
  gap: 0.5rem;

  ion-text {
    padding: 0.25rem 0;
  }

  ion-icon {
    font-size: 1.25rem;
  }
}
</style>
