<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content>
      <div class="container">
        <div class="about-container">
          <about-view />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import AboutView from '@/views/about/AboutView.vue';
import { IonContent, IonPage } from '@ionic/vue';
</script>

<style scoped lang="scss">
.container {
  margin: 3rem 2rem 2rem 2rem;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem;
  box-shadow:
    0 1px 1px 0 rgba(0, 0, 0, 0.05),
    0 1px 4px 0 rgba(0, 0, 0, 0.03),
    0 0 1px 0 rgba(0, 0, 0, 0.2);
}
.about-container {
  border-bottom: 1px solid var(--parsec-color-light-primary-100);
  width: 40%;
}
</style>
