// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BmsOrganization } from '@/services/bms';
import { DateTime } from 'luxon';

export enum ClientAreaPages {
  BillingDetails = 'billing-details',
  CustomOrderBillingDetails = 'custom-order-billing-details',
  Contracts = 'contracts',
  Dashboard = 'dashboard',
  Invoices = 'invoices',
  CustomOrderInvoices = 'custom-order-invoices',
  PaymentMethods = 'payment-methods',
  PersonalData = 'personal-data',
  Statistics = 'statistics',
}

export const DefaultBmsOrganization: BmsOrganization = {
  bmsId: '',
  createdAt: DateTime.utc(),
  parsecId: '',
  name: '',
  bootstrapLink: '',
  isSubscribed: () => false,
};

export function isDefaultOrganization(organization: BmsOrganization): boolean {
  return organization.bmsId === DefaultBmsOrganization.bmsId;
}
