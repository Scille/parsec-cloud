<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="invoices-year"
    :class="{ visible: isVisible }"
  >
    <!-- year -->
    <div
      class="invoices-year-text"
      @click="isVisible = !isVisible"
    >
      <ion-text class="invoices-year-text__title title-h3">
        {{ $msTranslate(title) }}
      </ion-text>
      <ion-icon
        class="invoices-year-text__icon"
        :icon="chevronDown"
      />
    </div>

    <!-- invoices list -->
    <invoices-list
      v-if="invoices.length > 0 && isVisible"
      :invoices="invoices"
      :months-filter="monthsFilter"
    />
  </div>
</template>

<script setup lang="ts">
import InvoicesList from '@/components/client-area/invoices/InvoicesList.vue';
import { Invoice } from '@/services/bms';
import { IonIcon, IonText } from '@ionic/vue';
import { chevronDown } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';
import { ref } from 'vue';

defineProps<{
  invoices: Array<Invoice>;
  title: Translatable;
  monthsFilter?: Array<number>;
}>();

const isVisible = ref(true);
</script>

<style scoped lang="scss">
.invoices-year {
  display: flex;
  flex-direction: column;
  align-items: start;
  padding: 0.5rem;
  margin-top: 0.5rem;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-premiere);
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-contract-period: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;

  &-text {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    cursor: pointer;
    position: sticky;
    top: -2rem;
    background: var(--parsec-color-light-secondary-premiere);
    z-index: 3;
    transition: all 0.2s ease;
    width: 100%;

    &:hover {
      background: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-6);

      .invoices-year-text__title {
        color: var(--parsec-color-light-primary-500);
      }
    }

    &__title {
      color: var(--parsec-color-light-secondary-text);
      transition: all 0.2s ease;
    }

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
      transform: rotate(0deg);
      transition: transform 0.2s;
    }
  }

  &.visible {
    padding: 0.5rem;

    .invoices-year-text__icon {
      transform: rotate(180deg);
    }
  }
}
</style>
