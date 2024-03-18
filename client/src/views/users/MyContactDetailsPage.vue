<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="page">
        <template v-if="clientInfo">
          <!-- user info -->
          <div class="user-info">
            <div class="user-info-header">
              <h2 class="title-h2 user-info-header__name">
                {{ clientInfo.humanHandle.label }}
              </h2>
              <tag-profile :profile="clientInfo.currentProfile" />
            </div>

            <!-- inputs fields -->
            <div class="user-info-inputs">
              <div class="user-info-inputs-item">
                <ion-text class="user-info-inputs-item__label form-input">
                  {{ $t('ContactDetailsPage.email') }}
                </ion-text>
                <ms-input
                  :placeholder="clientInfo.humanHandle.email"
                  name="fullname"
                  :disabled="true"
                  class="user-info-inputs-item__input"
                />
              </div>
              <div class="user-info-inputs-item">
                <ion-text class="user-info-inputs-item__label form-input">
                  {{ $t('ContactDetailsPage.authenticationMethod') }}
                </ion-text>
                <div
                  class="user-info-inputs-item__password"
                  v-show="currentDevice && currentDevice.ty === DeviceFileType.Password"
                >
                  <ms-input
                    :placeholder="'••••••••••'"
                    name="fullname"
                    :disabled="true"
                    class="user-info-inputs-item__input"
                  />
                  <ion-button
                    id="change-password-button"
                    @click="openChangePassword()"
                    fill="clear"
                    size="small"
                  >
                    {{ $t('ContactDetailsPage.changePasswordButton') }}
                  </ion-button>
                </div>
                <div v-show="currentDevice && currentDevice.ty === DeviceFileType.Keyring">
                  {{ $t('ContactDetailsPage.keyring') }}
                </div>
              </div>
            </div>
          </div>

          <!-- when the add avatar feature will be implemented -->
          <!--
          <div
            class="avatar"
          >
            <h3 class="title-h3 avatar-title">
              {{ $t('ContactDetailsPage.avatarTitle') }}
            </h3>
            <user-avatar-name
              :user-avatar="clientInfo.userId"
              class="avatar-image"
            />
            <ion-chip class="caption-caption avatar-unavailable">
              {{ $t('ContactDetailsPage.avatarUnavailable') }}
            </ion-chip>
          </div>
          -->
        </template>
        <template v-else>
          <div class="device-not-found">
            <ion-icon
              :icon="warning"
              size="large"
            />
            <ion-text class="body">
              {{ $t('ContactDetailsPage.errors.failedToRetrieveInformation') }}
            </ion-text>
          </div>
        </template>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { MsInput } from '@/components/core';
import TagProfile from '@/components/users/TagProfile.vue';
import { AvailableDevice, ClientInfo, DeviceFileType, getClientInfo, getCurrentAvailableDevice } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import UpdatePasswordModal from '@/views/users/UpdatePasswordModal.vue';
import { IonButton, IonContent, IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { Ref, inject, onMounted, ref } from 'vue';

const clientInfo: Ref<ClientInfo | null> = ref(null);
const currentDevice: Ref<AvailableDevice | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;

async function openChangePassword(): Promise<void> {
  const modal = await modalController.create({
    component: UpdatePasswordModal,
    cssClass: 'change-password-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

onMounted(async () => {
  const deviceResult = await getCurrentAvailableDevice();
  const result = await getClientInfo();

  if (!result.ok || !deviceResult.ok) {
    informationManager.present(
      new Information({
        message: translate('ContactDetailsPage.errors.failedToRetrieveInformation'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    clientInfo.value = result.value;
    currentDevice.value = deviceResult.value;
  }
});
</script>

<style scoped lang="scss">
.page {
  width: 40em;
  margin: 2em;
}

.user-info {
  display: flex;
  flex-direction: column;
  gap: 2.5rem;

  &-header {
    display: flex;
    color: var(--parsec-color-light-primary-800);
    gap: 1.5rem;

    &__name {
      margin: 0;
    }
  }

  &-inputs {
    display: flex;
    flex-direction: column;
    gap: 2rem;

    &-item {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      &__label {
        min-width: 14rem;
        color: var(--parsec-color-light-secondary-text);
      }

      &__input {
        width: 100%;
        max-width: 20rem;
      }

      &__password {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        width: 100%;
      }
    }
  }
}

.device-not-found {
  background: var(--parsec-color-light-danger-100);
  color: var(--parsec-color-light-danger-700);
  padding: 1rem;
  display: flex;
  gap: 0.75rem;
  border-left: 0.25rem solid var(--parsec-color-light-danger-500);

  ion-text {
    padding: 0.25rem 0;
  }
}

// when the add avatar feature is implemented
// .avatar {
//   display: flex;
//   flex-wrap: wrap;
//   margin-bottom: 1.5rem;
//   justify-content: space-between;
//   position: relative;
//   opacity: 0.5;

//   &-title {
//     flex-basis: 100%;
//   }

//   &-image {
//     width: 200%;
//   }

//   &-unavailable {
//     margin-left: auto;
//     position: absolute;
//     top: 40%;
//     right: 5%;
//   }
// }
</style>
