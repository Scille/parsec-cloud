<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="user-card-item ion-no-padding"
    :detail="false"
    :class="{
      selected: user.isSelected,
      'no-padding-end': !user.isSelected,
      revoked: user.isRevoked(),
      frozen: user.isFrozen(),
      'user-hovered': !user.isSelected && (menuOpened || isHovered),
    }"
    @click="$emit('click', $event, user)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    ref="itemRef"
  >
    <div
      class="user-card-checkbox"
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
      v-if="user.isRevoked()"
      class="user-revoked"
    >
      <user-status-tag :revoked="user.isRevoked()" />
    </div>
    <div
      class="user-card-option"
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
        <ion-text class="user-card-info__name body">
          <span>{{ user.humanHandle.label }}</span>
          <span
            v-if="user.isCurrent"
            class="body name-you"
          >
            {{ $msTranslate('UsersPage.currentUser') }}
          </span>
        </ion-text>
        <ion-text class="user-card-info__email body-sm">
          {{ user.humanHandle.email }}
        </ion-text>
      </div>
      <div class="user-card-profile">
        <tag-profile :profile="user.currentProfile" />
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
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { MsCheckbox } from 'megashark-lib';

const isHovered = ref(false);
const menuOpened = ref(false);
const itemRef = ref();

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

onMounted(async () => {
  itemRef.value.$el.addEventListener('contextmenu', onOptionsClick);
});

onBeforeUnmount(async () => {
  itemRef.value.$el.removeEventListener('contextmenu');
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
  --background: none;
  --background-hover: none;

  border: 1px solid var(--parsec-color-light-secondary-medium);
  width: 14rem;
  border-radius: var(--parsec-radius-12);
  position: relative;
  height: fit-content;

  &::part(native) {
    --inner-padding-end: 0px;
  }

  &:hover:not(.revoked) {
    background: var(--parsec-color-light-primary-30);

    .user-card-info__email {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &.revoked {
    position: relative;

    .user-revoked {
      position: absolute;
      top: 0.725rem;
      right: 0.725rem;
    }
  }

  &.selected:not(.revoked) {
    --background: var(--parsec-color-light-primary-100);
    border: 1px solid var(--parsec-color-light-primary-100);

    .user-card-info__name {
      color: var(--parsec-color-light-primary-700);
    }

    .user-card-info__email {
      color: var(--parsec-color-light-secondary-text);
    }
  }
}

.user-card-option,
.user-card-checkbox {
  position: absolute;
  z-index: 10;
}

.user-card-checkbox {
  top: 1rem;
  right: 1rem;
}

.user-card-option {
  color: var(--parsec-color-light-secondary-grey);
  text-align: right;
  display: flex;
  align-items: center;
  bottom: 0;
  right: 0;
  font-size: 1.5rem;
  padding: 1rem;
  cursor: pointer;

  &:hover {
    color: var(--parsec-color-light-primary-500);
  }
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
        color: var(--parsec-color-light-primary-600);
      }
    }

    &__email {
      color: var(--parsec-color-light-secondary-grey);
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }
  }
}

.frozen {
  filter: blur(1px);
}
</style>
