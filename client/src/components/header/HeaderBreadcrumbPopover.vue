<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content class="popover">
    <ion-list>
      <ion-item
        v-for="(breadcrumb, index) in filteredBreadcrumbs"
        class="breadcrumb-item ion-no-padding"
        :key="index"
        @click="onClick(breadcrumb)"
      >
        <ion-icon
          :icon="breadcrumb.popoverIcon ? breadcrumb.popoverIcon : returnDownForward"
          class="breadcrumb-item__icon"
        />
        <ion-text class="breadcrumb-item__text">{{ breadcrumb.display }}</ion-text>
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
.breadcrumb {
  &-item {
    color: ms.color('text-base-label');
    --background: none;

    &::part(native) {
      padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
    }

    &:hover {
      cursor: pointer;
      background: ms.color('surface-base-default-secondary');
    }

    &__text {
      @include ms.font('label-md-medium');
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
    }

    &__icon {
      font-size: 1rem;
      color: ms.color('text-neutral-default');
      margin-right: 0.625rem;
    }

    &--disabled {
      pointer-events: none;
      color: ms.color('text-base-body');
      opacity: ms.opacity('5');

      &.breadcrumb-item__icon {
        color: ms.color('text-base-body');
      }
    }
  }
}
</style>
