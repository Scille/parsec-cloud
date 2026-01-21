<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="recommendation-checklist-container">
    <div class="checklist-header">
      <ion-text class="checklist-header__title title-h4">{{ $msTranslate('SideMenu.checklist.title') }}</ion-text>
      <ion-text class="checklist-header__description body">{{ $msTranslate('SideMenu.checklist.description') }}</ion-text>
    </div>
    <div class="checklist-list">
      <div
        class="checklist-list-item subtitles-sm"
        v-show="securityWarnings.needsSecondOwner"
        :class="{ done: securityWarnings.soloOwnerWorkspaces.length === 0, clickable: workspaceWarningClickable }"
        @click="workspaceWarningClickable && onClick(RecommendationAction.AddWorkspaceOwner)"
      >
        <ion-icon
          class="checklist-list-item__icon icon-left"
          :icon="securityWarnings.soloOwnerWorkspaces.length === 0 ? checkmarkCircle : radioButtonOff"
        />
        <ion-text class="checklist-list-item__text">
          {{
            $msTranslate({
              key: 'SideMenu.checklist.items.ownerRequired',
              count: securityWarnings.soloOwnerWorkspaces.length,
              data: {
                workspace: securityWarnings.soloOwnerWorkspaces.length === 1 ? securityWarnings.soloOwnerWorkspaces[0].currentName : '',
                workspacesCount: securityWarnings.soloOwnerWorkspaces.length,
              },
            })
          }}
        </ion-text>
        <ion-icon
          class="checklist-list-item__icon icon-right"
          v-show="workspaceWarningClickable"
          :icon="chevronForward"
        />
      </div>
      <div
        class="checklist-list-item subtitles-sm"
        :class="{ done: securityWarnings.hasMultipleDevices, clickable: !securityWarnings.hasMultipleDevices }"
        @click="!securityWarnings.hasMultipleDevices && onClick(RecommendationAction.AddDevice)"
      >
        <ion-icon
          class="checklist-list-item__icon icon-left"
          :icon="securityWarnings.hasMultipleDevices ? checkmarkCircle : radioButtonOff"
        />
        <ion-text class="checklist-list-item__text">{{ $msTranslate('SideMenu.checklist.items.multipleDevices') }}</ion-text>
        <ion-icon
          class="checklist-list-item__icon icon-right"
          v-show="!securityWarnings.hasMultipleDevices"
          :icon="chevronForward"
        />
      </div>
      <div
        class="checklist-list-item subtitles-sm"
        :class="{ done: securityWarnings.hasRecoveryDevice, clickable: !securityWarnings.hasRecoveryDevice }"
        @click="!securityWarnings.hasRecoveryDevice && onClick(RecommendationAction.CreateRecoveryFiles)"
      >
        <ion-icon
          class="checklist-list-item__icon icon-left"
          :icon="securityWarnings.hasRecoveryDevice ? checkmarkCircle : radioButtonOff"
        />
        <ion-text class="checklist-list-item__text">{{ $msTranslate('SideMenu.checklist.items.createRecoveryFiles') }}</ion-text>
        <ion-icon
          class="checklist-list-item__icon icon-right"
          v-show="!securityWarnings.hasRecoveryDevice"
          :icon="chevronForward"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RecommendationAction, SecurityWarnings } from '@/components/misc/securityRecommendations';
import { currentRouteIs, Routes } from '@/router';
import { IonIcon, IonText, modalController, popoverController } from '@ionic/vue';
import { checkmarkCircle, chevronForward, radioButtonOff } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  securityWarnings: SecurityWarnings;
  isModal?: boolean;
}>();

const workspaceWarningClickable = computed(() => {
  return props.securityWarnings.soloOwnerWorkspaces.length > 0 && !currentRouteIs(Routes.Workspaces);
});

async function onClick(action: RecommendationAction): Promise<void> {
  if (props.isModal) {
    await modalController.dismiss({ action: action }, MsModalResult.Confirm);
  } else {
    await popoverController.dismiss({ action: action }, MsModalResult.Confirm);
  }
}
</script>

<style scoped lang="scss">
.recommendation-checklist-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.5rem 1.25rem;
  background-color: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
}

.checklist-header {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &__title {
    background: var(--parsec-color-light-gradient-background);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  &__description {
    color: var(--parsec-color-light-secondary-soft-text);
  }
}

.checklist-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &-item {
    order: 1;
    display: flex;
    gap: 0.5rem;
    padding: 0.625rem 0.5rem;
    align-items: center;
    text-wrap: wrap;
    box-shadow: var(--parsec-shadow-soft);
    background-color: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-8);
    transition: all 0.2s ease-in-out;

    &.clickable {
      cursor: pointer;
    }

    &__icon {
      color: var(--parsec-color-light-secondary-text);
      flex-shrink: 0;

      &.icon-left {
        font-size: 1.125rem;
        color: var(--parsec-color-light-secondary-grey);
      }

      &.icon-right {
        font-size: 0.825rem;
        position: relative;
        right: 0;
        transition: right 0.2s ease-in-out;
      }
    }

    &__text {
      color: var(--parsec-color-light-secondary-text);
      font-size: 0.875rem;
      width: 100%;
    }

    &:hover {
      background-color: var(--parsec-color-light-secondary-white);
      box-shadow: var(--parsec-shadow-soft);

      .icon-right {
        right: -0.25rem;
      }
    }

    &.done {
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-background);
      box-shadow: none;
      cursor: default;
      order: 0;

      .icon-left {
        color: var(--parsec-color-light-primary-500);
      }
    }
  }
}
</style>
