<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="authentication-container">
    <template v-if="currentDevice">
      <div class="authentication-content">
        <authentication-card
          :image-src="keypadGradient"
          image-alt="keypad icon"
          method-name="Authentication.method.system"
          v-show="currentDevice && currentDevice.ty === DeviceFileType.Keyring"
        />
        <authentication-card
          :image-src="ellipsisGradient"
          image-alt="ellipsis icon"
          method-name="Authentication.method.password"
          v-show="currentDevice && currentDevice.ty === DeviceFileType.Password"
        />
        <ion-button
          id="change-authentication-button"
          class="update-auth-button"
          fill="clear"
          @click="openChangeAuthentication()"
        >
          <ion-label class="update-auth-button__label">
            <span v-if="isWeb()">{{ $msTranslate('Authentication.changePasswordButton') }}</span>
            <span v-else>{{ $msTranslate('Authentication.changeAuthenticationButton') }}</span>
          </ion-label>
        </ion-button>
      </div>
    </template>
    <template v-else>
      <div class="device-not-found">
        <ion-icon :icon="warning" />
        <ion-text class="item-container__text body">
          {{ $msTranslate('MyProfilePage.errors.failedToRetrieveInformation') }}
        </ion-text>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { MsModalResult } from 'megashark-lib';
import { AvailableDevice, DeviceFileType, getCurrentAvailableDevice, isWeb } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import UpdateAuthenticationModal from '@/views/users/UpdateAuthenticationModal.vue';
import keypadGradient from '@/assets/images/keypad-gradient.svg';
import ellipsisGradient from '@/assets/images/keypad-gradient.svg';
import authenticationCard from '@/components/profile/AuthenticationCard.vue';
import { IonButton, IonIcon, IonText, modalController, IonLabel } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { Ref, inject, onMounted, ref } from 'vue';

const currentDevice: Ref<AvailableDevice | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;

async function openChangeAuthentication(): Promise<void> {
  const modal = await modalController.create({
    component: UpdateAuthenticationModal,
    cssClass: 'change-authentication-modal',
    componentProps: {
      currentDevice: currentDevice.value,
      informationManager: informationManager,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();
  if (role === MsModalResult.Confirm) {
    const result = await getCurrentAvailableDevice();
    if (result.ok) {
      currentDevice.value = result.value;
    }
  }
}

onMounted(async () => {
  const deviceResult = await getCurrentAvailableDevice();

  if (!deviceResult.ok) {
    informationManager.present(
      new Information({
        message: 'MyProfilePage.errors.failedToRetrieveInformation',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    currentDevice.value = deviceResult.value;
  }
});
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
  gap: 0.5rem;
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

.update-auth-button {
  margin-top: 1em;
  width: fit-content;
  --background: var(--parsec-color-light-secondary-text);
  --background-hover: var(--parsec-color-light-secondary-contrast);
  color: var(--parsec-color-light-secondary-white);

  @include ms.responsive-breakpoint('xs') {
    position: fixed;
    bottom: 7rem;
    left: 2rem;
    transform: translateX(50%, 50%);
    width: calc(100% - 4rem);
    margin: auto;
    z-index: 2;
    box-shadow: var(--parsec-shadow-strong);
    --overflow: visible;
    overflow: visible;
  }
}
</style>
