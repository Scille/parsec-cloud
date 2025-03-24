<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-breadcrumbs
    class="breadcrumb"
    @ion-collapsed-click="expandCollapsed()"
    :max-items="maxBreadcrumbs"
    :items-before-collapse="itemsBeforeCollapse"
    :items-after-collapse="itemsAfterCollapse"
  >
    <ion-breadcrumb
      v-for="path in fullPath"
      @click="navigateTo(path)"
      :path="path"
      class="breadcrumb-element breadcrumb-normal"
      :key="path.id"
    >
      <ion-icon
        class="main-icon"
        v-if="path.icon && isLargeDisplay"
        :icon="path.icon"
      />
      {{ path.display ? path.display : $msTranslate(path.title) }}
    </ion-breadcrumb>
  </ion-breadcrumbs>
</template>

<script lang="ts">
export interface RouterPathNode {
  id: number;
  display?: string;
  title?: Translatable;
  icon?: string;
  name: string;
  params?: object;
  query?: Query;
}
</script>

<script setup lang="ts">
import { Query } from '@/router';
import { Translatable, useWindowSize } from 'megashark-lib';
import { IonBreadcrumb, IonBreadcrumbs, IonIcon } from '@ionic/vue';
import { Ref, computed, ref } from 'vue';

const { isLargeDisplay } = useWindowSize();

const props = withDefaults(
  defineProps<{
    pathNodes: RouterPathNode[];
    itemsBeforeCollapse?: number;
    itemsAfterCollapse?: number;
    maxShown?: number;
  }>(),
  {
    itemsBeforeCollapse: 2,
    itemsAfterCollapse: 2,
    maxShown: 4,
  },
);

const emits = defineEmits<{
  (e: 'change', node: RouterPathNode): void;
}>();

// Using a computed to reset maxBreadcrumbs value automatically
const fullPath = computed(() => {
  // eslint-disable-next-line vue/no-side-effects-in-computed-properties
  maxBreadcrumbs.value = props.maxShown;
  return props.pathNodes;
});

const maxBreadcrumbs: Ref<number | undefined> = ref(props.maxShown);
let ignoreNextEvent = false;

function expandCollapsed(): void {
  maxBreadcrumbs.value = undefined;
  ignoreNextEvent = true;
}

function navigateTo(path: RouterPathNode): void {
  // ignoreNextEvent is used so that when "..." is clicked, we
  // don't try to travel. Didn't find a better way to do this.
  if (ignoreNextEvent) {
    ignoreNextEvent = false;
    return;
  }
  emits('change', path);
}
</script>

<style scoped lang="scss">
.breadcrumb {
  padding: 0;
  color: var(--parsec-color-light-secondary-grey);

  &-element {
    .main-icon {
      font-size: 1.125rem;
    }

    &::part(native) {
      cursor: pointer;
      padding: 0.25rem 0.5rem;
    }

    &::part(separator) {
      margin-inline: 0;
      color: var(--parsec-color-light-secondary-grey);
      cursor: default;
    }

    &::part(collapsed-indicator) {
      border-radius: var(--parsec-radius-8);
      background: var(--parsec-color-light-secondary-medium);
      color: var(--parsec-color-light-secondary-grey);
      margin-inline: 0.5rem;
    }

    &:hover:not(.breadcrumb-collapsed) {
      color: var(--parsec-color-light-secondary-text);
      position: relative;

      &::after {
        content: '';
        position: absolute;
        width: calc(100% - 0.25rem);
        height: 100%;
        border-radius: var(--parsec-radius-8);
        background: var(--parsec-color-light-secondary-medium);
        opacity: 0.6;
        z-index: -10;
      }

      @include ms.responsive-breakpoint('md') {
        &::after {
          left: 0.5rem;
          width: calc(100% - 1.25rem);
        }
      }
    }
  }

  // Defined by ionic
  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  &-active {
    color: var(--parsec-color-light-primary-700);
    pointer-events: none;

    &::part(native) {
      cursor: default;
    }

    .main-icon {
      color: var(--parsec-color-light-primary-700);
      margin-right: 0.5rem;
    }
  }
}
</style>
