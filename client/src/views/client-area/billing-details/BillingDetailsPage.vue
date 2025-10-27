<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-billing-page">
    <div class="billing-header">
      <ion-title class="billing-header__title title-h2">{{ $msTranslate('clientArea.billingDetailsPage.title') }}</ion-title>
    </div>
    <div class="form-container">
      <div class="form">
        <div
          class="form-item-content"
          id="address"
        >
          <ms-input
            class="form-item"
            v-model="line1"
            :maxlength="256"
            label="clientArea.billingDetailsPage.placeholders.line1"
            :disabled="!isEditing"
          />
          <ms-input
            class="form-item"
            v-model="line2"
            :maxlength="256"
            :placeholder="'clientArea.billingDetailsPage.placeholders.line2'"
            :disabled="!isEditing"
          />
        </div>
        <div class="form-row">
          <div
            class="form-item-content"
            id="postalCode"
          >
            <ms-input
              type="tel"
              v-model="postalCode"
              :maxlength="32"
              label="clientArea.billingDetailsPage.placeholders.postalCode"
              :disabled="!isEditing"
            />
          </div>
          <div
            class="form-item-content"
            id="city"
          >
            <ms-input
              v-model="city"
              :maxlength="128"
              label="clientArea.billingDetailsPage.placeholders.city"
              :disabled="!isEditing"
            />
          </div>
        </div>
        <ms-input
          class="form-item"
          v-model="country"
          :maxlength="128"
          label="clientArea.billingDetailsPage.placeholders.country"
          :disabled="!isEditing"
        />
      </div>

      <div class="form-submit">
        <ion-button
          v-show="!isEditing"
          @click="isEditing = true"
          class="toggle-button"
        >
          {{ $msTranslate('clientArea.billingDetailsPage.update') }}
        </ion-button>
        <ion-button
          v-show="isEditing"
          @click="onCancelEdit"
          class="cancel-button"
          fill="outline"
        >
          {{ $msTranslate('clientArea.billingDetailsPage.cancel') }}
        </ion-button>
        <ion-button
          v-show="isEditing"
          @click="submit"
          class="submit-button"
          :disabled="!isFormValid"
        >
          {{ $msTranslate('clientArea.billingDetailsPage.submit') }}
        </ion-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, BmsOrganization, DataType } from '@/services/bms';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { IonButton, IonTitle } from '@ionic/vue';
import { MsInput } from 'megashark-lib';
import { computed, inject, onMounted, ref } from 'vue';

defineProps<{
  organization: BmsOrganization;
}>();

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const informationManager: InformationManager = injectionProvider.getDefault().informationManager;

const line1 = ref<string>('');
const line2 = ref<string | undefined>(undefined);
const postalCode = ref<string>('');
const city = ref<string>('');
const country = ref<string>('');
const isEditing = ref(false);
const currentAddress = ref();
const isFormValid = computed(() => {
  return line1.value.length > 0 && postalCode.value.length > 0 && city.value.length > 0 && country.value.length > 0;
});

onMounted(async () => {
  const response = await BmsAccessInstance.get().getBillingDetails();
  if (!response.isError && response.data && response.data.type === DataType.BillingDetails) {
    currentAddress.value = response.data.address;
    assignCurrentAddress();
  }
});

function assignCurrentAddress(): void {
  line1.value = currentAddress.value.line1;
  line2.value = currentAddress.value.line2;
  postalCode.value = currentAddress.value.postalCode;
  city.value = currentAddress.value.city;
  country.value = currentAddress.value.country;
}

function updateCurrentAddress(): void {
  currentAddress.value.line1 = line1.value;
  currentAddress.value.line2 = line2.value;
  currentAddress.value.postalCode = postalCode.value;
  currentAddress.value.city = city.value;
  currentAddress.value.country = country.value;
}

async function submit(): Promise<void> {
  isEditing.value = false;
  const response = await BmsAccessInstance.get().updateBillingAddress({
    line1: line1.value,
    line2: line2.value,
    postalCode: postalCode.value,
    city: city.value,
    country: country.value,
  });
  if (response.isError) {
    informationManager.present(
      new Information({
        message: 'clientArea.billingDetailsPage.error',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: 'clientArea.billingDetailsPage.success',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  updateCurrentAddress();
}

async function onCancelEdit(): Promise<void> {
  isEditing.value = false;
  assignCurrentAddress();
}
</script>

<style scoped lang="scss">
.client-billing-page {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 32rem;
}

.billing-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1.5rem;

  &__title {
    color: var(--parsec-color-light-primary-700);
  }
}

.form-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  width: 100%;
}

.form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &-row {
    display: flex;
    gap: 1em;
  }

  #address {
    display: flex;
    flex-direction: column;
    gap: 1em;
  }

  #postalCode {
    max-width: 10rem;
  }

  #city {
    flex: 1;
  }

  &-submit {
    display: flex;

    .submit-button {
      margin-left: auto;
    }
  }
}
</style>
