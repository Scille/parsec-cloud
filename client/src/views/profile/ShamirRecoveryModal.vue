<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
        />
      </template>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { copyToClipboard } from '@/common/clipboard';
import ShamirOthers from '@/components/profile/ShamirOthers.vue';
import ShamirSelf from '@/components/profile/ShamirSelf.vue';
import { ShamirTab } from '@/components/profile/types';
import { OtherShamirRecoveryInfo, shamirRecoveryInvite } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { modalController } from '@ionic/vue';
import { MsModal, Translatable } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  informationManager: InformationManager;
  tab: ShamirTab;
}>();

defineEmits<{
  (e: 'startRecovery', info: OtherShamirRecoveryInfo): void;
}>();

interface ShamirRecoveryModalTexts {
  title: Translatable;
  subtitle: Translatable;
}

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

async function onCopyLinkClicked(info: OtherShamirRecoveryInfo): Promise<void> {
  const invitationResult = await shamirRecoveryInvite(info.owner.id, false);
  if (invitationResult.ok) {
    await copyToClipboard(
      invitationResult.value.addr[1],
      props.informationManager,
      'OrganizationRecovery.shamir.modalOthers.toasts.linkCopied',
      'OrganizationRecovery.shamir.modalOthers.toasts.linkNotCopied',
    );
  } else {
    await props.informationManager.present(
      new Information({
        message: {
          key: 'OrganizationRecovery.shamir.errors.failedToGetLink',
        },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function onStartRecovery(_info: OtherShamirRecoveryInfo): Promise<void> {
  // TODO: implement recovery greet flow
}

async function close(): Promise<void> {
  await modalController.dismiss();
}
</script>

<style scoped lang="scss">
.shamir-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
</style>
