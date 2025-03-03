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
    <div class="text-content">
      <div class="text-content-name">
        <ion-text class="body">
          {{ name }}
        </ion-text>
        <ion-icon
          :class="{ 'popover-is-open': isPopoverOpen }"
          slot="end"
          :icon="chevronDown"
        />
      </div>
      <ion-text
        class="text-content-update button-small"
        v-show="updateAvailability.updateAvailable"
      >
        {{ $msTranslate('HomePage.topbar.updateAvailable') }}
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { UserProfile, getConnectionInfo } from '@/parsec';
import { Routes, navigateTo, ProfilePages } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperationManager';
import { Answer, askQuestion, MsModalResult } from 'megashark-lib';
import ProfileHeaderPopover, { ProfilePopoverOption } from '@/views/header/ProfileHeaderPopover.vue';
import { IonIcon, IonItem, IonText, modalController, popoverController } from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';
import { Env } from '@/services/environment';
import UpdateAppModal from '@/views/about/UpdateAppModal.vue';
import { APP_VERSION } from '@/services/environment';

const isOnline = ref(false);
const isPopoverOpen = ref(false);

const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const updateAvailability: Ref<UpdateAvailabilityData> = ref({ updateAvailable: false });
let eventCbId: null | string = null;

const props = defineProps<{
  name: string;
  email: string;
  profile: UserProfile;
}>();

onMounted(async () => {
  eventCbId = await eventDistributor.registerCallback(
    Events.Offline | Events.Online | Events.UpdateAvailability,
    async (event: Events, data?: EventData) => {
      if (event === Events.Offline) {
        isOnline.value = false;
      } else if (event === Events.Online) {
        isOnline.value = true;
      } else if (event === Events.UpdateAvailability) {
        updateAvailability.value = data as UpdateAvailabilityData;
      }
    },
  );

  const connInfo = getConnectionInfo();
  if (connInfo) {
    isOnline.value = connInfo.isOnline;
  }

  window.electronAPI.getUpdateAvailability();
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
      email: props.email,
      profile: props.profile,
      updateAvailability: updateAvailability.value,
    },
    event: event,
    showBackdrop: false,
    alignment: 'end',
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  isPopoverOpen.value = false;

  if (data === undefined) {
    return;
  }
  if (data.option === ProfilePopoverOption.Update) {
    const modal = await modalController.create({
      component: UpdateAppModal,
      canDismiss: true,
      cssClass: 'update-app-modal',
      backdropDismiss: false,
      componentProps: {
        currentVersion: APP_VERSION,
        targetVersion: updateAvailability.value.version,
      },
    });
    await modal.present();
    const { role } = await modal.onWillDismiss();
    await modal.dismiss();

    if (role === MsModalResult.Confirm) {
      await eventDistributor.dispatchEvent(Events.LogoutRequested);
      window.electronAPI.prepareUpdate();
    }
  } else if (data.option === ProfilePopoverOption.LogOut) {
    const answer = await askQuestion(
      'HomePage.topbar.logoutConfirmTitle',
      fileOperationManager.hasOperations() ? 'HomePage.topbar.logoutImportsConfirmQuestion' : 'HomePage.topbar.logoutConfirmQuestion',
      {
        yesText: 'HomePage.topbar.logoutYes',
        noText: 'HomePage.topbar.logoutNo',
      },
    );

    if (answer === Answer.Yes) {
      await eventDistributor.dispatchEvent(Events.LogoutRequested);
    }
  } else if (data.option === ProfilePopoverOption.Settings) {
    await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.Settings } });
  } else if (data.option === ProfilePopoverOption.Documentation) {
    await Env.Links.openDocumentationLink();
  } else if (data.option === ProfilePopoverOption.Feedback) {
    await Env.Links.openContactLink();
  } else if (data.option === ProfilePopoverOption.About) {
    await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.About } });
  } else if (data.option === ProfilePopoverOption.Device) {
    await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.Devices } });
  } else if (data.option === ProfilePopoverOption.Authentication) {
    await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.Authentication } });
  } else if (data.option === ProfilePopoverOption.Recovery) {
    await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.Recovery } });
  }
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

.text-content {
  display: flex;
  flex-direction: column;

  &-name {
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

  &-update {
    color: var(--parsec-color-light-primary-500);
  }
}
</style>
