<template>
  <ion-label>
    {{ title }}
    <div
      class="ion-text-wrap"
      style="color: var(--ion-color-medium);"
    >
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
</style>
