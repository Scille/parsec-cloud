<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="ParsecAccount.isLoggedIn()"
    class="manage-account-page-container"
  >
    <div
      v-if="accountInfo"
      class="account-info"
    >
      <ion-title class="account-info__title title-h4">{{ $msTranslate('HomePage.profile.account.personalData.title') }}</ion-title>
      <div class="account-info-content">
        <ms-input
          label="HomePage.profile.account.personalData.name"
          :disabled="true"
          v-model="accountInfo.humanHandle.label"
        />
        <ms-input
          label="HomePage.profile.account.personalData.email"
          :disabled="true"
          v-model="accountInfo.humanHandle.email"
        />
      </div>
    </div>
    <div class="delete-account">
      <div class="delete-account-content">
        <ion-text class="delete-account__title title-h3">{{ $msTranslate('HomePage.profile.account.deleteAccount.title') }}</ion-text>
        <ion-text class="delete-account__description body">
          {{ $msTranslate('HomePage.profile.account.deleteAccount.description') }}
        </ion-text>
      </div>
      <ion-button
        @click="deleteAccount"
        :disabled="loading"
        fill="outline"
        class="delete-account__button"
      >
        {{ $msTranslate('HomePage.profile.account.deleteAccount.button') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AccountInfo, ParsecAccount } from '@/parsec';
import { getConnectionHandle, navigateTo, Routes, watchRoute } from '@/router';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import CodeValidationModal from '@/views/account/CodeValidationModal.vue';
import { IonButton, IonText, IonTitle, modalController } from '@ionic/vue';
import { Answer, askQuestion, MsInput, MsModalResult } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref } from 'vue';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const loading = ref(false);
const accountInfo = ref<AccountInfo | undefined>();
const accountLoggedIn = ref(false);

async function deleteAccount(): Promise<void> {
  if (!ParsecAccount.isLoggedIn()) {
    return;
  }
  loading.value = true;

  try {
    let informationManager!: InformationManager;
    const handle = getConnectionHandle();
    if (handle === null) {
      informationManager = injectionProvider.getDefault().informationManager;
    } else {
      informationManager = injectionProvider.getInjections(handle).informationManager;
    }

    const answer = await askQuestion(
      'HomePage.profile.account.deleteAccount.questionModal.title',
      'HomePage.profile.account.deleteAccount.questionModal.description',
      {
        yesText: 'HomePage.profile.account.deleteAccount.questionModal.sendCode',
        noText: 'HomePage.profile.account.deleteAccount.questionModal.cancel',
      },
    );

    if (answer !== Answer.Yes) {
      return;
    }

    const reqResult = await ParsecAccount.requestAccountDeletion();
    if (!reqResult.ok) {
      window.electronAPI.log('error', `Failed to request account deletion: ${reqResult.error.tag} (${reqResult.error.error})`);
      informationManager.present(
        new Information({
          message: 'HomePage.profile.account.deleteAccount.error.sendCode',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }

    const codeModal = await modalController.create({
      component: CodeValidationModal,
      cssClass: 'account-code-validation-modal',
      showBackdrop: true,
      canDismiss: true,
    });
    await codeModal.present();
    const { role } = await codeModal.onDidDismiss();
    await codeModal.dismiss();
    if (role !== MsModalResult.Confirm) {
      return;
    }

    informationManager.present(
      new Information({
        message: 'HomePage.profile.account.deleteAccount.success',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await navigateTo(Routes.Account, { skipHandle: true, replace: true });
  } finally {
    loading.value = false;
  }
}

const routeWatchCancel = watchRoute(async () => {
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
  if (accountLoggedIn.value) {
    const result = await ParsecAccount.getInfo();
    if (result.ok) {
      accountInfo.value = result.value;
    } else {
      accountInfo.value = undefined;
    }
  }
});

onMounted(async () => {
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
  const result = await ParsecAccount.getInfo();
  if (result.ok) {
    accountInfo.value = result.value;
  } else {
    accountInfo.value = undefined;
  }
  window.electronAPI.getUpdateAvailability();
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
});

onUnmounted(() => {
  routeWatchCancel();
});
</script>

<style scoped lang="scss">
.manage-account-page-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.account-info {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-top: 0.5rem;

  &__title {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
}

.delete-account {
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-background);
  padding: 1.5rem;
  border-radius: var(--parsec-radius-12);
  gap: 1rem;

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__description {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &__button {
    --color: var(--parsec-color-light-danger-500);
    --color-hover: var(--parsec-color-light-danger-700);
    width: fit-content;
    --border-color: var(--parsec-color-light-danger-500);

    @include ms.responsive-breakpoint(xs) {
      width: 100%;
    }

    &:hover {
      --border-color: var(--parsec-color-light-danger-700);
    }
  }
}
</style>
