<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item id="click-trigger" class="container">
    <!-- list od dropdown menu -->
    <ion-popover trigger="click-trigger" trigger-action="click">
      <ion-content class="ion-padding">
        <ion-list>
          <ion-item :button="true" :detail="false">Option 1</ion-item>
          <ion-item :button="true" :detail="false">Option 1</ion-item>
        </ion-list>
      </ion-content>
    </ion-popover>
    <!-- end of dropdown -->
    <ion-avatar slot="start" class="avatar">
      <img
        alt="Silhouette of a person's head"
        src="https://ionicframework.com/docs/img/demos/avatar.svg"
      >
    </ion-avatar>
    <div class="text-content">
      <ion-label class="body-normal">
        {{ lastname }} {{ firstname }}
      </ion-label>
      <ion-text class="connect-state body-sm">{{ $t('HeaderPage.loggedIn') }}</ion-text>
    </div>
    <ion-icon
      slot="end"
      :icon="chevronDown"
    />
  </ion-item>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  IonItem,
  IonIcon,
  IonLabel,
  IonAvatar
}from '@ionic/vue';
import {
  chevronDown, ellipse
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
.container{
  display: flex;
  flex-direction: column;
  --background: none;
  .item-native{
    padding-inline-start: 0;
    padding: 0;
  }
  .item-inner{
    border-width: 0;
  }
}
.avatar{
  margin: 0 .75em 0 0;
}
.text-content{
  display: flex;
  flex-direction: column;
  justify-content: center;
  color: var(--parsec-color-);
}
.connect-state{
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 0.375rem;
  color: var(--parsec-color-light-secondary-grey);
  &::after{
    content:'';
    height: 0.5rem;
    width: 0.5rem;
    border-radius: 50%;
    background-color: var(---parsec-color--success-500);
  }
}
</style>
