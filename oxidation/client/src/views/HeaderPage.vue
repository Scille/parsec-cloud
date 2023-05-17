<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header>
      <ion-toolbar class="topbar">
        <!-- icon visible when menu is hidden -->
        <ion-buttons slot="start">
          <ion-button
            v-if="!isPlatform('mobile') && !isSidebarMenuVisible()"
            slot="icon-only"
            id="trigger-toggle-menu-button"
            class="ion-hide-lg-down topbar-button__item"
            @click="resetSidebarMenu()"
          >
            <ion-icon
              slot="icon-only"
              :icon="menu"
            />
          </ion-button>
        </ion-buttons>
        <!-- end of icon visible when menu is hidden -->

        <!-- icon menu on mobile -->
        <ion-buttons slot="start">
          <ion-menu-button />
        </ion-buttons>
        <!-- end of icon menu on mobile -->

        <ion-breadcrumbs
          class="breadcrumb"
          :max-items="maxBreadcrumbs"
          :items-before-collapse="2"
          :items-after-collapse="2"
        >
          <ion-breadcrumb
            v-for="(path, index) in fullPath"
            @click="navigateTo($event, path)"
            class="breadcrumb-element"
            :key="path.id"
          >
            <ion-icon
              v-if="index === 0"
              class="home-icon"
              :icon="home"
            />
            <span v-if="fullPath.length === 1 || (fullPath.length >= 1 && index >= 1)">
              {{ path.display }}
            </span>
            <ion-icon
              slot="separator"
              :icon="caretForward"
            />
          </ion-breadcrumb>
        </ion-breadcrumbs>

        <!-- top right icon + profile -->
        <ion-buttons
          slot="primary"
          class="topbar-right"
        >
          <div class="topbar-button__list">
            <ion-button
              v-if="!isPlatform('mobile')"
              slot="icon-only"
              id="trigger-search-button"
              class="topbar-button__item"
            >
              <ion-icon
                slot="icon-only"
                :icon="search"
              />
            </ion-button>
            <ion-button
              v-if="!isPlatform('mobile')"
              slot="icon-only"
              id="trigger-notifications-button"
              class="topbar-button__item"
            >
              <ion-icon
                slot="icon-only"
                :icon="notifications"
              />
            </ion-button>
          </div>

          <profile-header
            :firstname="'toto'"
            :lastname="'toto'"
            class="profile-header"
          />
        </ion-buttons>
        <!-- top right icon + profil -->
      </ion-toolbar>
    </ion-header>

    <ion-content>
      <ion-router-outlet />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonHeader,
  IonToolbar,
  IonButton,
  IonIcon,
  IonMenuButton,
  IonButtons,
  isPlatform,
  IonContent,
  IonRouterOutlet,
  IonPage,
  IonBreadcrumb,
  IonBreadcrumbs
} from '@ionic/vue';
import {
  menu,
  home,
  search,
  notifications,
  caretForward
} from 'ionicons/icons';
import { useI18n } from 'vue-i18n';
import { useRouter, useRoute } from 'vue-router';
import ProfileHeader from '@/components/ProfileHeader.vue';
import useSidebarMenu from '@/services/sidebarMenu';
import { computed, ref, Ref } from 'vue';
import { parse as parsePath } from '@/common/path';

const currentRoute = useRoute();
const router = useRouter();
const { t } = useI18n();
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu } = useSidebarMenu();
const maxBreadcrumbs: Ref<number | undefined> = ref(4);

interface RouterPathNode {
  id: number,
  display: string,
  name: string,
  params: any,
  query: any
}

// Dummy temporary function
function mockGetWorkspaceName(workspaceId: string): string {
  return `Workspace ${workspaceId}`;
}

const fullPath = computed(() => {
  // Update maxBreadcrumbs automatically if the route changes
  maxBreadcrumbs.value = 4; // eslint-disable-line vue/no-side-effects-in-computed-properties

  // Parse path and remove /deviceId
  const route = parsePath(currentRoute.path).slice(2);

  const finalPath: RouterPathNode[] = [];

  // Always put the document route first
  finalPath.push({
    id: 0,
    display: t('HeaderPage.Breadcrumbs.Documents'),
    name: 'workspaces',
    query: {},
    params: { deviceId: currentRoute.params.deviceId }
  });

  // Use for v-bind
  let id = 1;

  // If route is 'workspaces' and we have an id, we're visiting a workspace
  if (route.length >= 2 && route[0] === 'workspaces') {
    const rebuildPath: string[] = [];
    const workspacePath = parsePath(String(currentRoute.query.path));
    for (let i = 0; i < workspacePath.length; i++) {
      // Root folder
      if (workspacePath[i] === '/') {
        finalPath.push({
          id: id,
          display: mockGetWorkspaceName(String(currentRoute.params.workspaceId)),
          name: 'folder',
          query: { path: '/' },
          params: currentRoute.params
        });
      } else {
        rebuildPath.push(workspacePath[i]);
        finalPath.push({
          id: id,
          display: workspacePath[i],
          name: 'folder',
          query: { path: `/${rebuildPath.join('/')}` },
          params: currentRoute.params
        });
      }
    }
    id += 1;
  }

  return finalPath;
});

function navigateTo(event: PointerEvent, path: RouterPathNode): void {
  /*
  We're using ion-breadcrumbs, they're expecting a `href` attribute.
  We could create one using router.resolve(), but it makes links
  reload the whole page. Instead, we're handling the navigation ourselves
  using a click signal on the breadcrumb. This creates a problem when
  `max-items` is set, as the '...' icon counts as one of the element
  (and so has a valid `path`). We use a bit of trickery by checking what element
  was clicked. If it was the `...` icon we reset maxBreadcrumbs to display
  all the elements, else we navigate to the folder.
  */
  if (event.target && (event.target as any).nodeName === 'ION-BREADCRUMB')  {
    maxBreadcrumbs.value = undefined;
  } else {
    router.push({name: path.name, params: path.params, query: path.query});
  }
}

</script>

<style scoped lang="scss">
.topbar {
  --background: var(--parsec-color-light-secondary-background);
  display: flex;
  padding: 2em;
}

.topbar-right {
  display: flex;
  gap: 1.5em;
}

.home-icon {
  margin-right: 1rem;
}

.breadcrumb-element {
  cursor: pointer;
}

.topbar-button__list {
  display: flex;
  gap: 1.5em;
  align-items: center;
  &::after{
    content: '';
    display: block;
    width: 1px;
    height: 1.5em;
    margin: 0 .5em 0 1em;
    background: var(--parsec-color-light-secondary-light);
  }
}

.topbar-button__item, .sc-ion-buttons-md-s .button {
  border: 1px solid var(--parsec-color-light-secondary-light);
  color: var(--parsec-color-light-primary-700);
  border-radius: 50%;
  --padding-top: 0;
  --padding-end: 0;
  --padding-bottom: 0;
  --padding-start: 0;
  width: 3em;
  height: 3em;

  &:hover {
    --background-hover: var(--parsec-color-light-primary-50);
    background: var(--parsec-color-light-primary-50);
    border: var(--parsec-color-light-primary-50);
  }

  .button-native {
    --padding-top: 0;
    --padding-end: 0;
    --padding-bottom: 0;
    --padding-start: 0;
  }

  ion-icon {
    font-size: 1.375rem;
  }
}

.breadcrumb {
  padding: 0;
  color: var(--parsec-color-light-secondary-grey);

  &-active {
    color: var(--parsec-color-light-primary-700)
  }
}

</style>
