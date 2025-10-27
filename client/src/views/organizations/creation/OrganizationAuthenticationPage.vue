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
      ref="chooseAuthentication"
      class="authentication-content"
      @field-update="onFieldUpdated"
    />

    <ion-footer class="authentication-page-footer">
      <div class="modal-footer-buttons">
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
          @click="chooseAuthenticationRef?.getSaveStrategy() && $emit('authenticationChosen', chooseAuthenticationRef.getSaveStrategy())"
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
      </div>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { DeviceSaveStrategy } from '@/parsec';
import { IonButton, IonFooter, IonIcon, IonPage } from '@ionic/vue';
import { chevronBack, chevronForward } from 'ionicons/icons';
import { asyncComputed } from 'megashark-lib';
import { useTemplateRef } from 'vue';

defineProps<{
  hideBackButton?: boolean;
  isLastStep?: boolean;
}>();

defineEmits<{
  (e: 'authenticationChosen', saveStrategy: DeviceSaveStrategy): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();

const chooseAuthenticationRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('chooseAuthentication');
const valid = asyncComputed(async () => {
  if (!chooseAuthenticationRef.value) {
    return false;
  }
  return await chooseAuthenticationRef.value.areFieldsCorrect();
});

// TODO: Since valid is supposed to be read-only, we should check if this is really necessary
async function onFieldUpdated(): Promise<void> {
  if (!chooseAuthenticationRef.value) {
    return;
  }
  valid.value = await chooseAuthenticationRef.value.areFieldsCorrect();
}
</script>

<style scoped lang="scss">
.authentication-content {
  @include ms.responsive-breakpoint('sm') {
    padding: 0 1rem;
  }
}
</style>
