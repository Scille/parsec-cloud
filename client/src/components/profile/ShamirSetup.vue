<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <div v-if="step === ShamirSetupStep.Initial">
      <ion-button @click="startSetup"> START SETUP </ion-button>
      {{ `${SHAMIR_THRESHOLD} persones minimum` }}
    </div>
    <div v-if="step === ShamirSetupStep.AddPeople && allUsers.length">
      <ion-list>
        <ion-item
          v-for="user in selectedUsers"
          :key="user.id"
        >
          {{ user.humanHandle.label }} {{ user.humanHandle.email }}
          <ion-icon
            :icon="personRemove"
            @click="removeUser(user)"
          />
        </ion-item>
      </ion-list>
      <ion-list>
        <ion-item
          v-for="user in availableUsers"
          :key="user.id"
        >
          {{ user.humanHandle.label }} {{ user.humanHandle.email }}
          <ion-icon
            :icon="personAdd"
            @click="addUser(user)"
          />
        </ion-item>
      </ion-list>
      <ion-button
        @click="setupShamir"
        :disabled="selectedUsers.length < SHAMIR_THRESHOLD"
      >
        SETUP
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getClientInfo, listUsers, UserInfo } from '@/parsec';
import { setupShamirRecovery } from '@/parsec/shamir';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonItem, IonList } from '@ionic/vue';
import { personAdd, personRemove } from 'ionicons/icons';
import { computed, inject, Ref, ref } from 'vue';

enum ShamirSetupStep {
  Initial = 'shamir-setup-step-initial',
  AddPeople = 'shamir-setup-step-add-people',
}

const SHAMIR_THRESHOLD = 2;
const SHAMIR_PARTS_PER_RECIPIENT = 1;

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;

const step = ref<ShamirSetupStep>(ShamirSetupStep.Initial);
const allUsers = ref<Array<UserInfo>>([]);
const selectedUsers = ref<Array<UserInfo>>([]);

const availableUsers = computed(() => {
  return allUsers.value.filter((user) => selectedUsers.value.find((selectedUser) => selectedUser.id === user.id) === undefined);
});

const emits = defineEmits<{
  (e: 'shamirSetup'): void;
}>();

async function addUser(user: UserInfo): Promise<void> {
  if (selectedUsers.value.find((selectedUser) => selectedUser.id === user.id)) {
    return;
  }
  selectedUsers.value.push(user);
}

async function removeUser(user: UserInfo): Promise<void> {
  const index = selectedUsers.value.findIndex((selectedUser) => selectedUser.id === user.id);
  if (index !== -1) {
    selectedUsers.value.splice(index, 1);
  }
}

async function setupShamir(): Promise<void> {
  if (selectedUsers.value.length < SHAMIR_THRESHOLD) {
    return;
  }
  const result = await setupShamirRecovery(selectedUsers.value, SHAMIR_THRESHOLD, SHAMIR_PARTS_PER_RECIPIENT);
  if (result.ok) {
    informationManager.value.present(
      new Information({
        message: 'SHAMIR CREATED',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    emits('shamirSetup');
  } else {
    informationManager.value.present(
      new Information({
        message: 'FAILED TO CREATE SHAMIR',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function startSetup(): Promise<void> {
  Promise.all([listUsers(true), getClientInfo()]).then(([usersResult, clientResult]) => {
    if (usersResult.ok && clientResult.ok) {
      allUsers.value = usersResult.value.filter((user) => user.id !== clientResult.value.userId);
      step.value = ShamirSetupStep.AddPeople;
    }
  });
}
</script>

<style scoped lang="scss">
.remove {
  background-color: orange;
  border-radius: 2px;
  border: 1px solid red;
}

.add {
  background-color: greenyellow;
  border-radius: 2px;
  border: 1px solid green;
}
</style>
