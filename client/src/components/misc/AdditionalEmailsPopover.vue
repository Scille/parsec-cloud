<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="emails-list">
    <ion-item
      class="emails-list-item"
      lines="none"
      v-for="(email, index) in emails"
      :key="index"
    >
      <ion-text class="emails-list-item__text">
        {{ email }}
      </ion-text>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { IonItem, IonList, IonText } from '@ionic/vue';

defineProps<{
  emails: Array<string>;
}>();
</script>

<style lang="scss" scoped>
.emails-list {
  display: flex;
  flex-direction: column;
  padding: ms.spacing('padding-none');

  &::part(native) {
    --padding-start: #{ms.spacing('padding-none')};
    --padding-end: #{ms.spacing('padding-none')};
  }

  &-item {
    @include ms.font('body-md-regular');
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
    --background: none;
    --background-hover: none;
    --background-hover-opacity: #{ms.opacity('10')};
    --inner-padding-end: #{ms.spacing('padding-none')};
    color: ms.color('text-base-body');

    &::part(native) {
      padding: ms.spacing('padding-none');
      cursor: default;
    }

    &__text {
      word-break: break-all;
    }

    &:not(:last-child) {
      border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    }
  }
}
</style>
