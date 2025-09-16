<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list invitations-container-list">
    <ion-list-header
      class="invitations-list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-text class="invitations-list-header__label cell-title label-email">
        {{ $msTranslate('InvitationsPage.emailInvitation.listDisplayTitles.email') }}
      </ion-text>
      <ion-text class="invitations-list-header__label cell-title label-sentOn">
        {{ $msTranslate('InvitationsPage.emailInvitation.listDisplayTitles.sentOn') }}
      </ion-text>
      <ion-text class="invitations-list-header__label cell-title label-space" />
    </ion-list-header>
    <div
      class="invitation"
      v-for="invitation in invitations"
      :key="invitation.token"
    >
      <ion-item
        button
        class="invitation-list-item"
        lines="full"
      >
        <!-- invitation mail -->
        <div class="invitation-email">
          <ion-text class="invitation-email__label cell">
            {{ invitation.claimerEmail }}
          </ion-text>
        </div>

        <!-- invitation created on -->
        <div class="invitation-sentOn">
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
            >
              <ion-icon
                :icon="copy"
                class="button-icon"
              />
            </ion-button>
            <ion-button
              @click="$emit('sendEmailClick', invitation)"
              class="invitation-actions-secondary__button"
              fill="clear"
            >
              <ion-icon
                :icon="mail"
                class="button-icon"
              />
            </ion-button>
            <ion-button
              @click="$emit('deleteClick', invitation)"
              class="invitation-actions-secondary__button"
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
import { UserInvitation } from '@/parsec';
import { IonButton, IonList, IonText, IonIcon, IonItem, IonListHeader } from '@ionic/vue';
import { copy, mail, trash } from 'ionicons/icons';
import { formatTimeSince, useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();

defineProps<{
  invitations: Array<UserInvitation>;
}>();

defineEmits<{
  (e: 'greetClick', invitation: UserInvitation): void;
  (e: 'copyLinkClick', invitation: UserInvitation): void;
  (e: 'sendEmailClick', invitation: UserInvitation): void;
  (e: 'deleteClick', invitation: UserInvitation): void;
}>();
</script>

<style scoped lang="scss">
.invitations-container-list {
  padding: 0;
}

.invitation-list-item {
  --background-hover: var(--parsec-color-light-secondary-background);
  --background-hover-opacity: 1;

  &::part(native) {
    padding: 0.625rem 1rem 0.625rem 2rem;
    cursor: default;
  }

  .invitation-email {
    color: var(--parsec-color-light-secondary-text);
  }

  .invitation-sentOn {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
