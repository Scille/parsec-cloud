<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ms-image
        :image="PersonCircle"
        class="profile-info-card__avatar"
      />
    </div>
    <div class="text-content">
      <div class="text-content-name">
        <ion-text class="text-content-name__text">
          {{ name }}
        </ion-text>
        <ion-icon
          :class="{ 'popover-is-open': isPopoverOpen }"
          slot="end"
          :icon="chevronDown"
        />
      </div>
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
import { openUpdateAppModal } from '@/views/about';
import ProfileHeaderOrganizationPopover, { ProfilePopoverOption } from '@/views/header/ProfileHeaderOrganizationPopover.vue';
import { IonIcon, IonItem, IonText, popoverController } from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
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
  border-radius: ms.radius('2xl');
  padding: ms.spacing('padding-md') ms.spacing('padding-lg') ms.spacing('padding-md') ms.spacing('padding-sm');
  border: ms.border('thin') solid transparent;
  transition: all ease-in-out 200ms;
  flex-shrink: 0;
  max-width: 15rem;

  & * {
    pointer-events: none;
  }

  &:hover {
    --background-hover: none;
    border-color: ms.color('border-base-default');
    background: ms.color('surface-neutral-default-subtle-hover');
  }

  &-clicked {
    border-color: ms.color('border-base-default');
    background: ms.color('surface-neutral-default-subtle-hover');
  }
}

.avatar {
  position: relative;
  color: ms.color('icon-brand-default');
  display: flex;
  margin-right: ms.spacing('padding-lg');

  @include ms.responsive-breakpoint('md') {
    margin: 0;
  }

  &-icon {
    font-size: 2rem;
  }

  &::after {
    content: '';
    position: absolute;
    bottom: 2px;
    right: 1px;
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
  width: 100%;
  overflow: hidden;

  &-name {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    color: ms.color('text-neutral-default');

    &__text {
      @include ms.font('body-lg-medium');
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
    @include ms.font('label-sm-medium');
    color: ms.color('text-brand-default');

    @include ms.responsive-breakpoint('md') {
      display: none;
    }
  }
}
</style>
