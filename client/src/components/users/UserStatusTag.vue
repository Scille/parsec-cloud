<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label
    class="label-status status"
    @click="showTooltip ? openInformationTooltip($event, getStatusDescription()) : null"
  >
    <ion-chip
      class="tag-status button-medium"
      :class="{
        revoked: revoked,
        active: !revoked && !frozen,
        frozen: frozen,
      }"
    >
      <ion-icon
        :icon="revoked ? closeCircle : frozen ? pauseCircle : checkmarkCircle"
        :class="{ 'revoked-icon': revoked, 'active-icon': !revoked && !frozen, 'frozen-icon': frozen }"
      />
      {{ $msTranslate(statusText) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { IonChip, IonIcon, IonLabel } from '@ionic/vue';
import { checkmarkCircle, closeCircle, pauseCircle } from 'ionicons/icons';
import { Translatable, openInformationTooltip } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  revoked: boolean;
  frozen?: boolean;
  showTooltip?: boolean;
}>();

const statusText = computed((): Translatable => {
  if (props.revoked) {
    return 'UsersPage.status.revoked';
  }
  if (props.frozen) {
    return 'UsersPage.status.frozen';
  }
  return 'UsersPage.status.active';
});

function getStatusDescription(): Translatable {
  if (props.revoked) {
    return 'UsersPage.statusDescriptions.revoked';
  }
  if (props.frozen) {
    return 'UsersPage.statusDescriptions.frozen';
  }
  return 'UsersPage.statusDescriptions.active';
}
</script>

<style scoped lang="scss">
.label-status {
  margin: 0;
}

.tag-status {
  margin: 0;
  padding: 0.125rem 0;
  height: fit-content;
  min-height: 0;
  display: flex;
  gap: 0.25rem;
  cursor: default;
  --background: none;
  background: none;

  &:hover {
    cursor: pointer;
  }
}

* {
  --revoked-color: var(--parsec-color-light-secondary-hard-grey);
  --revoked-hover-color: var(--parsec-color-light-secondary-text);

  --active-color: var(--parsec-color-light-info-500);
  --active-hover-color: var(--parsec-color-light-info-700);

  --frozen-color: var(--parsec-color-light-warning-500);
  --frozen-hover-color: var(--parsec-color-light-warning-700);
}

.revoked {
  color: var(--revoked-color);

  &-icon {
    color: var(--revoked-color) !important;
  }

  &:hover {
    color: var(--revoked-hover-color);
  }
}

.active {
  color: var(--active-color);

  &-icon {
    color: var(--active-color) !important;
  }

  &:hover {
    color: var(--active-hover-color);
  }
}

.frozen {
  color: var(--frozen-color);

  &-icon {
    color: var(--frozen-color) !important;
  }

  &:hover {
    color: var(--frozen-hover-color);
  }
}

ion-icon {
  font-size: 1.15rem;
  flex-shrink: 0;
  margin-right: 0 !important;
}
</style>
