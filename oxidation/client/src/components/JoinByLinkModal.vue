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
      <ion-label position="stacked">
        {{ $t('JoinByLinkModal.urlInputLabel') }}
      </ion-label>
      <ion-input
        ref="urlInput"
        :autofocus="true"
        type="url"
        v-model="joinUrl"
        :placeholder="$t('JoinByLinkModal.urlPlaceholder')"
      />
    </ion-item>
  </ion-content>
  <ion-footer>
    <ion-toolbar>
      <ion-buttons slot="primary">
        <ion-button
          type="submit"
          @click="confirm()"
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
  IonLabel,
  IonInput,
  modalController,
  IonFooter,
  IonIcon
} from '@ionic/vue';
import { ref, nextTick, onMounted } from 'vue';
import {
  close
} from 'ionicons/icons';
const joinUrl = ref('');

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, 'cancel');
}
/* by the way pressing Enter won't send the form, you unfortunately have to click the button
see https://github.com/ionic-team/ionic-framework/issues/19368 */
function confirm(): Promise<boolean> {
  return modalController.dismiss(joinUrl.value, 'confirm');
}

onMounted(() => {
  focusOnEditButton();
});

const urlInput = ref();

// doesn't always work, ionic currently has an issue with the input component: https://github.com/ionic-team/ionic-framework/issues/24009
function focusOnEditButton(): void {
  nextTick(() => {
    urlInput.value.$el.setFocus();
    console.log('focused');
  });
}

</script>

<style scoped>
</style>
