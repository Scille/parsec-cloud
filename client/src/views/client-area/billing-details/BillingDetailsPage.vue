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
            @on-enter-keyup="submit"
          />
          <ms-input
            class="form-item"
            v-model="line2"
            :maxlength="256"
            :placeholder="'clientArea.billingDetailsPage.placeholders.line2'"
            @on-enter-keyup="submit"
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
              @on-enter-keyup="submit"
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
              @on-enter-keyup="submit"
            />
          </div>
        </div>
        <ms-input
          class="form-item"
          v-model="country"
          :maxlength="128"
          label="clientArea.billingDetailsPage.placeholders.country"
          @on-enter-keyup="submit"
        />
      </div>

      <div class="form-submit">
        <ion-button
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
import { IonButton, IonTitle, IonIcon } from '@ionic/vue';
import { pencil } from 'ionicons/icons';
import { BmsAccessInstance, BmsOrganization, DataType } from '@/services/bms';
import { MsInput } from 'megashark-lib';
import { onMounted, ref, computed, inject } from 'vue';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';

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

const isFormValid = computed(() => {
  return new Boolean(line1.value && postalCode.value && city.value && country.value);
});

onMounted(async () => {
  const response = await BmsAccessInstance.get().getBillingDetails();

  if (!response.isError && response.data && response.data.type === DataType.BillingDetails) {
    line1.value = response.data.address.line1;
    line2.value = response.data.address.line2;
    postalCode.value = response.data.address.postalCode;
    city.value = response.data.address.city;
    country.value = response.data.address.country;
  }
});

async function submit(): Promise<void> {
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
}
</style>
