<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-show="invitations.length > 0"
    @click="openInvitationsMenu($event)"
    id="invitations-button"
    class="button-medium"
    :class="{
      unread: invitations.length > 0,
      'gradient-button': isGradientButton,
    }"
  >
    <ion-text
      v-if="isGradientButton"
      :class="{ 'gradient-button-text': isGradientButton }"
    >
      <span class="title-h2">{{ invitations.length }}</span>
      <span class="button-large">
        {{ $msTranslate({ key: 'HeaderPage.invitations.smallDisplayTitle', data: { count: invitations.length } }) }}
      </span>
    </ion-text>
    <span
      class="unread-count"
      :class="{ 'unread-count--more': invitations.length > 99 }"
      v-if="invitations.length > 0 && !isGradientButton"
    >
      {{ invitations.length > 99 ? '99+' : invitations.length }}
    </span>
    <ion-icon
      v-if="!isGradientButton"
      :icon="mail"
      class="invitation-button-icon"
    />
    <ion-icon
      v-else
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
import { IonButton, IonIcon, IonText, modalController, popoverController } from '@ionic/vue';
import { mail, mailUnread } from 'ionicons/icons';
import { Answer, MsModalResult, askQuestion, useWindowSize } from 'megashark-lib';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
let eventCbId: string | null = null;
const invitations: Ref<UserInvitation[]> = ref([]);
const { isLargeDisplay } = useWindowSize();

defineProps<{
  isGradientButton?: boolean;
}>();

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
#invitations-button:not(.gradient-button) {
  overflow: visible;
  --background: var(--parsec-color-light-primary-50);
  --color: var(--parsec-color-light-primary-500);

  &::part(native) {
    --background-hover: none;
    padding: 0.625rem;
    transition: all 150ms ease-in-out;
    border-radius: var(--parsec-radius-12);

    &:hover {
      --background: var(--parsec-color-light-primary-100);
      color: var(--parsec-color-light-primary-700);
    }
  }

  .invitation-button-icon {
    font-size: 1.375rem;
  }

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
      right: -8px;
      top: -8px;
      padding-inline: 2px;
      min-width: 1.125rem;
      width: fit-content;
      height: 1.125rem;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 11px;
      font-weight: 600;
      color: var(--parsec-color-light-secondary-white);
      background: var(--parsec-color-light-danger-500);
      border: 2px solid var(--parsec-color-light-primary-50);
      border-radius: var(--parsec-radius-12);

      @include ms.responsive-breakpoint('sm') {
        left: 12px;
        right: auto;
        border-color: var(--parsec-color-light-secondary-white);
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
      padding: 0.5rem;
      border-radius: var(--parsec-radius-circle);
      --background: var(--parsec-color-light-secondary-white);
      box-shadow: var(--parsec-shadow-soft);
      overflow: visible;
    }
  }
}

.gradient-button {
  position: relative;

  &::part(native) {
    --background: var(--parsec-color-light-gradient-background);
    --background-hover: var(--parsec-color-light-primary-600);
    --color: var(--parsec-color-light-secondary-white);
    justify-content: flex-start;
    text-align: start;
    padding: 0.75rem 1rem;
  }

  &-text {
    display: flex;
    flex-direction: column;
    width: 100%;
    text-wrap: auto;
  }

  &-icon {
    font-size: 3.25rem;
    color: var(--parsec-color-light-secondary-white);
    position: absolute;
    right: -1.5rem;
    opacity: 0.2;
  }
}
</style>
