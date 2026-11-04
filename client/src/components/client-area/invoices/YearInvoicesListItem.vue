<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ion-text class="invoices-year-text__title">
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
  padding: ms.spacing('padding-lg');
  margin-top: 0.5rem;
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-page-secondary');
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-contract-period: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;

  &-text {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
    cursor: pointer;
    position: sticky;
    top: -2rem;
    background: ms.color('surface-base-page-secondary');
    z-index: 3;
    transition: all 0.2s ease;
    width: 100%;

    &:hover {
      background: ms.color('surface-neutral-default-subtle-hover');
      border-radius: ms.radius('md');

      .invoices-year-text__title {
        color: ms.color('text-brand-default-hover');
      }
    }

    &__title {
      @include ms.font('heading-h4');
      color: ms.color('text-base-body');
      transition: all 0.2s ease;
    }

    &__icon {
      color: ms.color('icon-neutral-default');
      transform: rotate(0deg);
      transition: transform 0.2s;
    }
  }

  &.visible {
    padding: ms.spacing('padding-lg');

    .invoices-year-text__icon {
      transform: rotate(180deg);
    }
  }
}
</style>
