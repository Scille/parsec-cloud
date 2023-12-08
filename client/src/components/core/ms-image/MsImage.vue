<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template v-if="imageResource" class="ms-image">
  <div
    v-if="imageResource?.isSvg"
    class="svg-container"
    v-html="imageResource?.data"
  />
  <img
    v-else
    :src="imageResource?.data"
  />
</template>

<script setup lang="ts">
import { MsImageName, MsImageResource, msImageExtensions } from '@/components/core/ms-types';
import { Ref, onMounted, ref } from 'vue';

onMounted(() => {
  loadImageResource();
});

const imageResource: Ref<MsImageResource | null> = ref(null);

const props = defineProps<{
  name: MsImageName;
}>();

async function loadImageResource(): Promise<void> {
  for (const extension of msImageExtensions) {
    const imageUrl = getImageUrl(props.name, extension);
    if (imageUrl) {
      try {
        const response = await fetch(imageUrl);
        // Fetch has failed, we fallback on next extension
        if (!response.ok) {
          throw new Error(`${response.status}`);
        }
        // SVG, we need the raw content
        if (extension === 'svg') {
          imageResource.value = {
            name: props.name,
            isSvg: true,
            data: await response.text(),
          };
          return;
        } else {
          // Other extensions, we need the source URL
          imageResource.value = {
            name: props.name,
            isSvg: false,
            data: response.url,
          };
          return;
        }
      } catch (error) {
        console.warn('An error occurred:', error);
      }
    }
  }
  // All extensions failed, warn
  console.warn(`Failed to load icon ${props.name}`);
}

function getImageUrl(name: MsImageName, extension: string): string {
  const path = `/assets/images/${name}.${extension}`;
  return new URL(path, import.meta.url).href;
}
</script>

<style scoped lang="scss">
.svg-container {
  color: var(--fill-color, var(--parsec-color-light-primary-700));
  display: flex;
}
</style>
