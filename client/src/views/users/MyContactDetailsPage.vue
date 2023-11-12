<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="page">
        <template v-if="clientInfo && currentDevice">
          <!-- user info -->
          <div class="user-info">
            <h2 class="title-h2 user-info-name">
              {{ clientInfo.humanHandle.label }}
            </h2>
            <tag-profile :profile="clientInfo.currentProfile" />
            <ion-text class="form-input user-info-email">
              {{ clientInfo.humanHandle.email }}
            </ion-text>
          </div>

          <!-- change avatar -->
          <div class="avatar">
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

          <!-- change password -->
          <div class="password-change">
            <h3 class="title-h3 password-change-title">
              {{ $t('ContactDetailsPage.title') }}
            </h3>
            <div class="ms-password-inputs">
              <div class="ms-password-inputs-container">
                <ms-password-input
                  :label="$t('Password.currentPassword')"
                  v-model="oldPassword"
                  @change="fieldsUpdated = !fieldsUpdated"
                />
              </div>

              <ms-choose-password-input
                :password-label="$t('Password.newPassword')"
                ref="choosePasswordInput"
              />
            </div>
            <ion-button
              class="password-change-button"
              @click="changePassword()"
              :disabled="!changeButtonIsEnabled"
            >
              {{ $t('ContactDetailsPage.changePasswordButton') }}
            </ion-button>
          </div>
        </template>
        <template v-else>
          {{ $t('ContactDetailsPage.errors.failedToRetrieveInformation') }}
        </template>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { asyncComputed } from '@/common/asyncComputed';
import { MsChoosePasswordInput, MsPasswordInput } from '@/components/core';
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import {
  AvailableDevice,
  ChangeAuthErrorTag,
  ClientInfo,
  getClientInfo,
  getCurrentAvailableDevice,
  changePassword as parsecChangePassword,
} from '@/parsec';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { IonButton, IonChip, IonContent, IonPage, IonText } from '@ionic/vue';
import { Ref, inject, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

const clientInfo: Ref<ClientInfo | null> = ref(null);
const currentDevice: Ref<AvailableDevice | null> = ref(null);
const oldPassword = ref('');
const choosePasswordInput: Ref<typeof MsChoosePasswordInput | null> = ref(null);
const fieldsUpdated = ref(false);
const notificationManager = inject(NotificationKey) as NotificationManager;
const { t } = useI18n();

const changeButtonIsEnabled = asyncComputed(async (): Promise<boolean> => {
  // forces the update
  fieldsUpdated.value;
  return choosePasswordInput.value && (await choosePasswordInput.value.areFieldsCorrect()) && oldPassword.value.length > 0;
});

async function changePassword(): Promise<void> {
  if (!currentDevice.value || !choosePasswordInput.value) {
    return;
  }

  const result = await parsecChangePassword(currentDevice.value, oldPassword.value, choosePasswordInput.value?.password);

  if (result.ok) {
    notificationManager.showToast(
      new Notification({
        title: t('ContactDetailsPage.passwordUpdated.title'),
        message: t('ContactDetailsPage.passwordUpdated.message'),
        level: NotificationLevel.Success,
      }),
    );
    oldPassword.value = '';
    choosePasswordInput.value.clear();
    fieldsUpdated.value = !fieldsUpdated.value;
  } else {
    switch (result.error.tag) {
      case ChangeAuthErrorTag.DecryptionFailed: {
        notificationManager.showToast(
          new Notification({
            title: t('ContactDetailsPage.errors.wrongPassword.title'),
            message: t('ContactDetailsPage.errors.wrongPassword.message'),
            level: NotificationLevel.Error,
          }),
        );
        break;
      }
      default:
        notificationManager.showToast(
          new Notification({
            title: t('ContactDetailsPage.errors.cannotChangePassword.title'),
            message: t('ContactDetailsPage.errors.cannotChangePassword.message'),
            level: NotificationLevel.Error,
          }),
        );
    }
  }
}

onMounted(async () => {
  const result = await getClientInfo();
  const deviceResult = await getCurrentAvailableDevice();

  if (!result.ok || !deviceResult.ok) {
    notificationManager.showToast(
      new Notification({
        title: t('ContactDetailsPage.errors.failedToRetrieveInformation.title'),
        message: t('ContactDetailsPage.errors.failedToRetrieveInformation.message'),
        level: NotificationLevel.Error,
      }),
    );
    return;
  }

  clientInfo.value = result.value;
  currentDevice.value = deviceResult.value;
});
</script>

<style scoped lang="scss">
.page {
  width: 50em;
  margin: 2em;
}

.user-info {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5rem;
  margin-bottom: 3em;
  align-items: center;

  &-name {
    color: var(--parsec-color-light-primary-800);
    margin: 0;
  }

  &-email {
    flex-basis: 100%;
    border-left: 2px solid var(--parsec-color-light-secondary-grey);
    padding-left: 1rem;
    color: var(--parsec-color-light-secondary-text);
  }
}

.avatar {
  // display: flex;
  display: none;
  flex-wrap: wrap;
  margin-bottom: 1.5rem;
  justify-content: space-between;
  position: relative;
  opacity: 0.5;

  &-title {
    flex-basis: 100%;
  }

  &-image {
    width: 200%;
  }

  &-unavailable {
    margin-left: auto;
    position: absolute;
    top: 40%;
    right: 5%;
  }
}

.password-change,
.avatar {
  background-color: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-8);
  padding: 1.5em;

  &-title {
    margin: 0 0 1.5rem;
    color: var(--parsec-color-light-primary-700);
  }
}

.ms-password-inputs-container {
  margin-bottom: 1em;
}

.password-change-button {
  display: flex;
  margin-left: auto;
  width: fit-content;
}
</style>
