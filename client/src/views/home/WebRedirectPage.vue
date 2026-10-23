<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="page">
    <div class="redirect-container">
      <ms-image
        :image="LogoIconGradient"
        class="redirect-logo"
      />

      <div class="redirect-text">
        <ion-text class="redirect-text__title">
          {{ $msTranslate('WebRedirectPage.title') }}
        </ion-text>
        <ion-text class="redirect-text__subtitle">
          {{ $msTranslate('WebRedirectPage.subtitle') }}
        </ion-text>
      </div>

      <div class="redirect-buttons">
        <ion-button
          class="redirect-buttons__item button-large button-primary"
          @click="Env.Links.openUrl(redirectLink)"
        >
          {{ $msTranslate('WebRedirectPage.desktop') }}
        </ion-button>
        <ion-button
          @click="openLinkInWeb()"
          class="redirect-buttons__item button-large"
          fill="clear"
        >
          {{ $msTranslate('WebRedirectPage.web') }}
        </ion-button>
      </div>
      <div class="redirect-download">
        <ion-text class="redirect-download__text">
          {{ $msTranslate('WebRedirectPage.downloadText') }}
        </ion-text>
        <a
          :href="$msTranslate('MenuPage.downloadParsecLink')"
          target="_blank"
          class="redirect-download__link"
        >
          {{ $msTranslate('WebRedirectPage.downloadLink') }}
        </a>
      </div>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { ParsedParsecAddrTag, parseParsecAddr } from '@/parsec';
import { getCurrentRouteQuery, navigateTo, Routes } from '@/router';
import { Env } from '@/services/environment';
import { InformationManager } from '@/services/informationManager';
import { handleFileLink } from '@/services/linkHandler';
import { IonButton, IonPage, IonText } from '@ionic/vue';
import { LogoIconGradient, MsImage } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const redirectLink = ref<string>('');
const linkType = ref<ParsedParsecAddrTag | undefined>(undefined);

onMounted(async () => {
  const query = getCurrentRouteQuery();
  redirectLink.value = query.webRedirectUrl || '';

  if (!redirectLink.value) {
    redirectLink.value = 'parsec3://';
    return;
  }
  const result = await parseParsecAddr(redirectLink.value);
  if (!result.ok) {
    // If the link is invalid, we convert it to parsec3:// to still open the app, but
    // without giving it any link to process
    redirectLink.value = 'parsec3://';
  } else {
    linkType.value = result.value.tag;
  }
});

async function openLinkInWeb(): Promise<void> {
  switch (linkType.value) {
    case ParsedParsecAddrTag.AsyncEnrollment:
      await navigateTo(Routes.Home, { skipHandle: true, query: { asyncEnrollmentLink: redirectLink.value } });
      break;
    case ParsedParsecAddrTag.OrganizationBootstrap:
      await navigateTo(Routes.Home, { skipHandle: true, query: { bootstrapLink: redirectLink.value } });
      break;
    case ParsedParsecAddrTag.InvitationUser:
    case ParsedParsecAddrTag.InvitationDevice:
      await navigateTo(Routes.Home, { skipHandle: true, query: { claimLink: redirectLink.value } });
      break;
    case ParsedParsecAddrTag.WorkspacePath:
      const informationManager = new InformationManager();
      const navigationOk = await handleFileLink(redirectLink.value, informationManager);
      if (!navigationOk) {
        await navigateTo(Routes.Home, { skipHandle: true });
      }
      break;
    case ParsedParsecAddrTag.TOTPReset:
      await navigateTo(Routes.Home, { skipHandle: true, query: { totpResetLink: redirectLink.value } });
      break;
    default:
      console.warn(`Unhandled link type '${linkType.value}', redirecting...`);
      await navigateTo(Routes.Home, { skipHandle: true });
      break;
  }
}
</script>

<style lang="scss" scoped>
.page {
  background-color: ms.color('surface-base-default');
}

.redirect-container {
  max-width: 28rem;
  display: flex;
  margin: auto;
  width: 100%;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

.redirect-logo {
  width: 4rem;
  margin-bottom: 1.75rem;
}

.redirect-text {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: ms.spacing('gap-3xl');
  margin-bottom: ms.spacing('padding-4xl');

  &__title {
    @include ms.font('heading-h2');
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
  }

  &__subtitle {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-description');
    text-align: center;
  }
}

.redirect-buttons {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: ms.spacing('gap-3xl');
  max-width: 22rem;
  width: 100%;

  &__item {
    @include ms.font('label-lg-medium');
    width: 100%;

    .link {
      color: ms.color('text-on-color-heading');
    }
  }
}

.redirect-download {
  margin-top: ms.spacing('padding-6xl');
  padding-top: ms.spacing('padding-4xl');
  display: flex;
  align-items: center;
  justify-content: center;
  gap: ms.spacing('gap-lg');
  border-top: ms.border('thin') solid ms.color('border-base-default');

  &__text {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
  }

  &__link {
    @include ms.font('label-md-medium');
    color: ms.color('text-brand-default');
    transition: all 0.2s ease-in-out;

    &:hover {
      text-decoration: underline;
      color: ms.color('text-brand-default-hover');
    }
  }
}
</style>
