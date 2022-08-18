<template>
  <ion-label class="ion-text-wrap">
    {{ title }}
    <div class="ion-text-wrap setting-description">
      {{ description }}
    </div>
  </ion-label>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import { IonLabel } from '@ionic/vue';

defineProps<{
  title: string
  description?: string
}>();
</script>

<style scoped>
  .setting-description {
    color: var(--ion-color-medium);
  }
</style>
