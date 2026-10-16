<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="header-breadcrumbs">
    <ion-breadcrumbs
      v-if="isLargeDisplay"
      class="breadcrumb"
      @ion-collapsed-click="openPopover($event)"
      :max-items="maxShown"
      :items-before-collapse="itemsBeforeCollapse"
      :items-after-collapse="itemsAfterCollapse"
    >
      <ion-breadcrumb
        v-for="path in pathNodes"
        :path="path"
        class="breadcrumb-element"
        :key="path.id"
        ref="breadcrumb"
      >
        <div
          @click="$emit('change', path)"
          class="breadcrumb-item-content"
        >
          <ion-icon
            class="main-icon"
            v-if="path.icon"
            :icon="path.icon"
          />
          <div
            class="breadcrumb-text"
            v-if="path.display || path.title"
          >
            {{ path.display ? path.display : $msTranslate(path.title) }}
          </div>
        </div>
      </ion-breadcrumb>
    </ion-breadcrumbs>
    <div
      v-if="isSmallDisplay && props.pathNodes.length > 0"
      class="breadcrumb-file-mobile"
      :class="{ is_browsing: props.pathNodes.length > (fromHeaderPage ? 2 : 1) }"
      @click="props.pathNodes.length > (fromHeaderPage ? 2 : 1) ? openPopover($event) : null"
    >
      <ion-text class="breadcrumb-file-mobile__title">{{ currentFolderName }}</ion-text>
      <ion-icon
        v-if="props.pathNodes.length > (fromHeaderPage ? 2 : 1)"
        class="breadcrumb-file-mobile__icon"
        :icon="chevronDown"
      />
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
import { WorkspaceName } from '@/parsec';
import { Query, Routes } from '@/router';
import { IonBreadcrumb, IonBreadcrumbs, IonIcon, IonText, popoverController } from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
import { Translatable, useWindowSize } from 'megashark-lib';
import { computed, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = withDefaults(
  defineProps<{
    workspaceName: WorkspaceName;
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

const currentFolderName = computed(() => {
  if (props.pathNodes.length === 0) {
    return '';
  }

  if (props.pathNodes.length === 1) {
    return props.workspaceName;
  }

  const lastNode = props.pathNodes[props.pathNodes.length - 1];
  return lastNode.display || props.workspaceName;
});

function setBreadcrumbWidth(): void {
  if (props.availableWidth > 0 && breadcrumbRef.value) {
    let visibleNodes = Math.min(props.pathNodes.length, props.maxShown);
    let breadcrumbWidth = props.availableWidth - 1;

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

async function openPopover(event: Event): Promise<void> {
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
    emits('change', result.data.breadcrumb);
  }
}
</script>

<style scoped lang="scss">
.header-breadcrumbs {
  display: flex;
  align-items: center;
  width: 100%;

  @include ms.responsive-breakpoint('sm') {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }
}

.breadcrumb {
  padding: ms.spacing('padding-none');
  opacity: ms.opacity('7');
  display: flex;
  flex-wrap: nowrap;
  user-select: none;

  &-item-content {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    width: 100%;
    padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
    position: relative;
    cursor: pointer;

    &::after {
      content: '';
      position: absolute;
      width: 100%;
      height: 100%;
      opacity: ms.opacity('6');
      z-index: -10;
      left: 0;
      border-radius: ms.radius('lg');
    }

    &:hover:not(.breadcrumb-collapsed) {
      color: ms.color('text-neutral-default-hover');
      position: relative;

      .main-icon {
        color: ms.color('text-neutral-default-hover');
      }

      &::after {
        background: ms.color('surface-neutral-default-subtle-hover');
      }
    }
  }

  &-element {
    --color: #{ms.color('text-base-label')};
    @include ms.font('label-md-medium');

    .main-icon {
      font-size: 1.125rem;
    }

    &::part(native) {
      cursor: default;
      padding: ms.spacing('padding-none');
      max-width: calc(v-bind(breadcrumbWidthProperty));
    }

    &:nth-child(1) {
      &::part(native) {
        display: flex;
        gap: ms.spacing('gap-lg');
      }
    }

    &::part(separator) {
      margin-inline: 0;
      color: ms.color('text-neutral-default');
    }

    &::part(collapsed-indicator) {
      border-radius: ms.radius('lg');
      background: ms.color('surface-neutral-default-subtle');
      color: ms.color('text-neutral-default');
      margin-inline: 0.5rem;
    }
  }

  // Defined by ionic
  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  &-active {
    @include ms.font('label-md-emphasis');
    pointer-events: none;

    &::part(native) {
      cursor: default;
    }
  }

  &-text {
    overflow-x: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

.breadcrumb-file-mobile {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-md');
  padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
  border-radius: ms.radius('lg');
  overflow: hidden;
  width: fit-content;
  white-space: nowrap;
  text-overflow: ellipsis;

  * {
    transition: all 0.15s ease-in-out;
  }

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-brand-default');
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  &__icon {
    color: ms.color('text-base-body');
    font-size: 0.75rem;
    padding: ms.spacing('padding-xs');
    flex-shrink: 0;
    border-radius: ms.radius('full');
    background: ms.color('surface-neutral-default-subtle');
  }

  &.is_browsing {
    cursor: pointer;
    transition: all 0.15s ease-in-out;

    &:hover {
      background: ms.color('surface-neutral-default-subtle-hover');

      .breadcrumb-file-mobile__icon {
        background: ms.color('surface-neutral-default-subtle-pressed');
        color: ms.color('icon-neutral-on-color');
      }
    }
  }
}
</style>
