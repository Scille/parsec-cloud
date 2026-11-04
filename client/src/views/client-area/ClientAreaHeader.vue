<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-header id="connected-header">
    <ion-toolbar class="topbar">
      <div class="topbar-left">
        <!-- icon visible when menu is hidden -->
        <ms-image
          v-if="isLargeDisplay"
          slot="start"
          id="trigger-toggle-menu-button"
          class="topbar-button__item"
          @click="isSidebarMenuVisible ? hideSidebarMenu() : resetSidebarMenu()"
          :image="SidebarToggle"
        />
        <ion-menu-toggle v-else>
          <ion-button
            fill="clear"
            class="toggle-menu-button"
          >
            <ms-image
              slot="start"
              id="trigger-toggle-menu-button"
              class="topbar-button__item"
              :image="SidebarToggle"
            />
          </ion-button>
        </ion-menu-toggle>
        <div class="topbar-left-text">
          <ion-text class="topbar-left-text__title">{{ $msTranslate(title) }}</ion-text>
        </div>
      </div>
      <div class="topbar-right">
        <!-- settings -->
        <ion-button
          fill="outline"
          class="topbar-settings color-secondary"
          @click="openSettingsModal"
        >
          <ion-icon
            :icon="cog"
            class="button-icon-left"
          />
          {{ $msTranslate('clientArea.header.settings') }}
        </ion-button>

        <!-- profile -->
        <div
          class="profile-header"
          @click="goToPageClicked(ClientAreaPages.PersonalData)"
        >
          <div class="avatar small">
            <ion-icon
              :icon="personCircle"
              class="avatar-icon"
            />
          </div>
          <div class="text-content">
            <ion-text class="text-content__name">{{ getUserName() }}</ion-text>
          </div>
        </div>
      </div>
    </ion-toolbar>
    <div
      class="current-organization"
      v-if="currentOrganization.name && isSmallDisplay"
    >
      <ion-text class="current-organization__title">{{ $msTranslate('clientArea.header.organization') }}</ion-text>
      <ion-text
        class="current-organization__name"
        @click="openOrganizationChoice($event)"
      >
        {{ currentOrganization.name }}
        <ion-icon :icon="sync" />
      </ion-text>
    </div>
  </ion-header>
</template>

<script setup lang="ts">
import OrganizationSwitchClientPopover from '@/components/organizations/OrganizationSwitchClientPopover.vue';
import { BmsAccessInstance, BmsOrganization, PersonalInformationResultData } from '@/services/bms';
import useSidebarMenu from '@/services/sidebarMenu';
import { ClientAreaPages } from '@/views/client-area/types';
import { openSettingsModal } from '@/views/settings';
import { IonButton, IonHeader, IonIcon, IonMenuToggle, IonText, IonToolbar, popoverController } from '@ionic/vue';
import { cog, personCircle, sync } from 'ionicons/icons';
import { MsImage, MsModalResult, SidebarToggle, Translatable, useWindowSize } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  title: Translatable;
  organizations: Array<BmsOrganization>;
  currentOrganization: BmsOrganization;
}>();

const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu, hide: hideSidebarMenu } = useSidebarMenu();
const { isLargeDisplay, isSmallDisplay } = useWindowSize();

const personalInformation = ref<PersonalInformationResultData | null>(null);

onMounted(async () => {
  if (BmsAccessInstance.get().isLoggedIn()) {
    personalInformation.value = await BmsAccessInstance.get().getPersonalInformation();
  }
});

const emits = defineEmits<{
  (e: 'pageSelected', page: ClientAreaPages): void;
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

async function goToPageClicked(page: ClientAreaPages): Promise<void> {
  emits('pageSelected', page);
}

function getUserName(): string {
  const info = personalInformation.value;
  if (info === null || !info.firstName || !info.lastName) {
    return '';
  }
  return `${info.firstName} ${info.lastName}`;
}

async function openOrganizationChoice(event: Event): Promise<void> {
  if (props.organizations.length === 0) {
    return;
  }
  const popover = await popoverController.create({
    component: OrganizationSwitchClientPopover,
    componentProps: {
      currentOrganization: props.currentOrganization,
      organizations: props.organizations,
    },
    cssClass: 'dropdown-popover',
    id: 'organization-switch-popover',
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  const { data, role } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm && data) {
    emits('organizationSelected', data.organization);
  }
}
</script>

<style scoped lang="scss">
.topbar {
  --background: #{ms.color('surface-base-default')};
  padding: ms.spacing('padding-3xl') ms.spacing('padding-4xl');
  border-bottom: ms.border('thin') solid ms.color('border-base-default');
  justify-content: space-around;

  @include ms.responsive-breakpoint('sm') {
    padding: 0.875rem ms.spacing('padding-4xl') 0.875rem;
  }

  &::part(content) {
    display: flex;
  }

  &-left {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-3xl');
    overflow: hidden;
    width: 100%;

    &-text {
      display: flex;
      flex-direction: column;
      overflow: hidden;
      gap: ms.spacing('gap-lg');

      &__title {
        @include ms.font('heading-h3');
        background: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
      }

      &__organization {
        color: ms.color('text-base-description');
        font-size: 0.875rem;
        margin-top: -0.25rem;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }

    #trigger-toggle-menu-button {
      --fill-color: #{ms.color('icon-neutral-default')};
      padding: ms.spacing('padding-xl');
      border-radius: ms.radius('2xl');
      cursor: pointer;

      &:hover {
        background: ms.color('surface-neutral-default-subtle-hover');
        --fill-color: #{ms.color('icon-neutral-default-hover')};
      }
    }

    .toggle-menu-button::part(native) {
      padding: ms.spacing('padding-none');
      border-radius: ms.radius('2xl');
    }
  }

  &-right {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-3xl');
  }
}

.current-organization {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-lg');
  padding: ms.spacing('padding-lg') 2.25rem;
  background: ms.color('surface-base-page');
  border-bottom: ms.border('thin') solid ms.color('border-base-default');
  box-shadow: ms.shadow('light');

  &__title {
    @include ms.font('heading-h6');
    color: ms.color('text-base-description');
  }

  &__name {
    @include ms.font('heading-h6');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-sm');
    color: ms.color('text-base-body');
    cursor: pointer;

    &:hover {
      color: ms.color('text-brand-default-hover');
    }
  }
}

.profile-header {
  --background: none;
  --background-hover: none;
  border-radius: ms.radius('2xl');
  padding: ms.spacing('padding-md') ms.spacing('padding-lg') ms.spacing('padding-md') ms.spacing('padding-sm');
  border: ms.border('thin') solid transparent;
  transition: all ease-in-out 200ms;
  flex-shrink: 0;
  max-width: 15rem;
  display: flex;
  align-items: center;
  cursor: pointer;

  & * {
    pointer-events: none;
  }

  &:hover {
    --background-hover: none;
    border-color: ms.color('border-base-default-hover');
    background: ms.color('surface-neutral-default-subtle-hover');
  }

  &-clicked {
    border-color: ms.color('border-base-default');
    background: ms.color('surface-base-page');
  }

  .avatar {
    position: relative;
    color: ms.color('text-brand-default');
    display: flex;
    margin-right: 0.375rem;

    @include ms.responsive-breakpoint('md') {
      margin: 0;
    }

    &-icon {
      font-size: 2rem;
    }
  }

  .text-content {
    display: flex;
    width: 100%;
    overflow: hidden;

    &__name {
      @include ms.font('body-lg-medium');
      color: ms.color('text-neutral-default');
      font-size: 0.9375rem;
      width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;

      @include ms.responsive-breakpoint('md') {
        display: none;
      }
    }
  }
}
</style>
