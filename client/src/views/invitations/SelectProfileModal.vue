<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="InvitationsPage.asyncEnrollmentRequest.selectProfileModal.title"
    :close-button="{ visible: true }"
    :confirm-button="{
      disabled: selectedProfile === undefined,
      label: 'InvitationsPage.asyncEnrollmentRequest.selectProfileModal.actions.confirm',
      onClick: validateProfile,
    }"
    :cancel-button="{
      label: 'InvitationsPage.asyncEnrollmentRequest.selectProfileModal.actions.cancel',
      disabled: false,
      onClick: dismissModal,
    }"
  >
    <div class="select-profile-modal-container">
      <user-information
        class="user-details"
        :default-email="email"
        :default-name="name"
        :email-enabled="false"
        :name-enabled="false"
      />
      <ms-dropdown
        class="dropdown"
        title="InvitationsPage.asyncEnrollmentRequest.selectProfileModal.profileLabel"
        label="InvitationsPage.asyncEnrollmentRequest.selectProfileModal.profilePlaceholder"
        :options="profileOptions"
        @change="selectedProfile = $event.option.key"
      />
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import UserInformation from '@/components/users/UserInformation.vue';
import { UserProfile } from '@/parsec';
import { modalController } from '@ionic/vue';
import { MsDropdown, MsModal, MsModalResult, MsOptions } from 'megashark-lib';
import { ref } from 'vue';

defineProps<{
  name: string;
  email: string;
}>();

const selectedProfile = ref<UserProfile | undefined>(undefined);

const profileOptions: MsOptions = new MsOptions([
  {
    key: UserProfile.Admin,
    label: 'UsersPage.profile.admin.label',
    description: 'UsersPage.profile.admin.description',
  },
  {
    key: UserProfile.Standard,
    label: 'UsersPage.profile.standard.label',
    description: 'UsersPage.profile.standard.description',
  },
  {
    key: UserProfile.Outsider,
    label: 'UsersPage.profile.outsider.label',
    description: 'UsersPage.profile.outsider.description',
  },
]);

async function validateProfile(): Promise<boolean> {
  if (!selectedProfile.value) {
    return false;
  }
  return await modalController.dismiss({ profile: selectedProfile.value }, MsModalResult.Confirm);
}

async function dismissModal(): Promise<boolean> {
  return await modalController.dismiss(undefined, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.select-profile-modal-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.5rem;
}

.dropdown,
.user-details {
  width: 100%;
}
</style>
