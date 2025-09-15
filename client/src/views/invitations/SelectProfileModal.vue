<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="SELECT PROFILE"
    :close-button="{ visible: true }"
    :confirm-button="{
      disabled: selectedProfile === undefined,
      label: 'SELECT PROFILE',
      onClick: validateProfile,
    }"
    :cancel-button="{
      label: 'CANCEL',
      disabled: false,
      onClick: dismissModal,
    }"
  >
    <user-information
      class="user-details"
      :default-email="email"
      :default-name="name"
      :email-enabled="false"
      :name-enabled="false"
    />
    <ms-dropdown
      class="dropdown"
      title="PROFILE DROPDOWN"
      label="PROFILE DROPDOWN PLACEHOLDER"
      :options="profileOptions"
      @change="selectedProfile = $event.option.key"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import { modalController } from '@ionic/vue';
import { ref } from 'vue';
import UserInformation from '@/components/users/UserInformation.vue';
import { MsModalResult, MsOptions, MsModal, MsDropdown } from 'megashark-lib';
import { UserProfile } from '@/parsec';

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

<style scoped lang="scss"></style>
