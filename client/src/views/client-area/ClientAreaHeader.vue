<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
          <ion-text class="topbar-left-text__title title-h2">{{ $msTranslate(title) }}</ion-text>
        </div>
      </div>
      <div class="topbar-right">
        <!-- settings -->
        <ion-text
          class="button-medium custom-button custom-button-fill"
          button
          @click="openSettingsModal"
        >
          <ion-icon :icon="cog" />
          {{ $msTranslate('clientArea.header.settings') }}
        </ion-text>

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
            <ion-text class="text-content__name subtitles-normal">{{ getUserName() }}</ion-text>
          </div>
        </div>
      </div>
    </ion-toolbar>
    <div
      class="current-organization"
      v-if="currentOrganization.name && isSmallDisplay"
    >
      <ion-text class="current-organization__title title-h5">{{ $msTranslate('clientArea.header.organization') }}</ion-text>
      <ion-text
        class="current-organization__name title-h5"
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
  --background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem 2rem 1rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  justify-content: space-around;

  @include ms.responsive-breakpoint('sm') {
    padding: 0.875rem 1.5rem 0.875rem;
  }

  &::part(content) {
    display: flex;
  }

  &-left {
    display: flex;
    align-items: center;
    gap: 1rem;
    overflow: hidden;
    width: 100%;

    &-text {
      display: flex;
      flex-direction: column;
      overflow: hidden;
      gap: 0.5rem;

      &__title {
        background: var(--parsec-color-light-gradient-background);
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
      }

      &__organization {
        color: var(--parsec-color-light-secondary-grey);
        font-size: 0.875rem;
        margin-top: -0.25rem;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }

    #trigger-toggle-menu-button {
      --fill-color: var(--parsec-color-light-secondary-grey);
      padding: 0.625rem;
      border-radius: var(--parsec-radius-12);
      cursor: pointer;

      &:hover {
        background: var(--parsec-color-light-secondary-premiere);
        --fill-color: var(--parsec-color-light-secondary-hard-grey);
      }
    }

    .toggle-menu-button::part(native) {
      padding: 0;
      border-radius: var(--parsec-radius-12);
    }
  }

  &-right {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
}

.current-organization {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 2.25rem;
  background: var(--parsec-color-light-secondary-background);
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  box-shadow: var(--parsec-shadow-soft);

  &__title {
    color: var(--parsec-color-light-secondary-grey);
    font-size: 0.875rem;
  }

  &__name {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    color: var(--parsec-color-light-secondary-text);
    cursor: pointer;

    &:hover {
      color: var(--parsec-color-light-primary-600);
    }
  }
}

.profile-header {
  --background: none;
  --background-hover: none;
  border-radius: var(--parsec-radius-12);
  padding: 0.375rem 0.5rem 0.375rem 0.25rem;
  border: 1px solid transparent;
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
    border-color: var(--parsec-color-light-secondary-medium);
    background: var(--parsec-color-light-secondary-background);
  }

  &-clicked {
    border-color: var(--parsec-color-light-secondary-medium);
    background: var(--parsec-color-light-secondary-background);
  }

  .avatar {
    position: relative;
    color: var(--parsec-color-light-primary-600);
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
      color: var(--parsec-color-light-secondary-soft-text);
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
