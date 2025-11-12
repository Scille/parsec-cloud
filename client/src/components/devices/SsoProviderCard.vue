<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="provider === OpenBaoAuthConfigTag.OIDCProConnect"
    class="sso-provider-card"
  >
    <div class="proconnect-group">
      <button class="proconnect-button">
        <span class="proconnect-sr-only">{{ $msTranslate('proConnect.title') }}</span>
      </button>
      <p>
        <a @click.stop="Env.Links.openUrl(I18n.translate('proConnect.link'))">
          {{ $msTranslate('proConnect.description') }}
        </a>
      </p>
    </div>
  </div>
  <div
    v-if="provider === OpenBaoAuthConfigTag.OIDCHexagone"
    class="sso-provider-card"
  >
    {{ 'Hexagone' }}
  </div>
</template>

<script setup lang="ts">
import { OpenBaoAuthConfigTag } from '@/parsec';
import { Env } from '@/services/environment';
import { I18n } from 'megashark-lib';

defineProps<{
  provider: OpenBaoAuthConfigTag;
}>();
</script>

<style scoped lang="scss">
* {
  --text-action-high-blue-france: #000091;
  --underline-img: linear-gradient(0deg, currentColor, currentColor);
  --underline-max-width: 100%;
  --underline-x: calc(var(--underline-max-width) * 0);
  --underline-thickness: 0.0625em;
  --underline-idle-width: var(--underline-max-width);
  --underline-hover-width: 0;
  --idle: transparent;
  --hover-tint: var(--idle);
  --text-spacing: 0 0 1rem 0;
}

.sso-provider-card {
  display: flex;
  justify-content: center;
  width: 100%;
}

.proconnect-sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

.proconnect-button {
  background-color: transparent !important;
  background-image: url('@/assets/images/pro-connect2.svg');
  background-position: 50% 50%;
  background-repeat: no-repeat;
  width: 214px;
  height: 56px;
  border: none;
  margin-bottom: 0.75rem;
}

.proconnect-button:hover {
  background-image: url('@/assets/images/pro-connect1.svg');
}

p {
  height: fit-content !important;
  font-size: 1rem;
  line-height: 1.5rem;
  margin: var(--text-spacing);
}

a {
  font-family: Marianne, arial, sans-serif;
  color: var(--text-action-high-blue-france) !important;
  font-size: 0.875rem;
  line-height: 1.5rem;
  outline-width: 2px;
  background-image: var(--underline-img), var(--underline-img);
  background-position:
    var(--underline-x) 100%,
    var(--underline-x) calc(100% - var(--underline-thickness));
  background-repeat: no-repeat, no-repeat;
  /* eslint-disable max-len */
  background-size:
    var(--underline-hover-width) calc(var(--underline-thickness) * 2),
    var(--underline-idle-width) var(--underline-thickness);
  /* eslint-enable max-len */
  --hover-tint: var(--idle);
  --active-tint: var(--active);
  color: inherit;
  -webkit-tap-highlight-color: transparent;
  text-decoration: none;
  transition: background-size 0s;
  cursor: pointer;
}

a:focus:not(:focus-visible) {
  outline-style: none !important;
}

a:focus:not(:focus-visible) {
  outline-color: #0a76f6 !important;
  outline-offset: 2px;
  outline-style: solid;
  outline-width: 2px;
}

a:hover {
  --underline-hover-width: var(--underline-max-width);
  background-color: var(--hover-tint);
}

a,
button {
  -webkit-tap-highlight-color: transparent;
}

[target='_blank']:after {
  --icon-size: 1rem;
  background-color: currentColor;
  content: '';
  display: inline-block;
  flex: 0 0 auto;
  height: var(--icon-size);
  margin-left: 0.25rem;
  -webkit-mask-image: url('@/assets/images/external-link-line.svg');
  mask-image: url('@/assets/images/external-link-line.svg');
  -webkit-mask-size: 100% 100%;
  mask-size: 100% 100%;
  vertical-align: calc((0.75em - var(--icon-size)) * 0.5);
  width: var(--icon-size);
}
</style>
