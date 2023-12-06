<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template v-if="imageResource" class="ms-image">
  <div
    v-if="isSvg()"
    class="svg-container"
    v-html="svgData"
  />
  <img
    v-else
    :src="sourceUrl"
    @error="onError()"
  />
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { MsImageExtension, MsImageResource, MsImages, msImageResourceMap } from '@/components/core/ms-types';
import { onMounted } from 'vue';

const imageResource = computed((): MsImageResource | null => {
  return msImageResourceMap.get(props.name) || null;
});

const sourceUrl = computed((): string => {
  if (imageResource.value && imageResource.value.extensions.at(currentExtensionIndex.value)) {
    return `/src/assets/images/${imageResource.value.name}.${imageResource.value.extensions.at(currentExtensionIndex.value)}`;
  }
  return '';
});

onMounted(() => {
  svgFileToString();
});

const currentExtensionIndex = ref(0);
const svgData = ref('');

const props = defineProps<{
  name: MsImages;
}>();

function onError(): void {
  if (imageResource.value && imageResource.value.extensions.at(currentExtensionIndex.value + 1)) {
    currentExtensionIndex.value += 1;
  }
}

function isSvg(): boolean {
  return imageResource.value?.extensions.at(currentExtensionIndex.value) === MsImageExtension.Svg;
}

async function svgFileToString(): Promise<string> {
  if (sourceUrl.value && isSvg()) {
    try {
      const response = await fetch(sourceUrl.value);
      if (!response.ok) {
        throw new Error(`${response.status}`);
      }
      svgData.value = await response.text();
    } catch (error) {
      onError();
      console.warn('An error occurred:', error);
    }
  }
  return '';
}
</script>

<style scoped lang="scss">
.svg-container {
  color: var(--fill-color, var(--parsec-color-light-primary-700));
  display: flex;
}
</style>
