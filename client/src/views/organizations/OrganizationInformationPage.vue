<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="organization-page">
    <ion-content
      :fullscreen="true"
      class="organization-page-content"
    >
      <div
        class="organization-header"
        @click="openOrganizationChoice($event)"
        v-if="isSmallDisplay && userInfo && !querying"
      >
        <ion-avatar class="organization-header-avatar body-lg">
          <span v-if="!isTrialOrg">{{ userInfo.organizationId.substring(0, 2) }}</span>
          <ms-image
            v-else
            :image="LogoIconGradient"
            class="avatar-logo"
          />
        </ion-avatar>
        <ion-text class="organization-header-text title-h2">{{ userInfo.organizationId }}</ion-text>
        <ms-image
          :image="ChevronExpand"
          class="header-icon"
        />
      </div>

      <template v-if="orgInfo && userInfo">
        <organization-user-information
          :org-info="orgInfo"
          :user-info="userInfo"
        />
        <!-- ------------- Information ------------- -->
        <organization-configuration-information :org-info="orgInfo" />

        <!-- ------------- Storage list ------------- -->
        <organization-storage-information
          v-show="false"
          :org-info="orgInfo"
        />
      </template>

      <template v-else-if="!querying">
        <div class="information-not-found">
          <ion-icon
            :icon="warning"
            size="large"
          />
          <ion-text class="body">
            {{ $msTranslate('OrganizationPage.getInfoFailed') }}
          </ion-text>
        </div>
      </template>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  ClientInfo,
  OrganizationInfo,
  getOrganizationInfo,
  getClientInfo as parsecGetClientInfo,
  getCurrentAvailableDevice,
  AvailableDevice,
} from '@/parsec';
import { IonContent, IonIcon, IonPage, IonText, IonAvatar, popoverController } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { switchOrganization } from '@/router';
import { ChevronExpand, MsImage, LogoIconGradient, MsModalResult } from 'megashark-lib';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';
import useUploadMenu from '@/services/fileUploadMenu';
import OrganizationSwitchPopover from '@/components/organizations/OrganizationSwitchPopover.vue';
import OrganizationUserInformation from '@/components/organizations/OrganizationUserInformation.vue';
import OrganizationConfigurationInformation from '@/components/organizations/OrganizationConfigurationInformation.vue';
import OrganizationStorageInformation from '@/components/organizations/OrganizationStorageInformation.vue';
import { isTrialOrganizationDevice } from '@/common/organization';
import { useWindowSize } from 'megashark-lib';
import { navigateTo, Routes } from '@/router';
import { isUserAction, UserAction } from '@/views/users/types';
import { EventData, EventDistributor, EventDistributorKey, Events, MenuActionData } from '@/services/eventDistributor';

const { isSmallDisplay } = useWindowSize();

const orgInfo: Ref<OrganizationInfo | null> = ref(null);
const userInfo: Ref<ClientInfo | null> = ref(null);
const isTrialOrg = ref(false);
const currentDevice: Ref<AvailableDevice | null> = ref(null);
const querying = ref(true);
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

async function openOrganizationChoice(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: OrganizationSwitchPopover,
    cssClass: 'dropdown-popover',
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  const { data, role } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm) {
    const menuCtrls = useUploadMenu();
    menuCtrls.hide();
    switchOrganization(data.handle);
  }
}

async function performOrganizationAction(action: UserAction): Promise<void> {
  if (!userInfo.value) {
    return;
  }
  if (action === UserAction.Invite) {
    await navigateTo(Routes.Users, { query: { openInvite: true } });
  }
}

let eventCbId: string | undefined = undefined;

onMounted(async () => {
  const result = await getOrganizationInfo();
  const infoResult = await parsecGetClientInfo();

  const currentDeviceResult = await getCurrentAvailableDevice();

  if (currentDeviceResult.ok) {
    currentDevice.value = currentDeviceResult.value;
    isTrialOrg.value = isTrialOrganizationDevice(currentDevice.value);
  } else {
    window.electronAPI.log('error', `Failed to retrieve current device ${JSON.stringify(currentDeviceResult.error)}`);
  }

  if (!result.ok) {
    return;
  }
  orgInfo.value = result.value;

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(infoResult.error)}`);
  }

  eventCbId = await eventDistributor.registerCallback(Events.MenuAction, async (event: Events, data?: EventData) => {
    if (event === Events.MenuAction && isUserAction((data as MenuActionData).action.action)) {
      await performOrganizationAction((data as MenuActionData).action.action);
    }
  });

  querying.value = false;
});

onUnmounted(async () => {
  if (eventCbId) {
    await eventDistributor.removeCallback(eventCbId);
  }
});
</script>

<style scoped lang="scss">
.organization-page {
  background: var(--parsec-color-light-secondary-premiere) !important;
  display: flex;
  flex-wrap: wrap;
  justify-content: start;
  gap: 2rem;

  &-content {
    --background: none;

    &::part(scroll) {
      display: flex;
      flex-direction: column;
      gap: 2rem;
      padding: 2em;

      @include ms.responsive-breakpoint('sm') {
        align-items: center;
        gap: 1.5rem;
        padding: 1.5rem;
        margin-bottom: 4.75rem;
      }
    }
  }
}

.organization-header {
  display: flex;
  width: 100%;
  max-width: 30rem;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-white);
  gap: 0.75rem;

  &-avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 2.5rem;
    width: 2.5rem;
    height: 2.5rem;
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-12);
    padding: 0.25rem;

    .avatar-logo {
      width: 1.5rem;
    }

    span {
      font-size: 1.25rem;
      font-weight: bold;
      color: var(--parsec-color-light-primary-600);
    }
  }

  &-text {
    width: 100%;
    color: var(--parsec-color-light-secondary-text);
  }
}

.information-not-found {
  margin: 2rem;
  max-width: fit-content;
  background: var(--parsec-color-light-danger-50);
  color: var(--parsec-color-light-danger-700);
  padding: 1rem;
  display: flex;
  gap: 0.75rem;
  border-left: 0.25rem solid var(--parsec-color-light-danger-700);

  ion-text {
    padding: 0.25rem 0;
  }

  ion-icon {
    font-size: 1rem !important;
  }
}
</style>
