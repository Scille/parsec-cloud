<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
      <ion-icon
        :icon="personCircle"
      />
    </ion-avatar>
    <div class="text-icon">
      <ion-text class="body">
        {{ name }}
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
  popoverController,
}from '@ionic/vue';
import { chevronDown, personCircle } from 'ionicons/icons';
import { defineProps, ref, inject } from 'vue';
import ProfileHeaderPopover from '@/views/header/ProfileHeaderPopover.vue';
import { ProfilePopoverOption } from '@/views/header/ProfileHeaderPopover.vue';
import { useRouter } from 'vue-router';
import { routerNavigateTo } from '@/router';
import { useI18n } from 'vue-i18n';
import { logout as parsecLogout } from '@/parsec';
import { askQuestion, Answer } from '@/components/core/ms-modal/MsQuestionModal.vue';
import { NotificationCenter, Notification, NotificationKey, NotificationLevel } from '@/services/notificationCenter';

const isPopoverOpen = ref(false);
const chevron = ref();
const router = useRouter();
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationCenter: NotificationCenter = inject(NotificationKey)!;
const { t } = useI18n();

const props = defineProps<{
  name: string
}>();

async function openPopover(ev: Event): Promise<void> {
  const popover = await popoverController.create({
    component: ProfileHeaderPopover,
    componentProps: {
      name: props.name,
    },
    cssClass: 'profile-header-popover',
    event: ev,
    showBackdrop: false,
  });
  await popover.present();

  popover.onDidDismiss().then(async (value) => {
    chevron.value.$el.style.setProperty('transition', 'transform ease-out 300ms');
    isPopoverOpen.value = false;

    if (value.data === undefined) {
      return;
    }
    if (value.data.option === ProfilePopoverOption.LogOut) {
      const answer = await askQuestion(t('HomePage.topbar.logoutConfirm'));

      if (answer === Answer.Yes) {
        const result = await parsecLogout();
        if (!result.ok) {
          notificationCenter.showToast(new Notification({
            message: t('HomePage.topbar.logoutFailed'),
            level: NotificationLevel.Error,
          }));
        } else {
          router.replace({ name: 'home' });
        }
      }
    } else if (value.data.option === ProfilePopoverOption.Settings) {
      routerNavigateTo('settings');
    } else if (value.data.option === ProfilePopoverOption.MyDevices) {
      routerNavigateTo('devices');
    } else if (value.data.option === ProfilePopoverOption.Help) {
      window.open(t('MenuPage.helpLink'), '_blank');
    }  else if (value.data.option === ProfilePopoverOption.App) {
      routerNavigateTo('about');
    }
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

  ion-icon {
    width: 100%;
    height: 100%;
  }

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
