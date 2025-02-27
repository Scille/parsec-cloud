<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
  padding: 0.5rem;
  margin-top: 0.5rem;
  background: var(--parsec-color-light-secondary-premiere);
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-contract-period: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;
  border-radius: var(--parsec-radius-12);

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
    border-radius: var(--parsec-radius-8);
  }

  &-year {
    margin: 0.5rem 1rem;
    height: 1.5rem;
    width: 5rem;
  }

  &-header-list {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);
    position: relative;
    align-items: center;
  }

  &-content-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &-item {
      background: var(--parsec-color-light-secondary-white);
      display: flex;
      justify-content: space-between;
      gap: 0.5rem;
      border-radius: var(--parsec-radius-8);
      position: relative;
      align-items: center;
      padding: 0.5rem 1rem;
    }
  }
}
</style>
