<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="user-list-item"
    lines="full"
    :detail="false"
    :class="{
      selected: user.isSelected && !user.isRevoked(),
      revoked: user.isRevoked(),
      'current-user': user.isCurrent,
      'no-padding-end': !user.isSelected,
      'user-hovered': !user.isSelected && (menuOpened || isHovered),
    }"
    @click="$emit('select', user, !user.isSelected)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div class="user-selected">
      <ms-checkbox
        :class="{ 'checkbox-mobile': isSmallDisplay }"
        :checked="user.isSelected"
        v-show="user.isSelected || isHovered || showCheckbox"
        v-if="!user.isRevoked() && !user.isCurrent"
        @change="$emit('select', user, $event)"
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
      class="user-name"
      v-if="isLargeDisplay"
    >
      <ion-text class="user-name__label cell">
        <user-avatar-name
          :user-avatar="user.humanHandle.label"
          :user-name="user.humanHandle.label"
        />
        <span
          v-if="user.isCurrent"
          class="body user-name__you"
        >
          {{ $msTranslate('UsersPage.currentUser') }}
        </span>
      </ion-text>
    </div>
    <div
      class="user-mobile"
      v-if="isSmallDisplay"
    >
      <user-avatar-name
        :user-avatar="user.humanHandle.label"
        class="user-mobile-avatar"
        :class="{
          'hide-avatar': showCheckbox || isHovered,
          'disable-avatar': user.isRevoked() || user.isCurrent,
        }"
      />
      <div class="user-mobile-text">
        <ion-text class="button-medium">
          <span class="user-mobile-text__name">{{ user.humanHandle.label }}</span>
          <span
            v-if="user.isCurrent"
            class="body-sm user-mobile-text__you"
          >
            {{ $msTranslate('UsersPage.currentUser') }}
          </span>
        </ion-text>
        <ion-text class="cell user-mobile-text__email">
          {{ user.humanHandle.email }}
        </ion-text>
        <div class="user-mobile-text__profile-status">
          <user-profile-tag
            :profile="user.currentProfile"
            class="user-mobile-text__profile"
          />
          <user-status-tag
            :revoked="user.isRevoked()"
            :frozen="user.isFrozen()"
            :show-tooltip="true"
            v-if="!user.isActive()"
            class="user-mobile-text__status"
          />
        </div>
      </div>
    </div>

    <!-- user profile -->
    <div class="user-profile">
      <user-profile-tag :profile="user.currentProfile" />
    </div>

    <!-- user mail -->
    <div class="user-email">
      <ion-text
        class="user-email__label cell"
        :title="user.humanHandle.email"
        @click.stop="onCopyEmailClicked(user.humanHandle.email)"
      >
        {{ user.humanHandle.email }}
        <ion-icon
          v-if="!user.isCurrent"
          :icon="emailCopied ? checkmark : copy"
          class="email-copy-icon"
          :class="{ 'email-copy-icon--copied': emailCopied }"
        />
      </ion-text>
    </div>

    <!-- user joined on -->
    <div class="user-join">
      <ion-text class="user-join-label cell">
        {{ $msTranslate(formatTimeSince(user.createdOn, '--', 'short')) }}
      </ion-text>
    </div>

    <!-- user status -->
    <div
      class="user-status"
      :class="user.isRevoked() ? 'user-revoked' : ''"
    >
      <user-status-tag
        :revoked="user.isRevoked()"
        :frozen="user.isFrozen()"
        :show-tooltip="true"
      />
    </div>

    <!-- options -->
    <div class="user-options ion-item-child-clickable">
      <ion-button
        v-show="(isHovered || menuOpened || isSmallDisplay) && !user.isCurrent"
        fill="clear"
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
  showCheckbox: boolean;
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
.user-name {
  &__label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    overflow: hidden;
  }

  &__you {
    color: var(--parsec-color-light-secondary-text);
    font-weight: 700;
  }
}

.user-mobile {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 0.75rem;
  padding: 0.75rem 0.5rem;

  .user-mobile-avatar {
    pointer-events: none;
    padding: 0.5rem;

    &.hide-avatar {
      opacity: 0;
    }

    &.disable-avatar {
      filter: grayscale(100%);
      opacity: 0.6;
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;

    &__name {
      color: var(--parsec-color-light-secondary-text);
    }

    &__you {
      color: var(--parsec-color-light-primary-600);
      margin-left: 0.25rem;
    }

    &__email {
      color: var(--parsec-color-light-secondary-grey);
    }

    &__profile-status {
      display: flex;
      gap: 0.5rem;
    }
  }
}

.user-status,
.user-join,
.user-email {
  color: var(--parsec-color-light-secondary-grey);
}

.user-list-item {
  .user-email__label {
    display: flex;
    align-items: center;
    cursor: pointer;
    gap: 0.125rem;
  }

  .email-copy-icon {
    display: none;
    color: var(--parsec-color-light-secondary-soft-grey);
    padding: 0.375rem;
    border-radius: var(--parsec-radius-6);
    font-size: 1rem;
    cursor: pointer;
    color: var(--parsec-color-light-secondary-grey);
    flex-shrink: 0;

    &--copied {
      color: var(--parsec-color-light-success-700) !important;
      display: flex !important;
    }
  }

  &.user-hovered {
    .email-copy-icon {
      display: flex;
    }
    .user-email__label:hover {
      color: var(--parsec-color-light-primary-600);

      .email-copy-icon {
        color: var(--parsec-color-light-primary-600);
      }
    }
  }

  &.current-user {
    --background: var(--parsec-color-light-secondary-background);

    .lock-icon {
      width: 1.25rem;
      --fill-color: var(--parsec-color-light-secondary-grey);
    }
  }
}
</style>
