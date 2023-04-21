<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item id="click-trigger" class="container">
    <!-- dropdown profile -->
    <ion-popover
      trigger="click-trigger"
      trigger-action="click"
      class="popover-profile"
    >
      <ion-content class="ion-padding">
        <ion-list>
          <ion-item class="profile-email">
            <ion-text class="body-sm">thomas.linere@mycompany.fr</ion-text>
          </ion-item>
          <!-- item -->
          <ion-item class="item">
            <ion-icon
              :icon="phonePortrait"
              slot="start"
            />
            <ion-label class="body">Mes appareils</ion-label>
          </ion-item>
          <!-- end of item -->
        </ion-list>
      </ion-content>
    </ion-popover>
    <!-- end of dropdown profile-->
    <ion-avatar slot="start" class="avatar">
      <img
        alt="Silhouette of a person's head"
        src="https://ionicframework.com/docs/img/demos/avatar.svg"
      >
    </ion-avatar>
    <div class=" text-icon">
      <ion-text class="body">
        {{ lastname }} {{ firstname }}
      </ion-text>
      <ion-icon
        slot="end"
        :icon="chevronDown"
      />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import {
  IonItem,
  IonIcon,
  IonLabel,
  IonAvatar,
  IonList,
  IonPopover,
  IonContent,
  IonText
}from '@ionic/vue';
import {
  chevronDown, phonePortrait
} from 'ionicons/icons';
import { defineProps } from 'vue';
import { useI18n } from 'vue-i18n';

const { t, d } = useI18n();

defineProps<{
  firstname: string,
  lastname: string
}>();
function onClickMenu() : void {
  console.log('click');
}

</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-direction: column;
  --background: none;
  cursor: pointer;

}

.avatar {
  margin: 0 .75em 0 0;
  position: relative;
  &::after {
    content:'';
    position: absolute;
    bottom: 0;
    right: -4px;
    height: 0.625rem;
    width: 0.625rem;
    border-radius: 50%;
    border: var(--parsec-color-light-secondary-background) solid 0.125rem;
    background-color: var(---parsec-color--success-500);
  }
}
.text-icon {
  display: flex;
  align-items: center;
  gap: .1em;
}

.text-content {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.popover-profile {
  --backdrop-opacity: 0;

  ion-item {
    --background: none;
  }

  .profile-email {
    color: var(--parsec-color-light-secondary-grey);
  }

  .item:not(:first-child)  {
    margin-inline-end: 2px;
    color: var(--parsec-color-light-secondary-text);
    border-radius: 0.25rem;

    &:hover {
      --background: var(--parsec-color-light-primary-30);
      color: var(--parsec-color-light-primary-600);

      ion-icon {
        color: var(--parsec-color-light-primary-600);
      }
    }

    ion-icon {
      color: var(--parsec-color-light-secondary-text);
      margin-inline-end: 0.75rem;
      margin-top: 1rem;
      margin-bottom: 1rem;
    }
  }
}
</style>
