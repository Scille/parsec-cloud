<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card-content class="topbar">
    <div class="topbar-content">
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
    </div>
  </ion-card-content>
</template>

<script setup lang="ts">
import { MsModalResult } from '@/components/core';
import HomePagePopover, { HomePageAction } from '@/views/home/HomePagePopover.vue';
import { IonButton, IonButtons, IonCardContent, IonCardTitle, IonIcon, popoverController } from '@ionic/vue';
import { chevronBack, cog } from 'ionicons/icons';
import { ref } from 'vue';

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
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  padding: 0;

  .topbar-content {
    padding: 3.5rem 3.5rem 1.5rem;
    max-width: var(--parsec-max-content-width);
    display: flex;
    align-items: center;
  }

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
