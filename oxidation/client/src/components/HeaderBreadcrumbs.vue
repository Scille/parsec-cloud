<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

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
      class="breadcrumb-element"
      :key="path.id"
    >
      <ion-icon
        class="icon"
        v-if="path.icon"
        :icon="path.icon"
      />

      {{ path.display }}
      <ion-icon
        slot="separator"
        :icon="caretForward"
      />
    </ion-breadcrumb>
  </ion-breadcrumbs>
</template>

<script lang="ts">
export interface RouterPathNode {
  id: number,
  display: string,
  icon?: string,
  name: string,
  params?: any,
  query?: any
}
</script>

<script setup lang="ts">
import { ref, Ref, computed } from 'vue';
import {
  IonBreadcrumb,
  IonBreadcrumbs,
  IonIcon
} from '@ionic/vue';
import {
  caretForward
} from 'ionicons/icons';
import { useRouter } from 'vue-router';

const props = defineProps<{
  pathNodes: RouterPathNode[]
}>();

// Using a computed to reset maxBreadcrumbs value automatically
const fullPath = computed(() => {
  // eslint-disable-next-line vue/no-side-effects-in-computed-properties
  maxBreadcrumbs.value = 4;
  return props.pathNodes;
});

const maxBreadcrumbs: Ref<number | undefined> = ref(4);
const router = useRouter();
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
  router.push({name: path.name, params: path.params, query: path.query});
}
</script>

<style scoped lang="scss">
.breadcrumb-element {
  cursor: pointer;
}

.breadcrumb-active {
  .icon {
    color: var(--parsec-color-light-primary-700);
  }
}

.breadcrumb {
  padding: 0;
  color: var(--parsec-color-light-secondary-grey);

  &-active {
    color: var(--parsec-color-light-primary-700)
  }
}

</style>
