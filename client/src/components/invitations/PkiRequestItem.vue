<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="pkiRequest-list-item"
    lines="full"
  >
    <!-- request - mobile version -->
    <div
      class="pkiRequest-mobile"
      v-if="isSmallDisplay"
    >
      <div class="pkiRequest-mobile-header">
        <ion-text class="pkiRequest-mobile-header__name subtitles-normal">{{ request.humanHandle.label }}</ion-text>
        <ion-text class="pkiRequest-mobile-header__email button-medium">{{ request.humanHandle.email }}</ion-text>
      </div>
      <div class="pkiRequest-mobile-content">
        <ion-text class="pkiRequest-mobile-content__createdOn body-sm">
          {{ $msTranslate(formatTimeSince(request.createdOn, '--', 'short')) }}
        </ion-text>
        <div
          class="certificate button-small"
          :class="`certificate-${request.validity}`"
        >
          <ion-icon
            :icon="validity.icon"
            class="certificate-icon"
          />
          {{ $msTranslate(validity.text) }}
        </div>
      </div>
    </div>

    <!-- request avatar -->
    <div
      class="pkiRequest-name"
      v-if="isLargeDisplay"
    >
      <ion-text class="pkiRequest-name__label cell">
        <user-avatar-name
          :user-avatar="request.humanHandle.label"
          :user-name="request.humanHandle.label"
        />
      </ion-text>
    </div>

    <!-- request mail -->
    <div
      class="pkiRequest-email"
      v-if="isLargeDisplay"
    >
      <ion-text class="pkiRequest-email__label cell">
        {{ request.humanHandle.email }}
      </ion-text>
    </div>

    <!-- request created on -->
    <div
      class="pkiRequest-createdOn"
      v-if="isLargeDisplay"
    >
      <ion-text class="pkiRequest-createdOn__label cell">
        {{ $msTranslate(formatTimeSince(request.createdOn, '--', 'short')) }}
      </ion-text>
    </div>

    <!-- request certificate -->
    <div
      class="pkiRequest-certificate"
      v-if="isLargeDisplay"
    >
      <ion-text class="pkiRequest-certificate__label cell">
        <div
          class="certificate button-small"
          :class="`certificate-${request.validity}`"
        >
          <ion-icon
            :icon="validity.icon"
            class="certificate-icon"
          />
          {{ $msTranslate(validity.text) }}
        </div>
      </ion-text>
    </div>

    <!-- actions -->
    <div class="pkiRequest-actions">
      <ion-button
        @click="$emit('acceptClick', request)"
        class="primary-button button-medium button-default"
        size="default"
      >
        {{ $msTranslate('InvitationsPage.pkiRequests.accept') }}
      </ion-button>
      <div class="pkiRequest-actions-secondary">
        <ion-button
          @click="$emit('rejectClick', request)"
          class="pkiRequest-actions-secondary__button"
          fill="clear"
          ref="rejectButton"
        >
          <ion-icon
            :icon="closeCircle"
            class="button-icon"
          />
        </ion-button>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { JoinRequestValidity, OrganizationJoinRequest } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { checkmarkCircle, closeCircle, warning } from 'ionicons/icons';
import { attachMouseOverTooltip, formatTimeSince, useWindowSize } from 'megashark-lib';
import { computed, onMounted, useTemplateRef } from 'vue';

const { isSmallDisplay, isLargeDisplay } = useWindowSize();
const rejectButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('rejectButton');

const props = defineProps<{
  request: OrganizationJoinRequest;
}>();

defineEmits<{
  (e: 'acceptClick', invitation: OrganizationJoinRequest): void;
  (e: 'rejectClick', invitation: OrganizationJoinRequest): void;
}>();

const validity = computed(() => {
  if (props.request.validity === JoinRequestValidity.Valid) {
    return { text: 'InvitationsPage.pkiRequests.certificate.valid', icon: checkmarkCircle };
  } else if (props.request.validity === JoinRequestValidity.Invalid) {
    return { text: 'InvitationsPage.pkiRequests.certificate.invalid', icon: closeCircle };
  } else {
    return { text: 'InvitationsPage.pkiRequests.certificate.unknown', icon: warning };
  }
});

onMounted(async () => {
  attachMouseOverTooltip(rejectButtonRef.value?.$el, 'InvitationsPage.pkiRequests.tooltips.reject');
});
</script>

<style lang="scss" scoped>
.pkiRequest-name {
  color: var(--parsec-color-light-secondary-text);
}

.pkiRequest-email,
.pkiRequest-createdOn {
  color: var(--parsec-color-light-secondary-grey);
}

.certificate {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  background: var(--parsec-color-light-success-50);
  color: var(--parsec-color-light-success-700);
  border-radius: var(--parsec-radius-12);
  padding: 0.125rem 0.5rem;
  width: fit-content;
  flex-shrink: 0;

  &-valid {
    background: var(--parsec-color-light-success-50);
    color: var(--parsec-color-light-success-700);
  }

  &-invalid {
    background: var(--parsec-color-light-danger-50);
    color: var(--parsec-color-light-danger-700);
  }

  &-unknown {
    background: var(--parsec-color-light-warning-50);
    color: var(--parsec-color-light-warning-700);
  }
}

.pkiRequest-actions {
  position: sticky;
  z-index: 10;
  right: 0;

  @include ms.responsive-breakpoint('sm') {
    position: initial;
    display: flex;
    flex-direction: row-reverse;
    justify-content: space-between;
    gap: 0.5rem;
    width: 100%;
    background: var(--parsec-color-light-secondary-background);
    padding: 0.5rem 0.75rem;
  }
}

.pkiRequest-mobile {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
  gap: 1rem;
  padding: 1rem 0.75rem;

  &-header {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    overflow: hidden;
    gap: 0.25rem;

    &__name {
      color: var(--parsec-color-light-secondary-text);
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
    }

    &__email {
      color: var(--parsec-color-light-secondary-grey);
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
    }
  }

  &-content {
    display: flex;
    justify-content: space-between;
    align-items: center;

    &__createdOn {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}
</style>
