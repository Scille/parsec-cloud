<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    id="click-trigger"
    class="profile-header ion-no-padding"
    :class="{ 'profile-header-clicked': isPopoverOpen }"
    @click="openOrganizationPopover($event)"
  >
    <div
      class="avatar small"
      :class="{ online: isOnline, offline: !isOnline }"
    >
      <ion-icon
        :icon="personCircle"
        class="avatar-icon"
      />
    </div>
    <div class="text-content">
      <div class="text-content-name">
        <ion-text class="text-content-name__text subtitles-normal">
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
import { openBugReportModal } from '@/components/misc';
import { getClientInfo, UserProfile } from '@/parsec';
import { navigateTo, ProfilePages, Routes } from '@/router';
import { APP_VERSION, Env } from '@/services/environment';
import { EventData, EventDistributor, EventDistributorKey, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperation/manager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import UpdateAppModal from '@/views/about/UpdateAppModal.vue';
import ProfileHeaderOrganizationPopover, { ProfilePopoverOption } from '@/views/header/ProfileHeaderOrganizationPopover.vue';
import { IonIcon, IonItem, IonText, modalController, popoverController } from '@ionic/vue';
import { chevronDown, personCircle } from 'ionicons/icons';
import { Answer, askQuestion, MsModalResult } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';

const isOnline = ref(false);
const isPopoverOpen = ref(false);

const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const informationManager: InformationManager = inject(InformationManagerKey)!;
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

  const clientInfoResult = await getClientInfo();
  if (clientInfoResult.ok) {
    isOnline.value = clientInfoResult.value.isServerOnline;
  }

  window.electronAPI.getUpdateAvailability();
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

async function openOrganizationPopover(event: Event): Promise<void> {
  isPopoverOpen.value = !isPopoverOpen.value;
  const popover = await popoverController.create({
    component: ProfileHeaderOrganizationPopover,
    cssClass: 'profile-header-organization-popover',
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
  } else if (data.option === ProfilePopoverOption.ReportBug) {
    const result = await openBugReportModal();
    if (result === MsModalResult.Confirm) {
      informationManager.present(
        new Information({
          message: 'bugReport.sent',
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  }
}
</script>

<style lang="scss" scoped>
.profile-header {
  --background: none;
  --background-hover: none;
  border-radius: var(--parsec-radius-12);
  padding: 0.375rem 0.5rem 0.375rem 0.25rem;
  border: 1px solid transparent;
  transition: all ease-in-out 200ms;
  flex-shrink: 0;
  max-width: 15rem;

  & * {
    pointer-events: none;
  }

  &:hover {
    --background-hover: none;
    border-color: var(--parsec-color-light-secondary-medium);
    background: var(--parsec-color-light-secondary-background);
  }

  &-clicked {
    border-color: var(--parsec-color-light-secondary-medium);
    background: var(--parsec-color-light-secondary-background);
  }
}

.avatar {
  position: relative;
  color: var(--parsec-color-light-primary-600);
  display: flex;
  margin-right: 0.5rem;

  @include ms.responsive-breakpoint('md') {
    margin: 0;
  }

  &-icon {
    font-size: 2rem;
  }

  &::after {
    content: '';
    position: absolute;
    bottom: 0px;
    right: 0px;
    height: 0.5rem;
    width: 0.5rem;
    border-radius: 50%;
    border: var(--parsec-color-light-secondary-white) solid 2px;
    z-index: 2;
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
  width: 100%;
  overflow: hidden;

  &-name {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-soft-text);

    &__text {
      font-size: 0.9375rem;
      width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;

      @include ms.responsive-breakpoint('md') {
        display: none;
      }
    }

    ion-icon {
      transition: transform ease-out 300ms;
      font-size: 1rem;
      flex-shrink: 0;

      &.popover-is-open {
        transform: rotate(180deg);
      }
    }
  }

  &-update {
    color: var(--parsec-color-light-primary-500);

    @include ms.responsive-breakpoint('md') {
      display: none;
    }
  }
}
</style>
