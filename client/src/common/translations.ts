// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ComposerTranslation } from 'vue-i18n';
import { WorkspaceRole, InvitationStatus } from '@/parsec';

export function translateWorkspaceRole(t: ComposerTranslation, role: WorkspaceRole | null): string {
  switch (role) {
    case null: {
      return t('workspaceRoles.none');
    }
    case WorkspaceRole.Reader: {
      return t('workspaceRoles.reader');
    }
    case WorkspaceRole.Contributor: {
      return t('workspaceRoles.contributor');
    }
    case WorkspaceRole.Manager: {
      return t('workspaceRoles.manager');
    }
    case WorkspaceRole.Owner: {
      return t('workspaceRoles.owner');
    }
  }
}

export function translateInvitationStatus(t: ComposerTranslation, status: InvitationStatus): string {
  switch (status) {
    case InvitationStatus.Ready:
      return t('UsersPage.invitation.status.ready');
    case InvitationStatus.Idle:
      return t('UsersPage.invitation.status.idle');
    case InvitationStatus.Deleted:
      return t('UsersPage.invitation.status.deleted');
  }
}
