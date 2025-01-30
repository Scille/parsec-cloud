<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-show="invitations.length > 0"
    @click="openInvitationsPopover($event)"
    id="invitations-button"
    class="button-medium"
    :class="{
      unread: invitations,
    }"
  >
    {{ $msTranslate({ key: 'HeaderPage.invitations.title', data: { count: invitations.length }, count: invitations.length }) }}
    <ion-icon :icon="mail" />
  </ion-button>
</template>

<script setup lang="ts">
import { Answer, MsModalResult, askQuestion } from 'megashark-lib';
import { InvitationAction } from '@/components/users';
import InvitationsListPopover from '@/components/users/InvitationsListPopover.vue';
import { ClientCancelInvitationErrorTag, UserInvitation, cancelInvitation, listUserInvitations } from '@/parsec';
import { Routes, navigateTo } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import { IonButton, IonIcon, modalController, popoverController } from '@ionic/vue';
import { mail } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
let eventCbId: string | null = null;
const invitations: Ref<UserInvitation[]> = ref([]);

onMounted(async () => {
  eventCbId = await eventDistributor.registerCallback(Events.InvitationUpdated, async (event: Events, _data?: EventData) => {
    if (event === Events.InvitationUpdated) {
      await updateInvitations();
    }
  });
  await updateInvitations();
});

onUnmounted(async () => {
  if (eventCbId) {
    await eventDistributor.removeCallback(eventCbId);
  }
});

async function updateInvitations(): Promise<void> {
  const result = await listUserInvitations();
  if (result.ok) {
    invitations.value = result.value;
  }
}

async function openInvitationsPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const popover = await popoverController.create({
    component: InvitationsListPopover,
    alignment: 'center',
    event: event,
    cssClass: 'invitations-list-popover',
    showBackdrop: false,
    componentProps: {
      informationManager: informationManager,
    },
  });
  await popover.present();
  const { role, data } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm && data.action) {
    if (data.action === InvitationAction.Greet) {
      await greetUser(data.invitation);
    } else if (data.action === InvitationAction.Cancel) {
      await cancelUserInvitation(data.invitation);
    } else if (data.action === InvitationAction.Invite) {
      await navigateTo(Routes.Users, { query: { openInvite: true } });
      await updateInvitations();
    }
  }
}

async function cancelUserInvitation(invitation: UserInvitation): Promise<void> {
  const answer = await askQuestion(
    'UsersPage.invitation.cancelInvitation.title',
    { key: 'UsersPage.invitation.cancelInvitation.message', data: { email: invitation.claimerEmail } },
    {
      yesText: 'UsersPage.invitation.cancelInvitation.yes',
      noText: 'UsersPage.invitation.cancelInvitation.no',
      yesIsDangerous: true,
    },
  );

  if (answer === Answer.No) {
    return;
  }

  const result = await cancelInvitation(invitation.token);

  if (result.ok) {
    informationManager.present(
      new Information({
        message: 'UsersPage.invitation.cancelSuccess',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await updateInvitations();
  } else {
    // In all those cases we can just refresh the list and the invitation should disappear, no need
    // to warn the user.
    if (
      result.error.tag === ClientCancelInvitationErrorTag.NotFound ||
      result.error.tag === ClientCancelInvitationErrorTag.NotAllowed ||
      result.error.tag === ClientCancelInvitationErrorTag.AlreadyCancelled ||
      result.error.tag === ClientCancelInvitationErrorTag.Completed
    ) {
      await updateInvitations();
    } else {
      informationManager.present(
        new Information({
          message: 'UsersPage.invitation.cancelFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
}

async function greetUser(invitation: UserInvitation): Promise<void> {
  const modal = await modalController.create({
    component: GreetUserModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'greet-organization-modal',
    componentProps: {
      invitation: invitation,
      informationManager: informationManager,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
  await updateInvitations();
}
</script>

<style scoped lang="scss">
#invitations-button {
  overflow: visible;

  &::part(native) {
    background: var(--parsec-color-light-warning-100);
    --background-hover: none;
    color: var(--parsec-color-light-warning-500);
    padding: 0.375rem 0.625rem;
    border: 1px solid var(--parsec-color-light-warning-100);
    transition: all 150ms ease-in-out;

    &:hover {
      color: var(--parsec-color-light-warning-700);
    }
  }

  ion-icon {
    font-size: 1.375rem;
    margin-left: 0.5rem;
  }

  &.unread {
    position: relative;

    &::after {
      content: '';
      position: absolute;
      right: 6px;
      top: 5px;
      width: 0.625rem;
      height: 0.625rem;
      background: var(--parsec-color-light-danger-500);
      border: 2px solid var(--parsec-color-light-warning-100);
      border-radius: var(--parsec-radius-12);
    }
  }
}
</style>
