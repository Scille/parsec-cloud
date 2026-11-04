<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="container">
    <ion-avatar
      class="avatar person-avatar"
      v-for="person in people.slice(0, maxDisplay)"
      :key="person"
    >
      {{ person.substring(0, 2) }}
    </ion-avatar>
    <ion-avatar
      class="avatar extra-avatar"
      v-if="people.length > maxDisplay"
    >
      + {{ people.length - maxDisplay }}
    </ion-avatar>
  </div>
</template>

<script setup lang="ts">
import { IonAvatar } from '@ionic/vue';

defineProps<{
  people: string[];
  maxDisplay: number;
}>();
</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-wrap: nowrap;
  background: ms.color('surface-base-default');
  border-radius: ms.radius('6xl');

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    text-transform: uppercase;
    margin-left: -0.6em;
    height: 2rem;
    width: 2rem;
    border: ms.border('thick') solid ms.color('surface-base-default');
    @include ms.font('body-sm-regular');

    &:first-of-type {
      margin-left: 0;
    }
  }
}

.person-avatar {
  background-color: ms.color('surface-brand-default-subtle');
  color: ms.color('text-brand-default');
}

.extra-avatar {
  background-color: ms.color('surface-base-default-secondary');
  color: ms.color('text-base-description');
}
</style>
