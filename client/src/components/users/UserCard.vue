<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="user-card-item ion-no-padding"
    :detail="false"
    :class="{
      selected: user.isSelected,
      revoked: user.isRevoked(),
      suspended: user.isFrozen(),
      'current-user': user.isCurrent,
      'no-padding-end': !user.isSelected,
      'user-card-item--hovered': !user.isSelected && (menuOpened || isHovered),
    }"
    @click="allowSelection && $emit('select', user, !user.isSelected)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div
      class="card-checkbox"
      v-if="!user.isRevoked()"
    >
      <ms-checkbox
        :checked="user.isSelected"
        v-if="allowSelection && (user.isSelected || isHovered || activeCheckbox) && !user.isCurrent"
        @click.stop
        @change="allowSelection && $emit('select', user, $event)"
      />
      <ms-image
        v-if="user.isCurrent"
        :image="LockClosedIcon"
        class="lock-icon"
      />
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
        class="user-card-avatar large"
        :user-avatar="user.humanHandle.label"
      />
      <div class="user-card-info">
        <ion-text
          class="user-card-info__name"
          :title="user.humanHandle.label"
        >
          <span>{{ user.humanHandle.label }}</span>
          <span
            v-if="user.isCurrent"
            class="user-name--self"
          >
            {{ $msTranslate('UsersPage.currentUser') }}
          </span>
        </ion-text>
        <ion-text
          class="user-card-info__email label-email"
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
      <div class="user-card-profile">
        <user-profile-tag :profile="user.currentProfile" />
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
        <ion-text class="user-card-join-label">
          {{ $msTranslate(formatTimeSince(user.createdOn, '--', 'short')) }}
        </ion-text>
      </div>
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
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { checkmark, copy, ellipsisHorizontal } from 'ionicons/icons';
import { formatTimeSince, LockClosedIcon, MsCheckbox, MsImage } from 'megashark-lib';
import { inject, ref, Ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const emailCopied = ref(false);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;

const emits = defineEmits<{
  (e: 'click', event: Event, user: UserModel): void;
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
  (e: 'select', user: UserModel, selected: boolean): void;
}>();

const props = defineProps<{
  user: UserModel;
  activeCheckbox: boolean;
  allowSelection: boolean;
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

async function onCopyEmailClicked(email: string): Promise<void> {
  emailCopied.value = true;

  setTimeout(() => {
    emailCopied.value = false;
  }, 5000);

  await copyToClipboard(email, informationManager.value, 'UsersPage.copyEmail.success', 'UsersPage.copyEmail.failed');
}
</script>

<style scoped lang="scss">
.user-card-item {
  max-width: 15rem;
  width: 100%;
  height: fit-content;

  &:hover:not(.revoked) {
    background: ms.color('surface-brand-default-subtle-hover');

    .user-card-info__email {
      color: ms.color('text-base-body');
    }
  }

  &.selected:not(.revoked) {
    .user-card-info__name {
      color: ms.color('text-brand-default');
    }

    .user-card-info__email {
      color: ms.color('text-base-body');
    }
  }

  &.current-user {
    --background: #{ms.color('surface-base-default-secondary')};

    .lock-icon {
      width: 1.25rem;
      --fill-color: #{ms.color('icon-neutral-default')};
    }
  }

  .user-card {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-3xl') ms.spacing('padding-lg') ms.spacing('padding-3xl') ms.spacing('padding-3xl');
    width: 100%;
    margin: auto;
    color: ms.color('text-base-body');

    &-info {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-sm');

      &__name {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-body');
        display: flex;
        gap: ms.spacing('gap-lg');

        span {
          text-overflow: ellipsis;
          white-space: nowrap;
          overflow: hidden;
        }

        .user-name--self {
          @include ms.font('body-md-regular');
          color: ms.color('text-base-description');
        }
      }

      &__email {
        @include ms.font('body-sm-regular');
        color: ms.color('text-base-description');
        text-overflow: ellipsis;
        white-space: nowrap;
        overflow: hidden;
        display: flex;
        align-items: center;
        cursor: pointer;
        gap: ms.spacing('gap-xs');

        .label-email {
          text-overflow: ellipsis;
          white-space: nowrap;
          overflow: hidden;

          &__text {
            text-overflow: ellipsis;
            white-space: nowrap;
            overflow: hidden;
          }
        }

        .email-copy-icon {
          display: none;
          padding: 0 ms.spacing('padding-md');
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
    }

    &-join-label {
      @include ms.font('body-sm-regular');
    }

    &-profile {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-3xl');
    }
  }

  &.user-card-item--hovered {
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

.card-checkbox {
  top: 1rem;
  right: 1rem;
}

.card-option {
  padding: ms.spacing('padding-3xl');
  right: 0;
  bottom: 0;
}
</style>
