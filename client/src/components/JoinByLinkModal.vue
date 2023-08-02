<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-buttons
      slot="end"
      class="closeBtn-container"
    >
      <ion-button
        slot="icon-only"
        @click="closeModal()"
        class="closeBtn"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
    <div class="modal">
      <ion-header class="modal-header">
        <ion-title class="modal-header__title title-h2">
          {{ $t('JoinByLinkModal.pageTitle') }}
        </ion-title>
        <ion-text
          class="modal-header__text body"
        >
          {{ $t('JoinByLinkModal.pleaseEnterUrl') }}
        </ion-text>
      </ion-header>
      <div class="modal-content inner-content">
        <organization-link-page
          ref="urlInput"
          :autofocus="true"
          v-model="joinLink"
        />
      </div>
      <ion-footer class="modal-footer">
        <ion-toolbar>
          <ion-buttons
            slot="primary"
            class="modal-footer-buttons"
          >
            <ion-button
              fill="solid"
              size="default"
              id="next-button"
              type="submit"
              @click="confirm()"
              :disabled="claimLinkValidator(joinLink) !== Validity.Valid"
            >
              {{ $t('JoinByLinkModal.join') }}
            </ion-button>
          </ion-buttons>
        </ion-toolbar>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonText,
  IonPage,
  IonHeader,
  IonTitle,
  IonToolbar,
  IonButtons,
  IonButton,
  modalController,
  IonFooter,
  IonIcon,
} from '@ionic/vue';
import { ref } from 'vue';
import { close } from 'ionicons/icons';
import { claimLinkValidator, Validity } from '@/common/validators';
import OrganizationLinkPage from '@/components/JoinOrganization/OrganizationLinkPage.vue';
import { ModalResultCode } from '@/common/constants';

const joinLink = ref('');
const urlInput = ref();

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Cancel);
}
/* by the way pressing Enter won't send the form, you unfortunately have to click the button
see https://github.com/ionic-team/ionic-framework/issues/19368 */
function confirm(): Promise<boolean> {
  return modalController.dismiss(joinLink.value.trim(), ModalResultCode.Confirm);
}
</script>

<style lang="scss" scoped>
.modal {
  padding: 3.5rem;
  justify-content: start;
}

.closeBtn-container {
    position: absolute;
    top: 2rem;
    right: 2rem;
  }

.closeBtn-container, .closeBtn {
  margin: 0;
  --padding-start: 0;
  --padding-end: 0;
}

.closeBtn {
  width: fit-content;
  height: fit-content;
  --border-radius: var(--parsec-radius-4);
  --background-hover: var(--parsec-color-light-primary-50);
  border-radius: var(--parsec-radius-4);

  &__icon {
    padding: 4px;
    color: var(--parsec-color-light-primary-500);

    &:hover {
      --background-hover: var(--parsec-color-light-primary-50);
    }
  }

  &:active {
    border-radius: var(--parsec-radius-4);
    background: var(--parsec-color-light-primary-100);
  }
}

.modal-header {
  margin-bottom: 2rem;

  &__title {
    padding: 0;
    margin-bottom: 1.5rem;
    color: var(--parsec-color-light-primary-600);
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
}

.modal-content {
  --background: transparent;
}

.modal-footer {
  margin-top: 2.5rem;

  &::before {
    background: transparent;
  }

  &-buttons {
    display: flex;
    justify-content: end;
    gap: 1rem;
  }
}
</style>
