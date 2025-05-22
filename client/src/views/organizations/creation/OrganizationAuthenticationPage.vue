<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="authentication-page page-modal-container">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.authentication"
      subtitle="CreateOrganization.subtitle.authentication"
      :small-display-stepper="true"
    />

    <choose-authentication
      ref="chooseAuthenticationRef"
      @field-update="onFieldUpdated"
      class="authentication-content"
    />

    <ion-footer class="authentication-page-footer">
      <ion-buttons
        slot="primary"
        class="modal-footer-buttons"
      >
        <ion-button
          v-show="!hideBackButton"
          fill="clear"
          size="default"
          id="previous-button"
          @click="$emit('goBackRequested')"
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
          @click="$emit('authenticationChosen', chooseAuthenticationRef.getSaveStrategy())"
          :disabled="!valid"
        >
          <span>
            {{ isLastStep ? $msTranslate('CreateOrganization.button.create') : $msTranslate('CreateOrganization.button.next') }}
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
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import { DeviceSaveStrategy } from '@/parsec';
import { chevronForward, chevronBack } from 'ionicons/icons';
import { IonPage, IonButton, IonButtons, IonIcon, IonFooter } from '@ionic/vue';
import { asyncComputed } from 'megashark-lib';
import { ref } from 'vue';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

defineProps<{
  hideBackButton?: boolean;
  isLastStep?: boolean;
}>();

defineEmits<{
  (e: 'authenticationChosen', saveStrategy: DeviceSaveStrategy): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();

const chooseAuthenticationRef = ref();
const valid = asyncComputed(async () => {
  if (!chooseAuthenticationRef.value) {
    return false;
  }
  return await chooseAuthenticationRef.value.areFieldsCorrect();
});

async function onFieldUpdated(): Promise<void> {
  console.log('FIELD UPDATED');
  valid.value = await chooseAuthenticationRef.value.areFieldsCorrect();
  console.log(valid.value);
}
</script>

<style scoped lang="scss">
.authentication-content {
  @include ms.responsive-breakpoint('sm') {
    padding: 0 1rem;
  }
}
</style>
