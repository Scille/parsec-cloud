<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="invitation-list-item__waiting">
    <ion-icon
      :icon="time"
    />
    <ion-text class="caption-caption">
      {{ t('UsersPage.invitation.waiting') }}
    </ion-text>
  </div>
  <ion-label
    class="body invitation-list-item__label"
  >
    {{ invitation.email }}
  </ion-label>
  <ion-buttons class="invitation-list-item__buttons">
    <ion-button
      fill="clear"
      class="danger"
      @click.stop="$emit('reject-user', props.invitation)"
    >
      {{ t('UsersPage.invitation.rejectUser') }}
    </ion-button>
    <ion-button
      class="button-default"
      fill="solid"
      @click.stop="$emit('greet-user', props.invitation)"
    >
      {{ t('UsersPage.invitation.greetUser') }}
    </ion-button>
  </ion-buttons>
</template>

<script setup lang="ts">
import {
  IonLabel,
  IonButtons,
  IonButton,
  IonIcon,
  IonText,
} from '@ionic/vue';
import {
  time,
} from 'ionicons/icons';
import { MockInvitation } from '@/common/mocks';
import { useI18n } from 'vue-i18n';
import { defineProps } from 'vue';

const { t } = useI18n();

const props = defineProps<{
  invitation: MockInvitation,
}>();

defineEmits<{
  (e: 'reject-user', invitation: MockInvitation) : void,
  (e: 'greet-user', invitation: MockInvitation) : void,
}>();
</script>

<style scoped lang="scss">
.invitation-list-item__waiting {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: .2rem;
  color: var(--parsec-color-light-secondary-grey);
  margin-left: auto;
  font-style: italic;
}

.invitation-list-item__label {
  color: var(--parsec-color-light-secondary-text);
  margin: 1rem 0;
  width: 100%;
}

.invitation-list-item__buttons {
  display: flex;
  gap: .5rem;
  margin-left: auto;
  width: 100%;

  ion-button {
    width: 100%;
  }
}
</style>
