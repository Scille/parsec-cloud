<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-header>
    <ion-toolbar>
      <ion-title>{{ $t('JoinByLinkModal.pageTitle') }}</ion-title>
      <ion-buttons slot="end">
        <ion-button
          slot="icon-only"
          @click="closeModal()"
        >
          <ion-icon
            :icon="close"
            size="large"
          />
        </ion-button>
      </ion-buttons>
    </ion-toolbar>
  </ion-header>
  <ion-content class="ion-padding">
    <p>{{ $t('JoinByLinkModal.pleaseEnterUrl') }}</p>
    <ion-item>
      <ion-input
        :label="$t('JoinByLinkModal.urlInputLabel')"
        label-placement="stacked"
        ref="urlInput"
        :autofocus="true"
        type="url"
        v-model="joinLink"
        :placeholder="$t('JoinByLinkModal.urlPlaceholder')"
      />
    </ion-item>
  </ion-content>
  <ion-footer class="footer">
    <ion-toolbar>
      <ion-buttons slot="primary">
        <ion-button
          type="submit"
          @click="confirm()"
          :disabled="claimLinkValidator(joinLink) !== Validity.Valid"
        >
          {{ $t('JoinByLinkModal.join') }}
        </ion-button>
      </ion-buttons>
    </ion-toolbar>
  </ion-footer>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonTitle,
  IonToolbar,
  IonButtons,
  IonButton,
  IonItem,
  IonInput,
  modalController,
  IonFooter,
  IonIcon,
} from '@ionic/vue';
import { ref, nextTick, onMounted } from 'vue';
import { close } from 'ionicons/icons';
import { claimLinkValidator, Validity } from '@/common/validators';
import { ModalResultCode } from '@/common/constants';

const joinLink = ref('');

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Cancel);
}
/* by the way pressing Enter won't send the form, you unfortunately have to click the button
see https://github.com/ionic-team/ionic-framework/issues/19368 */
function confirm(): Promise<boolean> {
  return modalController.dismiss(joinLink.value.trim(), ModalResultCode.Confirm);
}

onMounted(() => {
  focusOnEditButton();
});

const urlInput = ref();

// doesn't always work, ionic currently has an issue with the input component: https://github.com/ionic-team/ionic-framework/issues/24009
function focusOnEditButton(): void {
  nextTick(() => {
    urlInput.value.$el.setFocus();
  });
}
</script>

<style lang="scss" scoped>
.footer {
  margin-top: -120px;
  padding-right: 32px;
}
</style>
