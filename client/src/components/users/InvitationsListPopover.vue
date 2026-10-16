<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="invitations-list-container">
    <div class="invitations-list">
      <div class="invitations-list-header">
        <ion-text class="invitations-list-header__title">
          {{ $msTranslate('invitationList.title') }}
        </ion-text>
        <span class="invitations-list-header__counter">
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
            class="button-icon button-icon-left"
          />
          {{ $msTranslate('invitationList.invite') }}
        </ion-button>
      </div>
      <ion-list class="invitations-list-content">
        <div
          class="invitations-list-content__empty"
          v-if="invitations.length === 0"
        >
          <ion-text>
            {{ $msTranslate('invitationList.noInvitations') }}
          </ion-text>
        </div>
        <invitation-popover-item
          v-for="invitation in invitations"
          :invitation="invitation"
          :key="invitation.claimerEmail"
          @cancel="cancelInvitation"
          @greet-user="greetUser"
          :information-manager="informationManager"
        />
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import InvitationPopoverItem from '@/components/users/InvitationPopoverItem.vue';
import { InvitationAction } from '@/components/users/types';
import { UserInvitation, listUserInvitations } from '@/parsec';
import { InformationManager } from '@/services/informationManager';
import { IonButton, IonIcon, IonList, IonText, popoverController } from '@ionic/vue';
import { personAdd } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';
import { Ref, onMounted, ref } from 'vue';

defineProps<{
  informationManager: InformationManager;
}>();

const invitations: Ref<UserInvitation[]> = ref([]);

onMounted(async () => {
  const listResult = await listUserInvitations({ skipOthers: false });

  if (listResult.ok) {
    invitations.value = listResult.value;
  }
});

async function cancelInvitation(invitation: UserInvitation): Promise<void> {
  await popoverController.dismiss({ invitation: invitation, action: InvitationAction.Cancel }, MsModalResult.Confirm);
}

async function greetUser(invitation: UserInvitation): Promise<void> {
  await popoverController.dismiss({ invitation: invitation, action: InvitationAction.Greet }, MsModalResult.Confirm);
}

async function onInviteClick(): Promise<void> {
  await popoverController.dismiss({ action: InvitationAction.Invite }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.invitations-list-container {
  display: flex;
  align-items: center;
  flex-direction: column;
  --fill-color: #{ms.color('text-base-heading')};
  overflow: visible;
}

.invitations-list {
  width: 100%;
  border-radius: ms.radius('2xl');
  overflow: hidden;

  &-header {
    background: var(--parsec-color-popover-header-background);
    color: ms.color('text-on-color-heading');
    padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');

    &__title {
      @include ms.font('heading-h5');
      padding: ms.spacing('padding-none');
      display: flex;
      align-items: center;
    }

    &__counter {
      @include ms.font('body-sm-regular');
      margin-right: auto;
      padding: 0 ms.spacing('padding-sm');
      background: ms.color('surface-neutral-on-color');
      border-radius: ms.radius('2xl');
      display: flex;
      height: fit-content;
      align-items: center;
    }

    &__button {
      --background: none !important;
      --background-hover: #{ms.color('surface-neutral-on-color')} !important;

      .button-icon {
        font-size: 0.875rem;
        margin-right: 0.5rem;
      }
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__toggle {
      color: ms.color('text-on-color-label');
      margin-left: auto;
      display: flex;
      align-items: center;
      cursor: pointer;

      &::part(label) {
        transition: opacity 150ms ease-in-out;
        opacity: ms.opacity('6');
        margin-right: 0.5rem;
      }

      &:hover,
      &.toggle-checked {
        &::part(label) {
          opacity: ms.opacity('10');
        }
      }
    }
  }

  &-content {
    background: ms.color('surface-base-default');
    color: ms.color('text-base-heading');
    border-radius: 0 0 ms.radius('md') ms.radius('md');
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    padding: ms.spacing('padding-none');
    height: 40vh;
    max-height: 25rem;
    transition: all 250ms ease-in-out;

    &__empty {
      @include ms.font('body-lg-regular');
      overflow: hidden;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: ms.spacing('gap-lg');
      margin: auto;
      color: ms.color('text-base-description');
    }
  }
}
</style>
