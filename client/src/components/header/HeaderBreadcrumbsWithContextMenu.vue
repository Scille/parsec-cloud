<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="header-breadcrumbs">
    <ion-breadcrumbs
      v-if="isLargeDisplay"
      class="breadcrumb-container"
      @ion-collapsed-click="openPopover($event)"
      :max-items="maxShown"
      :items-before-collapse="itemsBeforeCollapse"
      :items-after-collapse="itemsAfterCollapse"
    >
      <ion-breadcrumb
        v-for="path in pathNodes"
        @click="path.display !== currentFolderName ? navigateTo(path) : breadcrumbOptionClick(path, $event)"
        :path="path"
        class="breadcrumb-element breadcrumb-normal"
        :class="{
          'breadcrumb-element--home': props.pathNodes.length === 1,
          'breadcrumb-element--active': path.display === currentFolderName || props.pathNodes.length === 1,
        }"
        :key="path.id"
        ref="breadcrumb"
      >
        <ion-icon
          class="main-icon"
          v-if="path.icon"
          :icon="path.icon"
        />
        <div class="breadcrumb-text">
          {{ path.display ? path.display : $msTranslate(path.title) }}
        </div>
        <ion-icon
          v-if="currentFolderName === path.display"
          :icon="ellipsisHorizontal"
          class="option-icon"
        />
      </ion-breadcrumb>
    </ion-breadcrumbs>
    <div
      v-if="isSmallDisplay && props.pathNodes.length > 0"
      class="breadcrumb-file-mobile"
      :class="{ is_browsing: props.pathNodes.length > 2 }"
      @click="props.pathNodes.length > 2 ? openPopover($event) : null"
    >
      <ion-text class="breadcrumb-file-mobile__title title-h3">{{ currentFolderName }}</ion-text>
      <ion-icon
        v-if="props.pathNodes.length > 2"
        class="breadcrumb-file-mobile__icon"
        :icon="chevronDown"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import HeaderBreadcrumbPopover from '@/components/header/HeaderBreadcrumbPopover.vue';
import { RouterPathNode } from '@/components/header/utils';
import { WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { EventDistributor } from '@/services/eventDistributor';
import { InformationManager } from '@/services/informationManager';
import { StorageManager } from '@/services/storageManager';
import { WorkspaceAttributes } from '@/services/workspaceAttributes';
import { IonBreadcrumb, IonBreadcrumbs, IonIcon, IonText, popoverController } from '@ionic/vue';
import { chevronDown, ellipsisHorizontal } from 'ionicons/icons';
import { useWindowSize } from 'megashark-lib';
import { computed, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = withDefaults(
  defineProps<{
    pathNodes: RouterPathNode[];
    itemsBeforeCollapse?: number;
    itemsAfterCollapse?: number;
    maxShown?: number;
    workspaceAttributes: WorkspaceAttributes;
    workspace: WorkspaceInfo | undefined;
    ownRole: WorkspaceRole | undefined;
    eventDistributor: EventDistributor;
    informationManager: InformationManager;
    storageManager: StorageManager;
    availableWidth?: number;
  }>(),
  {
    itemsBeforeCollapse: 2,
    itemsAfterCollapse: 1,
    maxShown: 3,
    availableWidth: 0,
  },
);

const { windowWidth, isLargeDisplay, isSmallDisplay } = useWindowSize();
const breadcrumbRef = useTemplateRef<HTMLIonBreadcrumbElement>('breadcrumb');
const breadcrumbWidthProperty = ref('');
let ignoreNextEvent = false;

const watchWindowWidthCancel = watch(windowWidth, () => {
  setBreadcrumbWidth();
});

const watchNodeSizeCancel = watch(
  () => props.pathNodes.length,
  () => {
    setBreadcrumbWidth();
  },
);

const emits = defineEmits<{
  (e: 'change', node: RouterPathNode): void;
  (e: 'openWorkspaceContextMenu', event: Event): void;
  (e: 'openFolderContextMenu', event: Event): void;
}>();

onMounted(() => {
  setBreadcrumbWidth();
});

onUnmounted(() => {
  watchWindowWidthCancel();
  watchNodeSizeCancel();
});

const currentFolderName = computed(() => {
  if (props.pathNodes.length === 0) {
    return '';
  }

  if (props.pathNodes.length === 1) {
    return props.workspace ? props.workspace.currentName : '';
  }

  const lastNode = props.pathNodes[props.pathNodes.length - 1];
  return lastNode.display || (props.workspace ? props.workspace.currentName : '');
});

async function breadcrumbOptionClick(path: RouterPathNode, event: Event): Promise<void> {
  if (path.display === (props.workspace ? props.workspace.currentName : '')) {
    emits('openWorkspaceContextMenu', event);
  } else {
    emits('openFolderContextMenu', event);
  }
}

function setBreadcrumbWidth(): void {
  if (props.availableWidth > 0 && breadcrumbRef.value) {
    let visibleNodes = Math.min(props.pathNodes.length, props.maxShown);
    let breadcrumbWidth = props.availableWidth - 1;

    if (props.pathNodes.length > props.maxShown || (isSmallDisplay.value && props.pathNodes.length !== 1)) {
      // Deduce collapsed element or popover button width if present
      breadcrumbWidth -= isLargeDisplay.value ? 4.125 : 5.375;
    }

    if (isLargeDisplay.value) {
      if (props.pathNodes.length <= props.maxShown) {
        // Deduce separator(s) width if present, 1.25 rem / separator
        breadcrumbWidth -= props.pathNodes.length === 2 ? 1.25 : 2.5;
      }

      visibleNodes -= 1;
      breadcrumbWidth -= 2.4;
      // Small display only has one element so this division is done only on large display
      breadcrumbWidth /= visibleNodes;
    }
    breadcrumbWidthProperty.value = `${breadcrumbWidth}rem`;
  }
}

function getCollapsedItems(): Array<RouterPathNode> {
  if (isLargeDisplay.value) {
    return props.pathNodes.slice(props.itemsBeforeCollapse, props.itemsBeforeCollapse + props.pathNodes.length - props.maxShown);
  }
  return props.pathNodes.slice(0, props.pathNodes.length - 1);
}

async function openPopover(event: Event): Promise<void> {
  ignoreNextEvent = true;
  const popover = await popoverController.create({
    component: HeaderBreadcrumbPopover,
    cssClass: 'breadcrumbs-popover',
    alignment: 'center',
    event: event,
    showBackdrop: false,
    componentProps: {
      breadcrumbs: getCollapsedItems(),
    },
  });
  await popover.present();
  const result = await popover.onDidDismiss();
  await popover.dismiss();
  if (result.data && result.data.breadcrumb) {
    navigateTo(result.data.breadcrumb);
  }
}

function navigateTo(path: RouterPathNode): void {
  // ignoreNextEvent is used so that when "..." is clicked, we
  // don't try to travel. Didn't find a better way to do this.
  if (isLargeDisplay.value && ignoreNextEvent) {
    ignoreNextEvent = false;
    return;
  }
  emits('change', path);
}
</script>

<style scoped lang="scss">
.breadcrumb-element {
  &::part(native) {
    max-width: calc(v-bind(breadcrumbWidthProperty));
  }

  .option-icon {
    font-size: 1.25rem;
    padding: 0.25rem;
    color: var(--parsec-color-light-secondary-text);
    margin-left: 0.25rem;
    opacity: 0.3;
    pointer-events: all;
    flex-shrink: 0;
  }

  &:hover:not(.breadcrumb-collapsed) {
    .option-icon {
      color: var(--parsec-color-light-primary-700);
      opacity: 1;
    }
  }
}
</style>
