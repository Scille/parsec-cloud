<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="list-item user-list-item"
    lines="full"
    :detail="false"
    :class="{
      selected: user.isSelected && !user.isRevoked(),
      revoked: user.isRevoked(),
      'current-user': user.isCurrent,
      'no-padding-end': !user.isSelected,
      'user-list-item--hovered': !user.isSelected && (menuOpened || isHovered),
    }"
    @click="allowSelection && $emit('select', user, !user.isSelected)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div class="list-item-column user-selected">
      <ms-checkbox
        :class="{ 'checkbox-mobile': isSmallDisplay }"
        :checked="user.isSelected"
        v-show="allowSelection && (user.isSelected || isHovered || activeCheckbox)"
        v-if="!user.isRevoked() && !user.isCurrent"
        @change="allowSelection && $emit('select', user, $event)"
        @click.stop
      />
      <ms-image
        v-if="user.isCurrent"
        :image="LockClosedIcon"
        class="lock-icon"
      />
    </div>

    <!-- user name -->
    <div
      class="list-item-column user-name"
      v-if="isLargeDisplay"
    >
      <ion-text class="list-item-label label-name cell">
        <user-avatar-name
          :user-avatar="user.humanHandle.label"
          :user-name="user.humanHandle.label"
        />
        <span
          v-if="user.isCurrent"
          class="user-name--self"
        >
          {{ $msTranslate('UsersPage.currentUser') }}
        </span>
      </ion-text>
    </div>
    <div
      class="list-item-column user-mobile"
      v-if="isSmallDisplay"
    >
      <user-avatar-name
        :user-avatar="user.humanHandle.label"
        class="user-mobile-avatar"
        :class="{
          'hide-avatar': activeCheckbox || (isHovered && allowSelection),
          'disable-avatar': user.isRevoked() || user.isCurrent,
        }"
      />
      <div class="user-mobile-text">
        <ion-text class="user-mobile-text__title">
          <span class="user-mobile-text__name">{{ user.humanHandle.label }}</span>
          <span
            v-if="user.isCurrent"
            class="user-mobile-text__name--self"
          >
            {{ $msTranslate('UsersPage.currentUser') }}
          </span>
        </ion-text>
        <ion-text class="user-mobile-text__email">
          {{ user.humanHandle.email }}
        </ion-text>
        <div class="user-mobile-text-info">
          <user-profile-tag
            :profile="user.currentProfile"
            class="user-mobile-text-info__profile"
          />
          <user-status-tag
            :revoked="user.isRevoked()"
            :frozen="user.isFrozen()"
            :show-tooltip="true"
            v-if="!user.isActive()"
            class="user-mobile-text-info__status"
          />
        </div>
      </div>
    </div>

    <!-- user profile -->
    <div class="list-item-column user-profile">
      <user-profile-tag :profile="user.currentProfile" />
    </div>

    <!-- user mail -->
    <div class="list-item-column user-email">
      <ion-text
        class="list-item-label label-email cell"
        :title="user.humanHandle.email"
        @click.stop="onCopyEmailClicked(user.humanHandle.email)"
      >
        <span class="label-email__text">{{ user.humanHandle.email }}</span>
        <ion-icon
          v-if="!user.isCurrent"
          :icon="emailCopied ? checkmark : copy"
          class="email-copy-icon"
          :class="{ 'email-copy-icon--copied': emailCopied }"
        />
      </ion-text>
    </div>

    <!-- user joined on -->
    <div class="list-item-column user-join">
      <ion-text class="list-item-label label-join-date cell">
        {{ $msTranslate(formatTimeSince(user.createdOn, '--', 'short')) }}
      </ion-text>
    </div>

    <!-- user status -->
    <div
      class="list-item-column user-status"
      :class="user.isRevoked() ? 'user-revoked' : ''"
    >
      <user-status-tag
        :revoked="user.isRevoked()"
        :frozen="user.isFrozen()"
        :show-tooltip="true"
      />
    </div>

    <!-- options -->
    <div class="list-item-end user-options ion-item-child-clickable">
      <ion-button
        fill="clear"
        slot="icon-only"
        v-show="(isHovered || menuOpened || isSmallDisplay) && !user.isCurrent"
        class="options-button"
        @click.stop="onOptionsClick($event)"
      >
        <ion-icon
          slot="icon-only"
          :icon="ellipsisHorizontal"
          class="options-button__icon"
        />
      </ion-button>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { copyToClipboard } from '@/common/clipboard';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import UserProfileTag from '@/components/users/UserProfileTag.vue';
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { UserModel } from '@/components/users/types';
import { InformationManager, InformationManagerKey } from '@/services/informationManager';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { checkmark, copy, ellipsisHorizontal } from 'ionicons/icons';
import { formatTimeSince, LockClosedIcon, MsCheckbox, MsImage, useWindowSize } from 'megashark-lib';
import { inject, ref, Ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const emailCopied = ref(false);
const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;

const props = defineProps<{
  user: UserModel;
  activeCheckbox: boolean;
  allowSelection: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, user: UserModel): void;
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
  (e: 'select', user: UserModel, selected: boolean): void;
}>();

defineExpose({
  isHovered,
});

async function onCopyEmailClicked(email: string): Promise<void> {
  emailCopied.value = true;

  setTimeout(() => {
    emailCopied.value = false;
  }, 5000);

  await copyToClipboard(email, informationManager.value, 'UsersPage.copyEmail.success', 'UsersPage.copyEmail.failed');
}

async function onOptionsClick(event: Event): Promise<void> {
  event.preventDefault();
  event.stopPropagation();
  menuOpened.value = true;
  emits('menuClick', event, props.user, () => {
    menuOpened.value = false;
  });
}
</script>

<style scoped lang="scss">
.user-selected .checkbox {
  @include ms.responsive-breakpoint('sm') {
    max-width: 100%;
    width: 100%;
  }
}

.user-name {
  .label-name {
    @include ms.font('label-md-medium');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
  }

  &--self {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
  }
}

.user-mobile {
  display: flex;
  align-items: center;
  width: 100%;
  gap: ms.spacing('gap-2xl');
  padding: ms.spacing('padding-2xl') ms.spacing('padding-lg');

  &-avatar {
    pointer-events: none;
    padding: ms.spacing('padding-lg');

    &.hide-avatar {
      opacity: 0;
    }

    &.disable-avatar {
      filter: grayscale(100%);
      opacity: ms.opacity('6');
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-md');
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    &__title {
      @include ms.font('label-md-medium');
    }

    &__name {
      color: ms.color('text-base-body');
    }

    &__name--self {
      @include ms.font('body-sm-regular');
    }

    &__email {
      @include ms.font('label-md-medium');
      color: ms.color('text-base-description');
    }

    &-info {
      margin-top: 0.25rem;
      display: flex;
      gap: ms.spacing('gap-lg');
    }
  }
}

// manage copy email icon visibility on hover
.user-list-item {
  .label-join-date {
    @include ms.font('label-md-medium');
  }

  .user-email {
    .label-email {
      @include ms.font('label-md-medium');
      cursor: pointer;
      display: flex;
      align-items: center;
      flex-wrap: nowrap;
      width: 100%;
      gap: ms.spacing('gap-xs');

      &__text {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }

    .email-copy-icon {
      display: none;
      padding: ms.spacing('padding-md');
      border-radius: ms.radius('md');
      font-size: 1rem;
      cursor: pointer;
      color: ms.color('icon-neutral-default');
      flex-shrink: 0;

      &--copied {
        color: ms.color('icon-success-default') !important;
        display: flex !important;
      }
    }
  }

  &.user-list-item--hovered {
    .email-copy-icon {
      display: flex;
    }

    .label-email:hover {
      color: ms.color('text-brand-default-hover');

      .email-copy-icon {
        color: ms.color('icon-brand-default-hover');
      }
    }
  }
}

.current-user {
  .user-mobile-avatar {
    visibility: hidden;
  }

  .lock-icon {
    z-index: 2;
    width: 1.25rem;
    --fill-color: #{ms.color('icon-neutral-default')};
  }
}
</style>
