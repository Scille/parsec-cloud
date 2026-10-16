<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div
    class="tab-bar-menu"
    id="tab-bar-options"
    v-show="actions.length > 0"
  >
    <div
      v-for="(action, index) of displayedActions"
      :key="index"
      @click="action.action"
    >
      <div
        class="tab-bar-menu-button"
        :class="action.danger ? 'tab-bar-menu-button-danger' : ''"
      >
        <ion-icon
          class="tab-bar-menu-button__icon"
          v-if="action.icon && !action.image"
          :icon="action.icon"
        />
        <ms-image
          class="tab-bar-menu-button__icon"
          v-if="!action.icon && action.image"
          :image="action.image"
        />
        <ion-text class="tab-bar-menu-button__text">
          {{ $msTranslate(action.label) }}
        </ion-text>
      </div>
    </div>
    <div
      v-if="actions && actions.length > 4"
      @click="isMenuOpen ? closeMenuModal() : openMenuModal()"
    >
      <div class="tab-bar-menu-button">
        <ion-icon
          class="tab-bar-menu-button__icon"
          :icon="ellipsisHorizontal"
        />
        <ion-text class="tab-bar-menu-button__text">
          {{ $msTranslate(isMenuOpen ? 'SideMenu.tabbarOptions.less' : 'SideMenu.tabbarOptions.more') }}
        </ion-text>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import TabBarOptionsModal from '@/views/menu/TabBarOptionsModal.vue';
import { MenuAction } from '@/views/menu/types';
import { IonIcon, IonText, modalController } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { MsImage, MsModalResult } from 'megashark-lib';
import { onUnmounted, ref, watch } from 'vue';

const props = defineProps<{
  actions: MenuAction[];
}>();
const displayedActions = ref<MenuAction[]>(props.actions.slice(0, props.actions.length > 4 ? 3 : 4));
const additionalActions = ref<MenuAction[]>(props.actions.slice(3));

const watchActionsCancel = watch(
  () => props.actions,
  () => {
    displayedActions.value = props.actions.slice(0, props.actions.length > 4 ? 3 : 4);
    additionalActions.value = props.actions.slice(3);
  },
);

onUnmounted(() => {
  watchActionsCancel();
});

const isMenuOpen = ref(false);

async function closeMenuModal(): Promise<void> {
  if (!isMenuOpen.value) {
    return;
  }
  isMenuOpen.value = false;

  const modal = await modalController.getTop();
  if (modal) {
    await modal.dismiss();
  }
}

async function openMenuModal(): Promise<void> {
  if (isMenuOpen.value) {
    return;
  }
  isMenuOpen.value = true;

  const modal = await modalController.create({
    component: TabBarOptionsModal,
    cssClass: 'tab-menu-modal',
    showBackdrop: true,
    handle: true,
    backdropDismiss: true,
    breakpoints: [0, 1],
    expandToScroll: false,
    initialBreakpoint: 1,
    componentProps: {
      actions: additionalActions.value,
    },
  });
  await modal.present();
  const { data, role } = await modal.onDidDismiss();

  if (role === MsModalResult.Confirm && data?.action) {
    await data.action();
  }
  await modal.dismiss();
  isMenuOpen.value = false;
}
</script>

<style lang="scss" scoped>
.tab-bar-menu {
  background: ms.color('surface-base-default');
  padding: ms.spacing('padding-lg') ms.spacing('padding-3xl') ms.spacing('padding-4xl');
  justify-content: space-between;
  border: none;
  border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  position: relative;
  z-index: 19;
  width: 100%;
  display: grid;
  grid-template-columns: repeat(4, 1fr);

  &-button {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: ms.spacing('gap-md');
    cursor: pointer;
    color: ms.color('text-neutral-default');

    &-danger {
      color: ms.color('text-error-default');
    }

    &__icon {
      font-size: 1.5rem;
      width: 1.5rem;
      height: 1.5rem;
      --fill-color: #{ms.color('icon-neutral-default')};
    }

    &__text {
      @include ms.font('label-xs-medium');
      text-align: center;
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      max-width: 4.75rem;
      width: 100%;
    }
  }
}
</style>
