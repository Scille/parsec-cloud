<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <!-- skeleton invoices -->
  <div class="skeleton">
    <!-- skeleton year -->
    <ion-skeleton-text
      :animated="true"
      class="skeleton-year"
    />

    <!-- skeleton row header -->
    <div class="skeleton-header-list">
      <ion-skeleton-text
        :animated="true"
        class="invoices-date"
      />
      <ion-skeleton-text
        :animated="true"
        class="invoices-number"
      />
      <ion-skeleton-text
        :animated="true"
        class="invoices-organization"
      />
      <ion-skeleton-text
        :animated="true"
        class="invoices-amount"
      />
      <ion-skeleton-text
        :animated="true"
        class="invoices-contract-period"
      />
      <ion-skeleton-text
        :animated="true"
        class="invoices-status"
      />
    </div>

    <!-- skeleton row invoices -->
    <div class="skeleton-content-list">
      <div
        class="skeleton-content-list-item"
        v-for="index in 3"
        :key="index"
      >
        <ion-skeleton-text
          :animated="true"
          class="invoices-date"
        />
        <ion-skeleton-text
          :animated="true"
          class="invoices-number"
        />
        <ion-skeleton-text
          :animated="true"
          class="invoices-organization"
        />
        <ion-skeleton-text
          :animated="true"
          class="invoices-amount"
        />
        <ion-skeleton-text
          :animated="true"
          class="invoices-contract-period"
        />
        <ion-skeleton-text
          :animated="true"
          class="invoices-status"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonSkeletonText } from '@ionic/vue';
</script>

<style scoped lang="scss">
.skeleton {
  display: flex;
  flex-direction: column;
  padding: ms.spacing('padding-lg');
  margin-top: 0.5rem;
  background: ms.color('surface-base-page-secondary');
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-contract-period: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;
  border-radius: ms.radius('2xl');

  [class^='invoices-'] {
    height: 1rem;
    width: 100%;
  }

  .invoices-date {
    max-width: calc(var(--max-width-date) - 1rem);
  }

  .invoices-number {
    max-width: calc(var(--max-width-date) - 1rem);
  }

  .invoices-organization {
    max-width: calc(var(--max-width-organization) - 1rem);
  }

  .invoices-contract-period {
    max-width: calc(var(--max-width-contract-period) - 1rem);
  }

  .invoices-amount {
    max-width: calc(var(--max-width-amount) - 1rem);
  }

  ion-skeleton-text {
    border-radius: ms.radius('lg');
  }

  &-year {
    margin: 0.5rem 1rem;
    height: 1.5rem;
    width: 5rem;
  }

  &-header-list {
    display: flex;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
    background: ms.color('surface-base-page-secondary');
    border-radius: ms.radius('lg');
    position: relative;
    align-items: center;
  }

  &-content-list {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');

    &-item {
      background: ms.color('surface-base-default');
      display: flex;
      justify-content: space-between;
      gap: ms.spacing('gap-lg');
      border-radius: ms.radius('lg');
      position: relative;
      align-items: center;
      padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
    }
  }
}
</style>
