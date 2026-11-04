<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="workspace-list-item no-padding-end"
    :detail="false"
    :class="{
      'workspace-list-item--hovered': isHovered || menuOpened,
      'workspace-list-item--hidden': isHidden || workspace.isArchived,
    }"
    @click="$emit('click', workspace, $event)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu.stop="onOptionsClick"
  >
    <div class="workspace-list-item-content">
      <div
        v-show="!workspace.isArchived"
        class="list-item-column workspace-favorite-icon"
        :class="{
          'workspace-favorite-icon__on': isFavorite,
          'workspace-favorite-icon__off': !isFavorite,
        }"
        @click.stop="$emit('favoriteClick', workspace, $event)"
      >
        <ion-icon :icon="star" />
      </div>
      <ion-icon
        v-if="!workspace.isArchived"
        class="cloud-overlay"
        :class="workspace.availableOffline ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
        :icon="workspace.availableOffline ? cloudDone : cloudOffline"
      />
      <!-- workspace name -->
      <div
        class="list-item-column workspace-name"
        :title="workspace.name"
      >
        <ion-text class="workspace-name__label">
          {{ workspace.name }}
        </ion-text>
      </div>
      <ion-icon
        v-if="workspace.isArchived"
        class="list-item-column workspace-archive"
        :icon="archive"
      />
      <div
        class="list-item-column workspace-hidden"
        v-if="isHidden && !workspace.isArchived"
      >
        <ion-icon
          class="workspace-hidden__icon"
          :icon="eyeOff"
        />
        <ion-text class="workspace-hidden__text">{{ $msTranslate('WorkspacesPage.Workspace.hidden') }}</ion-text>
      </div>

      <!-- role user -->
      <div class="list-item-column workspace-role">
        <workspace-role-tag
          v-if="!workspace.isArchived && windowWidth > WindowSizeBreakpoints.XS"
          :role="workspace.selfRole"
          class="workspace-role-tag"
        />
        <ion-label
          v-if="workspace.isArchived"
          class="archived-label"
        >
          <ion-text class="archived-label-text">
            {{ $msTranslate('WorkspacesPage.archiveWorkspace.readOnly') }}
          </ion-text>
        </ion-label>
        <workspace-missing-ownership
          class="missing-ownership"
          v-if="workspace.canSelfPromoteToOwner"
          :show-text="isLargeDisplay && workspace.canSelfPromoteToOwner"
          @click.stop="$emit('selfPromoteClick', workspace, $event)"
        />
      </div>

      <!-- user avatars -->
      <div
        class="list-item-column workspace-users"
        v-show="clientProfile !== UserProfile.Outsider"
        v-if="isLargeDisplay && windowWidth >= WindowSizeBreakpoints.MD && !workspace.isArchived"
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
        class="list-item-column workspace-last-update"
        v-show="workspace.isArchived"
      >
        <ion-label class="list-item-label label-last-update cell">
          {{ $msTranslate(formatTimeSince(workspace.lastUpdated, '--', 'short')) }}
        </ion-label>
      </div>

      <!-- workspace size -->
      <div
        class="list-item-column workspace-size"
        v-show="false"
      >
        <ion-label class="label-size cell">
          {{ $msTranslate(formatFileSize(workspace.size)) }}
        </ion-label>
      </div>

      <!-- options -->
      <div class="list-item-end workspace-options">
        <ion-button
          slot="icon-only"
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
          slot="icon-only"
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
import WorkspaceMissingOwnership from '@/components/workspaces/WorkspaceMissingOwnership.vue';
import WorkspaceRoleTag from '@/components/workspaces/WorkspaceRoleTag.vue';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonLabel, IonText } from '@ionic/vue';
import { archive, cloudDone, cloudOffline, ellipsisHorizontal, eyeOff, shareSocial, star } from 'ionicons/icons';
import { formatTimeSince, useWindowSize, WindowSizeBreakpoints } from 'megashark-lib';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const { isLargeDisplay, windowWidth } = useWindowSize();

const props = defineProps<{
  workspace: WorkspaceInfo;
  clientProfile: UserProfile;
  isFavorite: boolean;
  isHidden: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'favoriteClick', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'menuClick', workspace: WorkspaceInfo, event: Event, onFinished: () => void): void;
  (e: 'shareClick', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'selfPromoteClick', workspace: WorkspaceInfo, event?: Event): void;
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
  border-radius: ms.radius('2xl');
  border: ms.border('thin') solid ms.color('border-base-default');
  --background: #{ms.color('surface-base-default')};
  width: 100%;
  overflow: hidden;
  transition: all 0.15s ease-in-out;

  &::part(native) {
    cursor: pointer !important;
    width: -webkit-fill-available;
    padding-left: 0;
    margin: ms.spacing('padding-sm');
  }

  &-content {
    display: flex;
    background: ms.color('surface-elevated-subtle');
    border-radius: ms.radius('lg');
    align-items: center;
    width: 100%;
  }

  .list-item-column {
    cursor: pointer !important;
  }

  &:hover {
    box-shadow: ms.shadow('input');

    .workspace-list-item-content {
      background: ms.color('surface-brand-default-subtle');
    }
  }
}

.workspace-favorite-icon {
  display: flex;
  align-items: center;
  font-size: 1.25rem;
  padding: ms.spacing('padding-sm');
  margin-left: ms.spacing('padding-lg');
  border-radius: ms.radius('md');
  transition: color 150ms ease-out;
  flex-shrink: 0;
  position: relative;
  z-index: 20;

  &__on {
    color: ms.color('icon-brand-default');

    &:hover {
      background: ms.color('surface-brand-default-subtle-hover');
      color: ms.color('icon-brand-default-hover');
    }
  }

  &__off {
    color: ms.color('icon-disabled-default');

    &:hover {
      background: ms.color('surface-brand-default-subtle-hover');
      color: ms.color('icon-brand-default-hover');
    }
  }
}

.cloud-overlay {
  position: sticky;
  margin-left: ms.spacing('padding-2xl');
  left: 0;
  display: flex;
  align-items: center;
  right: 0;
  font-size: 1rem;
  bottom: 1px;
  padding: 2px;
  background: ms.color('surface-base-default');
  border-radius: ms.radius('full');
  flex-shrink: 0;

  &-ok {
    color: ms.color('icon-brand-default');
  }

  &-ko {
    color: ms.color('icon-neutral-default');
  }
}

.workspace-archive {
  font-size: 7rem;
  color: ms.color('icon-disabled-default');
  z-index: 0;
}

.workspace-name {
  padding-inline: ms.spacing('padding-lg') ms.spacing('padding-3xl');
  width: 100%;
  height: 100%;
  min-width: 15rem;
  max-width: 45vw;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;

  @include ms.responsive-breakpoint('lg') {
    min-width: 10rem;
  }

  @include ms.responsive-breakpoint('sm') {
    min-width: 5rem;
  }

  &__label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: left;
    @include ms.font('heading-h5');
    color: ms.color('text-base-label');
    min-width: 0;
  }
  z-index: 1;
}

.workspace-hidden {
  background: ms.color('surface-base-default');
  color: ms.color('text-base-label');
  text-align: left;
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-md');
  padding: 3px ms.spacing('padding-lg');
  border-radius: ms.radius('lg');
  flex-shrink: 0;
  width: 5rem;

  &__icon {
    font-size: 0.875rem;
    flex-shrink: 0;
  }

  &__text {
    @include ms.font('body-md-medium');
    @include ms.responsive-breakpoint('xs') {
      display: none;
    }
  }
}

.workspace-role {
  min-width: 8rem;
  width: 100%;
  height: 100%;
  max-width: 15vw;
  flex-grow: 2;
  position: relative;
  z-index: 10;
  gap: ms.spacing('gap-lg');

  @include ms.responsive-breakpoint('lg') {
    min-width: 13rem;
    max-width: 11rem;
  }

  @include ms.responsive-breakpoint('md') {
    min-width: 11rem;
    max-width: 11rem;
  }

  @include ms.responsive-breakpoint('sm') {
    padding-inline: ms.spacing('padding-xs');
    min-width: 5rem;
    max-width: 8rem;
  }

  .archived-label {
    color: ms.color('text-base-label');
    display: flex;
    align-items: center;
    border: ms.border('thin') solid ms.color('border-base-default');
    border-radius: ms.radius('4xl');
    background: ms.color('surface-base-default');

    &-text {
      background-color: none;
      padding: 0.2rem ms.spacing('padding-lg');
      color: ms.color('text-brand-default');
    }
  }

  .missing-ownership {
    flex-shrink: 0;
    left: 0;
    height: fit-content;
  }

  &:has(.missing-ownership) {
    .workspace-role-tag {
      display: none;

      @include ms.responsive-breakpoint('sm') {
        display: flex;
      }
    }
  }
}

.workspace-users {
  min-width: 10rem;
  max-width: 15rem;
  flex-grow: 0;
  overflow: visible;

  .shared-group {
    padding: ms.spacing('padding-sm');
    transition: background 0.2s ease-in-out;

    &:hover {
      background: ms.color('surface-brand-default-subtle-hover');
    }
  }

  .not-shared-label {
    color: ms.color('text-base-description');
    padding: ms.spacing('padding-md') ms.spacing('padding-lg');
    border-radius: ms.radius('md');
    transition: background 0.15s ease-in-out;
    flex-grow: 0 !important;
    position: relative;
    z-index: 2;

    &:hover {
      background: ms.color('surface-brand-default-subtle-hover');
      color: ms.color('text-brand-default-hover') !important;
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
  margin-right: 1rem;
}

.workspace-list-item--hidden {
  border: ms.border('thin') solid ms.color('border-base-default-hover');

  .workspace-list-item-content {
    opacity: ms.opacity('9');
    filter: brightness(0.85);
  }

  &:hover {
    .workspace-list-item-content {
      background: ms.color('surface-base-page-secondary');
    }
  }

  .workspace-name {
    max-width: calc(45vw - 5rem);
  }
}
</style>
