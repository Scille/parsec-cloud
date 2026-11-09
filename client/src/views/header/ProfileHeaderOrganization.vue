<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-item
    button
    id="click-trigger"
    class="profile-header ion-no-padding"
    :class="{ 'profile-header-clicked': isPopoverOpen, 'profile-header--collapsed': collapsed }"
    :title="collapsed ? name : undefined"
    @click="openOrganizationPopover($event)"
  >
    <div
      class="avatar"
      :class="{ online: isOnline, offline: !isOnline }"
    >
      <ms-image
        :image="PersonCircle"
        class="avatar__image"
      />
    </div>
    <div
      v-if="!collapsed"
      class="text-content"
    >
      <ion-text class="text-content__name">
        {{ name }}
      </ion-text>
      <ion-text class="text-content__profile">
        {{ $msTranslate(getProfileTranslationKey(profile)) }}
      </ion-text>
      <ion-text
        class="text-content-update"
        v-show="updateAvailability.updateAvailable"
      >
        {{ $msTranslate('HomePage.topbar.updateAvailable') }}
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import PersonCircle from '@/assets/images/person-circle.svg?raw';
import { openBugReportModal } from '@/components/misc';
import { getClientInfo, UserProfile } from '@/parsec';
import { navigateTo, ProfilePages, Routes } from '@/router';
import { Env } from '@/services/environment';
import { EventData, EventDistributor, EventDistributorKey, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperation/manager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { getProfileTranslationKey } from '@/services/translation';
import { openUpdateAppModal } from '@/views/about';
import ProfileHeaderOrganizationPopover, { ProfilePopoverOption } from '@/views/header/ProfileHeaderOrganizationPopover.vue';
import { IonItem, IonText, popoverController } from '@ionic/vue';
import { Answer, askQuestion, MsImage, MsModalResult } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';

const isOnline = ref(false);
const isPopoverOpen = ref(false);

const fileOperationManager: Ref<FileOperationManager> = inject(FileOperationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const updateAvailability: Ref<UpdateAvailabilityData> = ref({ updateAvailable: false });
let eventCbId: null | string = null;

const props = defineProps<{
  name: string;
  email: string;
  profile: UserProfile;
  collapsed?: boolean;
}>();

onMounted(async () => {
  eventCbId = await eventDistributor.value.registerCallback(
    [Events.Offline, Events.Online, Events.UpdateAvailability],
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

  window.nativeAPI.getUpdateAvailability();
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.value.removeCallback(eventCbId);
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
    side: 'right',
    alignment: 'end',
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  isPopoverOpen.value = false;

  if (data === undefined) {
    return;
  }
  if (data.option === ProfilePopoverOption.Update && updateAvailability.value.version) {
    const answer = await openUpdateAppModal(updateAvailability.value.version);

    if (answer === Answer.Yes) {
      await eventDistributor.value.dispatchEvent(Events.LogoutRequested);
      window.nativeAPI.prepareUpdate();
    }
  } else if (data.option === ProfilePopoverOption.LogOut) {
    const answer = await askQuestion(
      'HomePage.topbar.logoutConfirmTitle',
      fileOperationManager.value.hasOperations() ? 'HomePage.topbar.logoutImportsConfirmQuestion' : 'HomePage.topbar.logoutConfirmQuestion',
      {
        yesText: 'HomePage.topbar.logoutYes',
        noText: 'HomePage.topbar.logoutNo',
      },
    );

    if (answer === Answer.Yes) {
      await eventDistributor.value.dispatchEvent(Events.LogoutRequested);
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
      informationManager.value.present(
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
  --min-height: 0;
  --inner-padding-end: 0;
  border-radius: ms.radius('lg');
  padding: ms.spacing('padding-md') ms.spacing('padding-lg');
  border: ms.border('thin') solid transparent;
  transition: border-color ease-in-out 150ms;
  width: 100%;

  & * {
    pointer-events: none;
  }

  &:hover,
  &-clicked {
    --background-hover: none;
    border-color: ms.color('border-neutral-on-color');
  }

  &-clicked {
    background: ms.color('surface-neutral-on-color');
  }

  &--collapsed {
    width: fit-content;
    margin: 0 auto;
    padding: ms.spacing('padding-sm');
  }
}

.avatar {
  position: relative;
  display: flex;
  flex-shrink: 0;
  margin-right: ms.spacing('gap-lg');

  .profile-header--collapsed & {
    margin-right: 0;
  }

  &__image {
    width: 2rem;
    height: 2rem;
  }

  &::after {
    content: '';
    position: absolute;
    bottom: 1px;
    right: 0;
    height: 0.5rem;
    width: 0.5rem;
    border-radius: ms.radius('full');
    border: ms.color('border-base-on-color') solid ms.border('thick');
    z-index: 2;
  }

  &.online::after {
    background-color: ms.color('surface-success-default');
  }

  &.offline::after {
    background-color: ms.color('surface-disabled-default');
  }
}

.text-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-xs');
  overflow: hidden;

  &__name,
  &__profile {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  &__name {
    @include ms.font('label-md-medium');
    color: ms.color('text-on-color-heading');
  }

  &__profile {
    @include ms.font('label-sm-medium');
    color: ms.color('text-on-color-label');
    opacity: ms.opacity('8');
  }

  &-update {
    @include ms.font('label-sm-medium');
    color: ms.color('text-on-color-heading');
  }
}
</style>
