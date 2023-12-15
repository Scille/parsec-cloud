<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    id="click-trigger"
    class="container"
    @click="openPopover($event)"
  >
    <ion-avatar
      slot="start"
      class="avatar"
    >
      <ion-icon :icon="personCircle" />
    </ion-avatar>
    <div class="text-icon">
      <ion-text class="body">
        {{ name }}
      </ion-text>
      <ion-icon
        :class="{ 'popover-is-open': isPopoverOpen }"
        slot="end"
        :icon="chevronDown"
      />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { Answer, askQuestion } from '@/components/core';
import { logout as parsecLogout } from '@/parsec';
import { routerNavigateTo } from '@/router';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import ProfileHeaderPopover, { ProfilePopoverOption } from '@/views/header/ProfileHeaderPopover.vue';
import { IonAvatar, IonIcon, IonItem, IonText, popoverController } from '@ionic/vue';
import { chevronDown, personCircle } from 'ionicons/icons';
import { defineProps, inject, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

const isPopoverOpen = ref(false);
const router = useRouter();
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationManager: NotificationManager = inject(NotificationKey)!;
const { t } = useI18n();

const props = defineProps<{
  name: string;
}>();

async function openPopover(event: Event): Promise<void> {
  isPopoverOpen.value = !isPopoverOpen.value;
  const popover = await popoverController.create({
    component: ProfileHeaderPopover,
    cssClass: 'profile-header-popover',
    componentProps: {
      name: props.name,
    },
    event: event,
    showBackdrop: false,
    alignment: 'end',
  });
  await popover.present();

  popover.onDidDismiss().then(async (value) => {
    isPopoverOpen.value = false;

    if (value.data === undefined) {
      return;
    }
    if (value.data.option === ProfilePopoverOption.LogOut) {
      const answer = await askQuestion(t('HomePage.topbar.logoutConfirmTitle'), t('HomePage.topbar.logoutConfirmQuestion'));

      if (answer === Answer.Yes) {
        const result = await parsecLogout();
        if (!result.ok) {
          notificationManager.showToast(
            new Notification({
              message: t('HomePage.topbar.logoutFailed'),
              level: NotificationLevel.Error,
            }),
          );
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
    } else if (value.data.option === ProfilePopoverOption.App) {
      routerNavigateTo('about');
    } else if (value.data.option === ProfilePopoverOption.MyContactDetails) {
      routerNavigateTo('myContactDetails');
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
  &:hover {
    --background-hover: none;
  }
  & * {
    pointer-events: none;
  }
}

.avatar {
  margin: 0 0.75em 0 0;
  position: relative;

  ion-icon {
    width: 100%;
    height: 100%;
  }

  &::after {
    content: '';
    position: absolute;
    bottom: 0;
    right: -4px;
    height: 0.625rem;
    width: 0.625rem;
    border-radius: 50%;
    border: var(--parsec-color-light-secondary-white) solid 0.25rem;
    background-color: var(--parsec-color-light-success-500);
  }
}

.text-icon {
  display: flex;
  align-items: center;
  color: var(--parsec-color-light-secondary-text);

  ion-icon {
    transition: transform ease-out 300ms;
    font-size: 1.125rem;
    &.popover-is-open {
      transform: rotate(180deg);
    }
  }
}
</style>
