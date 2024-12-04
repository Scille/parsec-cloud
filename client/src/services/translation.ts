// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { InvitationStatus, UserProfile, WorkspaceRole } from '@/parsec';
import { CustomOrderRequestStatus, CustomOrderStatus, InvoiceStatus } from '@/services/bms';
import { Locale, Translatable } from 'megashark-lib';

export function getProfileTranslationKey(profile: UserProfile): Translatable {
  if (profile === UserProfile.Admin) {
    return 'UsersPage.profile.admin.label';
  } else if (profile === UserProfile.Standard) {
    return 'UsersPage.profile.standard.label';
  } else if (profile === UserProfile.Outsider) {
    return 'UsersPage.profile.outsider.label';
  }
  return '';
}

export function getInvoiceStatusTranslationKey(status: InvoiceStatus): Translatable {
  switch (status) {
    case InvoiceStatus.Paid:
      return 'clientArea.invoices.status.paid';
    case InvoiceStatus.Draft:
      return 'clientArea.invoices.status.draft';
    case InvoiceStatus.Open:
      return 'clientArea.invoices.status.toPay';
    case InvoiceStatus.Uncollectible:
      return 'clientArea.invoices.status.uncollectible';
    default:
      return 'clientArea.invoices.status.void';
  }
}

interface WorkspaceRoleTranslations {
  label: Translatable;
  description?: Translatable;
}

export function getWorkspaceRoleTranslationKey(role: WorkspaceRole | null): WorkspaceRoleTranslations {
  switch (role) {
    case null: {
      return {
        label: 'workspaceRoles.none',
      };
    }
    case WorkspaceRole.Reader: {
      return {
        label: 'workspaceRoles.reader.label',
        description: 'workspaceRoles.reader.description',
      };
    }
    case WorkspaceRole.Contributor: {
      return {
        label: 'workspaceRoles.contributor.label',
        description: 'workspaceRoles.contributor.description',
      };
    }
    case WorkspaceRole.Manager: {
      return {
        label: 'workspaceRoles.manager.label',
        description: 'workspaceRoles.manager.description',
      };
    }
    case WorkspaceRole.Owner: {
      return {
        label: 'workspaceRoles.owner.label',
        description: 'workspaceRoles.owner.description',
      };
    }
  }
}

interface CustomOrderStatusTranslations {
  title: Translatable;
  description: Translatable;
}

export function getCustomOrderStatusTranslationKey(
  statusBms: CustomOrderStatus,
  statusSellsy: CustomOrderRequestStatus,
): CustomOrderStatusTranslations {
  const key = `${statusBms}-${statusSellsy}`;

  switch (key) {
    case `${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Received}`:
      return {
        title: 'clientArea.dashboard.step.requestSent.title',
        description: 'clientArea.dashboard.step.requestSent.description',
      };
    case `${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Processing}`:
      return {
        title: 'clientArea.dashboard.step.processing.title',
        description: 'clientArea.dashboard.step.processing.description',
      };
    case `${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Finished}`:
      return {
        title: 'clientArea.dashboard.step.validate.title',
        description: 'clientArea.dashboard.step.validate.description',
      };
    case `${CustomOrderStatus.InvoiceToBePaid}-${CustomOrderRequestStatus.Finished}`:
      return {
        title: 'clientArea.dashboard.step.invoiceToBePaid.title',
        description: 'clientArea.dashboard.step.invoiceToBePaid.description',
      };
    case `${CustomOrderStatus.InvoicePaid}-${CustomOrderRequestStatus.Finished}`:
      return {
        title: 'clientArea.dashboard.step.organizationAvailable.title',
        description: 'clientArea.dashboard.step.organizationAvailable.description',
      };
    default:
      return {
        title: 'clientArea.dashboard.step.error.title',
        description: 'clientArea.dashboard.step.error.description',
      };
  }
}

export function getInvitationStatusTranslationKey(status: InvitationStatus): Translatable {
  switch (status) {
    case InvitationStatus.Pending:
      return 'UsersPage.invitation.status.pending';
    case InvitationStatus.Finished:
      return 'UsersPage.invitation.status.finished';
    case InvitationStatus.Cancelled:
      return 'UsersPage.invitation.status.cancelled';
  }
}

export function longLocaleCodeToShort(longCode: Locale): string {
  return longCode.split('-')[0];
}
