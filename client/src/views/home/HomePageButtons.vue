<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item
      class="container__item"
      @click="clicked(HomePageAction.CreateOrganization)"
    >
      <ion-icon
        :icon="addCircle"
        slot="start"
      />
      <div class="text-content">
        <ion-label class="body">
          {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationTitle') }}
        </ion-label>
        <ion-text class="body-sm sub-text">
          {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationSubtitle') }}
        </ion-text>
      </div>
    </ion-item>
    <ion-item
      class="container__item"
      @click="clicked(HomePageAction.JoinOrganization)"
    >
      <ion-icon
        :icon="mail"
        slot="start"
      />
      <div class="text-content">
        <ion-label class="body">
          {{ $msTranslate('HomePage.noExistingOrganization.joinOrganizationTitle') }}
        </ion-label>
        <ion-text class="body-sm sub-text">
          {{ $msTranslate('HomePage.noExistingOrganization.joinOrganizationSubtitle') }}
        </ion-text>
      </div>
    </ion-item>
  </ion-list>
</template>

<script lang="ts">
export enum HomePageAction {
  CreateOrganization = 'create',
  JoinOrganization = 'join',
}
</script>

<script setup lang="ts">
import { IonIcon, IonItem, IonLabel, IonList, IonText, popoverController } from '@ionic/vue';
import { addCircle, mail } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';

async function clicked(action: HomePageAction): Promise<void> {
  await popoverController.dismiss({ action: action }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.container {
  padding: 0.5rem;
}

.container__item {
  --background: none;
  width: 100%;
  --min-height: 1rem;
  user-select: none;
  cursor: pointer;
  margin-inline-end: 2px;
  color: var(--parsec-color-light-primary-600);
  border-radius: 0.25rem;
  ion-icon {
    color: var(--parsec-color-light-primary-600);
    margin-inline-end: 0.75rem;
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
  &:hover {
    --background: var(--parsec-color-light-primary-30);
    color: var(--parsec-color-light-primary-600);
  }
  .text-content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    .sub-text {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}
</style>
