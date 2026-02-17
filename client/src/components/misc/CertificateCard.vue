<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="certificate-card"
    :class="{
      selectable: !certificate.isExpired(),
      disabled: certificate.isExpired(),
      isSelected: isSelected,
    }"
    @click="$emit('clicked', certificate)"
  >
    <span>{{ certificate.name }}</span>
    <br />
    <span>{{ $msTranslate(I18n.formatDate(certificate.createdOn, 'short')) }}</span>
    ->
    <span>{{ $msTranslate(I18n.formatDate(certificate.expireOn, 'short')) }}</span>
    <span v-if="certificate.isExpired()">
      {{ $msTranslate('EXPIRED') }}
    </span>
    <br />
    <span v-if="certificate.emails.length > 0">{{ certificate.emails[0] }}</span>
    <span
      v-if="certificate.emails.length > 1"
      ref="additionalEmails"
    >
      + {{ certificate.emails.length }}
    </span>
    <br />
    <span>{{ certificate.id }}</span>
    <br />
    <span v-if="isSelected">SELECTED</span>
  </div>
</template>

<script setup lang="ts">
import { Certificate } from '@/parsec';
import {} from '@ionic/vue';
import { I18n, attachMouseOverTooltip } from 'megashark-lib';
import { onMounted, useTemplateRef } from 'vue';

const props = defineProps<{
  certificate: Certificate;
  isSelected: boolean;
}>();

defineEmits<{
  (e: 'clicked', certificate: Certificate): void;
}>();

const additionalEmailsRef = useTemplateRef<HTMLSpanElement>('additionalEmails');

onMounted(() => {
  if (additionalEmailsRef.value) {
    attachMouseOverTooltip(additionalEmailsRef.value, I18n.valueAsTranslatable(props.certificate.emails.join('\n')), 500);
  }
});
</script>

<style scoped lang="scss">
.certificate-card {
  border: 10px dotted green;
}

.isSelected {
  border: 10px solid red;
}

.isSelectable {
  background-color: pink;
  cursor: pointer;
}

.disabled {
  background-color: grey;
}
</style>
