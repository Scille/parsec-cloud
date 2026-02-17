<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-users">
    <div class="card-header">
      <div class="card-header-text">
        <ion-title class="card-header-text__title title-h3">
          {{ $msTranslate('OrganizationPage.users.title') }}
        </ion-title>
        <ion-text class="card-header-text__subtitle subtitles-sm">
          {{ $msTranslate('OrganizationPage.users.subtitle') }}
        </ion-text>
      </div>
    </div>

    <div class="card-content">
      <div class="users-list users-status">
        <ion-text class="users-list__title title-h5">
          {{ $msTranslate('OrganizationPage.users.listTitle') }}
        </ion-text>
        <!-- Active users -->
        <div class="users-list-item">
          <div class="icon-container">
            <ms-image
              :image="PersonCheck"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.activeUsers', count: orgInfo.users.active }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.active }}
          </ion-text>
        </div>

        <!-- Revoked -->
        <div class="users-list-item">
          <div class="icon-container">
            <ms-image
              :image="PersonRemove"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.revokedUsers', count: orgInfo.users.revoked }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.revoked }}
          </ion-text>
        </div>

        <!-- Frozen -->
        <div class="users-list-item">
          <div class="icon-container">
            <ms-image
              :image="PersonFreeze"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.frozenUsers', count: orgInfo.users.frozen }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.frozen }}
          </ion-text>
        </div>
      </div>

      <div class="users-list users-active">
        <ion-text class="users-list__title title-h5">
          {{ $msTranslate('OrganizationPage.users.profileTitle') }}
        </ion-text>
        <!-- Active users -->
        <div class="users-list-item">
          <div class="icon-container">
            <ion-icon
              :icon="flower"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.adminUsers', count: orgInfo.users.admins }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.admins }}
          </ion-text>
        </div>

        <div class="users-list-item">
          <div class="icon-container">
            <ion-icon
              :icon="personCircle"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.memberUsers', count: orgInfo.users.standards }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.standards }}
          </ion-text>
        </div>

        <div
          class="users-list-item"
          v-if="orgInfo.outsidersAllowed"
        >
          <div class="icon-container">
            <ion-icon
              :icon="idCard"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.externalUsers', count: orgInfo.users.outsiders }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.outsiders }}
          </ion-text>
        </div>
      </div>

      <div class="users-list users-mfa">
        <ion-text class="users-list__title title-h5">
          {{ $msTranslate('OrganizationPage.users.mfaTitle') }}
        </ion-text>
        <!-- MFA active -->
        <div class="users-list-item">
          <div class="icon-container">
            <ion-icon
              :icon="shieldCheckmark"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.mfaActive', count: orgInfo.users.mfaActive }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.mfaActive }}
          </ion-text>
        </div>

        <div class="users-list-item">
          <div class="icon-container">
            <ms-image
              :image="ShieldCancel"
              class="users-list-item__icon"
            />
          </div>
          <ion-text class="users-list-item__title subtitles-sm">
            {{ $msTranslate({ key: 'OrganizationPage.users.mfaInactive', count: orgInfo.users.mfaInactive }) }}
          </ion-text>
          <ion-text class="users-list-item__number title-h4">
            {{ orgInfo.users.mfaInactive }}
          </ion-text>
        </div>
      </div>

      <div
        class="invitation-card-list"
        v-show="userInfo.currentProfile === UserProfile.Admin && isSmallDisplay"
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

    <div class="card-buttons">
      <ion-button
        fill="clear"
        class="button-secondary"
        id="see-users-button"
        @click="navigateTo(Routes.Users)"
      >
        {{ $msTranslate('OrganizationPage.users.seeUsers') }}
      </ion-button>

      <ion-button
        fill="clear"
        class="button-secondary"
        id="user-invite-button"
        @click="inviteUser"
        v-show="userInfo.currentProfile === UserProfile.Admin"
      >
        <ion-icon
          :icon="personAdd"
          class="button-secondary__icon"
        />
        {{ $msTranslate('OrganizationPage.users.addUser') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import PersonCheck from '@/assets/images/person-check.svg?raw';
import PersonFreeze from '@/assets/images/person-freeze.svg?raw';
import PersonRemove from '@/assets/images/person-remove.svg?raw';
import ShieldCancel from '@/assets/images/shield-cancel.svg?raw';
import { ClientInfo, listAsyncEnrollments, listUserInvitations, OrganizationInfo, UserProfile } from '@/parsec';
import { navigateTo, Routes } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { InvitationView } from '@/views/invitations/types';
import { IonButton, IonIcon, IonText, IonTitle } from '@ionic/vue';
import { flower, idCard, link, mailUnread, personAdd, personCircle, shieldCheckmark } from 'ionicons/icons';
import { MsImage, useWindowSize } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';

defineProps<{
  userInfo: ClientInfo;
  orgInfo: OrganizationInfo;
}>();

const { isSmallDisplay } = useWindowSize();
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
  gap: 1.5rem;
  padding: 1.5rem;
  background: var(--parsec-color-light-secondary-white);
  width: 100%;
  height: fit-content;
  border-radius: var(--parsec-radius-18);
  box-shadow: var(--parsec-shadow-input);
  position: relative;
  max-width: 80rem;

  @include ms.responsive-breakpoint('md') {
    max-width: 30rem;
  }

  .card-header {
    display: flex;
    align-items: center;
    width: 100%;
    justify-content: space-between;
    gap: 0.75rem;

    &-text {
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }
  }

  .card-content {
    display: flex;
    gap: 1rem;

    @include ms.responsive-breakpoint('lg') {
      flex-wrap: wrap;
    }
  }

  .card-buttons {
    display: flex;
    position: absolute;
    top: 1.5rem;
    right: 1.5rem;
    gap: 0.75rem;

    @include ms.responsive-breakpoint('md') {
      position: initial;
      justify-content: center;
      flex-direction: column;
    }

    .button-secondary {
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
        margin-right: 0.25rem;
      }
    }
  }
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

.users-list {
  display: flex;
  justify-content: flex-start;
  flex-direction: column;
  padding: 0.75rem;
  gap: 0.75rem;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-background);
  box-shadow: var(--parsec-shadow-input);
  width: 100%;

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-item {
    display: flex;
    gap: 0.75rem;
    width: 100%;
    align-items: center;
    background: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-8);
    padding: 0.75rem 1rem 0.75rem 0.5rem;
    min-width: 14.375rem;

    .icon-container {
      border-radius: var(--parsec-radius-8);
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      padding: 0.375rem;
    }

    &__icon {
      flex-shrink: 0;
      width: 1.125rem;
      height: 1.125rem;
      font-size: 1.125rem;
    }

    &__title {
      color: var(--parsec-color-light-secondary-text);
      width: 100%;
    }

    &__number {
      color: var(--parsec-color-light-secondary-text);
    }

    &:hover {
      outline: 1px solid var(--parsec-color-light-secondary-light);
      box-shadow: var(--parsec-shadow-input);
      cursor: pointer;
    }
  }
}

.users-status {
  .users-list-item {
    &:nth-of-type(1) .icon-container {
      background: var(--parsec-color-light-success-100);
      --fill-color: var(--parsec-color-light-success-700);
    }

    &:nth-of-type(2) .icon-container {
      background: var(--parsec-color-light-danger-100);
      --fill-color: var(--parsec-color-light-danger-700);
    }

    &:nth-of-type(3) .icon-container {
      background: var(--parsec-color-light-secondary-disabled);
      --fill-color: var(--parsec-color-light-secondary-hard-grey);
    }
  }
}

.users-active {
  .users-list-item {
    &:nth-of-type(1) .icon-container {
      background: var(--parsec-color-tags-indigo-50);
      color: var(--parsec-color-tags-indigo-700);
    }

    &:nth-of-type(2) .icon-container {
      background: var(--parsec-color-tags-blue-50);
      color: var(--parsec-color-tags-blue-700);
    }

    &:nth-of-type(3) .icon-container {
      background: var(--parsec-color-tags-orange-50);
      color: var(--parsec-color-tags-orange-700);
    }
  }
}

.users-mfa {
  .users-list-item {
    &:nth-of-type(1) .icon-container {
      background: var(--parsec-color-light-success-100);
      color: var(--parsec-color-light-success-700);
    }

    &:nth-of-type(2) .icon-container {
      background: var(--parsec-color-light-danger-100);
      --fill-color: var(--parsec-color-light-danger-700);
    }
  }
}
</style>
