<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="goBack()"
  >
    <ion-icon
      :icon="chevronBack"
    />

    <ion-label
      v-if="!short"
    >
      {{ $t('HeaderPage.previous') }}
    </ion-label>
  </ion-button>

  <div
    v-if="short"
    class="vertical-spacer"
  />
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { chevronBack } from 'ionicons/icons';
import { IonLabel, IonIcon, IonButton } from '@ionic/vue';

defineProps<{
  short: boolean,
}>();

const router = useRouter();

function goBack(): void {
  router.go(-1);
}
</script>

<style scoped lang="scss">

</style>
