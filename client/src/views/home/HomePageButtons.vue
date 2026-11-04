<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list class="list-homepage-buttons">
    <ion-item
      class="list-homepage-buttons__item"
      @click="clicked(HomePageAction.CreateOrganization)"
    >
      <ion-icon
        :icon="addCircle"
        slot="start"
        class="item-icon"
      />
      <div class="text-content">
        <ion-text class="item-title">
          {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationTitle') }}
        </ion-text>
        <ion-text class="item-description">
          {{ $msTranslate('HomePage.noExistingOrganization.createOrganizationSubtitle') }}
        </ion-text>
      </div>
    </ion-item>
    <ion-item
      class="list-homepage-buttons__item"
      @click="clicked(HomePageAction.JoinOrganization)"
    >
      <ion-icon
        :icon="mail"
        slot="start"
        class="item-icon"
      />
      <div class="text-content">
        <ion-text class="item-title">
          {{ $msTranslate('HomePage.noExistingOrganization.joinOrganizationTitle') }}
        </ion-text>
        <ion-text class="item-description">
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
import { IonIcon, IonItem, IonList, IonText, popoverController } from '@ionic/vue';
import { addCircle, mail } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';

async function clicked(action: HomePageAction): Promise<void> {
  await popoverController.dismiss({ action: action }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.list-homepage-buttons {
  padding: ms.spacing('padding-lg');
  display: flex;
  flex-direction: column;
}

.list-homepage-buttons__item {
  --background: none;
  width: 100%;
  --min-height: 1rem;
  user-select: none;
  cursor: pointer;
  color: ms.color('text-brand-default');

  &::part(native) {
    padding: 0.325rem 0.5rem;
  }

  .item-icon {
    color: ms.color('icon-brand-default');
    margin-inline-end: ms.spacing('gap-2xl');
    font-size: 1.25rem;
  }

  &:hover {
    --background: #{ms.color('surface-brand-default-subtle-hover')};
    color: ms.color('text-brand-default-hover');
    border-radius: ms.radius('sm');
  }

  .text-content {
    display: flex;
    flex-direction: column;
    justify-content: center;

    .item-title {
      @include ms.font('label-md-medium');
    }

    .item-description {
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-description');
    }
  }
}
</style>
