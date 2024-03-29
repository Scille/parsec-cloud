<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    id="click-trigger"
    class="container ion-no-padding"
    @click="openPopover($event)"
  >
    <user-avatar-name
      :user-avatar="name"
      class="avatar medium"
      :class="{ online: isOnline, offline: !isOnline }"
    />
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
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { logout as parsecLogout } from '@/parsec';
import { Routes, navigateTo } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import useUploadMenu from '@/services/fileUploadMenu';
import { ImportManager, ImportManagerKey } from '@/services/importManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import ProfileHeaderPopover, { ProfilePopoverOption } from '@/views/header/ProfileHeaderPopover.vue';
import { IonIcon, IonItem, IonText, popoverController } from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref } from 'vue';

const isOnline = ref(true);
const isPopoverOpen = ref(false);

const informationManager: InformationManager = inject(InformationManagerKey)!;
const importManager: ImportManager = inject(ImportManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

let eventCbId: null | string = null;

const props = defineProps<{
  name: string;
}>();

onMounted(async () => {
  eventCbId = await eventDistributor.registerCallback(Events.Offline | Events.Online, async (event: Events, _data: EventData) => {
    if (event === Events.Offline) {
      isOnline.value = false;
    } else if (event === Events.Online) {
      isOnline.value = true;
    }
  });
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

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
      const answer = await askQuestion(
        translate('HomePage.topbar.logoutConfirmTitle'),
        importManager.isImporting()
          ? translate('HomePage.topbar.logoutImportsConfirmQuestion')
          : translate('HomePage.topbar.logoutConfirmQuestion'),
        {
          yesText: translate('HomePage.topbar.logoutYes'),
          noText: translate('HomePage.topbar.logoutNo'),
        },
      );

      if (answer === Answer.Yes) {
        const menuCtrls = useUploadMenu();
        menuCtrls.hide();
        await importManager.stop();
        const result = await parsecLogout();
        if (!result.ok) {
          informationManager.present(
            new Information({
              message: translate('HomePage.topbar.logoutFailed'),
              level: InformationLevel.Error,
            }),
            PresentationMode.Toast,
          );
        } else {
          await navigateTo(Routes.Home, { replace: true, skipHandle: true });
        }
      }
    } else if (value.data.option === ProfilePopoverOption.Settings) {
      await navigateTo(Routes.Settings);
    } else if (value.data.option === ProfilePopoverOption.MyDevices) {
      await navigateTo(Routes.Devices);
    } else if (value.data.option === ProfilePopoverOption.Help) {
      window.open(translate('MenuPage.helpLink'), '_blank');
    } else if (value.data.option === ProfilePopoverOption.App) {
      await navigateTo(Routes.About);
    } else if (value.data.option === ProfilePopoverOption.MyContactDetails) {
      await navigateTo(Routes.ContactDetails);
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

  &::after {
    content: '';
    position: absolute;
    bottom: 0;
    right: -3px;
    height: 0.625rem;
    width: 0.625rem;
    border-radius: 50%;
    border: var(--parsec-color-light-secondary-white) solid 3px;
  }

  &.online::after {
    background-color: var(--parsec-color-light-success-500);
  }

  &.offline::after {
    background-color: var(--parsec-color-light-secondary-light);
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
