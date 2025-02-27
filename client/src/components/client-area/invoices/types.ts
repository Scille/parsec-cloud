// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DataType, InvoiceStatus } from '@/services/bms';
import { DateTime } from 'luxon';

export interface InvoiceData {
  type: DataType.CustomOrderDetails | DataType.BmsInvoice;
  id: string;
  date: DateTime;
  number: string;
  organizationId: string;
  price: number;
  licenseStart?: DateTime;
  licenseEnd?: DateTime;
  status: InvoiceStatus;
  link: string;
}

export interface InvoicesData {
  type: DataType.CustomOrderDetails | DataType.BmsInvoice;
  invoices: InvoiceData[];
}
