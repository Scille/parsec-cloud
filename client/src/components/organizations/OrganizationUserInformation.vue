<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-users">
    <div class="card-header">
      <ion-title class="card-header__title title-h3">
        {{ $msTranslate('OrganizationPage.users.title') }}
      </ion-title>

      <ion-button
        fill="clear"
        class="card-header__button user-invite-button"
        @click="inviteUser"
        v-show="userInfo.currentProfile === UserProfile.Admin"
      >
        <ion-icon
          :icon="personAdd"
          class="user-invite-button__icon"
        />
      </ion-button>
    </div>

    <div class="card-content">
      <div
        button
        class="users-list"
        @click="navigateTo(Routes.Users)"
      >
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

      <div class="user-active">
        <div class="user-active-list">
          <!-- Admin -->
          <div class="user-active-list-item">
            <user-profile-tag :profile="UserProfile.Admin" />
            <ion-text class="user-active-list-item__value title-h4">
              {{ orgInfo.users.admins }}
            </ion-text>
          </div>
          <!-- Standard -->
          <div class="user-active-list-item">
            <user-profile-tag :profile="UserProfile.Standard" />
            <ion-text class="user-active-list-item__value title-h4">
              {{ orgInfo.users.standards }}
            </ion-text>
          </div>
          <!-- Outsiders if allowed -->
          <div
            v-if="orgInfo.outsidersAllowed"
            class="user-active-list-item"
          >
            <user-profile-tag :profile="UserProfile.Outsider" />
            <ion-text class="user-active-list-item__value title-h4">
              {{ orgInfo.users.outsiders }}
            </ion-text>
          </div>
        </div>

        <ion-button
          fill="clear"
          class="user-active__button"
          @click="navigateTo(Routes.Users)"
        >
          {{ $msTranslate('OrganizationPage.users.seeUsers') }}
        </ion-button>
      </div>

      <div
        class="invitation-card-list"
        v-show="userInfo.currentProfile === UserProfile.Admin"
      >
        <div
          button
          class="invitation-card-list-item"
          @click="goToInvitations(InvitationView.EmailInvitation)"
        >
          <ion-text class="invitation-card-list-item__number title-h1">{{ invitationCount }}</ion-text>
          <ion-text class="invitation-card-list-item__title button-large">
            {{ $msTranslate('OrganizationPage.users.invitations.emailInvitations') }}
          </ion-text>
          <ion-icon
            class="invitation-card-list-item__icon"
            :icon="mailUnread"
          />
        </div>

        <div
          v-if="false"
          button
          class="invitation-card-list-item"
          @click="goToInvitations(InvitationView.AsyncEnrollmentRequest)"
        >
          <ion-text class="invitation-card-list-item__number title-h1">{{ asyncEnrollmentsCount }}</ion-text>
          <ion-text class="invitation-card-list-item__title button-large">
            {{ $msTranslate('OrganizationPage.users.invitations.linkRequests') }}
          </ion-text>
          <ion-icon
            class="invitation-card-list-item__icon"
            :icon="link"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import UserProfileTag from '@/components/users/UserProfileTag.vue';
import { ClientInfo, listAsyncEnrollments, listUserInvitations, OrganizationInfo, UserProfile } from '@/parsec';
import { navigateTo, Routes } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { InvitationView } from '@/views/invitations/types';
import { IonButton, IonIcon, IonText, IonTitle } from '@ionic/vue';
import { link, mailUnread, personAdd } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';

defineProps<{
  userInfo: ClientInfo;
  orgInfo: OrganizationInfo;
}>();

const invitationCount = ref(0);
const asyncEnrollmentsCount = ref(0);
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
let cbId: string | undefined = undefined;

onMounted(async () => {
  cbId = await eventDistributor.value.registerCallback(Events.InvitationUpdated, async (event: Events, _data?: EventData) => {
    if (event === Events.InvitationUpdated) {
      await refresh();
    }
  });
  await refresh();
});

onUnmounted(async () => {
  if (cbId) {
    await eventDistributor.value.removeCallback(cbId);
    cbId = undefined;
  }
});

async function refresh(): Promise<void> {
  const invResult = await listUserInvitations();
  invitationCount.value = invResult.ok ? invResult.value.length : 0;
  const enrollmentsResult = await listAsyncEnrollments();
  asyncEnrollmentsCount.value = enrollmentsResult.ok ? enrollmentsResult.value.length : 0;
}

async function goToInvitations(view: InvitationView): Promise<void> {
  await navigateTo(Routes.Invitations, { replace: false, query: { invitationView: view } });
}

async function inviteUser(): Promise<void> {
  await navigateTo(Routes.Invitations, { query: { openInvite: true } });
}
</script>

<style scoped lang="scss">
.organization-users {
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-white);
  align-items: center;
  gap: 1rem;
  width: 100%;
  max-width: 30rem;
  border-radius: var(--parsec-radius-12);
  height: fit-content;
  box-shadow: var(--parsec-shadow-input);
  padding: 1.5rem;
}

.card-header {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 0.75rem;

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__button {
    align-self: stretch;
    color: var(--parsec-color-light-secondary-soft-text);
    border-radius: var(--parsec-radius-8);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    --background-hover: var(--parsec-color-light-secondary-background);
    box-shadow: var(--parsec-shadow-input);

    &::part(native) {
      padding: 0.625rem 1rem;
    }
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 1rem;
}

.invitation-card-list {
  display: flex;
  justify-content: center;
  width: 100%;
  gap: 1rem;

  &-item {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 0.25rem;
    position: relative;
    padding: 1rem 0.75rem;
    background: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-8);
    width: 100%;
    cursor: pointer;
    box-shadow: none;
    transition:
      background 0.2s,
      box-shadow 0.2s;

    &:hover {
      background: var(--parsec-color-light-secondary-contrast);
      box-shadow: var(--parsec-shadow-soft);
    }

    &__number {
      color: var(--parsec-color-light-secondary-white);
    }

    &__title {
      color: var(--parsec-color-light-secondary-white);
      opacity: 0.9;
    }

    &__icon {
      position: absolute;
      top: 0.5rem;
      right: 0.5rem;
      font-size: 2rem;
      color: var(--parsec-color-light-secondary-white);
      opacity: 0.2;
    }
  }
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
      width: 100%;
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
    color: var(--parsec-color-light-secondary-soft-text);
    --background-hover: var(--parsec-color-light-secondary-background);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    box-shadow: var(--parsec-shadow-input);
    border-radius: var(--parsec-radius-8);

    &::part(native) {
      padding: 0.625rem 0.75rem;
    }

    &__icon {
      color: var(--parsec-color-light-secondary-soft-text);
      font-size: 1rem;
    }
  }

  .user-active {
    display: flex;
    flex-direction: column;
    width: 100%;
    background: var(--parsec-color-light-secondary-background);
    padding: 1.5rem 1rem 1rem;
    border-radius: var(--parsec-radius-6);
    gap: 1rem;
    align-items: center;

    &-list {
      display: flex;
      gap: 0.25rem;
      width: 100%;

      &-item {
        width: 100%;
        display: flex;
        align-items: center;
        flex-direction: column;
        gap: 0.25rem;
      }
    }

    &__button {
      color: var(--parsec-color-light-secondary-soft-text);
      --background: var(--parsec-color-light-secondary-white);
      --background-hover: var(--parsec-color-light-secondary-background);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      box-shadow: var(--parsec-shadow-input);
      border-radius: var(--parsec-radius-8);
      width: 100%;

      &::part(native) {
        padding: 0.625rem 1rem;
      }
    }
  }
}
</style>
