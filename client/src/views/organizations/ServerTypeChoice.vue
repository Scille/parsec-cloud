<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CHOOSE WISELY"
    />
    <div>
      <div
        class="server-type-container"
        @click="serverChoice = ServerType.Saas"
        :class="{ selected: serverChoice === ServerType.Saas }"
      >
        SAAS
      </div>
      <div
        class="server-type-container"
        @click="serverChoice = ServerType.Trial"
        :class="{ selected: serverChoice === ServerType.Trial }"
      >
        TRIAL
      </div>
    </div>

    <ion-button @click="$emit('serverChosen', ServerType.Custom)"> CUSTOM SERVER </ion-button>
    <ion-button
      @click="onChoiceMade"
      :disabled="serverChoice === undefined"
    >
      CONTINUE
    </ion-button>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonButton } from '@ionic/vue';
import { ref } from 'vue';
import { ServerType } from '@/services/parsecServers';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

const emits = defineEmits<{
  (e: 'serverChosen', serverType: ServerType): void;
  (e: 'closeRequested'): void;
}>();

const serverChoice = ref<ServerType | undefined>(undefined);

async function onChoiceMade(): Promise<void> {
  if (serverChoice.value === undefined) {
    return;
  }
  emits('serverChosen', serverChoice.value);
}
</script>

<style scoped lang="scss">
.server-type-container {
  width: 12em;
  height: 12em;
  border: 1px solid red;
}
.selected {
  background-color: pink;
}
</style>
