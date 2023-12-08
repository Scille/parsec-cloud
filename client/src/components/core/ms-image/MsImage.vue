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
import * as images from '@/assets/images/images';
import { MsImageResource, MsImages, msImageExtensions } from '@/components/core/ms-types';
import { Ref, onMounted, ref } from 'vue';

onMounted(() => {
  loadImageResource();
});

const imageResource: Ref<MsImageResource | null> = ref(null);

// eslint-disable-next-line prefer-const
let method: number = 2;
// eslint-disable-next-line prefer-const
let imageUrlMethod: number = 1;

const props = defineProps<{
  name: MsImages;
}>();

async function loadImageResource(): Promise<void> {
  // iter over extensions
  for (const extension of msImageExtensions) {
    const imageUrl = getImageUrl(props.name, extension);
    console.log(imageUrl);

    if (imageUrl) {
      switch (method) {
        case 1:
          try {
            const response = await fetch(imageUrl);
            // failed to fetch, try next extension
            if (!response.ok) {
              throw new Error(`${response.status}`);
            }
            // if svg, we want the content
            if (extension === 'svg') {
              imageResource.value = {
                name: props.name,
                isSvg: true,
                data: await response.text(),
              };
              return;
            } else {
              // else we want the URL
              // console.log(await response.text());
              imageResource.value = {
                name: props.name,
                isSvg: false,
                data: imageUrl,
              };
              return;
            }
          } catch (error) {
            console.warn('An error occurred:', error);
          }
          break;
        case 2:
          try {
            imageFileToImport();
            return;
            // if (extension === 'svg') {
            //   imageResource.value = {
            //     name: props.name,
            //     isSvg: true,
            //     data: '',
            //   };
            //   return;
            // } else {
            //   // else we want the URL
            //   // console.log(await response.text());
            //   imageResource.value = {
            //     name: props.name,
            //     isSvg: false,
            //     data: imageUrl,
            //   };
            //   return;
            // }
          } catch (error) {
            console.warn('An error occurred:', error);
          }
      }
    }
  }
  // All extensions failed, warn
  console.warn(`Failed to load icon ${props.name}`);
}

function getImageUrl(name: MsImages, extension: string): string | null {
  let imageUrl = null;
  switch (imageUrlMethod) {
    case 1:
      return `/src/assets/images/${name}.${extension}`;
    case 2:
      try {
        const path = `/src/assets/images/${name}.${extension}`;
        imageUrl = new URL(path, import.meta.url);
      } catch (e) {
        if (e instanceof TypeError) {
          // error handling procedure...
          console.log(e);
        } else {
          throw e;// cause we dont know what it is or we want to only handle TypeError
        }
      } finally {
        // eslint-disable-next-line no-unsafe-finally
        return imageUrl?.href || null;
      }
    case 3:
      return `/images/${name}.${extension}`;
    default:
      return null;
  }
}

function imageFileToImport(): void {
  if (props.name === MsImages.LogoRowWhite) {
    imageResource.value = {
      name: props.name,
      isSvg: false,
      data: images.logoRowWhiteWebp,
    };
  }
}
</script>

<style scoped lang="scss">
.svg-container {
  color: var(--fill-color, var(--parsec-color-light-primary-700));
  display: flex;
}
</style>
