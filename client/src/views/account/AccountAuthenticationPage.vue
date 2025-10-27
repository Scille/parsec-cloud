<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="ParsecAccount.isLoggedIn()"
    class="account-authentication-page-container"
  >
    <div class="authentication-method-list">
      <div
        v-for="method of authMethods"
        :key="method.authMethodId"
        class="authentication-method-item"
        :class="{ 'authentication-method-item--current': method.current }"
      >
        <div class="authentication-method-item-icon">
          <ms-image
            :image="method.usePassword ? EllipsisGradient : KeypadGradient"
            class="authentication-method-item-icon__image"
          />
        </div>
        <div class="authentication-method-item-details">
          <ion-text class="authentication-method-item-details__title subtitles-normal">
            {{
              method.usePassword
                ? $msTranslate('HomePage.profile.authentication.password')
                : $msTranslate('HomePage.profile.authentication.other')
            }}
          </ion-text>
          <ion-text class="authentication-method-item-details__date body">
            <span>{{ $msTranslate('HomePage.profile.authentication.createdOn') }}</span>
            <span>{{ $msTranslate(I18n.formatDate(method.createdOn, 'short')) }}</span>
          </ion-text>
        </div>
        <ion-icon
          v-if="method.current"
          class="authentication-method-item-checkmark"
          :icon="checkmarkCircle"
        />
      </div>
    </div>

    <ion-button
      v-if="authMethods.find((method) => method.usePassword === true && method.current) !== undefined"
      @click="updatePassword"
      class="button-medium account-authentication-button"
    >
      {{ $msTranslate('HomePage.profile.authentication.changeMethod') }}
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import EllipsisGradient from '@/assets/images/ellipsis-gradient.svg';
import KeypadGradient from '@/assets/images/keypad-gradient.svg';
import { AuthMethodInfo, ParsecAccount } from '@/parsec';
import { getConnectionHandle } from '@/router';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import AccountUpdateAuthenticationModal from '@/views/account/AccountUpdateAuthenticationModal.vue';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, MsImage, MsModalResult } from 'megashark-lib';
import { inject, onMounted, ref } from 'vue';

const authMethods = ref<Array<AuthMethodInfo>>([]);
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;

onMounted(async () => {
  const result = await ParsecAccount.listAuthenticationMethod();
  if (result.ok) {
    authMethods.value = result.value;
  }
});

async function updatePassword(): Promise<void> {
  const modal = await modalController.create({
    component: AccountUpdateAuthenticationModal,
    cssClass: 'account-update-authentication',
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  if (role === MsModalResult.Confirm) {
    let informationManager!: InformationManager;
    const handle = getConnectionHandle();
    if (handle === null) {
      informationManager = injectionProvider.getDefault().informationManager;
    } else {
      informationManager = injectionProvider.getInjections(handle).informationManager;
    }
    informationManager.present(
      new Information({
        message: 'HomePage.profile.authentication.updateAuthMethod.success',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
}
</script>

<style scoped lang="scss">
.account-authentication-page-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.authentication-method-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.authentication-method-item {
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  display: flex;
  align-items: center;
  overflow: hidden;
  gap: 1rem;
  padding-right: 1rem;
  max-width: 34rem;

  &:not(.authentication-method-item--current) {
    filter: grayscale(100%);
  }

  &-icon {
    background: var(--parsec-color-light-secondary-background);
    padding: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;

    &__image {
      width: 1.5rem;
      height: 1.5rem;
      flex-shrink: 0;
    }
  }

  &-details {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    width: 100%;

    &__title {
      color: var(--parsec-color-light-primary-600);
    }

    &__date {
      color: var(--parsec-color-light-secondary-hard-grey);
      display: flex;
      gap: 0.125rem;
    }
  }

  &-checkmark {
    color: var(--parsec-color-light-primary-600);
    font-size: 1.25rem;
    flex-shrink: 0;
  }
}

.account-authentication-button {
  width: fit-content;

  &::part(native) {
    padding: 0.75rem 1.125rem;
    --background: var(--parsec-color-light-secondary-text);
    --background-hover: var(--parsec-color-light-secondary-contrast);
    color: var(--parsec-color-light-secondary-white);
  }
}
</style>
