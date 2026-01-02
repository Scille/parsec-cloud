<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-header id="connected-header">
    <ion-toolbar class="topbar">
      <div class="topbar-left">
        <!-- icon visible when menu is hidden -->
        <ms-image
          v-if="!isMobile() && isLargeDisplay"
          slot="start"
          id="trigger-toggle-menu-button"
          class="topbar-button__item"
          @click="isSidebarMenuVisible ? hideSidebarMenu() : resetSidebarMenu()"
          :image="SidebarToggle"
        />
        <ion-menu-toggle v-if="!isMobile() && isSmallDisplay">
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
        <ion-text class="topbar-left__title title-h2">{{ $msTranslate(title) }}</ion-text>
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
  </ion-header>
</template>

<script setup lang="ts">
import { isMobile } from '@/parsec';
import { BmsAccessInstance, PersonalInformationResultData } from '@/services/bms';
import useSidebarMenu from '@/services/sidebarMenu';
import { ClientAreaPages } from '@/views/client-area/types';
import { openSettingsModal } from '@/views/settings';
import { IonButton, IonHeader, IonIcon, IonMenuToggle, IonText, IonToolbar } from '@ionic/vue';
import { cog, personCircle } from 'ionicons/icons';
import { MsImage, SidebarToggle, Translatable, useWindowSize } from 'megashark-lib';
import { onMounted, ref } from 'vue';

defineProps<{
  title: Translatable;
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
</script>

<style scoped lang="scss">
.topbar {
  --background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem 2rem 1rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  justify-content: space-around;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 1.5rem 1rem;
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

    &__title {
      background: var(--parsec-color-light-gradient-background);
      background-clip: text;
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
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
