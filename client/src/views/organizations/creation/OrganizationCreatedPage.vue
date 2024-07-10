<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="created-page">
    <ms-informative-text>
      {{ $msTranslate('CreateOrganization.organizationCreated') }}
    </ms-informative-text>

    <ion-footer class="created-page-footer">
      <ion-buttons
        slot="primary"
        class="modal-footer-buttons"
      >
        <ion-button
          fill="solid"
          size="default"
          @click="$emit('goClicked')"
        >
          <span>
            {{ $msTranslate('CreateOrganization.button.done') }}
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
import { OrganizationID } from '@/parsec';
import { MsInformativeText } from 'megashark-lib';
import { chevronForward } from 'ionicons/icons';
import { IonPage, IonButton, IonFooter, IonButtons, IonIcon } from '@ionic/vue';

defineProps<{
  organizationName: OrganizationID;
}>();

defineEmits<{
  (e: 'goClicked'): void;
}>();
</script>

<style scoped lang="scss">
.created-page {
  padding: 2.5rem;
  display: flex;
  height: auto;
  width: 100%;

  &-footer {
    display: flex;
    justify-content: space-between;
    margin-top: 2.5rem;
  }
}
</style>
