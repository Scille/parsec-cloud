<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-breadcrumbs
    class="breadcrumb"
    @ion-collapsed-click="expandCollapsed()"
    :max-items="maxBreadcrumbs"
    :items-before-collapse="2"
    :items-after-collapse="2"
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
        v-if="path.icon"
        :icon="path.icon"
      />

      {{ path.display }}
      <ion-icon
        class="separator-icon"
        slot="separator"
        :icon="caretForward"
      />
    </ion-breadcrumb>
  </ion-breadcrumbs>
</template>

<script lang="ts">
export interface RouterPathNode {
  id: number;
  display: string;
  icon?: string;
  name: string;
  params?: any;
  query?: any;
}
</script>

<script setup lang="ts">
import { ref, Ref, computed } from 'vue';
import { IonBreadcrumb, IonBreadcrumbs, IonIcon } from '@ionic/vue';
import { caretForward } from 'ionicons/icons';

const props = defineProps<{
  pathNodes: RouterPathNode[];
}>();

const emits = defineEmits<{
  (e: 'change', node: RouterPathNode): void;
}>();

// Using a computed to reset maxBreadcrumbs value automatically
const fullPath = computed(() => {
  // eslint-disable-next-line vue/no-side-effects-in-computed-properties
  maxBreadcrumbs.value = 4;
  return props.pathNodes;
});

const maxBreadcrumbs: Ref<number | undefined> = ref(4);
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

  // Defined by ionic
  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  &-active {
    color: var(--parsec-color-light-primary-700);

    .main-icon {
      color: var(--parsec-color-light-primary-700);
      margin-right: 0.5rem;
    }
  }
}
.breadcrumb-element {
  cursor: pointer;

  .main-icon {
    font-size: 1.125rem;
  }

  .separator-icon {
    color: var(--parsec-color-light-secondary-grey);
    font-size: 0.75rem;
  }
}
</style>
