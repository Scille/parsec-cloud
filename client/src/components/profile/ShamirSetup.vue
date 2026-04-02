<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <div v-if="step === ShamirSetupStep.Initial">
      <ion-button
        @click="startSetup"
        :disabled="allUsers.length < shamirThreshold"
      >
        START SETUP
      </ion-button>
      {{ `${shamirThreshold} PERSONS REQUIRED` }}
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
        :disabled="selectedUsers.length < shamirThreshold"
      >
        SETUP
      </ion-button>
      <ms-report-text
        :theme="MsReportTheme.Error"
        v-if="error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getClientInfo, listUsers, UserInfo } from '@/parsec';
import { getRequiredShamirThreshold, setupShamirRecovery } from '@/parsec/shamir';
import { IonButton, IonIcon, IonItem, IonList } from '@ionic/vue';
import { personAdd, personRemove } from 'ionicons/icons';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

enum ShamirSetupStep {
  Initial = 'shamir-setup-step-initial',
  AddPeople = 'shamir-setup-step-add-people',
}

const step = ref<ShamirSetupStep>(ShamirSetupStep.Initial);
const allUsers = ref<Array<UserInfo>>([]);
const selectedUsers = ref<Array<UserInfo>>([]);
const error = ref('');
const shamirThreshold = ref(3);

const availableUsers = computed(() => {
  return allUsers.value.filter((user) => selectedUsers.value.find((selectedUser) => selectedUser.id === user.id) === undefined);
});

const emits = defineEmits<{
  (e: 'shamirSetup'): void;
}>();

onMounted(async () => {
  const [usersResult, clientResult, threshold] = await Promise.all([listUsers(true), getClientInfo(), getRequiredShamirThreshold()]);
  if (usersResult.ok && clientResult.ok) {
    allUsers.value = usersResult.value.filter((user) => user.id !== clientResult.value.userId);
    shamirThreshold.value = threshold;
  }
});

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
  if (selectedUsers.value.length < shamirThreshold.value) {
    return;
  }
  const result = await setupShamirRecovery(selectedUsers.value, shamirThreshold.value, 1);
  if (result.ok) {
    emits('shamirSetup');
  } else {
    error.value = 'FAILED TO SETUP SHAMIR';
  }
}

async function startSetup(): Promise<void> {
  step.value = ShamirSetupStep.AddPeople;
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
