<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content class="popover">
    <ion-list>
      <ion-item
        class="popover-item ion-no-padding"
        v-for="(breadcrumb, index) in filteredBreadcrumbs"
        :key="index"
        @click="onClick(breadcrumb)"
      >
        <ion-icon
          :icon="breadcrumb.popoverIcon ? breadcrumb.popoverIcon : returnDownForward"
          class="popover-item__icon"
        />
        <ion-text class="button-medium">{{ breadcrumb.display }}</ion-text>
      </ion-item>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import { IonContent, IonIcon, IonItem, IonList, IonText, popoverController } from '@ionic/vue';
import { returnDownForward } from 'ionicons/icons';

const props = defineProps<{
  breadcrumbs: RouterPathNode[];
}>();

const filteredBreadcrumbs = props.breadcrumbs.filter((breadcrumb) => breadcrumb.display !== undefined);

async function onClick(breadcrumb: RouterPathNode): Promise<void> {
  await popoverController.dismiss({ breadcrumb: breadcrumb });
}
</script>

<style scoped lang="scss">
.popover {
  &-item {
    color: var(--parsec-color-light-secondary-soft-text);
    --background: none;

    &::part(native) {
      padding: 0.75rem 1rem;
    }

    &:hover {
      cursor: pointer;
      background: var(--parsec-color-light-secondary-background);
    }

    ion-text {
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
    }

    &__icon {
      font-size: 1rem;
      color: var(--parsec-color-light-secondary-grey);
      margin-right: 0.625rem;
    }
  }
}
</style>
