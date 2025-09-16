<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
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
import { IonButton, IonText, IonIcon, IonItem } from '@ionic/vue';
import { link, mail, trash } from 'ionicons/icons';
import { attachMouseOverTooltip, formatTimeSince } from 'megashark-lib';
import { onMounted, useTemplateRef } from 'vue';

defineProps<{
  invitation: UserInvitation;
}>();

const deleteButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('deleteButton');
const resendEmailButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('resendEmailButton');
const copyLinkButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('copyLinkButton');

onMounted(async () => {
  attachMouseOverTooltip(deleteButtonRef.value?.$el, 'InvitationsPage.emailInvitation.tooltips.delete');
  attachMouseOverTooltip(resendEmailButtonRef.value?.$el, 'InvitationsPage.emailInvitation.tooltips.resend');
  attachMouseOverTooltip(copyLinkButtonRef.value?.$el, 'InvitationsPage.emailInvitation.tooltips.copy');
});

defineEmits<{
  (e: 'greetClick', invitation: UserInvitation): void;
  (e: 'copyLinkClick', invitation: UserInvitation): void;
  (e: 'sendEmailClick', invitation: UserInvitation): void;
  (e: 'deleteClick', invitation: UserInvitation): void;
}>();
</script>

<style scoped lang="scss">
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

  .invitation-actions {
    position: relative;
    z-index: 10000;
  }
}
</style>
