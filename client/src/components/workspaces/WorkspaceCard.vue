<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div
    class="workspace-card-item ion-no-padding"
    :class="{
      'workspace-card-item--hovered': isHovered || menuOpened,
      'workspace-card-item--hidden': isHidden,
      'workspace-card-item--archived': workspace.isArchived,
      'workspace-card-item--trashed': workspace.isTrashed,
    }"
    @click="$emit('click', workspace, $event)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu.stop="onOptionsClick"
  >
    <div class="workspace-card-content">
      <div
        v-show="!workspace.isArchived && !workspace.isTrashed"
        class="workspace-favorite-icon"
        :class="{
          'workspace-favorite-icon__on': isFavorite,
          'workspace-favorite-icon__off': !isFavorite,
        }"
        @click.stop="$emit('favoriteClick', workspace, $event)"
      >
        <ion-icon :icon="star" />
      </div>

      <ion-text
        class="workspace-card-content__title"
        :title="workspace.name"
      >
        {{ workspace.name }}
      </ion-text>

      <div class="workspace-card-content-info">
        <ion-text
          class="workspace-card-content__update"
          v-if="!workspace.isArchived && !workspace.isTrashed"
        >
          <span v-if="false">{{ $msTranslate(formatTimeSince(workspace.lastUpdated, '--', 'short')) }}</span>
          <ion-icon
            class="cloud-overlay"
            :class="workspace.availableOffline ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
            :icon="workspace.availableOffline ? cloudDone : cloudOffline"
          />
        </ion-text>
        <ion-text
          class="workspace-card-content__update"
          v-else
        >
          <span v-if="workspace.isArchived && workspace.archivedOn">{{
            $msTranslate({
              key: 'WorkspacesPage.archiveWorkspace.timestamp',
              data: { timestamp: $msTranslate(I18n.formatDate(workspace.archivedOn, 'full')) },
            })
          }}</span>
          <span v-if="workspace.isTrashed && workspace.deletionDate">{{ $msTranslate('WorkspacesPage.trashWorkspace.timestamp') }}</span>
          <span
            v-if="workspace.isTrashed && workspace.deletionDate"
            class="deletion-date"
          >
            {{ $msTranslate(I18n.formatDate(workspace.deletionDate, 'full')) }}
          </span>
        </ion-text>
        <ion-text
          class="workspace-card-content__size"
          v-if="false"
        >
          {{ $msTranslate(formatFileSize(workspace.size)) }}
        </ion-text>

        <div
          class="workspace-hidden"
          v-if="isHidden && !workspace.isArchived && !workspace.isTrashed"
        >
          <ion-icon
            class="cloud-overlay"
            :icon="eyeOff"
          />
          <ion-text>{{ $msTranslate('WorkspacesPage.Workspace.hidden') }}</ion-text>
        </div>
        <ion-icon
          v-if="workspace.isArchived"
          class="custom-icon"
          :icon="archive"
        />
        <ion-icon
          v-if="workspace.isTrashed"
          class="custom-icon"
          :icon="trash"
        />
      </div>
    </div>
    <div class="workspace-card-bottom">
      <workspace-missing-ownership
        v-if="workspace.canSelfPromoteToOwner"
        :show-text="true"
        @click.stop="$emit('selfPromoteClick', workspace, $event)"
      />
      <workspace-role-tag
        v-else-if="!workspace.isArchived"
        class="workspace-card-bottom__role"
        :role="workspace.selfRole"
      />
      <ion-text
        v-else
        class="archived-label"
      >
        {{ $msTranslate('WorkspacesPage.archiveWorkspace.readOnly') }}
      </ion-text>
      <div class="workspace-card-bottom__icons">
        <div
          v-show="clientProfile !== UserProfile.Outsider && !workspace.isArchived && !workspace.canSelfPromoteToOwner"
          class="icon-share-container"
          @click.stop="$emit('shareClick', workspace, $event)"
        >
          <ion-icon
            :icon="shareSocial"
            class="icon-share"
          />
        </div>
        <div
          class="icon-option-container"
          @click.stop="onOptionsClick($event)"
        >
          <ion-icon
            :icon="ellipsisHorizontal"
            class="icon-option"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { WorkspaceRoleTag } from '@/components/workspaces';
import WorkspaceMissingOwnership from '@/components/workspaces/WorkspaceMissingOwnership.vue';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonIcon, IonText } from '@ionic/vue';
import { archive, cloudDone, cloudOffline, ellipsisHorizontal, eyeOff, shareSocial, star, trash } from 'ionicons/icons';
import { formatTimeSince, I18n } from 'megashark-lib';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

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
.workspace-card-item {
  cursor: pointer;
  text-align: center;
  user-select: none;
  border-radius: ms.radius('2xl');
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  background: ms.color('surface-base-default');
  box-shadow: ms.shadow('elevation-sm');
  max-width: 16rem;
  width: 100%;
  overflow: hidden;
  padding: ms.spacing('padding-sm');
  transition: all 0.15s ease-in-out;
  min-width: 14rem;

  @include ms.responsive-breakpoint('sm') {
    max-width: 100%;
  }

  @include ms.responsive-breakpoint('xs') {
    max-width: 100%;
  }

  &:hover,
  &--hovered {
    border-color: ms.color('border-neutral-default-subtle-hover');
    box-shadow: ms.shadow('elevation-xs');

    .workspace-card-content {
      background: ms.color('surface-elevated-subtle-hover');
    }

    .workspace-card-bottom__icons {
      opacity: 1;
    }
  }
}

.workspace-favorite-icon {
  display: flex;
  align-items: center;
  font-size: 1rem;
  margin-left: auto;
  padding: ms.spacing('padding-sm');
  border-radius: ms.radius('md');
  transition: color 150ms ease-out;
  position: absolute;
  right: ms.spacing('padding-2xl');
  top: ms.spacing('padding-2xl');
  z-index: 10;

  &__on {
    color: ms.color('icon-brand-default');

    &:hover {
      background: ms.color('surface-brand-default-subtle-hover');
      color: ms.color('icon-brand-default-hover');
    }
  }

  &__off {
    color: ms.color('icon-disabled-default');
    opacity: ms.opacity('disabled');

    &:hover {
      background: ms.color('surface-brand-default-subtle-hover');
      color: ms.color('icon-brand-default-hover');
      opacity: ms.opacity('9');
    }
  }
}

.workspace-card-content {
  transition: all 0.15s ease-in-out;
  padding: ms.spacing('padding-3xl') ms.spacing('padding-3xl');
  width: 100%;
  background: ms.color('surface-elevated-subtle');
  border-radius: ms.radius('lg');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
  overflow: hidden;
  position: relative;

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-base-heading');
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    align-items: center;
    gap: ms.spacing('gap-lg');
    margin-right: 1.125rem;

    ion-text {
      width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  &__update {
    display: flex;
    justify-content: flex-start;
    @include ms.font('body-md-medium');
    color: ms.color('text-base-description');
    gap: ms.spacing('gap-sm');

    .cloud-overlay {
      font-size: 1rem;
      padding: ms.spacing('padding-sm');
      border-radius: ms.radius('full');
      background: ms.color('surface-base-default');
    }

    .cloud-overlay-ok {
      color: ms.color('icon-brand-default');
    }

    .cloud-overlay-ko {
      color: ms.color('icon-neutral-default');
    }
  }

  &-info {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  &__size {
    @include ms.font('body-sm-regular');
    color: ms.color('text-base-description');
  }
}

.workspace-card-bottom {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: ms.spacing('padding-md') ms.spacing('padding-lg') ms.spacing('padding-md') ms.spacing('padding-md');
  position: relative;

  &__icons {
    color: ms.color('icon-neutral-default');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-2xl');
    opacity: 0;
    transition: opacity 0.15s ease-in-out;

    @include ms.responsive-breakpoint('sm') {
      opacity: 1;
    }

    .icon-option-container,
    .icon-share-container {
      display: flex;
      align-items: center;
      padding: ms.spacing('padding-sm');
      border-radius: ms.radius('md');
      transition: background 0.15s ease-in-out;
      gap: ms.spacing('gap-sm');

      .icon-option,
      .icon-share {
        font-size: 1.375rem;

        @include ms.responsive-breakpoint('sm') {
          font-size: 1.5rem;
        }
      }

      &:hover {
        background: ms.color('surface-brand-default-subtle');
        color: ms.color('icon-brand-default');
      }
    }

    @include ms.responsive-breakpoint('sm') {
      padding: 0 ms.spacing('padding-sm');
    }
  }
}

.workspace-card-item--hidden {
  .workspace-hidden {
    @include ms.font('label-md-medium');
    background: ms.color('surface-base-default-secondary');
    color: ms.color('text-base-label');
    text-align: left;
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-md');
    padding: 3px ms.spacing('padding-lg');
    border-radius: ms.radius('lg');

    ion-icon {
      font-size: 0.875rem;
    }
  }
}

.workspace-card-item--archived,
.workspace-card-item--trashed {
  position: relative;

  .custom-icon {
    color: ms.color('icon-neutral-default');
    opacity: ms.opacity('2');
    position: absolute;
    top: ms.spacing('padding-sm');
    right: -1rem;
    margin: 0 0 auto auto;
    font-size: 4.5rem;
  }

  .workspace-card-bottom__icons {
    padding: ms.spacing('padding-none');
    opacity: 1;
  }

  .workspace-card-content {
    box-shadow: ms.shadow('elevation-xs');
  }
}

.workspace-card-item--archived {
  .archived-label {
    @include ms.font('label-md-medium');
    display: flex;
    align-items: center;
    background: ms.color('surface-neutral-default-subtle');
    border-radius: ms.radius('lg');
    padding: 0.1875rem ms.spacing('padding-md');
    color: ms.color('text-information-default');
  }
}

.workspace-card-item--trashed {
  .workspace-card-content__update {
    position: relative;
    text-align: start;
    z-index: 10;
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');

    .deletion-date {
      @include ms.font('label-md-medium');
      width: fit-content;
      background: ms.color('surface-error-default-subtle');
      color: ms.color('text-error-default');
      border-radius: ms.radius('md');
      padding: 0.1875rem ms.spacing('padding-md');
      box-shadow: ms.shadow('elevation-xs');
    }
  }
}
</style>
