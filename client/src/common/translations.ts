// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
import { InvitationStatus, WorkspaceRole } from '@/parsec';
import { ComposerTranslation } from 'vue-i18n';

interface WorkspaceRoleTranslation {
  label: string;
  description?: string;
}

export function translateWorkspaceRole(t: ComposerTranslation, role: WorkspaceRole | null): WorkspaceRoleTranslation {
  switch (role) {
    case null: {
      return {
        label: t('workspaceRoles.none'),
      };
    }
    case WorkspaceRole.Reader: {
      return {
        label: t('workspaceRoles.reader.label'),
        description: t('workspaceRoles.reader.description'),
      };
    }
    case WorkspaceRole.Contributor: {
      return {
        label: t('workspaceRoles.contributor.label'),
        description: t('workspaceRoles.contributor.description'),
      };
    }
    case WorkspaceRole.Manager: {
      return {
        label: t('workspaceRoles.manager.label'),
        description: t('workspaceRoles.manager.description'),
      };
    }
    case WorkspaceRole.Owner: {
      return {
        label: t('workspaceRoles.owner.label'),
        description: t('workspaceRoles.owner.description'),
      };
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
