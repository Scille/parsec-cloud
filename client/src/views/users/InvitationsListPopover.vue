<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="invitations-list-container">
    <div class="invitations-list">
      <div class="invitations-list-header">
        <ion-text class="invitations-list-header__title title-h4">
          {{ $t('invitationList.title') }}
        </ion-text>
        <span class="invitations-list-header__counter body-sm">
          {{ invitations.length }}
        </span>
        <ion-button
          @click="onInviteClick"
          class="invitations-list-header__button"
          size="small"
        >
          <ion-icon
            slot="start"
            :icon="personAdd"
            class="button-icon"
          />
          {{ $t('invitationList.invite') }}
        </ion-button>
      </div>
      <ion-list class="invitations-list-content">
        <div
          class="invitations-list-content__empty"
          v-if="invitations.length === 0"
        >
          <ion-text class="body-lg">
            {{ $t('invitationList.noInvitations') }}
          </ion-text>
        </div>
        <invitation-popover-item
          v-for="invitation in invitations"
          :invitation="invitation"
          :key="invitation.claimerEmail"
          @cancel="cancelInvitation"
          @greet-user="greetUser"
        />
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MsModalResult } from '@/components/core';
import InvitationPopoverItem from '@/components/users/InvitationPopoverItem.vue';
import { UserInvitation, listUserInvitations } from '@/parsec';
import { IonButton, IonIcon, IonList, IonText, popoverController } from '@ionic/vue';
import { personAdd } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

const invitations: Ref<UserInvitation[]> = ref([]);

onMounted(async () => {
  const listResult = await listUserInvitations();

  if (listResult.ok) {
    invitations.value = listResult.value;
  }
});

async function cancelInvitation(invitation: UserInvitation): Promise<void> {
  await popoverController.dismiss({ invitation: invitation, action: 'cancel' }, MsModalResult.Confirm);
}

async function greetUser(invitation: UserInvitation): Promise<void> {
  await popoverController.dismiss({ invitation: invitation, action: 'greet' }, MsModalResult.Confirm);
}

async function onInviteClick(): Promise<void> {
  await popoverController.dismiss({ action: 'invite' }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.invitations-list-container {
  display: flex;
  align-items: center;
  flex-direction: column;
  --fill-color: var(--parsec-color-light-primary-900);
  overflow: visible;
}

.invitations-list {
  width: 100%;
  border-radius: var(--parsec-radius-12);
  overflow: hidden;

  &-header {
    background: var(--parsec-color-light-primary-800);
    color: var(--parsec-color-light-primary-30);
    padding: 0.5rem 1.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;

    &__title {
      padding: 0;
      display: flex;
      align-items: center;
    }

    &__counter {
      margin-right: auto;
      padding: 0 0.25rem;
      background: var(--parsec-color-light-primary-30-opacity15);
      border-radius: var(--parsec-radius-12);
      display: flex;
      height: fit-content;
      align-items: center;
    }

    &__button {
      --background: none !important;
      --background-hover: var(--parsec-color-light-primary-30-opacity15) !important;

      .button-icon {
        font-size: 0.875rem;
        margin-right: 0.5rem;
      }
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__toggle {
      color: var(--parsec-color-light-secondary-medium);
      margin-left: auto;
      display: flex;
      align-items: center;
      cursor: pointer;

      &::part(label) {
        transition: opacity 150ms ease-in-out;
        opacity: 0.6;
        margin-right: 0.5rem;
      }

      &:hover,
      &.toggle-checked {
        &::part(label) {
          opacity: 1;
        }
      }
    }
  }

  &-content {
    background: var(--parsec-color-light-secondary-white);
    color: var(--parsec-color-light-primary-900);
    border-radius: 0 0 var(--parsec-radius-6) var(--parsec-radius-6);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    padding: 0;
    height: 40vh;
    max-height: 25rem;
    transition: all 250ms ease-in-out;

    &__empty {
      overflow: hidden;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 0.5rem;
      margin: auto;
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}
</style>
