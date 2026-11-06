<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="getShamirRecoveryModalTexts.title"
    :subtitle="getShamirRecoveryModalTexts.subtitle"
    :close-button="{ visible: true }"
  >
    <div class="shamir-container">
      <template v-if="tab === ShamirTab.Self">
        <shamir-self
          :information-manager="props.informationManager"
          @close-modal="close()"
        />
      </template>
      <template v-if="tab === ShamirTab.Others">
        <shamir-others
          @copy-link="onCopyLinkClicked"
          @start-recovery="onStartRecovery"
          :loading="loading"
        />
      </template>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { copyToClipboard } from '@/common/clipboard';
import ShamirOthers from '@/components/shamir/ShamirOthers.vue';
import ShamirSelf from '@/components/shamir/ShamirSelf.vue';
import { ShamirAction, ShamirTab } from '@/components/shamir/types';
import {
  AccessToken,
  InvitationEmailSentStatus,
  InvitationStatus,
  InviteListItemShamirRecovery,
  listShamirInvitations,
  OtherShamirRecoveryInfo,
  ParsecInvitationRedirectionURL,
  shamirRecoveryInvite,
  startShamirGreet,
  UserID,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { modalController } from '@ionic/vue';
import { MsModal, MsModalResult, Translatable } from 'megashark-lib';
import { computed, ref } from 'vue';

const props = defineProps<{
  informationManager: InformationManager;
  tab: ShamirTab;
}>();

interface ShamirRecoveryModalTexts {
  title: Translatable;
  subtitle: Translatable;
}

const loading = ref(false);

const getShamirRecoveryModalTexts = computed<ShamirRecoveryModalTexts>(() => {
  switch (props.tab) {
    case ShamirTab.Self:
      return {
        title: { key: 'OrganizationRecovery.shamir.modalSelf.title' },
        subtitle: { key: 'OrganizationRecovery.shamir.modalSelf.subtitle' },
      };
    case ShamirTab.Others:
      return {
        title: { key: 'OrganizationRecovery.shamir.modalOthers.title' },
        subtitle: { key: 'OrganizationRecovery.shamir.modalOthers.subtitle' },
      };
    default:
      return {
        title: { key: '' },
        subtitle: { key: '' },
      };
  }
});

async function getExistingInvitation(userId: UserID): Promise<InviteListItemShamirRecovery | undefined> {
  const invitationsResult = await listShamirInvitations();
  if (invitationsResult.ok) {
    const existingInvitation = invitationsResult.value.find(
      (invitation) => invitation.claimerUserId === userId && invitation.status === InvitationStatus.Pending,
    );
    return existingInvitation;
  }
  return undefined;
}

async function onCopyLinkClicked(info: OtherShamirRecoveryInfo): Promise<void> {
  try {
    loading.value = true;
    let link: ParsecInvitationRedirectionURL | undefined = undefined;
    const invitation = await getExistingInvitation(info.owner.id);
    if (invitation) {
      link = invitation.addr[1];
    } else {
      const invitationResult = await shamirRecoveryInvite(info.owner.id, true);
      if (invitationResult.ok) {
        link = invitationResult.value.addr[1];
      }
    }
    if (link) {
      await copyToClipboard(
        link,
        props.informationManager,
        'OrganizationRecovery.shamir.modalOthers.toasts.linkCopied',
        'OrganizationRecovery.shamir.modalOthers.toasts.linkNotCopied',
      );
    } else {
      props.informationManager.present(
        new Information({
          message: {
            key: 'OrganizationRecovery.shamir.errors.failedToGetLink',
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } finally {
    loading.value = false;
  }
}

async function onStartRecovery(info: OtherShamirRecoveryInfo): Promise<void> {
  async function startError(): Promise<void> {
    props.informationManager.present(
      new Information({
        message: {
          key: 'OrganizationRecovery.shamir.greet.errors.startFailed',
        },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }

  try {
    loading.value = true;
    let token: AccessToken | undefined = undefined;

    const invitation = await getExistingInvitation(info.owner.id);
    if (invitation) {
      token = invitation.token;
    }
    if (!token) {
      const inviteResult = await shamirRecoveryInvite(info.owner.id, true);
      if (!inviteResult.ok) {
        await startError();
        return;
      }
      if (inviteResult.value.emailSentStatus !== InvitationEmailSentStatus.Success) {
        props.informationManager.present(
          new Information({
            message: {
              key: 'OrganizationRecovery.shamir.greet.errors.emailNotSent',
            },
            level: InformationLevel.Warning,
          }),
          PresentationMode.Toast,
        );
        window.nativeAPI.log(
          'warn',
          `Created a new shamir invitation but email was not sent (reason: ${inviteResult.value.emailSentStatus})`,
        );
      }
      token = inviteResult.value.token;
    }
    const startResult = await startShamirGreet(token);
    if (!startResult.ok) {
      await startError();
      return;
    }
    await modalController.dismiss(
      { action: ShamirAction.Greet, handle: startResult.value.handle, claimerName: info.owner.humanHandle },
      MsModalResult.Confirm,
    );
  } finally {
    loading.value = false;
  }
}

async function close(): Promise<void> {
  await modalController.dismiss();
}
</script>

<style scoped lang="scss">
.shamir-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
}
</style>
