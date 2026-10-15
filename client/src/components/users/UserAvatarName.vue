<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="avatar-container"
    :class="{ clickable: clickable }"
  >
    <ion-avatar class="avatar person-avatar">
      {{ userAvatar.substring(0, 2) }}
    </ion-avatar>

    <div class="person-name-container">
      <ion-text
        class="person-name"
        v-if="userName"
        :title="userName"
      >
        {{ userName }}
      </ion-text>

      <ion-text
        class="person-description"
        :title="userAvatar"
        v-if="userDescription"
      >
        {{ $msTranslate(userDescription) }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonAvatar, IonText } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
defineProps<{
  userAvatar: string;
  userName?: string;
  userDescription?: Translatable;
  clickable?: boolean;
}>();
</script>

<style lang="scss" scoped>
.avatar-container {
  display: flex;
  flex-wrap: nowrap;
  align-items: center;
  gap: ms.spacing('gap-lg');
  overflow: hidden;

  --width: 2rem;
  --height: 2rem;

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    text-transform: uppercase;
    height: var(--height);
    min-width: var(--width);
    max-width: var(--width);
    @include ms.font('label-md-emphasis');
    border: ms.border('thin') solid ms.color('border-base-default');

    &:first-of-type {
      margin-left: 0;
    }
  }

  .person-name {
    @include ms.font('label-md-emphasis');
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  &.small {
    --width: 1.5rem;
    --height: 1.5rem;

    .avatar,
    .person-name {
      @include ms.font('label-sm-emphasis');
    }
  }

  &.large {
    --width: 2.5rem;
    --height: 2.5rem;

    .avatar {
      @include ms.font('label-lg-emphasis');
    }
  }

  &:not(.main-cell) .person-name {
    color: ms.color('text-base-label');
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }

  &.clickable {
    cursor: pointer;
    &:hover {
      .person-name {
        color: ms.color('text-base-label');
        text-decoration: underline;
        text-decoration-color: ms.color('text-brand-default');
      }
    }
  }

  .person-name-container {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-none');

    .person-description {
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-description');
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.main-cell {
  color: ms.color('text-base-label');
}

.person-avatar {
  color: ms.color('text-brand-default');
  background-color: ms.color('surface-brand-default-subtle');
}
</style>
