<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <audio
    controls
    v-if="src.length"
    :src="src"
  />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});
</script>

<style scoped lang="scss"></style>
