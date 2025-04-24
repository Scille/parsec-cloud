<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-show="invitations.length > 0"
    @click="openInvitationsPopover($event)"
    id="invitations-button"
    class="button-medium"
    :class="{
      unread: invitations,
      'gradient-button': isGradientButton,
    }"
  >
    <span
      v-if="isLargeDisplay"
      class="button-text"
    >
      {{ $msTranslate({ key: 'HeaderPage.invitations.title', data: { count: invitations.length }, count: invitations.length }) }}
    </span>
    <ion-text
      v-if="isGradientButton"
      :class="{ 'gradient-button-text': isGradientButton }"
    >
      <span class="title-h2">{{ invitations.length }}</span>
      <span class="button-large">
        {{ $msTranslate({ key: 'HeaderPage.invitations.smallDisplayTitle', data: { count: invitations.length } }) }}
      </span>
    </ion-text>
    <ion-icon
      v-if="!isGradientButton"
      :icon="mail"
    />
    <ion-icon
      v-else
      :icon="mailUnread"
      class="gradient-button-icon"
    />
  </ion-button>
</template>

<script setup lang="ts">
import { Answer, MsModalResult, askQuestion, useWindowSize } from 'megashark-lib';
import { InvitationAction } from '@/components/users';
import InvitationsListPopover from '@/components/users/InvitationsListPopover.vue';
import { ClientCancelInvitationErrorTag, UserInvitation, cancelInvitation, listUserInvitations } from '@/parsec';
import { Routes, navigateTo } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import { IonButton, IonIcon, IonText, modalController, popoverController } from '@ionic/vue';
import { mail, mailUnread } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
let eventCbId: string | null = null;
const invitations: Ref<UserInvitation[]> = ref([]);
const { isLargeDisplay } = useWindowSize();

defineProps<{
  isGradientButton?: boolean;
}>();

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
  const result = await listUserInvitations({ skipOthers: false });
  if (result.ok) {
    invitations.value = result.value;
  }
}

async function openInvitationsPopover(event: Event): Promise<void> {
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
        informationManager: informationManager,
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
      handle: false,
      backdropDismiss: true,
      breakpoints: isLargeDisplay.value ? undefined : [1],
      // https://ionicframework.com/docs/api/modal#scrolling-content-at-all-breakpoints
      // expandToScroll: false, should be added to scroll with Ionic 8
      initialBreakpoint: isLargeDisplay.value ? undefined : 1,
      componentProps: {
        informationManager: informationManager,
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

  ion-icon {
    font-size: 1.375rem;
    margin-left: 0.5rem;

    @include ms.responsive-breakpoint('lg') {
      margin-left: 0;
    }
  }

  .button-text {
    @include ms.responsive-breakpoint('lg') {
      display: none;
    }
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
      border: 2px solid var(--parsec-color-light-primary-50);
      border-radius: var(--parsec-radius-12);
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
