<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="invitation-list-item"
    lines="full"
  >
    <div
      class="invitation-mobile"
      v-if="isSmallDisplay"
    >
      <div class="invitation-mobile-header">
        <ion-text class="invitation-mobile-header__email subtitles-normal">{{ invitation.claimerEmail }}</ion-text>
      </div>
      <div class="invitation-mobile-content">
        <ion-text class="invitation-mobile-content__createdOn body-sm">
          {{ $msTranslate(formatTimeSince(invitation.createdOn, '--', 'short')) }}
        </ion-text>
      </div>
    </div>

    <!-- invitation mail -->
    <div
      class="invitation-email"
      v-if="isLargeDisplay"
    >
      <ion-text class="invitation-email__label cell">
        {{ invitation.claimerEmail }}
      </ion-text>
    </div>

    <!-- invitation created on -->
    <div
      class="invitation-sentOn"
      v-if="isLargeDisplay"
    >
      <ion-text class="invitation-sentOn__label cell">
        {{ $msTranslate(formatTimeSince(invitation.createdOn, '--', 'short')) }}
      </ion-text>
    </div>

    <!-- actions -->
    <div class="invitation-actions">
      <ion-button
        @click="$emit('greetClick', invitation)"
        class="primary-button button-medium button-default"
        size="default"
      >
        {{ $msTranslate('InvitationsPage.emailInvitation.greet') }}
      </ion-button>
      <div class="invitation-actions-secondary">
        <ion-button
          @click="$emit('copyLinkClick', invitation)"
          class="invitation-actions-secondary__button"
          fill="clear"
          ref="copyLinkButton"
        >
          <ion-icon
            :icon="link"
            class="button-icon"
          />
        </ion-button>
        <ion-button
          @click="$emit('sendEmailClick', invitation)"
          class="invitation-actions-secondary__button"
          fill="clear"
          ref="resendEmailButton"
        >
          <ion-icon
            :icon="mail"
            class="button-icon"
          />
        </ion-button>
        <ion-button
          @click.stop="$emit('deleteClick', invitation)"
          class="invitation-actions-secondary__button"
          fill="clear"
          ref="deleteButton"
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
import { UserInvitation } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { link, mail, trash } from 'ionicons/icons';
import { attachMouseOverTooltip, formatTimeSince, useWindowSize } from 'megashark-lib';
import { onMounted, useTemplateRef } from 'vue';

const { isSmallDisplay, isLargeDisplay } = useWindowSize();

defineProps<{
  invitation: UserInvitation;
}>();

const deleteButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('deleteButton');
const resendEmailButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('resendEmailButton');
const copyLinkButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('copyLinkButton');

onMounted(async () => {
  attachMouseOverTooltip(deleteButtonRef.value?.$el, 'InvitationsPage.emailInvitation.tooltips.delete');
  attachMouseOverTooltip(resendEmailButtonRef.value?.$el, 'InvitationsPage.emailInvitation.tooltips.resend');
  attachMouseOverTooltip(copyLinkButtonRef.value?.$el, 'InvitationsPage.emailInvitation.tooltips.copyLink');
});

defineEmits<{
  (e: 'greetClick', invitation: UserInvitation): void;
  (e: 'copyLinkClick', invitation: UserInvitation): void;
  (e: 'sendEmailClick', invitation: UserInvitation): void;
  (e: 'deleteClick', invitation: UserInvitation): void;
}>();
</script>

<style scoped lang="scss">
.invitation-email {
  color: var(--parsec-color-light-secondary-text);
}

.invitation-sentOn {
  color: var(--parsec-color-light-secondary-grey);
}

.invitation-actions {
  position: sticky;
  right: 0;
  z-index: 10;

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

.invitation-mobile {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
  gap: 1rem;
  padding: 1rem 0.75rem;

  &-header {
    display: flex;
    overflow: hidden;

    &__email {
      color: var(--parsec-color-light-secondary-text);
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
