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
        <ion-text class="invitation-mobile-header__email">{{ invitation.claimerEmail }}</ion-text>
      </div>
      <div class="invitation-mobile-content">
        <ion-text class="invitation-mobile-content__createdOn">
          {{ $msTranslate(formatTimeSince(invitation.createdOn, '--', 'short')) }}
        </ion-text>
      </div>
    </div>

    <!-- invitation mail -->
    <div
      class="list-item-column invitation-email"
      v-if="isLargeDisplay"
    >
      <ion-text class="list-item-label label-email">
        {{ invitation.claimerEmail }}
      </ion-text>
    </div>

    <!-- invitation created on -->
    <div
      class="list-item-column invitation-sentOn"
      v-if="isLargeDisplay"
    >
      <ion-text class="list-item-label label-sent-on">
        {{ $msTranslate(formatTimeSince(invitation.createdOn, '--', 'short')) }}
      </ion-text>
    </div>

    <!-- actions -->
    <div class="list-item-end invitation-actions">
      <div class="invitation-actions-primary">
        <ion-button
          @click="$emit('greetClick', invitation)"
          class="primary-button button-default"
        >
          {{ $msTranslate('InvitationsPage.emailInvitation.greet') }}
        </ion-button>
      </div>
      <div class="invitation-actions-secondary">
        <ion-button
          @click="$emit('copyLinkClick', invitation)"
          class="invitation-actions-secondary__button"
          fill="outline"
          ref="copyLinkButton"
          slot="icon-only"
        >
          <ion-icon
            :icon="link"
            class="button-icon"
          />
        </ion-button>
        <ion-button
          @click="$emit('sendEmailClick', invitation)"
          class="invitation-actions-secondary__button"
          fill="outline"
          ref="resendEmailButton"
          slot="icon-only"
        >
          <ion-icon
            :icon="mail"
            class="button-icon"
          />
        </ion-button>
        <ion-button
          @click.stop="$emit('deleteClick', invitation)"
          class="invitation-actions-secondary__button"
          fill="outline"
          ref="deleteButton"
          slot="icon-only"
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
  color: ms.color('text-base-body');
  @include ms.font('label-md-medium');
}

.invitation-sentOn {
  color: ms.color('text-base-description');
  @include ms.font('label-md-medium');
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
    gap: ms.spacing('gap-lg');
    width: 100%;
    background: ms.color('surface-base-default-secondary');
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
  }
}

.invitation-mobile {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-3xl') ms.spacing('padding-2xl');

  &-header {
    display: flex;
    overflow: hidden;

    &__email {
      @include ms.font('label-lg-medium');
      color: ms.color('text-base-body');
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

.invitation-actions-secondary__button {
  &:last-of-type {
    position: relative;
    display: flex;
    align-items: center;
    margin-left: 1rem;

    &::before {
      content: '';
      position: absolute;
      left: -1rem;
      width: 1px;
      height: calc(100% - 0.825rem);
      background: ms.color('border-base-default');
      margin-right: 0.75rem;
    }
  }
}
</style>
