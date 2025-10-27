<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="create-join-modal">
    <create-organization-modal-header
      @close-clicked="onCloseRequested"
      title="HomePage.noExistingOrganization.createOrganization"
    />
    <div class="create-join-modal-content">
      <ion-text class="create-join-modal-content__description">
        {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationDescription') }}
      </ion-text>
      <ion-list class="create-join-modal-list">
        <ion-item
          class="create-join-modal-list__item"
          @click="clicked(HomePageAction.CreateOrganization)"
        >
          <ion-icon
            :icon="addCircle"
            slot="start"
            class="item-icon"
          />
          <div class="item-content">
            <ion-label class="item-content__title subtitles-normal">
              {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationTitle') }}
            </ion-label>
            <ion-text class="item-content__description body">
              {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationSubtitle') }}
            </ion-text>
          </div>
        </ion-item>
        <ion-item
          class="create-join-modal-list__item"
          @click="clicked(HomePageAction.JoinOrganization)"
        >
          <ion-icon
            :icon="mailUnread"
            slot="start"
            class="item-icon"
          />
          <div class="item-content">
            <ion-label class="item-content__title subtitles-normal">
              {{ $msTranslate('HomePage.noExistingOrganization.joinOrganizationTitle') }}
            </ion-label>
            <ion-text class="item-content__description body">
              {{ $msTranslate('HomePage.noExistingOrganization.joinOrganizationSubtitle') }}
            </ion-text>
          </div>
        </ion-item>
      </ion-list>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { HomePageAction } from '@/views/home/HomePageButtons.vue';
import { IonIcon, IonItem, IonLabel, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { addCircle, mailUnread } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';

async function clicked(action: HomePageAction): Promise<void> {
  await modalController.dismiss({ action: action }, MsModalResult.Confirm);
}

async function onCloseRequested(): Promise<void> {
  await modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style lang="scss" scoped>
#create-join-modal {
  display: flex;
  flex-direction: column;
  overflow: auto;
}

.create-join-modal-content {
  padding: 0 1.5rem 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &__description {
    color: var(--parsec-color-light-secondary-soft-text);
    margin-bottom: 1rem;
  }
}

.create-join-modal-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  padding: 0;

  &__item {
    --background-hover: var(--parsec-color-light-primary-50);
    --background-hover-opacity: 1;
    cursor: pointer;
    transition: all 0.2s ease-in-out;

    &::part(native) {
      border-radius: var(--parsec-radius-12);
      background: var(--parsec-color-light-secondary-background);
      border: 1px solid var(--parsec-color-light-secondary-premiere);
      padding: 1.5rem;
      gap: 1rem;
    }

    &:hover {
      scale: 0.99;
    }

    .item-icon {
      color: var(--parsec-color-light-primary-600);
      font-size: 2rem;
    }

    .item-content {
      display: flex;
      flex-direction: column;
      justify-content: center;
      gap: 0.25rem;

      &__title {
        color: var(--parsec-color-light-primary-600);
      }

      &__description {
        color: var(--parsec-color-light-secondary-soft-text);
      }
    }
  }
}
</style>
