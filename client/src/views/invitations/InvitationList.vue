<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <div
      class="invitation"
      v-for="invitation in invitations"
      :key="invitation.token"
    >
      {{ invitation.claimerEmail }} {{ I18n.formatDate(invitation.createdOn, 'long') }}
      <ion-button @click="$emit('greetClick', invitation)">GREET</ion-button>
      <ion-button @click="$emit('copyLinkClick', invitation)">COPY LINK</ion-button>
      <ion-button @click="$emit('sendEmailClick', invitation)">RESEND EMAIL</ion-button>
      <ion-button @click="$emit('deleteClick', invitation)">DELETE</ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { UserInvitation } from '@/parsec';
import { IonButton } from '@ionic/vue';
import { I18n } from 'megashark-lib';

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
.invitation {
  font-size: 150%;
  text-shadow:
    5px 5px 0 red,
    -10px -10px 0 lime,
    20px 20px 50px cyan,
    -20px -20px 50px magenta;
}

.invitation {
  background: linear-gradient(90deg, hotpink, chartreuse, deepskyblue, orange, hotpink);
  background-size: 200% 200%;
  animation: gradientScroll 3s infinite linear;
}
@keyframes gradientScroll {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 100% 50%;
  }
}

.invitation {
  display: inline-block;
  animation: wobble 0.5s infinite ease-in-out alternate;
}
@keyframes wobble {
  from {
    transform: rotate(-10deg) scale(0.9);
  }
  to {
    transform: rotate(10deg) scale(1.1);
  }
}
</style>
