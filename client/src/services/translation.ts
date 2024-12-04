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
  description?: Translatable;
}

export function getCustomOrderStatusTranslationKey(
  statusBms: CustomOrderStatus,
  statusSellsy: CustomOrderRequestStatus,
): CustomOrderStatusTranslations {
  const key = `${statusBms}-${statusSellsy}`;
  const locale = 'clientArea.dashboard.step';

  switch (key) {
    case `${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Received}`:
      return {
        title: `${locale}.requestSent.title`,
        description: `${locale}.requestSent.description`,
      };
    case `${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Processing}`:
      return {
        title: `${locale}.processing.title`,
        description: `${locale}.processing.description`,
      };
    case `${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Finished}`:
      return {
        title: `${locale}.validate.title`,
        description: `${locale}.validate.description`,
      };
    case `${CustomOrderStatus.InvoiceToBePaid}-${CustomOrderRequestStatus.Finished}`:
      return {
        title: `${locale}.invoiceToBePaid.title`,
        description: `${locale}.invoiceToBePaid.description`,
      };
    case `${CustomOrderStatus.InvoicePaid}-${CustomOrderRequestStatus.Finished}`:
      return {
        title: `${locale}.organizationAvailable.title`,
        description: `${locale}.organizationAvailable.description`,
      };
    default:
      return {
        title: `${locale}.error.title`,
        description: `${locale}.error.description`,
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
