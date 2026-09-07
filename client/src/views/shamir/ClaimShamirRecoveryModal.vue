<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="ACCOUNT RECOVERY"
    :close-modal="{ visible: true }"
  >
    <div v-if="claimer && !exchangeStartedWith">
      <shamir-code-exchange-guest
        :claimer="claimer"
        @code-exchanged="onCodeExchanged"
        @interrupted="onCodeExchangeInterrupted"
      />
    </div>
    <div
      v-else-if="!finishedInfo"
      class="recipient-list"
    >
      <div class="parts">
        {{ `PARTS: ${shares}/${info.threshold}` }}
      </div>
      <div>
        <ion-list>
          <ion-item
            class="recipient"
            v-for="recipient in invitationInfo.recipients"
            :key="recipient.userId"
          >
            <div class="recipient-info">{{ recipient.humanHandle.label }}<br />{{ recipient.humanHandle.email }}</div>
            <div v-if="recipientHasContributed(recipient)">
              <ion-icon
                class="check-icon"
                :icon="checkmark"
              />
            </div>
            <div v-else>
              <ion-button
                class="button-start"
                v-if="exchangeStartedWith !== recipient.userId"
                :disabled="exchangeStartedWith !== undefined"
                @click="chooseRecipient(recipient)"
              >
                CHOOSE
              </ion-button>
              <div v-if="exchangeStartedWith === recipient.userId">
                <ms-spinner /> WAITING FOR THEM TO START THE PROCESS
                <ion-button @click="cancelWait">CANCEL</ion-button>
              </div>
            </div>
          </ion-item>
        </ion-list>
      </div>
      <div>SOME OTHER TEXT</div>
    </div>
    <div v-if="finishedInfo">
      ALL PARTS ACQUIRED

      <ion-button
        class="finish-button"
        @click="finish"
      >
        CONTINUE
      </ion-button>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import ShamirCodeExchangeGuest, { CodeExchangeInterruptedReason } from '@/components/shamir/ShamirCodeExchangeGuest.vue';
import {
  AnyClaimRetrievedInfoShamirRecovery,
  ClaimInProgressErrorTag,
  ShamirClaimer,
  shamirPickRecipient,
  ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient,
  ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice,
  ShamirRecoveryClaimMaybeRecoverDeviceInfoTag,
  ShamirRecoveryRecipient,
  UserID,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { modalController } from '@ionic/core';
import { IonButton, IonIcon, IonItem, IonList } from '@ionic/vue';
import { checkmark } from 'ionicons/icons';
import { MsModal, MsModalResult, MsSpinner, Translatable } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  invitationInfo: AnyClaimRetrievedInfoShamirRecovery;
  informationManager: InformationManager;
}>();

const info = ref<AnyClaimRetrievedInfoShamirRecovery | ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient>(props.invitationInfo);
const finishedInfo = ref<ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice | undefined>(undefined);
const exchangeStartedWith = ref<UserID | undefined>(undefined);
const claimer = ref<ShamirClaimer | undefined>(undefined);
const shares = computed(() => {
  if (info.value.tag === ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient) {
    return [...info.value.recoveredShares.values()].reduce((sum, value) => sum + value, 0);
  }
  return 0;
});

onMounted(async () => {
  console.log(info.value);
});

function recipientHasContributed(recipient: ShamirRecoveryRecipient): boolean {
  if (info.value.tag === ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient) {
    return info.value.recoveredShares.has(recipient.userId);
  }
  return false;
}

async function cancelWait(): Promise<void> {
  if (claimer.value) {
    claimer.value.cancel();
  }
  claimer.value = undefined;
  exchangeStartedWith.value = undefined;
}

async function chooseRecipient(recipient: ShamirRecoveryRecipient): Promise<void> {
  try {
    exchangeStartedWith.value = recipient.userId;
    const result = await shamirPickRecipient(info.value.handle, recipient.userId);
    if (!result.ok) {
      props.informationManager.present(
        new Information({
          message: 'PICK RECIPIENT FAILED',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }
    claimer.value = new ShamirClaimer(result.value);
    const waitResult = await claimer.value.initialWaitGreeter();
    console.log(waitResult);
    if (!waitResult.ok && waitResult.error.tag !== ClaimInProgressErrorTag.Cancelled) {
      props.informationManager.present(
        new Information({
          message: 'WAIT FAILED',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } finally {
    exchangeStartedWith.value = undefined;
  }
}

async function onCodeExchangeInterrupted(reason: CodeExchangeInterruptedReason): Promise<void> {
  let message!: Translatable;
  switch (reason) {
    default: {
      message = 'FAILED';
      break;
    }
  }
  props.informationManager.present(
    new Information({
      message: message,
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  claimer.value = undefined;
}

async function onCodeExchanged(): Promise<void> {
  if (!claimer.value) {
    window.nativeAPI.log('error', 'Claimer is undefined after code exchange, should not happen');
    return;
  }
  try {
    const claimResult = await claimer.value.claim();
    if (!claimResult.ok) {
      props.informationManager.present(
        new Information({
          message: 'CLAIM FAILED',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }
    const shareResult = await claimer.value.addShare(info.value.handle);
    if (!shareResult.ok) {
      props.informationManager.present(
        new Information({
          message: 'ADD SHARE FAILED',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }
    console.log(shareResult);
    if (shareResult.value.tag === ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient) {
      info.value = shareResult.value;
    } else {
      finishedInfo.value = shareResult.value;
    }
  } finally {
    claimer.value = undefined;
  }
}

async function finish(): Promise<void> {
  if (finishedInfo.value) {
    await modalController.dismiss({ info: finishedInfo.value }, MsModalResult.Confirm);
  }
}
</script>

<style scoped lang="scss"></style>
