<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card-content class="topbar">
    <ion-card-title
      color="tertiary"
      v-if="showBackButton"
    >
      <ion-button
        fill="clear"
        @click="$emit('backClick')"
        id="back-to-list-button"
      >
        <ion-icon
          slot="start"
          :icon="chevronBack"
        />
        {{ $t('HomePage.organizationLogin.backToList') }}
      </ion-button>
    </ion-card-title>
    <ion-button
      @click="togglePopover"
      size="large"
      id="create-organization-button"
      class="button-default"
    >
      {{ $t('HomePage.noExistingOrganization.createOrJoin') }}
    </ion-button>
    <ion-buttons
      slot="primary"
      class="topbar-icon__settings"
    >
      <ion-button
        slot="icon-only"
        id="trigger-settings-button"
        class="topbar-button__item"
        @click="$emit('settingsClick')"
      >
        <ion-icon
          slot="icon-only"
          :icon="cog"
        />
      </ion-button>
    </ion-buttons>
  </ion-card-content>
</template>

<script setup lang="ts">
import { popoverController, IonButtons, IonButton, IonIcon, IonCardTitle, IonCardContent } from '@ionic/vue';
import { cog, chevronBack } from 'ionicons/icons';
import { ref } from 'vue';
import HomePagePopover, { HomePageAction } from '@/views/home/HomePagePopover.vue';
import { MsModalResult } from '@/components/core';

defineProps<{
  showBackButton: boolean;
}>();

const emits = defineEmits<{
  (e: 'createOrganizationClick'): void;
  (e: 'joinOrganizationClick'): void;
  (e: 'settingsClick'): void;
  (e: 'backClick'): void;
}>();

const isPopoverOpen = ref(false);

async function togglePopover(event: Event): Promise<void> {
  isPopoverOpen.value = !isPopoverOpen.value;
  openPopover(event);
}

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: HomePagePopover,
    cssClass: 'homepage-popover',
    event: event,
    showBackdrop: false,
    alignment: 'end',
  });
  await popover.present();
  const result = await popover.onWillDismiss();
  await popover.dismiss();
  if (result.role !== MsModalResult.Confirm) {
    return;
  }
  if (result.data.action === HomePageAction.CreateOrganization) {
    emits('createOrganizationClick');
  } else if (result.data.action === HomePageAction.JoinOrganization) {
    emits('joinOrganizationClick');
  }
}
</script>

<style lang="scss" scoped>
.topbar {


  background-color: #ffe700;
  opacity: 0.8;
  background-image: linear-gradient(30deg, #ffe700 12%, transparent 12.5%, transparent 87%, #ffe700 87.5%, #ffe700),
    linear-gradient(150deg, #ffe700 12%, transparent 12.5%, transparent 87%, #ffe700 87.5%, #ffe700),
    linear-gradient(30deg, #ffe700 12%, transparent 12.5%, transparent 87%, #ffe700 87.5%, #ffe700),
    linear-gradient(150deg, #ffe700 12%, transparent 12.5%, transparent 87%, #ffe700 87.5%, #ffe700),
    linear-gradient(60deg, #3e4616 25%, transparent 25.5%, transparent 75%, #3e4616 75%, #3e4616),
    linear-gradient(60deg, #3e4616 25%, transparent 25.5%, transparent 75%, #3e4616 75%, #3e4616);
  background-size: 20px 35px;
  background-position:
    0 0,
    0 0,
    10px 18px,
    10px 18px,
    0 0,
    10px 18px;




  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  padding: 3.5rem 3.5rem 1.5rem;
  display: flex;
  align-items: center;

  #create-organization-button {
    margin-left: auto;
    margin-right: 1.5rem;
  }

  .topbar-button__item,
  .sc-ion-buttons-md-s .button {
    border: 1px solid var(--parsec-color-light-secondary-light);
    color: var(--parsec-color-light-primary-700);
    border-radius: 50%;
    --padding-top: 0;
    --padding-end: 0;
    --padding-bottom: 0;
    --padding-start: 0;
    width: 3em;
    height: 3em;

    &:hover {
      --background-hover: var(--parsec-color-light-primary-50);
      background: var(--parsec-color-light-primary-50);
      border: var(--parsec-color-light-primary-50);
    }

    ion-icon {
      font-size: 1.375rem;
    }
  }
}
</style>
