<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <!-- eslint-disable vue/html-indent -->
    <div
      v-if="
        shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupAllValid ||
        shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients
      "
    >
      <!-- eslint-enable vue/html-indent -->
      SHAMIR IS SETUP

      <ion-list v-if="showPeople">
        <ion-item
          v-for="recipient in shamirInfo.recipients"
          :key="recipient.id"
        >
          {{ recipient.humanHandle.label }} {{ recipient.isRevoked() }}
        </ion-item>
      </ion-list>

      <span v-show="shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients">WARNING</span>

      <ion-button
        v-if="!showPeople"
        @click="showPeople = true"
      >
        SHOW PEOPLE
      </ion-button>
      <ion-button @click="deleteShamir"> DELETE </ion-button>
    </div>
    <div v-if="shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupButUnusable">PROBLEM WITH THE SHAMIR</div>
  </div>
</template>

<script setup lang="ts">
import { SelfShamirRecoveryInfo } from '@/parsec';
import { deleteSelfShamirRecovery } from '@/parsec/shamir';
import { SelfShamirRecoveryInfoTag } from '@/plugins/libparsec';
import { IonButton, IonItem, IonList } from '@ionic/vue';
import { Answer, askQuestion } from 'megashark-lib';
import { ref } from 'vue';

defineProps<{
  shamirInfo: SelfShamirRecoveryInfo;
}>();

const emits = defineEmits<{
  (e: 'shamirDeleted'): void;
}>();

const showPeople = ref(false);

async function deleteShamir(): Promise<void> {
  const answer = await askQuestion('DELETE SHAMIR', 'THIS WILL DELETE THE SHAMIR, ARE YOU SURE', {
    noText: 'CANCEL',
    yesText: 'DELETE SHAMIR',
  });
  if (answer === Answer.No) {
    return;
  }
  const result = await deleteSelfShamirRecovery();
  if (result.ok) {
    emits('shamirDeleted');
  }
}
</script>

<style scoped lang="scss"></style>
