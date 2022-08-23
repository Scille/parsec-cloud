<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container">
    <ion-icon
      :icon="person"
      color="secondary"
      class="user-icon"
    />
    <div class="text-container">
      <ion-label class="text-labels ion-text-wrap">
        {{ name }}
        <p class="secondary-text ion-text-wrap">
          {{ email }}
        </p>
      </ion-label>
      <ion-select
        class="rightmost"
        :interface="isPlatform('mobile') ? 'action-sheet' : 'popover'"
        :interface-options="isPlatform('mobile') ? userRoleActionSheetOptions : undefined"
        :selected-text="role"
        @ion-change="changeRole($event.detail.value)"
      >
        <ion-select-option value="notShared">
          {{ $t('WorkspacesPage.WorkspacesShareModal.notShared') }}
        </ion-select-option>
        <ion-select-option value="reader">
          {{ $t('WorkspacesPage.WorkspacesShareModal.reader') }}
        </ion-select-option>
        <ion-select-option value="contributor">
          {{ $t('WorkspacesPage.WorkspacesShareModal.contributor') }}
        </ion-select-option>
        <ion-select-option value="manager">
          {{ $t('WorkspacesPage.WorkspacesShareModal.manager') }}
        </ion-select-option>
        <ion-select-option value="owner">
          {{ $t('WorkspacesPage.WorkspacesShareModal.owner') }}
        </ion-select-option>
      </ion-select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import { IonIcon, IonLabel, IonSelect, IonSelectOption, isPlatform } from '@ionic/vue';
import { person } from 'ionicons/icons';

const props = defineProps<{
  name: string
  email: string
  role: string
}>();

const emit = defineEmits<{
  (event: 'trigger-action-sheet'): void
}>();

function changeRole(role: string): void {
  console.log('role selected is:', role);
}

const userRoleActionSheetOptions = {
  header: props.name
};
</script>

<style lang="scss" scoped>
.user-icon {
    font-size: 32px;
}

.text-labels {
    padding-inline: 1em;
}

p {
    margin: 0;
}

.secondary-text {
    color: var(--ion-color-medium);
}

.container {
    display: flex;
    flex-direction: row;
    align-items: center;
    width: 100%;
}

$SCREEN_SM: "576px";

.text-container {
    display: flex;
    flex-direction: row;
    width: 100%;
    padding-top: 8px;
    padding-bottom: 8px;
    @media screen and (max-width: $SCREEN_SM) {
        flex-direction: column;
        align-items: flex-start;
        max-width: 80vw;
    }
}

.rightmost {
    @media screen and (min-width: $SCREEN_SM) {
        margin-left: auto;
    }
}
</style>
