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
      'no-padding-end': !user.isSelected,
      'user-hovered': !user.isSelected && (menuOpened || isHovered),
    }"
    @click="$emit('click', $event, user)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div class="user-selected">
      <!-- eslint-disable vue/no-mutating-props -->
      <ms-checkbox
        v-model="user.isSelected"
        v-show="user.isSelected || isHovered || showCheckbox"
        v-if="!user.isRevoked() && !user.isCurrent"
        @click.stop
        @change="$emit('select', user, $event)"
      />
      <!-- eslint-enable vue/no-mutating-props -->
    </div>

    <!-- user name -->
    <div class="user-name">
      <ion-label class="user-name__label cell">
        <user-avatar-name
          class="main-cell"
          :user-avatar="user.humanHandle.label"
          :user-name="user.humanHandle.label"
        />
        <span
          v-if="user.isCurrent"
          class="body user-name__you"
        >
          {{ $msTranslate('UsersPage.currentUser') }}
        </span>
      </ion-label>
    </div>

    <!-- user profile -->
    <div class="user-profile">
      <tag-profile :profile="user.currentProfile" />
    </div>

    <!-- user mail -->
    <div class="user-email">
      <ion-label class="user-email__label cell">
        {{ user.humanHandle.email }}
      </ion-label>
    </div>

    <!-- user joined on -->
    <div class="user-join">
      <ion-label class="user-join-label cell">
        {{ $msTranslate(formatTimeSince(user.createdOn, '--', 'short')) }}
      </ion-label>
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
        v-show="(isHovered || menuOpened) && !user.isCurrent"
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
import { formatTimeSince, MsCheckbox } from 'megashark-lib';
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { UserModel } from '@/components/users/types';
import { IonButton, IonIcon, IonItem, IonLabel } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

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
.user-selected {
  min-width: 4rem;
  justify-content: end;
  width: auto;
}

.user-name {
  padding: 0.5rem 1rem;
  width: auto;
  flex-grow: 1;
  min-width: 11.25rem;
  max-width: 25rem;

  &__label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    overflow: hidden;

    .main-cell {
      white-space: nowrap;
      overflow: hidden;
    }
  }

  &__you {
    color: var(--parsec-color-light-primary-600);
  }
}

.user-profile {
  min-width: 11.5rem;
  max-width: 10vw;
  width: auto;
  flex-grow: 2;
}

.user-email {
  max-width: 16rem;
  min-width: 16rem;
  flex-grow: 0;
  color: var(--parsec-color-light-secondary-grey);
  overflow: hidden;

  &__label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

.user-status {
  min-width: 8rem;
  width: auto;
  flex-grow: 0;
  color: var(--parsec-color-light-secondary-grey);
}

.user-join {
  min-width: 11.25rem;
  flex-grow: 0;
  overflow: hidden;
  color: var(--parsec-color-light-secondary-grey);
}

.user-options {
  min-width: 4rem;
  width: auto;
  flex-grow: 0;
  margin-left: auto;

  ion-button::part(native) {
    padding: 0;
  }

  .options-button {
    --background-hover: none;

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
    }

    &:hover {
      .options-button__icon {
        color: var(--parsec-color-light-primary-500);
      }
    }
  }
}
</style>
