<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-users">
    <div class="card-header">
      <ion-title class="card-header__title title-h3">
        {{ $msTranslate('OrganizationPage.users.title') }}
      </ion-title>

      <ion-button
        fill="clear"
        class="card-header__button"
        @click="navigateTo(Routes.Users)"
      >
        {{ $msTranslate('OrganizationPage.users.seeUsers') }}
      </ion-button>
    </div>

    <div class="card-content">
      <div class="users-list">
        <!-- Active users -->
        <div class="users-list-item">
          <ion-text class="users-list-item__title title-h2">
            {{ orgInfo.users.active }}
          </ion-text>
          <ion-text class="users-list-item__description subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.activeUsers', count: orgInfo.users.active }) }}
          </ion-text>
        </div>

        <!-- Revoked -->
        <div class="users-list-item">
          <ion-text class="users-list-item__title title-h2">
            {{ orgInfo.users.revoked }}
          </ion-text>
          <ion-text class="users-list-item__description subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.revokedUsers', count: orgInfo.users.revoked }) }}
          </ion-text>
        </div>

        <!-- Frozen -->
        <div class="users-list-item">
          <ion-text class="users-list-item__title title-h2">
            {{ orgInfo.users.frozen }}
          </ion-text>
          <ion-text class="users-list-item__description subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.frozenUsers', count: orgInfo.users.frozen }) }}
          </ion-text>
        </div>
      </div>

      <div class="user-active-list">
        <!-- Admin -->
        <div class="user-active-list-item">
          <tag-profile :profile="UserProfile.Admin" />
          <ion-text class="user-active-list-item__value title-h4">
            {{ orgInfo.users.admins }}
          </ion-text>
        </div>
        <!-- Standard -->
        <div class="user-active-list-item">
          <tag-profile :profile="UserProfile.Standard" />
          <ion-text class="user-active-list-item__value title-h4">
            {{ orgInfo.users.standards }}
          </ion-text>
        </div>
        <!-- Outsiders if allowed -->
        <div
          v-if="orgInfo.outsidersAllowed"
          class="user-active-list-item"
        >
          <tag-profile :profile="UserProfile.Outsider" />
          <ion-text class="user-active-list-item__value title-h4">
            {{ orgInfo.users.outsiders }}
          </ion-text>
        </div>
      </div>

      <invitations-button
        v-if="isSmallDisplay"
        :is-gradient-button="true"
      />

      <ion-button
        fill="clear"
        class="user-invite-button"
        @click="inviteUser"
        v-if="isSmallDisplay"
      >
        <ion-icon
          :icon="personAdd"
          class="user-invite-button__icon"
        />
        {{ $msTranslate('OrganizationPage.users.addUser') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import TagProfile from '@/components/users/TagProfile.vue';
import { UserProfile, OrganizationInfo, ClientInfo } from '@/parsec';
import { personAdd } from 'ionicons/icons';
import InvitationsButton from '@/components/header/InvitationsButton.vue';
import { navigateTo, Routes } from '@/router';
import { useWindowSize } from 'megashark-lib';
import { IonButton, IonIcon, IonText, IonTitle } from '@ionic/vue';

const { isSmallDisplay } = useWindowSize();

defineProps<{
  userInfo: ClientInfo;
  orgInfo: OrganizationInfo;
}>();

async function inviteUser(): Promise<void> {
  await navigateTo(Routes.Users, { query: { openInvite: true } });
}
</script>

<style scoped lang="scss">
.organization-users {
  display: flex;
  width: 100%;
  max-width: 30rem;
  align-items: center;
  flex-direction: column;
}

.card-header {
  display: flex;
  align-items: center;
  margin-left: 1rem;
  width: 100%;

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__button {
    margin-left: auto;
    color: var(--parsec-color-light-secondary-text);
    --background-hover: var(--parsec-color-light-secondary-background);
    font-weight: 500;
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem;
  border-radius: var(--parsec-radius-8);
  width: 100%;
  gap: 1rem;
}

.organization-users {
  flex-direction: column;
  gap: 1rem;

  .users-list {
    display: flex;
    justify-content: space-around;
    gap: 0.5rem;

    &-item {
      display: flex;
      gap: 0.25rem;
      flex-direction: column;
      align-items: center;

      &__title {
        color: var(--parsec-color-light-primary-700);
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }

  .user-invite-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-white);
    --background-hover: var(--parsec-color-light-secondary-contrast);

    &::part(native) {
      padding: 0.75rem 1rem;
      border-radius: var(--parsec-radius-8);
      background: var(--parsec-color-light-secondary-text);
    }

    &__icon {
      color: var(--parsec-color-light-secondary-white);
      font-size: 1rem;
      margin-right: 0.625rem;
    }
  }

  .user-active-list {
    display: flex;
    gap: 1rem;
    justify-content: space-around;
    background: var(--parsec-color-light-secondary-background);
    padding: 1.5rem 0.5rem 1rem;
    border-radius: var(--parsec-radius-6);

    &-item {
      display: flex;
      align-items: center;
      flex-direction: column;
      gap: 0.25rem;
    }
  }
}
</style>
