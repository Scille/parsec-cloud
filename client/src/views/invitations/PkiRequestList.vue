<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list pkiRequests-container-list">
    <ion-list-header
      class="pkiRequests-list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-text class="pkiRequests-list-header__label cell-title label-name">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.name') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-email">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.email') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-createdOn">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.createdOn') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-certificate">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.certificate') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-space" />
    </ion-list-header>
    <div
      class="request"
      v-for="req in requests"
      :key="req.certificate"
    >
      <ion-item
        button
        class="pkiRequest-list-item"
        lines="full"
      >
        <!-- request avatar -->
        <div
          class="pkiRequest-name"
          v-if="isLargeDisplay"
        >
          <ion-text class="pkiRequest-name__label cell">
            <user-avatar-name
              :user-avatar="req.humanHandle.label"
              :user-name="req.humanHandle.label"
            />
          </ion-text>
        </div>

        <!-- request mail -->
        <div class="pkiRequest-email">
          <ion-text class="pkiRequest-email__label cell">
            {{ req.humanHandle.email }}
          </ion-text>
        </div>

        <!-- request created on -->
        <div class="pkiRequest-createdOn">
          <ion-text class="pkiRequest-createdOn__label cell">
            {{ $msTranslate(formatTimeSince(req.createdOn, '--', 'short')) }}
          </ion-text>
        </div>

        <!-- request certificate -->
        <div class="pkiRequest-certificate">
          <ion-text class="pkiRequest-certificate__label cell">
            <div
              class="certificate button-small"
              :class="'certificate-' + req.validity"
            >
              <ion-icon
                :icon="certificateValidity(req.validity).icon"
                class="certificate-icon"
              />
              {{ $msTranslate(certificateValidity(req.validity).text) }}
            </div>
          </ion-text>
        </div>

        <!-- actions -->
        <div class="pkiRequest-actions">
          <ion-button
            @click="$emit('acceptClick', req)"
            class="primary-button button-medium button-default"
            size="default"
          >
            {{ $msTranslate('InvitationsPage.pkiRequests.accept') }}
          </ion-button>
          <div class="pkiRequest-actions-secondary">
            <ion-button
              @click="$emit('rejectClick', req)"
              class="pkiRequest-actions-secondary__button"
              fill="clear"
            >
              <ion-icon
                :icon="trash"
                class="button-icon"
              />
            </ion-button>
          </div>
        </div>
      </ion-item>
    </div>
  </ion-list>
</template>

<script setup lang="ts">
import { OrganizationJoinRequest, JoinRequestValidity } from '@/parsec';
import { IonButton, IonText, IonIcon, IonItem, IonList, IonListHeader } from '@ionic/vue';
import { checkmarkCircle, closeCircle, trash, warning } from 'ionicons/icons';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { formatTimeSince, Translatable, useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();

defineProps<{
  requests: Array<OrganizationJoinRequest>;
}>();

defineEmits<{
  (e: 'acceptClick', invitation: OrganizationJoinRequest): void;
  (e: 'rejectClick', invitation: OrganizationJoinRequest): void;
  (e: 'copyJoinLinkClick'): void;
}>();

interface Certificate {
  text: Translatable;
  icon: string;
}

function certificateValidity(certificate: JoinRequestValidity): Certificate {
  if (certificate === 'valid') {
    return { text: 'InvitationsPage.pkiRequests.certificate.valid', icon: checkmarkCircle };
  } else if (certificate === 'invalid') {
    return { text: 'InvitationsPage.pkiRequests.certificate.invalid', icon: closeCircle };
  } else {
    return { text: 'InvitationsPage.pkiRequests.certificate.unknown', icon: warning };
  }
}
</script>

<style scoped lang="scss">
.pkiRequests-container-list {
  padding: 0;
}

.pkiRequest-list-item {
  --background-hover: var(--parsec-color-light-secondary-background);
  --background-hover-opacity: 1;

  &::part(native) {
    padding: 0.625rem 1rem 0.625rem 2rem;
    cursor: default;
  }

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
}
</style>
