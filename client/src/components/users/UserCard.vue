<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="user-card-item ion-no-padding"
    :detail="false"
    :class="{
      selected: user.isSelected,
      'no-padding-end': !user.isSelected,
      revoked: user.isRevoked(),
      suspended: user.isFrozen(),
      'user-hovered': !user.isSelected && (menuOpened || isHovered),
    }"
    @click="$emit('click', $event, user)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div
      class="card-checkbox"
      v-if="!user.isRevoked() && !user.isCurrent"
    >
      <!-- eslint-disable vue/no-mutating-props -->
      <ms-checkbox
        v-model="user.isSelected"
        v-if="user.isSelected || isHovered || showCheckbox"
        @click.stop
        @ion-change="$emit('select', user, $event)"
      />
      <!-- eslint-enable vue/no-mutating-props -->
    </div>
    <div
      class="card-option"
      v-show="isHovered || menuOpened"
      @click.stop="onOptionsClick($event)"
    >
      <ion-icon :icon="ellipsisHorizontal" />
    </div>
    <div class="user-card">
      <user-avatar-name
        class="user-card-avatar medium"
        :user-avatar="user.humanHandle.label"
      />
      <div class="user-card-info">
        <ion-text
          class="user-card-info__name body"
          :title="user.humanHandle.label"
        >
          <span>{{ user.humanHandle.label }}</span>
          <span
            v-if="user.isCurrent"
            class="body name-you"
          >
            {{ $msTranslate('UsersPage.currentUser') }}
          </span>
        </ion-text>
        <ion-text
          class="user-card-info__email body-sm"
          :title="user.humanHandle.email"
        >
          {{ user.humanHandle.email }}
        </ion-text>
      </div>
      <div class="user-card-profile">
        <tag-profile :profile="user.currentProfile" />
        <div
          v-if="!user.isActive()"
          class="user-revoked user-suspended"
        >
          <user-status-tag
            :revoked="user.isRevoked()"
            :frozen="user.isFrozen()"
            :show-tooltip="true"
          />
        </div>
      </div>
      <div class="user-card-join">
        <ion-text class="user-card-join-label body-sm">
          {{ $msTranslate(formatTimeSince(user.createdOn, '--', 'short')) }}
        </ion-text>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { UserModel } from '@/components/users/types';
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { MsCheckbox, formatTimeSince } from 'megashark-lib';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

const emits = defineEmits<{
  (e: 'click', event: Event, user: UserModel): void;
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
  (e: 'select', user: UserModel, selected: boolean): void;
}>();

const props = defineProps<{
  user: UserModel;
  showCheckbox: boolean;
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
.user-card-item {
  max-width: 15rem;
  width: 100%;
  height: fit-content;

  &:hover:not(.revoked) {
    background: var(--parsec-color-light-primary-30);

    .user-card-info__email {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &.selected:not(.revoked) {
    .user-card-info__name {
      color: var(--parsec-color-light-primary-700);
    }

    .user-card-info__email {
      color: var(--parsec-color-light-secondary-text);
    }
  }
}

.card-checkbox {
  top: 1rem;
  right: 1rem;
}

.card-option {
  padding: 1rem;
  right: 0;
  bottom: 0;
}

.user-card {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1rem 0.5rem 1rem 1rem;
  width: 100%;
  margin: auto;
  color: var(--parsec-color-light-secondary-text);

  &-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    &__name {
      color: var(--parsec-color-light-secondary-text);
      display: flex;
      gap: 0.5rem;

      .name-you {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    &__email {
      color: var(--parsec-color-light-secondary-grey);
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }
  }

  &-profile {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
}
</style>
