<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="avatar-root">
    <div
      class="initials-element"
      v-if="type === AvatarType.Initials"
    >
      {{ getDisplayString() }}
    </div>
    <div
      v-if="type === AvatarType.Icon"
      v-html="generateSvg()"
      class="icon-element"
    />
  </div>
</template>

<script lang="ts">
export enum AvatarType {
  Initials = 'initials',
  Icon = 'icon',
}
</script>

<script setup lang="ts">
import { toSvg } from 'jdenticon';
import { hash } from '@/common/hash';

const COLORS = [
  '#1abc9c', '#2ecc71', '#3498db', '#9b59b6', '#34495e', '#16a085', '#27ae60',
  '#2980b9', '#8e44ad', '#2c3e50', '#f1c40f', '#e67e22', '#e74c3c', '#95a5a6',
  '#f39c12', '#d35400', '#c0392b', '#bdc3c7', '#7f8c8d', '#cd89ab', '#80cfcb',
  '#88ff80', '#c8c98d', '#f1aaff', '#e7dbbd', '#cb95c6', '#94d2ac', '#bd9c80',
  '#809ac7', '#ff8ce7',
];

const props = defineProps<{
  type: AvatarType;
  data: string;
  display?: string;
  backgroundColor?: string;
  textColor?: string;
}>();

function getDisplayString(): string {
  if (!props.display) {
    const splitted = props.data.split(' ').filter((word) => word.length > 0);
    const initials = splitted.map((word) => {
      return word.toUpperCase().charAt(0);
    });
    // Limit to 2 characters
    return initials.slice(0, 2).join('');
  }
  return props.display;
}

function getBackgroundColor(): string {
  if (props.backgroundColor) {
    return props.backgroundColor;
  }
  const colorIndex = hash(props.data) % COLORS.length;
  return COLORS[colorIndex];
}

function getTextColor(): string {
  return props.textColor ?? '#FFF';
}

function generateSvg(): string {
  const svg = toSvg(props.data, 64, { backColor: props.backgroundColor ?? '#FFFFFF' });
  return svg;
}
</script>

<style scoped lang="scss">
.avatar-root {
}

.initials-element {
  background-color: v-bind(getBackgroundColor());
  width: 2rem;
  height: 2rem;
  font: 1rem Arial;
  color: v-bind(getTextColor());
  text-align: center;
  line-height: 2rem;
  border-radius: 50%;
}

.icon-element {
}
</style>
