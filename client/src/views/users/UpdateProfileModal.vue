<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="UsersPage.updateProfile.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: selectedProfile === undefined,
        label: 'UsersPage.updateProfile.actionUpdate',
        onClick: updateProfile,
      }"
      :cancel-button="{
        label: 'UsersPage.updateProfile.actionCancel',
        disabled: false,
        onClick: dismissModal,
      }"
    >
      <div
        class="update-profile-content"
        v-show="affectedUsers.length > 0"
      >
        <div class="update-profile">
          <ion-text class="update-profile-title body">
            {{ $msTranslate({ key: 'UsersPage.updateProfile.affectedUsers', count: affectedUsers.length }) }}
          </ion-text>
          <div class="update-profile-user">
            <span
              v-for="user in affectedUsers.slice(0, 3)"
              :key="user.id"
              class="update-profile-user__item subtitles-sm"
            >
              {{ user.humanHandle.label }}
            </span>
            <span
              v-show="affectedUsers.length > 3"
              class="update-profile-user__more body"
            >
              {{
                $msTranslate({
                  key: 'UsersPage.updateProfile.additionalUsers',
                  count: affectedUsers.length - 3,
                  data: { count: affectedUsers.length - 3 },
                })
              }}
            </span>
          </div>
        </div>
        <div
          class="warn-outsiders"
          v-show="users.length !== affectedUsers.length"
        >
          <ms-report-text :theme="MsReportTheme.Warning">
            {{ $msTranslate('UsersPage.updateProfile.cannotUpdateOutsiders') }}
          </ms-report-text>
        </div>

        <ms-dropdown
          class="dropdown"
          title="UsersPage.greet.profileDropdownTitle"
          label="UsersPage.greet.profileDropdownPlaceholder"
          :options="profileOptions"
          @change="onProfileSelected"
          :disabled="affectedUsers.length === 0"
        />
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { UserInfo, UserProfile } from '@/parsec';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsDropdown, MsDropdownChangeEvent, MsModal, MsModalResult, MsOptions, MsReportText, MsReportTheme } from 'megashark-lib';
import { Ref, computed, onMounted, ref } from 'vue';

const profileOptions: Ref<MsOptions> = ref(new MsOptions([]));

const props = defineProps<{
  users: Array<UserInfo>;
}>();

const affectedUsers = computed(() => {
  return props.users.filter((u) => u.currentProfile !== UserProfile.Outsider);
});

const selectedProfile = ref<UserProfile | undefined>();

onMounted(async () => {
  profileOptions.value = new MsOptions([
    {
      key: UserProfile.Admin,
      label: 'UsersPage.profile.admin.label',
      description: 'UsersPage.profile.admin.description',
      disabled: props.users.length === 1 && props.users[0].currentProfile === UserProfile.Admin,
    },
    {
      key: UserProfile.Standard,
      label: 'UsersPage.profile.standard.label',
      description: 'UsersPage.profile.standard.description',
      disabled: props.users.length === 1 && props.users[0].currentProfile === UserProfile.Standard,
    },
  ]);
});

async function onProfileSelected(event: MsDropdownChangeEvent): Promise<void> {
  selectedProfile.value = event.option.key as UserProfile;
}

async function updateProfile(): Promise<boolean> {
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
.update-profile-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.update-profile {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &-title {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-user {
    display: flex;
    gap: 0.5rem;

    &__item {
      background: var(--parsec-color-light-secondary-premiere);
      border-radius: var(--parsec-radius-8);
      padding: 0.125rem 0.5rem;
      color: var(--parsec-color-light-secondary-text);
    }

    &__more {
      color: var(--parsec-color-light-secondary-text);
    }
  }
}

.warn-outsiders {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
</style>
