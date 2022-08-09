<template>
  <ion-page>
    <ion-header class="ion-margin-bottom">
      <ion-toolbar>
        <ion-title>{{ $t('CreateOrganization.pageTitle') }}</ion-title>
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
      <div v-if="pageStep === 1">
        <div class="flex-row">
          <ion-item class="flex-row-item ion-margin-bottom">
            <ion-label position="stacked">
              {{ $t('CreateOrganization.organizationName') }}
            </ion-label>
            <ion-input
              v-model="organisation"
              type="text"
              name="organisation"
              :placeholder="$t('CreateOrganization.organizationNamePlaceholder')"
            />
          </ion-item>
          <ion-item class="flex-row-item ion-margin-bottom">
            <ion-label position="stacked">
              {{ $t('CreateOrganization.fullname') }}
            </ion-label>
            <ion-input
              v-model="fullname"
              type="text"
              name="fullname"
              placeholder="Jean MARTIN"
            />
          </ion-item>
        </div>
        <ion-item class="ion-margin-bottom">
          <ion-label position="stacked">
            {{ $t('CreateOrganization.email') }}
          </ion-label>
          <ion-input
            v-model="email"
            type="email"
            name="email"
            :placeholder="$t('CreateOrganization.emailPlaceholder')"
          />
        </ion-item>
        <ion-list>
          <ion-radio-group
            v-model="showTos"
            value="parsecServer"
          >
            <ion-item>
              <ion-label class="ion-text-wrap">
                {{ $t('CreateOrganization.useParsecServer') }}
              </ion-label>
              <ion-radio
                slot="start"
                value="parsecServer"
              />
            </ion-item>
            <ion-item class="ion-margin-bottom">
              <ion-label class="ion-text-wrap">
                {{ $t('CreateOrganization.useMyOwnServer') }}
              </ion-label>
              <ion-radio
                slot="start"
                value="myOwnServer"
              />
            </ion-item>
          </ion-radio-group>
          <ion-item v-if="showTos != 'myOwnServer'">
            <ion-label class="ion-text-wrap">
              {{ $t('CreateOrganization.acceptTOS') }}
            </ion-label>
            <ion-checkbox
              ref="TosCheckbox"
              v-model="acceptTos"
              slot="start"
            />
          </ion-item>
          <ion-item v-else>
            <ion-label position="stacked">
              {{ $t('CreateOrganization.parsecServerUrl') }}
            </ion-label>
            <ion-input
              type="url"
              v-model="ownServerUrl"
              :placeholder="$t('CreateOrganization.parsecServerUrlPlaceholder')"
            />
          </ion-item>
        </ion-list>
      </div>
      <div v-else>
        <h5 class="ion-margin-bottom ion-margin-start">
          {{ $t('CreateOrganization.registeringDevice') }}
        </h5>
        <ion-item class="ion-margin-bottom">
          <ion-label position="stacked">
            {{ $t('CreateOrganization.deviceNameInputLabel') }}
          </ion-label>
          <ion-input
            v-model="deviceName"
            type="text"
            :placeholder="$t('CreateOrganization.deviceNamePlaceholder')"
          />
        </ion-item>
        <ion-item class="ion-margin-bottom">
          <ion-label position="stacked">
            {{ $t('CreateOrganization.password') }}
          </ion-label>
          <ion-input type="password" />
        </ion-item>
        <ion-item class="ion-margin-bottom">
          <ion-label position="stacked">
            {{ $t('CreateOrganization.confirmPassword') }}
          </ion-label>
          <ion-input type="password" />
        </ion-item>
      </div>
    </ion-content>
    <ion-footer>
      <ion-toolbar>
        <ion-buttons
          v-if="pageStep === 1"
          slot="primary"
        >
          <ion-button
            @click="nextStep()"
            :disabled="!firstPageIsFilled()"
          >
            {{ $t('CreateOrganization.next') }}
          </ion-button>
        </ion-buttons>
        <ion-buttons
          v-else
          slot="primary"
        >
          <ion-button
            @click="previousStep()"
            slot="start"
          >
            {{ $t('CreateOrganization.previous') }}
          </ion-button>
          <ion-button type="submit">
            {{ $t('CreateOrganization.done') }}
          </ion-button>
        </ion-buttons>
      </ion-toolbar>
    </ion-footer>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonTitle,
  IonToolbar,
  IonItem,
  IonLabel,
  IonList,
  IonRadio,
  IonRadioGroup,
  IonCheckbox,
  IonPage,
  IonInput,
  IonHeader,
  IonContent,
  IonButton,
  IonButtons,
  IonFooter,
  IonIcon,
  modalController
} from '@ionic/vue';

import {
  close
} from 'ionicons/icons';
import { ref } from 'vue';

const showTos = ref('parsecServer');
const ownServerUrl = ref('');
const acceptTos = ref(false);
const pageStep = ref(1);
const email = ref('');
const fullname = ref('');
const organisation = ref('');
const deviceName = ref('');

function nextStep(): void {
  pageStep.value = 2;
}

function previousStep(): void {
  pageStep.value = 1;
}

/* temporary test, we will do a more complete form validation */
function firstPageIsFilled(): boolean {
  return email.value.length > 0 && fullname.value.length > 0 && organisation.value.length > 0
  && ( acceptTos.value || ownServerUrl.value.length > 0 );
}

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, 'cancel');
}

</script>

<style lang="scss" scoped>
.flex-row {
  @media screen and (min-width: 576px) {
    display: flex;
    flex-flow: row wrap;
    justify-content: space-between;

    .flex-row-item {
      width: 48%;
    }
  }
}
</style>
