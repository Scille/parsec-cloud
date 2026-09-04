<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
            <ion-label class="item-content__title">
              {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationTitle') }}
            </ion-label>
            <ion-text class="item-content__description">
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
            <ion-label class="item-content__title">
              {{ $msTranslate('HomePage.noExistingOrganization.joinOrganizationTitle') }}
            </ion-label>
            <ion-text class="item-content__description">
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
  padding: ms.spacing('padding-none') ms.spacing('padding-4xl') ms.spacing('padding-4xl');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  &__description {
    color: ms.color('text-neutral-default');
    margin-bottom: 1rem;
  }
}

.create-join-modal-list {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-none');
  margin-bottom: 2rem;

  &__item {
    --background-hover: #{ms.color('surface-brand-default-subtle-hover')};
    --background-hover-opacity: #{ms.opacity('10')};
    cursor: pointer;
    transition: all 0.2s ease-in-out;

    &::part(native) {
      border-radius: ms.radius('2xl');
      background: ms.color('surface-base-default-secondary');
      border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
      padding: ms.spacing('padding-2xl');
      gap: ms.spacing('gap-2xl');
    }

    &:hover {
      scale: 0.99;
    }

    .item-icon {
      color: ms.color('icon-brand-default');
      font-size: 1.5rem;
    }

    .item-content {
      display: flex;
      flex-direction: column;
      justify-content: center;
      gap: ms.spacing('gap-xs');

      &__title {
        @include ms.font('label-lg-medium');
        color: ms.color('text-brand-default');
      }

      &__description {
        @include ms.font('body-md-regular');
        color: ms.color('text-neutral-default');
      }
    }
  }
}
</style>
