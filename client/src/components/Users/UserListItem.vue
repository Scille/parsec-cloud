<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="user-list-item"
    lines="full"
  >
    <!-- user name -->
    <div class="user-name">
      <ion-label class="user-name__label cell">
        {{ user?.name }}
      </ion-label>
    </div>

    <!-- user mail -->
    <div class="user-email">
      <ion-label class="user-email__label cell">
        {{ user.email }}
      </ion-label>
    </div>

    <!-- user role -->
    <div class="user-role">
      <tag-role
        :orgarole="user.role"
      />
    </div>

    <!-- user size -->
    <div class="join-on">
      <ion-label
        class="join-on-label cell"
      >
        {{ user.joined }}
      </ion-label>
    </div>

    <!-- options -->
    <div class="user-options">
      <ion-button
        fill="clear"
        class="options-button"
      >
        <ion-icon
          slot="icon-only"
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
  IonButton
} from '@ionic/vue';
import { MockUser } from '@/common/mocks';
import TagRole from '@/components/tagRole.vue';

defineProps<{
  user: MockUser
}>();
</script>

<style scoped lang="scss">
.user-list-item {
  border-radius: var(--parsec-radius-4);
  --show-full-highlight: 0;

  &::part(native) {
    --padding-start: 0px;
  }

  &:hover:not(.selected) {
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &:hover, &.selected {
    .cell, .options-button__icon {
      color: var(--parsec-color-light-secondary-text);
      --background: red;
    }
  }

  &:focus, &:active, &.selected {
    --background-focused: var(--parsec-color-light-primary-100);
    --background: var(--parsec-color-light-primary-100);
    --background-focused-opacity: 1;
    --border-width: 0;
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
  white-space: nowrap;
  overflow: hidden;

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
  }
}

.user-updatedBy {
  min-width: 11.25rem;
  max-width: 10vw;
  flex-grow: 2;
}

.user-lastUpdate {
  min-width: 11.25rem;
  flex-grow: 0;
}

.user-size {
  min-width: 11.25rem;
}

.user-options {
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

.label-size, .label-last-update {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
