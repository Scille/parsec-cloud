<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item
    button
    id="click-trigger"
    class="container"
    @click="isPopoverOpen = !isPopoverOpen; openPopover($event)"
  >
    <ion-avatar
      slot="start"
      class="avatar"
    >
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
        :class="{'popover-is-open': isPopoverOpen}"
        slot="end"
        :icon="chevronDown"
        ref="chevron"
      />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import {
  IonItem,
  IonIcon,
  IonAvatar,
  IonText,
  popoverController
}from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
import { defineProps, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import ProfileHeaderPopover from './ProfileHeaderPopover.vue';

const { t, d } = useI18n();
const isPopoverOpen = ref(false);
const chevron = ref();

defineProps<{
  firstname: string,
  lastname: string
}>();

async function openPopover(ev: Event): Promise<void> {
  const popover = await popoverController.create({
    component: ProfileHeaderPopover,
    cssClass: 'profile-header-popover',
    event: ev,
    showBackdrop: false
  });
  await popover.present();
  popover.onDidDismiss().then(() => {
    chevron.value.$el.style.setProperty('transition', 'transform ease-out 300ms');
    isPopoverOpen.value = false;
  });
}

</script>

<style lang="scss" scoped>

.container {
  display: flex;
  flex-direction: column;
  --background: none;
  cursor: pointer;
  &:hover{
    --background-hover: none;
  }
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
    background-color: var(--parsec-color-light-success-500);
  }
}

.text-icon {
  display: flex;
  align-items: center;
  gap: .1em;
  color: var(--parsec-color-light-secondary-text);

  ion-icon.popover-is-open {
    transform: rotate(180deg);
    transition: transform ease-out 300ms;
  }
}

.text-content {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

</style>
