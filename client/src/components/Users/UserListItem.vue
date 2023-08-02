<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="user-list-item"
    lines="full"
    :detail="false"
    :class="{ selected: isSelected, 'no-padding-end': !isSelected }"
    @click="$emit('click', $event, user)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div
      class="user-selected"
    >
      <ion-checkbox
        aria-label=""
        v-model="isSelected"
        v-show="isSelected || isHovered || showCheckbox"
        class="checkbox"
        @click.stop
        @ion-change="$emit('select', user, isSelected)"
      />
    </div>

    <!-- user name -->
    <div class="user-name">
      <ion-label class="user-name__label cell">
        <user-avatar-name
          class="main-cell"
          :user-avatar="user.name"
          :user-name="user.name"
        />
      </ion-label>
    </div>

    <!-- user mail -->
    <div class="user-email">
      <ion-label class="user-email__label cell">
        {{ user.email }}
      </ion-label>
    </div>

    <!-- user profile -->
    <div class="user-profile">
      <tag-profile
        :profile="user.profile"
      />
    </div>

    <!-- user size -->
    <div class="user-join">
      <ion-label
        class="user-join-label cell"
      >
        {{ timeSince(user.joined, '--', 'short') }}
      </ion-label>
    </div>

    <!-- options -->
    <div class="user-options ion-item-child-clickable">
      <ion-button
        fill="clear"
        class="options-button"
        @click.stop="$emit('menuClick', $event, user)"
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
import {
  IonItem,
  IonLabel,
  IonIcon,
  IonButton,
  IonCheckbox,
} from '@ionic/vue';
import {
  ellipsisHorizontal,
} from 'ionicons/icons';
import { MockUser } from '@/common/mocks';
import { FormattersKey, Formatters } from '@/common/injectionKeys';
import { defineProps, inject } from 'vue';
import { defineEmits, defineExpose, ref } from '@vue/runtime-core';
import UserAvatarName from '@/components/UserAvatarName.vue';
import TagProfile from '@/components/TagProfile.vue';

const isHovered = ref(false);
const isSelected = ref(false);

const props = defineProps<{
  user: MockUser,
  showCheckbox: boolean
}>();

defineEmits<{
  (e: 'click', event: Event, user: MockUser): void,
  (e: 'menuClick', event: Event, user: MockUser): void,
  (e: 'select', user: MockUser, selected: boolean): void
}>();

function getUser(): MockUser {
  return props.user;
}

defineExpose({
  isHovered,
  isSelected,
  getUser,
});

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince } = inject(FormattersKey)! as Formatters;
</script>

<style scoped lang="scss">
.user-list-item {
  border-radius: var(--parsec-radius-4);
  --show-full-highlight: 0;

  &::part(native) {
    --padding-start: 0px;
  }

  &:hover:not(.item-checkbox-checked) {
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &:hover, &.selected {
    .cell, .options-button__icon {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &:focus, &:active, &.selected {
    --background-focused: var(--parsec-color-light-primary-100);
    --background: var(--parsec-color-light-primary-100);
    --background-focused-opacity: 1;
  }

  &.selected, &:focus {
    --border-width: 0;
  }

  &.item-checkbox-checked {
    --background: var(--parsec-color-light-primary-100);
    --background-checked-opacity: 1;

    .cell, .options-button__icon {
      color: var(--parsec-color-light-secondary-text);
    }
  }
}

.user-list-item>[class^="user-"] {
  padding: 0 1rem;
  display: flex;
  align-items: center;
  height: 4rem;
}

.user-selected {
  min-width: 4rem;
  justify-content: end;
}

.user-name {
  padding: .75rem 1rem;
  width: 100%;
  max-width: 20vw;
  min-width: 11.25rem;
  white-space: nowrap;
  overflow: hidden;
}

.user-email {
  min-width: 17.5rem;
  flex-grow: 0;
  color: var(--parsec-color-light-secondary-grey);
}

.user-profile {
  min-width: 11.5rem;
  max-width: 10vw;
  flex-grow: 2;
}

.user-join {
  min-width: 11.25rem;
  flex-grow: 0;
  overflow: hidden;
  color: var(--parsec-color-light-secondary-grey);
}

.user-options {
  min-width: 4rem;
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
