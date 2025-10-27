<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="invitations-list-container">
    <div class="invitations-list">
      <div class="invitations-list-header">
        <ion-text class="invitations-list-header__title title-h4">
          {{ $msTranslate('invitationList.title') }}
        </ion-text>
        <span class="invitations-list-header__counter body-sm">
          {{ invitations.length }}
        </span>
        <ion-button
          @click="onInviteClick"
          class="invitations-list-header__button"
        >
          <ion-icon
            slot="start"
            :icon="personAdd"
            class="button-icon"
          />
        </ion-button>
      </div>
      <ion-list class="invitations-list-content">
        <div
          class="invitations-list-content__empty"
          v-if="invitations.length === 0"
        >
          <ion-text class="body-lg">
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
import { IonButton, IonIcon, IonList, IonText, modalController } from '@ionic/vue';
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
  await modalController.dismiss({ invitation: invitation, action: InvitationAction.Cancel }, MsModalResult.Confirm);
}

async function greetUser(invitation: UserInvitation): Promise<void> {
  await modalController.dismiss({ invitation: invitation, action: InvitationAction.Greet }, MsModalResult.Confirm);
}

async function onInviteClick(): Promise<void> {
  await modalController.dismiss({ action: InvitationAction.Invite }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.invitations-list {
  width: 100%;
  border-radius: 0;
  overflow: hidden;

  &-header {
    &__button {
      --background: var(--parsec-color-light-primary-30-opacity15) !important;
      --background-hover: var(--parsec-color-light-primary-30-opacity15) !important;

      .button-icon {
        font-size: 0.875rem;
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
}
</style>
