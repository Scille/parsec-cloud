<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="user-information-page page-modal-container">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.personalDetails"
    />

    <user-information
      ref="userInformationRef"
      :default-email="email"
      :default-name="name"
      :email-enabled="email === undefined"
      :name-enabled="name === undefined"
      @field-update="onFieldUpdated"
    />

    <ion-footer class="user-information-page-footer">
      <ion-buttons
        slot="primary"
        class="modal-footer-buttons"
      >
        <ion-button
          fill="clear"
          size="default"
          id="previous-button"
          @click="$emit('goBackRequested')"
          v-show="!hidePrevious"
        >
          {{ $msTranslate('CreateOrganization.button.previous') }}
          <ion-icon
            slot="start"
            :icon="chevronBack"
            size="small"
          />
        </ion-button>

        <ion-button
          fill="solid"
          size="default"
          @click="onButtonClicked"
          :disabled="!valid"
        >
          <span>
            {{ $msTranslate('CreateOrganization.button.next') }}
          </span>
          <ion-icon
            slot="start"
            :icon="chevronForward"
            size="small"
          />
        </ion-button>
      </ion-buttons>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonButton, IonButtons, IonFooter, IonIcon } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { chevronForward, chevronBack } from 'ionicons/icons';
import UserInformation from '@/components/users/UserInformation.vue';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

defineProps<{
  name?: string;
  email?: string;
  hidePrevious?: boolean;
}>();

const emits = defineEmits<{
  (e: 'userInformationFilled', name: string, email: string): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();

const userInformationRef = ref();
const valid = ref(false);

onMounted(async () => {
  await onFieldUpdated();
});

async function onButtonClicked(): Promise<void> {
  if (!userInformationRef.value) {
    console.log('Missing ref, should not happen');
    return;
  }
  emits('userInformationFilled', userInformationRef.value.fullName, userInformationRef.value.email);
}

async function onFieldUpdated(): Promise<void> {
  if (userInformationRef.value) {
    valid.value = await userInformationRef.value.areFieldsCorrect();
  } else {
    valid.value = false;
  }
}
</script>

<style scoped lang="scss"></style>
