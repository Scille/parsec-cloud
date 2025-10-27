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
        v-if="isSmallDisplay"
      >
        <template v-if="userInfo">
          <ion-avatar class="organization-header-avatar body-lg">
            <span v-if="!isTrialOrg">{{ userInfo.organizationId.substring(0, 2) }}</span>
            <!-- prettier-ignore -->
            <ms-image
              v-else
              :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
              class="avatar-logo"
            />
          </ion-avatar>
          <ion-text class="organization-header-text title-h2">{{ userInfo.organizationId }}</ion-text>
          <ms-image
            :image="ChevronExpand"
            class="header-icon"
          />
        </template>
        <template v-else-if="!error">
          <div class="skeleton-content">
            <ion-skeleton-text
              :animated="true"
              class="skeleton-content__text"
            />
          </div>
        </template>
        <template v-else>
          <ms-report-text
            :theme="MsReportTheme.Error"
            class="organization-page-error"
          >
            {{ $msTranslate(error) }}
          </ms-report-text>
        </template>
      </div>

      <template v-if="orgInfo && userInfo">
        <div class="organization-sections">
          <organization-user-information
            :org-info="orgInfo"
            :user-info="userInfo"
          />
          <!-- ------------- Information ------------- -->
          <organization-configuration-information :org-info="orgInfo" />

          <!-- ------------- Storage list ------------- -->
          <organization-storage-information :org-info="orgInfo" />
        </div>
      </template>

      <template v-else-if="!error">
        <div
          class="skeleton"
          v-for="n in 3"
          :key="n"
        >
          <ion-skeleton-text
            :animated="true"
            class="skeleton__title"
          />
          <div class="skeleton-content">
            <ion-skeleton-text
              :animated="true"
              class="skeleton-content__text"
            />
            <ion-skeleton-text
              :animated="true"
              class="skeleton-content__text"
            />
            <ion-skeleton-text
              :animated="true"
              class="skeleton-content__text"
            />
          </div>
        </div>
      </template>

      <template v-else>
        <ms-report-text
          :theme="MsReportTheme.Error"
          class="organization-page-error"
        >
          {{ $msTranslate(error) }}
        </ms-report-text>
      </template>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { isTrialOrganizationDevice } from '@/common/organization';
import OrganizationConfigurationInformation from '@/components/organizations/OrganizationConfigurationInformation.vue';
import OrganizationStorageInformation from '@/components/organizations/OrganizationStorageInformation.vue';
import OrganizationSwitchPopover from '@/components/organizations/OrganizationSwitchPopover.vue';
import OrganizationUserInformation from '@/components/organizations/OrganizationUserInformation.vue';
import {
  ClientInfo,
  OrganizationInfo,
  getCurrentAvailableDevice,
  getOrganizationInfo,
  getClientInfo as parsecGetClientInfo,
} from '@/parsec';
import { Routes, currentRouteIs, switchOrganization, watchRoute } from '@/router';
import useUploadMenu from '@/services/fileUploadMenu';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonAvatar, IonContent, IonPage, IonSkeletonText, IonText, popoverController } from '@ionic/vue';
import { ChevronExpand, LogoIconGradient, MsImage, MsModalResult, MsReportText, MsReportTheme, useWindowSize } from 'megashark-lib';
import { Ref, onMounted, onUnmounted, ref } from 'vue';

const { isSmallDisplay } = useWindowSize();

const orgInfo: Ref<OrganizationInfo | null> = ref(null);
const userInfo: Ref<ClientInfo | null> = ref(null);
const isTrialOrg = ref(false);
const error = ref('');

const watchRouteCancel = watchRoute(async () => {
  if (currentRouteIs(Routes.Organization)) {
    await updateInformation();
  }
});

async function updateInformation(): Promise<void> {
  getOrganizationInfo().then((result) => {
    if (result.ok) {
      orgInfo.value = result.value;
      error.value = '';
    } else {
      error.value = 'OrganizationPage.getInfoFailed';
      window.electronAPI.log('error', `Failed to retrieve organization info: ${result.error.tag})`);
    }
  });
  parsecGetClientInfo().then((result) => {
    if (result.ok) {
      error.value = '';
      userInfo.value = result.value;
    } else {
      error.value = 'OrganizationPage.getInfoFailed';
      window.electronAPI.log('error', `Failed to retrieve user info: ${result.error.tag} (${result.error.error})`);
    }
  });
  getCurrentAvailableDevice().then((result) => {
    if (result.ok) {
      isTrialOrg.value = isTrialOrganizationDevice(result.value);
    } else {
      window.electronAPI.log('error', `Failed to retrieve current device: ${result.error.tag}`);
    }
  });
}

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

onMounted(async () => {
  await updateInformation();
});

onUnmounted(() => {
  watchRouteCancel();
});
</script>

<style scoped lang="scss">
.organization-page {
  background: var(--parsec-color-light-secondary-premiere);
  display: flex;
  flex-wrap: wrap;
  justify-content: start;
  gap: 2rem;

  &:has(.organization-page-error) {
    background: var(--parsec-color-light-secondary-background);
  }

  &-content {
    --background: none;

    &::part(scroll) {
      display: flex;
      gap: 2rem;
      padding: 2em;

      @include ms.responsive-breakpoint('xl') {
        flex-wrap: wrap;
      }

      @include ms.responsive-breakpoint('sm') {
        gap: 1.5rem;
        padding: 1.5rem;
        justify-content: center;
        align-items: baseline;
      }
    }
  }

  .organization-sections {
    display: flex;
    gap: 2rem;
    height: fit-content;

    @include ms.responsive-breakpoint('xl') {
      flex-wrap: wrap;
    }

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
      gap: 1.5rem;
      justify-content: center;
    }
  }
}

.organization-header {
  display: flex;
  width: 100%;
  max-width: 30rem;
  height: fit-content;
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

.organization-page-error {
  height: fit-content;
}

.skeleton {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;
  max-width: 30rem;

  &__title {
    width: 100%;
    height: 1.5rem;
    max-width: 10rem;
  }

  .skeleton-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    max-width: 30rem;
    padding: 1.5rem;
    border-radius: var(--parsec-radius-8);
    background: var(--parsec-color-light-secondary-white);

    &__text {
      width: 100%;
      height: 2rem;
    }
  }
}
</style>
