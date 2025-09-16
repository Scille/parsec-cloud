<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
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
          :user-avatar="request.humanHandle.label"
          :user-name="request.humanHandle.label"
        />
      </ion-text>
    </div>

    <!-- request mail -->
    <div class="pkiRequest-email">
      <ion-text class="pkiRequest-email__label cell">
        {{ request.humanHandle.email }}
      </ion-text>
    </div>

    <!-- request created on -->
    <div class="pkiRequest-createdOn">
      <ion-text class="pkiRequest-createdOn__label cell">
        {{ $msTranslate(formatTimeSince(request.createdOn, '--', 'short')) }}
      </ion-text>
    </div>

    <!-- request certificate -->
    <div class="pkiRequest-certificate">
      <ion-text class="pkiRequest-certificate__label cell">
        <div
          class="certificate button-small"
          :class="'certificate-' + request.validity"
        >
          <ion-icon
            :icon="certificateValidity(request.validity).icon"
            class="certificate-icon"
          />
          {{ $msTranslate(certificateValidity(request.validity).text) }}
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
            :icon="trash"
            class="button-icon"
          />
        </ion-button>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { OrganizationJoinRequest, JoinRequestValidity } from '@/parsec';
import { IonButton, IonText, IonIcon, IonItem } from '@ionic/vue';
import { checkmarkCircle, closeCircle, trash, warning } from 'ionicons/icons';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { attachMouseOverTooltip, formatTimeSince, Translatable, useWindowSize } from 'megashark-lib';
import { onMounted, useTemplateRef } from 'vue';

const { isLargeDisplay } = useWindowSize();
const rejectButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('rejectButton');

defineProps<{
  request: OrganizationJoinRequest;
}>();

defineEmits<{
  (e: 'acceptClick', invitation: OrganizationJoinRequest): void;
  (e: 'rejectClick', invitation: OrganizationJoinRequest): void;
  (e: 'rejectClick', invitation: OrganizationJoinRequest): void;
  (e: 'copyJoinLinkClick'): void;
}>();

interface Certificate {
  text: Translatable;
  icon: string;
}

onMounted(async () => {
  attachMouseOverTooltip(rejectButtonRef.value?.$el, 'InvitationsPage.pkiRequests.tooltips.delete');
});

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

<style lang="scss" scoped>
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
