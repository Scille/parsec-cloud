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
import { UserProfile, logout as parsecLogout } from '@/parsec';
import { Routes, getConnectionHandle, navigateTo } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import useUploadMenu from '@/services/fileUploadMenu';
import { ImportManager, ImportManagerKey } from '@/services/importManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { Answer, askQuestion, I18n } from 'megashark-lib';
import ProfileHeaderPopover, { ProfilePopoverOption } from '@/views/header/ProfileHeaderPopover.vue';
import { openSettingsModal } from '@/views/settings';
import { IonIcon, IonItem, IonText, popoverController } from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';

const isOnline = ref(true);
const isPopoverOpen = ref(false);

const informationManager: InformationManager = inject(InformationManagerKey)!;
const importManager: ImportManager = inject(ImportManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const updateAvailability: Ref<UpdateAvailabilityData> = ref({ updateAvailable: false });
let eventCbId: null | string = null;
let intervalId: any = null;

const props = defineProps<{
  name: string;
  email: string;
  profile: UserProfile;
}>();

onMounted(async () => {
  eventCbId = await eventDistributor.registerCallback(
    Events.Offline | Events.Online | Events.UpdateAvailability,
    async (event: Events, data: EventData) => {
      if (event === Events.Offline) {
        isOnline.value = false;
      } else if (event === Events.Online) {
        isOnline.value = true;
      } else if (event === Events.UpdateAvailability) {
        updateAvailability.value = data as UpdateAvailabilityData;
      }
    },
  );

  intervalId = setInterval(async () => {
    window.electronAPI.getUpdateAvailability();
  }, 600000); // Checking every 10min
  // Also calling it right now
  window.electronAPI.getUpdateAvailability();
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
  if (intervalId) {
    clearInterval(intervalId);
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
  if (data.option === ProfilePopoverOption.LogOut) {
    const answer = await askQuestion(
      'HomePage.topbar.logoutConfirmTitle',
      importManager.isImporting() ? 'HomePage.topbar.logoutImportsConfirmQuestion' : 'HomePage.topbar.logoutConfirmQuestion',
      {
        yesText: 'HomePage.topbar.logoutYes',
        noText: 'HomePage.topbar.logoutNo',
      },
    );

    if (answer === Answer.Yes) {
      const handle = getConnectionHandle();
      if (!handle) {
        console.error('Already logged out');
        return;
      }
      const result = await parsecLogout();
      if (!result.ok) {
        informationManager.present(
          new Information({
            message: 'HomePage.topbar.logoutFailed',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
      } else {
        const menuCtrls = useUploadMenu();
        menuCtrls.hide();
        await injectionProvider.clean(handle);
        await navigateTo(Routes.Home, { replace: true, skipHandle: true });
      }
    }
  } else if (data.option === ProfilePopoverOption.Settings) {
    await openSettingsModal();
  } else if (data.option === ProfilePopoverOption.Help) {
    window.open(I18n.translate('MenuPage.helpLink'), '_blank');
  } else if (data.option === ProfilePopoverOption.App) {
    await navigateTo(Routes.About);
  } else if (data.option === ProfilePopoverOption.MyProfile) {
    await navigateTo(Routes.MyProfile);
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
