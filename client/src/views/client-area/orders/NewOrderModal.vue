<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="clientArea.orders.request.title"
      subtitle="clientArea.orders.request.subtitle"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: 'clientArea.orders.request.submit',
        disabled: !isFormValid || querying,
        onClick: submit,
      }"
    >
      <div class="new-order-modal-container">
        <div class="summary-request">
          <div class="form">
            <div class="form-item-content">
              <ion-text
                id="label"
                class="form-label"
              >
                {{ $msTranslate('clientArea.orders.request.userNeeds.label') }}
              </ion-text>
              <ms-dropdown
                :options="userOptions"
                :default-option-key="userNeeds"
                @change="userNeeds = $event.option.key"
                alignment="start"
              />
            </div>
            <div class="form-item-content">
              <ion-text
                id="label"
                class="form-label"
              >
                {{ $msTranslate('clientArea.orders.request.storageNeeds.label') }}
              </ion-text>
              <ms-dropdown
                :options="storageOptions"
                :default-option-key="storageNeeds"
                @change="storageNeeds = $event.option.key"
                alignment="start"
              />
            </div>
            <div class="form-item-content">
              <ion-text
                id="label"
                class="form-label"
              >
                {{ $msTranslate('clientArea.orders.request.description.label') }}
              </ion-text>
              <ms-textarea
                class="form-input form-field"
                v-model="description"
                mode="ios"
                placeholder="clientArea.orders.request.description.placeholder"
                :cols="80"
                :rows="8"
              />
            </div>
          </div>
          <ms-report-text
            :theme="MsReportTheme.Error"
            v-if="error"
          >
            {{ $msTranslate(error) }}
          </ms-report-text>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { BmsAccessInstance, CONNECTION_ERROR_STATUS } from '@/services/bms';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsDropdown, MsModal, MsModalResult, MsOptions, MsReportText, MsReportTheme, MsTextarea } from 'megashark-lib';
import { computed, ref } from 'vue';

const userOptions: MsOptions = new MsOptions([
  {
    key: 50,
    label: 'clientArea.orders.request.userNeeds.choices.50',
  },
  {
    key: 100,
    label: 'clientArea.orders.request.userNeeds.choices.100',
  },
  {
    key: 300,
    label: 'clientArea.orders.request.userNeeds.choices.300',
  },
  {
    key: 9999,
    label: 'clientArea.orders.request.userNeeds.choices.more',
  },
]);

const storageOptions: MsOptions = new MsOptions([
  {
    key: 100,
    label: 'clientArea.orders.request.storageNeeds.choices.100',
  },
  {
    key: 500,
    label: 'clientArea.orders.request.storageNeeds.choices.500',
  },
  {
    key: 1000,
    label: 'clientArea.orders.request.storageNeeds.choices.1000',
  },
  {
    key: 9999,
    label: 'clientArea.orders.request.storageNeeds.choices.more',
  },
]);

const userNeeds = ref<number>(50);
const storageNeeds = ref<number>(100);
const description = ref<string>('');
const error = ref('');
const querying = ref(false);

const isFormValid = computed(() => {
  return description.value.length > 0;
});

async function submit(): Promise<boolean> {
  if (!isFormValid.value) {
    return false;
  }

  querying.value = true;
  try {
    const response = await BmsAccessInstance.get().createCustomOrderRequest({
      standardUsers: userNeeds.value,
      storage: storageNeeds.value,
      needs: description.value,
    });

    if (response.isError) {
      if (response.status === CONNECTION_ERROR_STATUS) {
        error.value = 'clientArea.orders.request.connectionFailure';
      } else {
        error.value = 'clientArea.orders.request.serverFailure';
      }
      return false;
    }
    return await modalController.dismiss(null, MsModalResult.Confirm);
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-item-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
</style>
