<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="request-list-item"
    lines="full"
    :class="{
      'request-list-item--corrupted': request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted,
    }"
  >
    <!-- request - mobile version -->
    <div
      class="request-mobile"
      v-if="isSmallDisplay"
    >
      <div class="request-mobile-header">
        <ion-text class="request-mobile-header__name subtitles-normal">
          <span v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted">
            {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.errors.unknownName') }}
          </span>
          <span v-else>{{ humanHandle.label }}</span>
        </ion-text>
        <ion-text class="request-mobile-header__email button-medium">
          {{ humanHandle.email }}
        </ion-text>
      </div>
      <div class="request-mobile-content">
        <ion-text class="request-mobile-content__createdOn body-sm">
          {{ $msTranslate(formatTimeSince(request.submittedOn, '--', 'short')) }}
        </ion-text>
        <div class="button-small request-type">
          <ion-text
            class="request-type__label"
            v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.OpenBao"
          >
            {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.type.sso') }}
          </ion-text>
          <ion-text
            class="request-type__label"
            v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKI"
          >
            {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.type.pki') }}
          </ion-text>
          <ion-text
            class="request-type__label"
            v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted"
          >
            {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.type.pkiCorrupted') }}
          </ion-text>
        </div>
      </div>
    </div>

    <!-- request avatar -->
    <div
      class="request-name"
      v-if="isLargeDisplay"
    >
      <ion-text class="request-name__label cell">
        <user-avatar-name
          :user-avatar="humanHandle.label"
          :user-name="humanHandle.label"
        />
      </ion-text>
    </div>

    <!-- request mail -->
    <div
      class="request-email"
      v-if="isLargeDisplay"
    >
      <ion-text class="request-email__label cell">
        <ion-icon
          :icon="warning"
          class="error-icon"
          v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted"
        />

        {{ humanHandle.email }}
      </ion-text>
    </div>

    <!-- request created on -->
    <div
      class="request-createdOn"
      v-if="isLargeDisplay"
    >
      <ion-text class="request-createdOn__label cell">
        {{ $msTranslate(formatTimeSince(request.submittedOn, '--', 'short')) }}
      </ion-text>
    </div>

    <!-- request type -->
    <div
      class="request-type button-medium"
      v-if="isLargeDisplay"
    >
      <ion-text
        class="request-type__label"
        v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.OpenBao"
      >
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.type.sso') }}
      </ion-text>
      <ion-text
        class="request-type__label"
        v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKI"
      >
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.type.pki') }}
      </ion-text>
      <ion-text
        class="request-type__label"
        v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted"
        ref="rejectType"
      >
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.type.pkiCorrupted') }}
      </ion-text>
    </div>

    <!-- actions -->
    <div class="request-actions">
      <ion-button
        v-show="canAccept"
        @click="$emit('acceptClick', request)"
        class="primary-button button-medium button-default"
        size="default"
      >
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.accept') }}
      </ion-button>
      <ion-button
        v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted"
        @click="$emit('rejectClick', request)"
        class="primary-button button-medium button-default"
        size="default"
      >
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.reject') }}
      </ion-button>
      <div
        class="request-actions-secondary"
        v-if="request.identitySystem.tag !== AsyncEnrollmentIdentitySystemTag.PKICorrupted"
      >
        <ion-button
          @click="$emit('rejectClick', request)"
          class="request-actions-secondary__button"
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
import { AsyncEnrollmentIdentitySystemTag, AsyncEnrollmentUntrusted, HumanHandle, ServerConfig } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { closeCircle, warning } from 'ionicons/icons';
import { attachMouseOverTooltip, formatTimeSince, I18n, Translatable, useWindowSize } from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  request: AsyncEnrollmentUntrusted;
  pkiAvailable: boolean;
  serverConfig?: ServerConfig;
}>();

const { isSmallDisplay, isLargeDisplay } = useWindowSize();
const rejectTagRef = useTemplateRef<InstanceType<typeof IonButton>>('rejectButton');
const canAccept = ref(true);

defineEmits<{
  (e: 'acceptClick', invitation: AsyncEnrollmentUntrusted): void;
  (e: 'rejectClick', invitation: AsyncEnrollmentUntrusted): void;
}>();

const humanHandle = computed((): HumanHandle => {
  if (props.request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted) {
    return {
      label: I18n.translate('InvitationsPage.asyncEnrollmentRequest.errors.unknownName'),
      email: I18n.translate('InvitationsPage.asyncEnrollmentRequest.errors.unknownEmail'),
    };
  }
  return props.request.untrustedRequestedHumanHandle;
});

onMounted(async () => {
  canAccept.value = true;
  let reason: Translatable | undefined = undefined;
  if (props.request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.OpenBao) {
    if (!props.serverConfig?.openbao || props.serverConfig.openbao.auths.length === 0) {
      canAccept.value = false;
      reason = 'InvitationsPage.asyncEnrollmentRequest.errors.noSsoConfigured';
    }
  } else if (props.request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKI) {
    if (!props.pkiAvailable) {
      canAccept.value = false;
      reason = 'InvitationsPage.asyncEnrollmentRequest.errors.pkiNotAvailable';
    }
  } else if (props.request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted) {
    canAccept.value = false;
    reason = 'InvitationsPage.asyncEnrollmentRequest.errors.problemWithRequestCertificate';
  } else {
    canAccept.value = false;
    reason = 'InvitationsPage.asyncEnrollmentRequest.errors.unknownIdentitySystem';
  }

  if (!canAccept.value && reason) {
    attachMouseOverTooltip(rejectTagRef.value?.$el, reason);
  }
});
</script>

<style lang="scss" scoped>
.request-name {
  color: var(--parsec-color-light-secondary-text);
}

.request-email,
.request-createdOn {
  color: var(--parsec-color-light-secondary-hard-grey);
}

.request-type {
  display: flex;
  align-items: center;

  &__label {
    background: var(--parsec-color-light-secondary-premiere);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    color: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-12);
    padding: 3px 0.5rem;
    width: fit-content;
    flex-shrink: 0;
    margin: 0;
  }
}

.request-actions {
  position: sticky;
  z-index: 10;
  right: 0;
  display: flex;
  justify-content: flex-end;

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

  &-error {
    color: var(--parsec-color-light-danger-500);
    align-self: center;
  }
}

.request-mobile {
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

.error-icon {
  align-self: center;
  margin-right: 0.25rem;
  font-size: 1rem;
}

.request-list-item--corrupted {
  --background: var(--parsec-color-light-danger-50);
  --background-hover: var(--parsec-color-light-danger-50);

  .request-email__label {
    display: flex;
    align-items: center;
  }

  .request-type__label {
    background: transparent;
    border: 1px solid var(--parsec-color-light-danger-100);
    color: var(--parsec-color-light-danger-500);
  }

  .request-actions .primary-button {
    color: var(--parsec-color-light-secondary-white);

    &::part(native) {
      background: var(--parsec-color-light-danger-500);
      --background-hover: var(--parsec-color-light-danger-700);
    }
  }
}
</style>
