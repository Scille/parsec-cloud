// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BmsOrganization } from '@/services/bms';
import { DateTime } from 'luxon';

export enum ClientAreaPages {
  BillingDetails = 'billing-details',
  Contracts = 'contracts',
  Dashboard = 'dashboard',
  Invoices = 'invoices',
  PaymentMethods = 'payment-methods',
  PersonalData = 'personal-data',
  Statistics = 'statistics',
}

export enum BillingSystem {
  MonthlySubscription = 'monthlySubscription',
  CustomOrder = 'customOrder',
}

export const DefaultBmsOrganization: BmsOrganization = {
  bmsId: '',
  createdAt: DateTime.utc(),
  parsecId: '',
  name: '',
  bootstrapLink: '',
};

export function isDefaultOrganization(organization: BmsOrganization): boolean {
  return organization.bmsId === DefaultBmsOrganization.bmsId;
}
