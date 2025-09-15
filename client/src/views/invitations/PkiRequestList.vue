<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container">
    <ion-button @click="$emit('copyJoinLinkClick')">COPY LINK BUTTON</ion-button>
    <div
      class="request"
      v-for="req in requests"
      :key="req.certificate"
    >
      {{ req.humanHandle.label }} {{ req.humanHandle.email }} {{ I18n.formatDate(req.createdOn, 'long') }}
      <span class="validity">{{ req.validity }}</span>
      <ion-button @click="$emit('acceptClick', req)">ACCEPT</ion-button>
      <ion-button @click="$emit('rejectClick', req)">REJECT</ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { OrganizationJoinRequest } from '@/parsec';
import { IonButton } from '@ionic/vue';
import { I18n } from 'megashark-lib';

defineProps<{
  requests: Array<OrganizationJoinRequest>;
}>();

defineEmits<{
  (e: 'acceptClick', invitation: OrganizationJoinRequest): void;
  (e: 'rejectClick', invitation: OrganizationJoinRequest): void;
  (e: 'copyJoinLinkClick'): void;
}>();
</script>

<style scoped lang="scss">
// .request:nth-child(1) {
//   font-family: 'Papyrus', fantasy;
// }
// .request:nth-child(2) {
//   font-family: 'Wingdings';
// }
// .request:nth-child(3) {
//   font-family: 'Impact', sans-serif;
// }
// .request:nth-child(4) {
//   font-family: 'Courier New', monospace;
//   letter-spacing: 1em;
// }

// .request {
//   text-shadow:
//     5px 5px 0 red,
//     -5px -5px 0 lime,
//     10px 10px 20px cyan,
//     -10px -10px 20px magenta;
// }

// .container {
//   font-family: 'Comic Sans MS', cursive, sans-serif;
//   font-size: calc(10px + 2vw);
//   background: repeating-linear-gradient(45deg, red, yellow 10px, lime 20px, cyan 30px, magenta 40px);
//   color: transparent;
//   background-clip: text;
//   -webkit-background-clip: text;
//   text-shadow: 2px 2px 10px hotpink;
// }

// .request {
//   cursor: url('https://cur.cursors-4u.net/cursor.png'), progress;
// }

// .request {
//   animation: resize 0.2s infinite alternate;
// }
// @keyframes resize {
//   from {
//     width: 10%;
//   }
//   to {
//     width: 100%;
//   }
// }

// .validity {
//   border: 10px double magenta;
//   border-radius: 50% 20% 40% 80%;
//   outline: 5px dashed lime;
//   box-shadow:
//     inset 0 0 20px red,
//     0 0 30px cyan;
// }
</style>
