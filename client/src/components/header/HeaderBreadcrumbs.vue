<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <ion-breadcrumbs
      v-show="isLargeDisplay"
      class="breadcrumb"
      @ion-collapsed-click="openPopover($event)"
      :max-items="maxBreadcrumbs"
      :items-before-collapse="itemsBeforeCollapse"
      :items-after-collapse="itemsAfterCollapse"
    >
      <ion-breadcrumb
        v-for="path in pathNodes"
        @click="navigateTo(path)"
        :path="path"
        class="breadcrumb-element breadcrumb-normal"
        :key="path.id"
        ref="breadcrumbRef"
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
        class="breadcrumb-normal breadcrumb-small"
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
import { IonBreadcrumb, IonBreadcrumbs, IonButton, IonIcon, IonText, popoverController } from '@ionic/vue';
import { Ref, onMounted, onUnmounted, ref, watch } from 'vue';
import HeaderBreadcrumbPopover from '@/components/header/HeaderBreadcrumbPopover.vue';
import { caretDown } from 'ionicons/icons';

const { windowWidth, isLargeDisplay, isSmallDisplay } = useWindowSize();
const breadcrumbRef = ref();
const breadcrumbWidthProperty = ref('');

function setBreadcrumbWidth(): void {
  if (props.availableWidth > 0 && breadcrumbRef.value) {
    let visibleNodes = props.pathNodes.length > props.maxShown ? props.maxShown : props.pathNodes.length;
    visibleNodes -= props.fromHeaderPage ? 1 : 0;
    let breadcrumbWidth = props.availableWidth - 1;

    if (props.fromHeaderPage) {
      breadcrumbWidth -= breadcrumbRef.value[0].$el.clientWidth / 16; // First element is static in headerpage
    }
    if (props.pathNodes.length === 3 || (!props.fromHeaderPage && props.pathNodes.length === 2)) {
      breadcrumbWidth -= props.fromHeaderPage || props.pathNodes.length === 2 ? 1.25 : 2.5; // separator(s)
    } else if (props.pathNodes.length > props.maxShown) {
      breadcrumbWidth -= props.fromHeaderPage ? 4.125 : 5.375; // collapsed element + additional separator
    }
    breadcrumbWidthProperty.value = `${breadcrumbWidth / visibleNodes}rem`;
  }
}

const props = withDefaults(
  defineProps<{
    pathNodes: RouterPathNode[];
    itemsBeforeCollapse?: number;
    itemsAfterCollapse?: number;
    maxShown?: number;
    fromHeaderPage?: boolean;
    availableWidth?: number;
  }>(),
  {
    itemsBeforeCollapse: 2,
    itemsAfterCollapse: 1,
    maxShown: 3,
    fromHeaderPage: false,
    availableWidth: 0,
  },
);

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

const maxBreadcrumbs: Ref<number | undefined> = ref(props.maxShown);
let ignoreNextEvent = false;

function getCollapsedItems(): Array<RouterPathNode> {
  if (isLargeDisplay.value) {
    return props.pathNodes.slice(props.itemsBeforeCollapse, props.itemsBeforeCollapse + props.pathNodes.length - props.maxShown);
  }
  if (props.fromHeaderPage) {
    return props.pathNodes.slice(1, props.pathNodes.length - 1);
  }
  return props.pathNodes.slice(0, props.pathNodes.length - 1);
}

async function openPopover(event: CustomEvent): Promise<void> {
  ignoreNextEvent = true;
  const popover = await popoverController.create({
    component: HeaderBreadcrumbPopover,
    alignment: 'center',
    event: event,
    showBackdrop: false,
    componentProps: {
      breadcrumbs: getCollapsedItems(),
      firstElementIsWorkspace: isSmallDisplay.value && props.fromHeaderPage,
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
    transition: max-width 0.2s ease-in-out;

    @include ms.responsive-breakpoint('sm') {
      max-width: 17.5rem;
    }

    @include ms.responsive-breakpoint('xs') {
      max-width: 6.25rem;
    }

    &-active {
      color: var(--parsec-color-light-primary-700);
    }
  }

  .breadcrumb-popover-button {
    min-height: 0;
    padding: 0.125rem;

    &::part(native) {
      --background: none;
      --background-hover: none;
      --background-focused: none;
      border: 1px solid var(--parsec-color-light-secondary-light);
      padding: 0.25rem;
    }

    &__icon {
      font-size: 1rem;
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }
}
</style>
