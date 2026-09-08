<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="list-item request-list-item"
    lines="full"
    :class="{
      'request-list-item--corrupted': request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted,
    }"
  >
    <div class="request-list-item__content">
      <!-- request - mobile version -->
      <div
        class="request-mobile"
        v-if="isSmallDisplay"
      >
        <div class="request-mobile-header">
          <ion-text class="request-mobile-header__name">
            <span v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted">
              {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.errors.unknownName') }}
            </span>
            <span v-else>{{ humanHandle.label }}</span>
          </ion-text>
          <ion-text class="request-mobile-header__email">
            {{ humanHandle.email }}
          </ion-text>
        </div>
        <div class="request-mobile-content">
          <ion-text class="request-mobile-content__createdOn">
            {{ $msTranslate(formatTimeSince(request.submittedOn, '--', 'short')) }}
          </ion-text>
          <div class="request-type">
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
        class="list-item-column request-name"
        v-if="isLargeDisplay"
      >
        <ion-text class="list-item-label label-name cell">
          <user-avatar-name
            :user-avatar="humanHandle.label"
            :user-name="humanHandle.label"
          />
        </ion-text>
      </div>

      <!-- request mail -->
      <div
        class="list-item-column request-email"
        v-if="isLargeDisplay"
      >
        <ion-text class="list-item-label label-email cell">
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
        class="list-item-column request-createdOn"
        v-if="isLargeDisplay"
      >
        <ion-text class="list-item-label label-created-on cell">
          {{ $msTranslate(formatTimeSince(request.submittedOn, '--', 'short')) }}
        </ion-text>
      </div>

      <!-- request type -->
      <div
        class="list-item-column request-type"
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
      <div class="list-item-end request-actions">
        <div class="request-actions-primary">
          <ion-button
            v-show="canAccept"
            @click="$emit('acceptClick', request)"
            class="primary-button button-default"
            size="default"
          >
            {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.accept') }}
          </ion-button>
        </div>

        <ion-text
          class="request-actions-secondary__text"
          :class="{ 'request-actions-secondary__text--active': showErrorDetails }"
          v-if="!canAccept"
          @click="toggleErrorDetails"
        >
          <ion-icon
            :icon="warning"
            class="error-icon"
          />
          {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.errors.canNotAccept') }}
          <ion-icon
            :icon="chevronDown"
            class="error-icon"
            :style="{ transform: showErrorDetails ? 'rotate(180deg)' : 'rotate(0deg)' }"
          />
        </ion-text>
        <ion-button
          v-if="request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted"
          @click="$emit('rejectClick', request)"
          class="primary-button button-default"
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
              :icon="trash"
              class="button-icon"
            />
          </ion-button>
        </div>
      </div>
    </div>

    <div
      class="request-error-content"
      :class="{ 'request-error-content--visible': showErrorDetails }"
      v-if="!canAccept"
    >
      <ion-text class="request-error__title">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.errors.errorDetail') }}
      </ion-text>
      <ion-text class="request-error__details">
        {{ $msTranslate(reason) }}
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { AsyncEnrollmentIdentitySystemTag, AsyncEnrollmentUntrusted, HumanHandle, ServerConfig } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { chevronDown, trash, warning } from 'ionicons/icons';
import { formatTimeSince, I18n, Translatable, useWindowSize } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  request: AsyncEnrollmentUntrusted;
  pkiAvailable: boolean;
  serverConfig?: ServerConfig;
}>();

const reason = ref<Translatable | undefined>(undefined);
const { isSmallDisplay, isLargeDisplay } = useWindowSize();
const canAccept = ref(true);
const showErrorDetails = ref(false);

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

function toggleErrorDetails(): void {
  showErrorDetails.value = !showErrorDetails.value;
}

onMounted(async () => {
  canAccept.value = true;
  if (props.request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.OpenBao) {
    if (!props.serverConfig?.openbao || props.serverConfig.openbao.auths.length === 0) {
      canAccept.value = false;
      reason.value = 'InvitationsPage.asyncEnrollmentRequest.errors.noSsoConfigured';
    }
  } else if (props.request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKI) {
    if (!props.pkiAvailable) {
      canAccept.value = false;
      reason.value = 'InvitationsPage.asyncEnrollmentRequest.errors.pkiNotAvailable';
    }
  } else if (props.request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKICorrupted) {
    canAccept.value = false;
    reason.value = 'InvitationsPage.asyncEnrollmentRequest.errors.problemWithRequestCertificate';
  } else {
    canAccept.value = false;
    reason.value = 'InvitationsPage.asyncEnrollmentRequest.errors.unknownIdentitySystem';
  }
});
</script>

<style lang="scss" scoped>
.request-list-item {
  &::part(container) {
    display: flex;
    flex-direction: column;
  }

  &__content {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;

    @include ms.responsive-breakpoint('sm') {
      flex-direction: column;
      align-items: start;
    }
  }
}

.request-name {
  color: ms.color('text-base-body');
}

.request-email,
.request-createdOn {
  color: ms.color('text-base-description');
}

.request-type {
  display: flex;
  align-items: center;

  &__label {
    background: ms.color('surface-base-page-secondary');
    border: ms.border('thin') solid ms.color('border-base-default');
    color: ms.color('text-base-body');
    border-radius: ms.radius('2xl');
    padding: 3px ms.spacing('padding-lg');
    width: fit-content;
    flex-shrink: 0;
    margin: 0;
  }
}

.request-mobile-content .request-type {
  @include ms.font('label-sm-medium');
}

.list-item-column.request-type {
  @include ms.font('label-md-medium');
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
    gap: ms.spacing('gap-lg');
    width: 100%;
    background: ms.color('surface-base-default-secondary');
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
  }

  .primary-button {
    @include ms.font('label-md-medium');
  }

  &-secondary__text {
    @include ms.font('label-md-medium');
    display: flex;
    cursor: pointer;
    gap: ms.spacing('gap-sm');
    color: ms.color('text-error-default');
    align-self: center;
    padding: ms.spacing('padding-lg') ms.spacing('padding-lg');
    border-radius: ms.radius('lg');

    &:hover {
      background: ms.color('surface-error-default-subtle-hover');
    }

    &--active {
      background: ms.color('surface-error-default-subtle-hover');
      border: ms.border('thin') solid ms.color('border-error-default-subtle-hover');
    }
  }
}

.request-mobile {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-3xl') ms.spacing('padding-2xl');

  &-header {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    overflow: hidden;
    gap: ms.spacing('gap-sm');

    &__name {
      @include ms.font('label-lg-medium');
      color: ms.color('text-base-body');
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
    }

    &__email {
      @include ms.font('label-md-medium');
      color: ms.color('text-base-description');
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
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-description');
    }
  }
}

.error-icon {
  align-self: center;
  margin-right: 0.25rem;
  font-size: 1rem;
  transition: transform 0.2s ease-in-out;
}

.request-list-item--corrupted {
  --background: #{ms.color('surface-error-default-subtle-hover')};
  --background-hover: #{ms.color('surface-error-default-subtle-hover')};

  .label-email {
    display: flex;
  }

  .request-type__label {
    background: transparent;
    border: ms.border('thin') solid ms.color('border-error-default-subtle-hover');
    color: ms.color('text-error-default');
  }

  .request-actions .primary-button {
    color: ms.color('text-on-color-label');

    &::part(native) {
      background: ms.color('surface-error-default');
      --background-hover: #{ms.color('surface-error-default-hover')};
    }
  }
}

.request-error-content {
  display: flex;
  align-items: center;
  width: 100%;
  gap: ms.spacing('gap-lg');
  padding: ms.spacing('padding-none') ms.spacing('padding-2xl');
  margin: 0 0.25rem;
  border-radius: ms.radius('2xl');
  background: ms.color('surface-error-default-subtle-hover');
  max-height: 0;
  overflow: hidden;
  opacity: 0;
  transition:
    max-height 0.2s ease-in-out,
    opacity 0.2s ease-in-out,
    padding 0.2s ease-in-out,
    margin 0.2s ease-in-out;

  &--visible {
    max-height: 10rem;
    opacity: ms.opacity('10');
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
    margin: 1rem 0.25rem 0.25rem;
  }

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: start;
    border-radius: ms.radius('none');
    margin: 0;
  }

  .request-error__title {
    color: ms.color('text-base-body');
    padding-top: ms.spacing('padding-sm');
  }

  .request-error__details {
    color: ms.color('text-base-body');
  }
}
</style>
