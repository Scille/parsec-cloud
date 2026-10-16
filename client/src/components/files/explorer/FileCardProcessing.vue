<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item class="file-card-item ion-no-padding">
    <div class="card-content">
      <ms-spinner
        class="card-content__spinner"
        :size="20"
      />
      <ion-avatar class="card-content-icons">
        <ion-icon
          class="icon-item"
          :icon="documentIcon"
        />
      </ion-avatar>

      <ion-title class="card-content__title">
        {{ operation.entryName }}
      </ion-title>

      <ion-text class="card-content-last-update">
        <span>{{ $msTranslate(operationLabel) }}</span>
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { FileOperationCurrentFolder } from '@/components/files/types';
import { FileOperationDataType } from '@/services/fileOperation';
import { IonAvatar, IonIcon, IonItem, IonText, IonTitle } from '@ionic/vue';
import { document as documentIcon } from 'ionicons/icons';
import { MsSpinner, Translatable } from 'megashark-lib';

const props = defineProps<{
  operation: FileOperationCurrentFolder;
}>();

const operationLabel: Translatable = (() => {
  if (props.operation.type === FileOperationDataType.Copy) {
    return 'FoldersPage.File.copying';
  } else if (props.operation.type === FileOperationDataType.Move) {
    return 'FoldersPage.File.moving';
  } else if (props.operation.type === FileOperationDataType.Restore) {
    return 'FoldersPage.File.restoring';
  }
  return 'FoldersPage.File.importing';
})();
</script>

<style lang="scss" scoped>
.file-card-item {
  --background: #{ms.color('surface-base-default-secondary')};
  cursor: default;
  text-align: center;
  user-select: none;
  width: 10.5rem;
  transition: width 0.2s ease-in-out;

  @include ms.responsive-breakpoint('xs') {
    width: 9rem;
  }

  @include ms.responsive-breakpoint('xs') {
    width: 8rem;
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: ms.spacing('padding-4xl') ms.spacing('padding-lg');
  width: 100%;

  &__spinner {
    position: absolute;
    top: 0.75rem;
    left: 0.75rem;
  }

  &-icons {
    position: relative;
    color: ms.color('icon-brand-default-hover');
    height: fit-content;
    width: fit-content;
    margin: 0 auto 0.875rem;

    .icon-item {
      font-size: 3rem;
    }
  }

  &__title {
    @include ms.font('label-md-medium');
    color: ms.color('text-base-body');
    text-align: center;
    padding: ms.spacing('padding-none') ms.spacing('padding-none') ms.spacing('padding-sm');
    text-overflow: ellipsis;
    white-space: nowrap;
    width: inherit;

    ion-text {
      width: 100%;
      overflow: hidden;
    }
  }
}

.card-content-last-update {
  @include ms.font('body-sm-regular');
  color: ms.color('text-base-description');
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

/* No idea how to change the color of the ion-item */
.card-content__title::part(native),
.card-content-last-update::part(native) {
  background-color: ms.color('surface-base-default-secondary');
}
</style>
