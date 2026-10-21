<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        <template v-if="orgInfo">
          <ion-avatar class="organization-header-avatar">
            <span v-if="!isTrialOrg">{{ orgInfo.clientInfo.organizationId.substring(0, 2) }}</span>
            <!-- prettier-ignore -->
            <ms-image
              v-else
              :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
              class="avatar-logo"
            />
          </ion-avatar>
          <ion-text class="organization-header-text">{{ orgInfo.clientInfo.organizationId }}</ion-text>
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

      <template v-else>
        <template v-if="!orgInfo && !error">
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

        <template v-if="error">
          <ms-report-text
            :theme="MsReportTheme.Error"
            class="organization-page-error"
          >
            {{ $msTranslate(error) }}
          </ms-report-text>
        </template>
      </template>

      <template v-if="orgInfo">
        <div class="organization-sections">
          <organization-user-information :org-info="orgInfo" />
          <!-- ------------- Information ------------- -->
          <organization-configuration-information
            :org-info="orgInfo"
            :server-config="serverConfig"
          />

          <!-- ------------- Storage list ------------- -->
          <organization-storage-information :org-info="orgInfo" />
        </div>
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
import { OrganizationInfo, ServerConfig, getCurrentAvailableDevice, getOrganizationInfo, getServerConfig } from '@/parsec';
import { Routes, currentRouteIs, switchOrganization, watchRoute } from '@/router';
import useUploadMenu from '@/services/fileUploadMenu';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonAvatar, IonContent, IonPage, IonSkeletonText, IonText, popoverController } from '@ionic/vue';
import { ChevronExpand, LogoIconGradient, MsImage, MsModalResult, MsReportText, MsReportTheme, useWindowSize } from 'megashark-lib';
import { Ref, onMounted, onUnmounted, ref } from 'vue';

const { isSmallDisplay } = useWindowSize();

const orgInfo: Ref<OrganizationInfo | null> = ref(null);
const serverConfig: Ref<ServerConfig | null> = ref(null);
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
    }
  });
  getCurrentAvailableDevice().then(async (result) => {
    if (result.ok) {
      isTrialOrg.value = await isTrialOrganizationDevice(result.value);
      const serverInfo = await getServerConfig(result.value.serverAddr);
      if (serverInfo.ok) {
        serverConfig.value = serverInfo.value;
      }
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
  display: flex;
  flex-wrap: wrap;
  justify-content: start;
  gap: ms.spacing('gap-5xl');

  &-content {
    --background: none;

    &::part(scroll) {
      display: flex;
      gap: ms.spacing('gap-5xl');
      padding: ms.spacing('padding-5xl');

      @include ms.responsive-breakpoint('xl') {
        flex-wrap: wrap;
      }

      @include ms.responsive-breakpoint('sm') {
        gap: ms.spacing('gap-4xl');
        padding: ms.spacing('padding-4xl');
        justify-content: center;
        align-items: baseline;
      }
    }
  }

  .organization-sections {
    display: flex;
    gap: ms.spacing('gap-5xl');
    height: fit-content;

    @include ms.responsive-breakpoint('xl') {
      flex-wrap: wrap;
    }

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
      gap: ms.spacing('gap-4xl');
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
  padding: ms.spacing('padding-2xl');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-default');
  gap: ms.spacing('gap-2xl');

  &-avatar {
    @include ms.font('body-lg-regular');
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 2.5rem;
    width: 2.5rem;
    height: 2.5rem;
    background: ms.color('surface-neutral-default-subtle');
    border-radius: ms.radius('2xl');
    padding: ms.spacing('padding-sm');

    .avatar-logo {
      width: 1.5rem;
    }

    span {
      font-size: 1.25rem;
      font-weight: bold;
      color: ms.color('text-brand-default');
    }
  }

  &-text {
    @include ms.font('heading-h3');
    width: 100%;
    color: ms.color('text-base-body');
  }
}

.organization-page-error {
  height: fit-content;
}

.skeleton {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
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
    gap: ms.spacing('gap-lg');
    width: 100%;
    max-width: 30rem;
    padding: ms.spacing('padding-4xl');
    border-radius: ms.radius('lg');
    background: ms.color('surface-base-default');

    &__text {
      width: 100%;
      height: 2rem;
    }
  }
}
</style>
