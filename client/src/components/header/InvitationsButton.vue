<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-show="invitations.length > 0"
    @click="openInvitationsMenu($event)"
    id="invitations-button"
    size="large"
    :fill="isSmallDisplay ? 'outline' : 'clear'"
    :class="{ unread: invitations.length > 0 }"
    slot="icon-only"
  >
    <span
      class="unread-count"
      :class="{ 'unread-count--more': invitations.length > 99 }"
      v-if="invitations.length > 0"
    >
      {{ invitations.length > 99 ? '99+' : invitations.length }}
    </span>
    <ion-icon
      :icon="mailUnread"
      class="gradient-button-icon"
    />
  </ion-button>
</template>

<script setup lang="ts">
import { InvitationAction } from '@/components/users';
import InvitationsListModal from '@/components/users/InvitationsListModal.vue';
import InvitationsListPopover from '@/components/users/InvitationsListPopover.vue';
import { ClientCancelInvitationErrorTag, UserInvitation, cancelInvitation, listUserInvitations } from '@/parsec';
import { Routes, navigateTo } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import { IonButton, IonIcon, modalController, popoverController } from '@ionic/vue';
import { mailUnread } from 'ionicons/icons';
import { Answer, MsModalResult, askQuestion, useWindowSize } from 'megashark-lib';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
let eventCbId: string | null = null;
const invitations: Ref<UserInvitation[]> = ref([]);
const { isLargeDisplay, isSmallDisplay } = useWindowSize();

onMounted(async () => {
  eventCbId = await eventDistributor.value.registerCallback([Events.InvitationUpdated], async (event: Events, _data?: EventData) => {
    if (event === Events.InvitationUpdated) {
      await updateInvitations();
    }
  });
  await updateInvitations();
});

onUnmounted(async () => {
  if (eventCbId) {
    await eventDistributor.value.removeCallback(eventCbId);
  }
});

async function updateInvitations(): Promise<void> {
  const result = await listUserInvitations({ skipOthers: false });
  if (result.ok) {
    invitations.value = result.value;
  }
}

async function openInvitationsMenu(event: Event): Promise<void> {
  event.stopPropagation();
  let result: { data?: { action: InvitationAction; invitation?: UserInvitation }; role?: string };

  if (isLargeDisplay.value) {
    const popover = await popoverController.create({
      component: InvitationsListPopover,
      alignment: 'center',
      event: event,
      cssClass: 'invitations-list-popover',
      showBackdrop: false,
      componentProps: {
        informationManager: informationManager.value,
      },
    });
    await popover.present();
    result = await popover.onDidDismiss();
    await popover.dismiss();
  } else {
    const modal = await modalController.create({
      component: InvitationsListModal,
      cssClass: 'invitations-list-modal',
      showBackdrop: true,
      handle: true,
      backdropDismiss: true,
      breakpoints: isLargeDisplay.value ? undefined : [0, 0.5, 1],
      expandToScroll: false,
      initialBreakpoint: isLargeDisplay.value ? undefined : 0.5,
      componentProps: {
        informationManager: informationManager.value,
      },
    });
    await modal.present();
    result = await modal.onDidDismiss();
    await modal.dismiss();
  }
  if (result.role === MsModalResult.Confirm && result.data && result.data.action) {
    if (result.data.action === InvitationAction.Greet && result.data.invitation) {
      await greetUser(result.data.invitation);
    } else if (result.data.action === InvitationAction.Cancel && result.data.invitation) {
      await cancelUserInvitation(result.data.invitation);
    } else if (result.data.action === InvitationAction.Invite) {
      await navigateTo(Routes.Invitations, { query: { openInvite: true } });
    }
  }
}

async function cancelUserInvitation(invitation: UserInvitation): Promise<void> {
  const answer = await askQuestion(
    'UsersPage.invitation.cancelInvitation.title',
    { key: 'UsersPage.invitation.cancelInvitation.message', data: { email: invitation.claimerEmail } },
    {
      yesText: isLargeDisplay.value ? 'UsersPage.invitation.cancelInvitation.yes.long' : 'UsersPage.invitation.cancelInvitation.yes.short',
      noText: isLargeDisplay.value ? 'UsersPage.invitation.cancelInvitation.no.long' : 'UsersPage.invitation.cancelInvitation.no.short',
      yesIsDangerous: true,
    },
  );

  if (answer === Answer.No) {
    return;
  }

  const result = await cancelInvitation(invitation.token);

  if (result.ok) {
    informationManager.value.present(
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
      informationManager.value.present(
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
    showBackdrop: true,
    handle: false,
    breakpoints: isLargeDisplay.value ? undefined : [1],
    expandToScroll: false,
    initialBreakpoint: isLargeDisplay.value ? undefined : 1,
    componentProps: {
      invitation: invitation,
      informationManager: informationManager.value,
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
  @include ms.font('label-md-medium');
  overflow: visible;

  .button-text {
    @include ms.responsive-breakpoint('lg') {
      display: none;
    }
  }

  &.unread {
    position: relative;

    .unread-count {
      position: absolute;
      z-index: 3;
      right: -7px;
      top: -9px;
      padding-inline: ms.spacing('padding-xs');
      min-width: 1.125rem;
      width: fit-content;
      height: 1.125rem;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 11px;
      font-weight: 600;
      color: ms.color('text-error-on-color');
      background: ms.color('surface-error-default');
      border: ms.border('thick') solid ms.color('surface-brand-default-subtle');
      border-radius: ms.radius('2xl');

      @include ms.responsive-breakpoint('sm') {
        left: 12px;
        right: auto;
        border-color: ms.color('border-base-on-color');
      }

      &--more {
        font-size: 10px;
        right: -10px;
        justify-content: end;
      }
    }
  }

  @include ms.responsive-breakpoint('sm') {
    &::part(native) {
      overflow: visible;
    }
  }
}
</style>
