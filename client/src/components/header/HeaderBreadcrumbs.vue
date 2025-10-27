<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="header-breadcrumbs">
    <ion-breadcrumbs
      v-show="isLargeDisplay"
      class="breadcrumb"
      @ion-collapsed-click="openPopover($event)"
      :max-items="maxShown"
      :items-before-collapse="itemsBeforeCollapse"
      :items-after-collapse="itemsAfterCollapse"
    >
      <ion-breadcrumb
        v-for="path in pathNodes"
        @click="navigateTo(path)"
        :path="path"
        class="breadcrumb-element breadcrumb-normal"
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
      </ion-breadcrumb>
    </ion-breadcrumbs>
    <div
      v-if="isSmallDisplay && pathNodes.length > (fromHeaderPage ? 2 : 0)"
      class="breadcrumb-small-container"
    >
      <template v-if="showParentNode">
        <ion-text
          v-if="pathNodes.length > (fromHeaderPage ? 3 : 1)"
          class="breadcrumb-normal breadcrumb-small"
        >
          {{ '...' }}
        </ion-text>
        <ion-text
          v-if="pathNodes.length > (fromHeaderPage ? 3 : 1)"
          class="breadcrumb-normal breadcrumb-small"
        >
          {{ '/' }}
        </ion-text>
        <ion-text
          class="breadcrumb-normal breadcrumb-small breadcrumb-small-text"
          :class="fromHeaderPage ? '' : 'breadcrumb-small-active'"
        >
          {{ `${pathNodes[pathNodes.length - (fromHeaderPage ? 2 : 1)].display}` }}
        </ion-text>
        <ion-text
          v-if="fromHeaderPage"
          class="breadcrumb-normal breadcrumb-small"
        >
          {{ '/' }}
        </ion-text>
      </template>
      <div class="breadcrumb-popover-button-container">
        <ion-button
          v-if="pathNodes.length > 1"
          @click="openPopover"
          class="breadcrumb-popover-button"
          fill="outline"
        >
          <ion-icon
            :icon="caretDown"
            slot="icon-only"
            class="breadcrumb-popover-button__icon"
          />
        </ion-button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export interface RouterPathNode {
  id: number;
  display?: string;
  title?: Translatable;
  icon?: string;
  popoverIcon?: string;
  route: Routes;
  params?: object;
  query?: Query;
}
</script>

<script setup lang="ts">
import HeaderBreadcrumbPopover from '@/components/header/HeaderBreadcrumbPopover.vue';
import { Query, Routes } from '@/router';
import { IonBreadcrumb, IonBreadcrumbs, IonButton, IonIcon, IonText, popoverController } from '@ionic/vue';
import { caretDown } from 'ionicons/icons';
import { Translatable, useWindowSize } from 'megashark-lib';
import { onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = withDefaults(
  defineProps<{
    pathNodes: RouterPathNode[];
    itemsBeforeCollapse?: number;
    itemsAfterCollapse?: number;
    maxShown?: number;
    fromHeaderPage?: boolean;
    availableWidth?: number;
    showParentNode?: boolean;
  }>(),
  {
    itemsBeforeCollapse: 2,
    itemsAfterCollapse: 1,
    maxShown: 3,
    fromHeaderPage: false,
    availableWidth: 0,
    showParentNode: true,
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
}>();

onMounted(() => {
  setBreadcrumbWidth();
});

onUnmounted(() => {
  watchWindowWidthCancel();
  watchNodeSizeCancel();
});

function setBreadcrumbWidth(): void {
  if (props.availableWidth > 0 && breadcrumbRef.value) {
    let visibleNodes = Math.min(props.pathNodes.length, props.maxShown);
    let breadcrumbWidth = props.availableWidth - 1;

    if (isSmallDisplay.value && props.fromHeaderPage && props.pathNodes.length === props.maxShown) {
      // "... /" not showing, difference of 2rem
      breadcrumbWidth += 2;
    }
    if (props.pathNodes.length > props.maxShown || (isSmallDisplay.value && props.pathNodes.length !== 1)) {
      // Deduce collapsed element or popover button width if present
      breadcrumbWidth -= props.fromHeaderPage && isLargeDisplay.value ? 4.125 : 5.375;
    }

    if (isLargeDisplay.value) {
      if (props.pathNodes.length <= props.maxShown) {
        // Deduce separator(s) width if present, 1.25 rem / separator
        breadcrumbWidth -= props.fromHeaderPage || props.pathNodes.length === 2 ? 1.25 : 2.5;
      }
      if (props.fromHeaderPage) {
        // First element is static in headerpage, we deduce its 2.4rem width
        visibleNodes -= 1;
        breadcrumbWidth -= 2.4;
      }
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

async function openPopover(event: CustomEvent): Promise<void> {
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
.header-breadcrumbs {
  display: flex;
  align-items: center;
  width: 100%;
}

.breadcrumb {
  padding: 0;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  flex-wrap: nowrap;

  &-element {
    .main-icon {
      font-size: 1.125rem;
    }

    &::part(native) {
      cursor: pointer;
      padding: 0.25rem 0.5rem;
      max-width: calc(v-bind(breadcrumbWidthProperty));
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

  &-text {
    overflow-x: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

.breadcrumb-small-container {
  display: flex;
  align-items: center;
  color: var(--parsec-color-light-secondary-grey);
  gap: 0.5rem;

  .breadcrumb-small {
    color: var(--parsec-color-light-secondary-grey);
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
    max-width: calc(v-bind(breadcrumbWidthProperty));

    &-active {
      color: var(--parsec-color-light-primary-700);
    }
  }

  .breadcrumb-popover-button {
    min-width: 0;
    padding: 0;
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);

    &::part(native) {
      --background-hover: none;
      --background-focused: none;
      border: none;
      padding: 0.375rem;
    }

    &-container {
      display: flex;
    }

    &__icon {
      font-size: 0.875rem;
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &:hover .breadcrumb-popover-button__icon {
      color: var(--parsec-color-light-secondary-text);
    }
  }
}
</style>
