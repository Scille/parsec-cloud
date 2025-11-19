<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="workspace-list-item no-padding-end"
    :detail="false"
    :class="{ 'workspace-hovered': isHovered || menuOpened }"
    @click="$emit('click', workspace, $event)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div class="workspace-list-item-content">
      <div
        class="workspace-favorite-icon"
        :class="{
          'workspace-favorite-icon__on': isFavorite,
          'workspace-favorite-icon__off': !isFavorite,
        }"
        @click.stop="$emit('favoriteClick', workspace, $event)"
      >
        <ion-icon :icon="star" />
      </div>
      <ion-icon
        class="cloud-overlay"
        :class="workspace.availableOffline ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
        :icon="workspace.availableOffline ? cloudDone : cloudOffline"
      />
      <!-- workspace name -->
      <div
        class="workspace-name"
        :title="workspace.currentName"
      >
        <ion-text class="workspace-name__label title-h4">
          {{ workspace.currentName }}
        </ion-text>
      </div>

      <!-- role user -->
      <div class="workspace-role">
        <workspace-role-tag
          :role="workspace.currentSelfRole"
          class="workspace-role-tag"
        />
      </div>

      <!-- user avatars -->
      <div
        class="workspace-users"
        v-show="clientProfile !== UserProfile.Outsider"
        v-if="isLargeDisplay && windowWidth >= WindowSizeBreakpoints.MD"
      >
        <avatar-group
          v-show="workspace.sharing.length > 0"
          class="shared-group"
          :people="workspace.sharing.map((item) => item[0].humanHandle.label)"
          :max-display="2"
          @click.stop="$emit('shareClick', workspace, $event)"
        />
        <ion-label
          class="not-shared-label cell"
          v-show="workspace.sharing.length === 0"
          @click.stop="$emit('shareClick', workspace, $event)"
        >
          {{ $msTranslate('WorkspacesPage.Workspace.notShared') }}
        </ion-label>
      </div>

      <!-- last update -->
      <div
        class="workspace-last-update"
        v-show="false"
      >
        <ion-label class="label-last-update cell">
          {{ $msTranslate(formatTimeSince(workspace.lastUpdated, '--', 'short')) }}
        </ion-label>
      </div>

      <!-- workspace size -->
      <div
        class="workspace-size"
        v-show="false"
      >
        <ion-label class="label-size cell">
          {{ $msTranslate(formatFileSize(workspace.size)) }}
        </ion-label>
      </div>

      <!-- options -->
      <div class="workspace-options">
        <ion-button
          fill="clear"
          class="options-button"
          @click.stop="$emit('shareClick', workspace, $event)"
          v-if="windowWidth < WindowSizeBreakpoints.MD"
        >
          <ion-icon
            :icon="shareSocial"
            slot="icon-only"
            class="options-button__icon"
          />
        </ion-button>
        <ion-button
          fill="clear"
          class="options-button"
          @click.stop="onOptionsClick($event)"
        >
          <ion-icon
            :icon="ellipsisHorizontal"
            slot="icon-only"
            class="options-button__icon"
          />
        </ion-button>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import WorkspaceRoleTag from '@/components/workspaces/WorkspaceRoleTag.vue';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonLabel, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, ellipsisHorizontal, shareSocial, star } from 'ionicons/icons';
import { formatTimeSince, useWindowSize, WindowSizeBreakpoints } from 'megashark-lib';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const { isLargeDisplay, windowWidth } = useWindowSize();

const props = defineProps<{
  workspace: WorkspaceInfo;
  clientProfile: UserProfile;
  isFavorite: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'favoriteClick', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'menuClick', workspace: WorkspaceInfo, event: Event, onFinished: () => void): void;
  (e: 'shareClick', workspace: WorkspaceInfo, event?: Event): void;
}>();

async function onOptionsClick(event: Event): Promise<void> {
  event.preventDefault();
  menuOpened.value = true;
  emits('menuClick', props.workspace, event, () => {
    menuOpened.value = false;
  });
}
</script>

<style lang="scss" scoped>
.workspace-list-item {
  cursor: pointer;
  text-align: center;
  user-select: none;
  border-radius: var(--parsec-radius-12);
  border: 1px solid var(--parsec-color-light-secondary-disabled);
  --background: var(--parsec-color-light-secondary-white);
  width: 100%;
  overflow: hidden;
  transition: all 0.15s ease-in-out;

  &::part(native) {
    width: -webkit-fill-available;
    padding-left: 0;
    margin: 0.25rem;
    --background-hover: var(--parsec-color-light-secondary-white);
  }

  &-content {
    display: flex;
    background: var(--parsec-color-light-secondary-background);
    border-radius: var(--parsec-radius-8);
    align-items: center;
    width: 100%;
    height: 3rem;
  }

  &:hover {
    box-shadow: var(--parsec-shadow-input);

    .workspace-list-item-content {
      background: var(--parsec-color-light-primary-30);
    }
  }
}

.workspace-favorite-icon {
  display: flex;
  align-items: center;
  font-size: 1.25rem;
  padding: 0.25rem;
  margin-left: 0.5rem;
  border-radius: var(--parsec-radius-6);
  transition: color 150ms ease-out;
  flex-shrink: 0;

  &__on {
    color: var(--parsec-color-light-primary-600);

    &:hover {
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-700);
    }
  }

  &__off {
    color: var(--parsec-color-light-secondary-disabled);

    &:hover {
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-600);
    }
  }
}

.cloud-overlay {
  position: sticky;
  margin-left: 0.75rem;
  left: 0;
  display: flex;
  align-items: center;
  right: 0;
  font-size: 1rem;
  bottom: 1px;
  padding: 2px;
  background: var(--parsec-color-light-secondary-white);
  border-radius: 50%;
  flex-shrink: 0;

  &-ok {
    color: var(--parsec-color-light-primary-500);
  }

  &-ko {
    color: var(--parsec-color-light-secondary-text);
  }
}

.workspace-name {
  padding-inline: 0.5rem 1rem;
  width: 100%;
  height: 100%;
  min-width: 15rem;
  max-width: 45vw;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;

  @include ms.responsive-breakpoint('sm') {
    min-width: 5rem;
  }

  &__label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: left;
    color: var(--parsec-color-light-secondary-text);
    min-width: 0;
  }
}

.workspace-role {
  min-width: 8rem;
  max-width: 6vw;
  flex-grow: 2;
  position: relative;
  z-index: 10;

  .workspace-role-tag {
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-18);
    background: var(--parsec-color-light-secondary-white);
  }

  @include ms.responsive-breakpoint('md') {
    min-width: 5rem;
    max-width: 8rem;
  }

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 0.125rem;
  }
}

.workspace-users {
  min-width: 10rem;
  max-width: 15rem;
  flex-grow: 0;
  overflow: visible;

  .shared-group {
    padding: 0.25rem;
    transition: background 0.2s ease-in-out;

    &:hover {
      background: var(--parsec-color-light-primary-100);
    }
  }

  .not-shared-label {
    color: var(--parsec-color-light-secondary-grey);
    padding: 0.375rem 0.5rem;
    border-radius: var(--parsec-radius-6);
    transition: background 0.15s ease-in-out;
    flex-grow: 0 !important;
    position: relative;
    z-index: 2;

    &:hover {
      background: var(--parsec-color-light-primary-100);
      color: var(--parsec-color-light-primary-700) !important;
    }
  }
}

.workspace-last-update {
  min-width: 11.25rem;
  flex-grow: 0;
}

.workspace-size {
  max-width: 7.5rem;
}

.workspace-options {
  flex-grow: 0;
  margin-left: auto;
  align-items: center;
  flex-shrink: 0;
  display: flex;
  gap: 0.5rem;
  position: relative;
  z-index: 8;

  ion-button::part(native) {
    padding: 0;
  }

  .options-button {
    --background-hover: none;
    background: none;
    border-radius: var(--parsec-radius-8);
    padding: 0.375rem;

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
      font-size: 1.25rem;
      flex-shrink: 0;
    }

    &:hover {
      background: var(--parsec-color-light-secondary-disabled);

      .options-button__icon {
        color: var(--parsec-color-light-primary-500);
      }
    }
  }
}
</style>
