<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-users">
    <div class="card-header">
      <ion-title class="card-header__title">
        {{ $msTranslate('OrganizationPage.users.title') }}
      </ion-title>

      <ion-button
        fill="outline"
        slot="icon-only"
        class="card-header__button user-invite-button"
        @click="inviteUser"
        v-show="orgInfo.clientInfo.currentProfile === UserProfile.Admin"
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
          <ion-text class="users-list-item__title">
            {{ orgInfo.users.active }}
          </ion-text>
          <ion-text class="users-list-item__description">
            {{ $msTranslate({ key: 'OrganizationPage.users.activeUsers', count: orgInfo.users.active }) }}
          </ion-text>
        </div>

        <!-- Revoked -->
        <div class="users-list-item">
          <ion-text class="users-list-item__title">
            {{ orgInfo.users.revoked }}
          </ion-text>
          <ion-text class="users-list-item__description">
            {{ $msTranslate({ key: 'OrganizationPage.users.revokedUsers', count: orgInfo.users.revoked }) }}
          </ion-text>
        </div>

        <!-- Frozen -->
        <div class="users-list-item">
          <ion-text class="users-list-item__title">
            {{ orgInfo.users.frozen }}
          </ion-text>
          <ion-text class="users-list-item__description">
            {{ $msTranslate({ key: 'OrganizationPage.users.frozenUsers', count: orgInfo.users.frozen }) }}
          </ion-text>
        </div>
      </div>

      <div class="user-active">
        <div class="user-active-list">
          <!-- Admin -->
          <div class="user-active-list-item">
            <user-profile-tag :profile="UserProfile.Admin" />
            <ion-text class="user-active-list-item__value">
              {{ orgInfo.users.admins }}
            </ion-text>
          </div>
          <!-- Standard -->
          <div class="user-active-list-item">
            <user-profile-tag :profile="UserProfile.Standard" />
            <ion-text class="user-active-list-item__value">
              {{ orgInfo.users.standards }}
            </ion-text>
          </div>
          <!-- Outsiders if allowed -->
          <div
            v-if="orgInfo.clientInfo.serverOrganizationConfig.userProfileOutsiderAllowed"
            class="user-active-list-item"
          >
            <user-profile-tag :profile="UserProfile.Outsider" />
            <ion-text class="user-active-list-item__value">
              {{ orgInfo.users.outsiders }}
            </ion-text>
          </div>
        </div>

        <ion-button
          fill="outline"
          class="user-active__button"
          @click="navigateTo(Routes.Users)"
        >
          {{ $msTranslate('OrganizationPage.users.seeUsers') }}
        </ion-button>
      </div>

      <div
        class="invitation-card-list"
        v-show="orgInfo.clientInfo.currentProfile === UserProfile.Admin"
      >
        <div
          button
          class="invitation-card-list-item"
          @click="goToInvitations(InvitationView.EmailInvitation)"
        >
          <ion-text class="invitation-card-list-item__number">{{ invitationCount }}</ion-text>
          <ion-text class="invitation-card-list-item__title">
            {{ $msTranslate('OrganizationPage.users.invitations.emailInvitations') }}
          </ion-text>
          <ion-icon
            class="invitation-card-list-item__icon"
            :icon="mailUnread"
          />
        </div>

        <div
          button
          class="invitation-card-list-item"
          @click="goToInvitations(InvitationView.AsyncEnrollmentRequest)"
        >
          <ion-text class="invitation-card-list-item__number">{{ asyncEnrollmentsCount }}</ion-text>
          <ion-text class="invitation-card-list-item__title">
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
import { listAsyncEnrollments, listUserInvitations, OrganizationInfo, UserProfile } from '@/parsec';
import { navigateTo, Routes } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { InvitationView } from '@/views/invitations/types';
import { IonButton, IonIcon, IonText, IonTitle } from '@ionic/vue';
import { link, mailUnread, personAdd } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';

defineProps<{
  orgInfo: OrganizationInfo;
}>();

const invitationCount = ref(0);
const asyncEnrollmentsCount = ref(0);
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
let cbId: string | undefined = undefined;

onMounted(async () => {
  cbId = await eventDistributor.value.registerCallback([Events.InvitationUpdated], async (event: Events, _data?: EventData) => {
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
  background: ms.color('surface-base-default');
  align-items: center;
  gap: ms.spacing('gap-3xl');
  width: 100%;
  max-width: 30rem;
  border-radius: ms.radius('2xl');
  height: fit-content;
  box-shadow: var(--parsec-shadow-input);
  padding: ms.spacing('padding-4xl');
}

.card-header {
  display: flex;
  align-items: center;
  width: 100%;
  gap: ms.spacing('gap-2xl');

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-base-body');
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: ms.spacing('gap-3xl');
}

.invitation-card-list {
  display: flex;
  justify-content: center;
  width: 100%;
  gap: ms.spacing('gap-3xl');

  &-item {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: ms.spacing('gap-sm');
    position: relative;
    padding: ms.spacing('padding-3xl') ms.spacing('padding-2xl');
    background: ms.color('surface-neutral-on-color');
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    border-radius: ms.radius('lg');
    width: 100%;
    cursor: pointer;
    box-shadow: none;
    transition:
      background 0.2s,
      box-shadow 0.2s;

    &:hover {
      background: ms.color('surface-neutral-on-color-hover');
      box-shadow: var(--parsec-shadow-soft);
    }

    &__number {
      @include ms.font('heading-h2');
      color: ms.color('text-base-heading');
    }

    &__title {
      @include ms.font('label-lg-medium');
      color: ms.color('text-base-heading');
      opacity: ms.opacity('9');
    }

    &__icon {
      position: absolute;
      top: 0.5rem;
      right: 0.5rem;
      font-size: 2rem;
      color: ms.color('icon-neutral-default');
      opacity: ms.opacity('2');
    }
  }
}

.organization-users {
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  .users-list {
    display: flex;
    justify-content: space-around;
    gap: ms.spacing('gap-lg');

    &-item {
      display: flex;
      gap: ms.spacing('gap-sm');
      width: 100%;
      flex-direction: column;
      align-items: center;

      &__title {
        @include ms.font('heading-h3');
        color: ms.color('text-brand-default');
      }

      &__description {
        @include ms.font('body-md-medium');
        color: ms.color('text-base-description');
      }
    }
  }

  .user-active {
    display: flex;
    flex-direction: column;
    width: 100%;
    border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    padding: ms.spacing('padding-4xl') ms.spacing('padding-none') ms.spacing('padding-3xl');
    border-radius: ms.radius('md');
    gap: ms.spacing('gap-3xl');
    align-items: center;

    &-list {
      display: flex;
      gap: ms.spacing('gap-sm');
      width: 100%;

      &-item {
        width: 100%;
        display: flex;
        align-items: center;
        flex-direction: column;
        gap: ms.spacing('gap-sm');

        &__value {
          @include ms.font('heading-h5');
        }
      }
    }

    &__button {
      width: 100%;
    }
  }
}
</style>
