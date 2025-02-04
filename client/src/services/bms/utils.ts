// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { CustomOrderDetailsResultData, DataType } from '@/services/bms/types';
import { DateTime } from 'luxon';

enum CustomOrderRowName {
  Administrator = 'Psc_D0_Adm_M',
  Standard = 'Psc_D0_Std_User_M',
  Outsider = 'Psc_D0_Ext_User_M',
  // cspell:disable-next-line
  Storage = 'Psc_Stck_100_Go_M',
}

enum CustomOrderFieldName {
  LicenseStart = 'parsec-saas-custom-order-start-date',
  LicenseEnd = 'parsec-saas-custom-order-end-date',
  StandardCount = 'parsec-saas-custom-order-standard-license-count',
  OutsiderCount = 'parsec-saas-custom-order-outsider-license-count',
  AdministratorCount = 'parsec-saas-custom-order-admin-license-count',
  StorageCount = 'parsec-saas-custom-order-storage-license-count',
}

export function parseCustomOrderInvoice(data: any): CustomOrderDetailsResultData {
  const adminRow = data.rows?.find((row: any) => row.reference === CustomOrderRowName.Administrator);
  const standardRow = data.rows?.find((row: any) => row.reference === CustomOrderRowName.Standard);
  const outsiderRow = data.rows?.find((row: any) => row.reference === CustomOrderRowName.Outsider);
  const storageRow = data.rows?.find((row: any) => row.reference === CustomOrderRowName.Storage);

  const fields = data._embed?.custom_fields || [];
  const adminField = fields.find((field: any) => field.code === CustomOrderFieldName.AdministratorCount);
  const standardField = fields.find((field: any) => field.code === CustomOrderFieldName.StandardCount);
  const outsiderField = fields.find((field: any) => field.code === CustomOrderFieldName.OutsiderCount);
  const storageField = fields.find((field: any) => field.code === CustomOrderFieldName.StorageCount);
  const licenseStartField = fields.find((field: any) => field.code === CustomOrderFieldName.LicenseStart);
  const licenseEndField = fields.find((field: any) => field.code === CustomOrderFieldName.LicenseEnd);

  return {
    type: DataType.CustomOrderDetails,
    id: data.id,
    link: data.pdf_link,
    number: data.number,
    amountWithTaxes: parseFloat(data.amounts?.total_incl_tax),
    amountWithoutTaxes: parseFloat(data.amounts?.total_excl_tax),
    amountDue: parseFloat(data.amounts?.total_remaining_due_incl_tax),
    currency: data.currency,
    created: DateTime.fromISO(data.created, { zone: 'utc' }),
    dueDate: DateTime.fromISO(data.due_date, { zone: 'utc' }),
    licenseStart: licenseStartField ? DateTime.fromISO(licenseStartField.value, { zone: 'utc' }) : undefined,
    licenseEnd: licenseEndField ? DateTime.fromISO(licenseEndField.value, { zone: 'utc' }) : undefined,
    status: data.status,
    administrators: {
      quantityOrdered: adminField ? parseInt(adminField.value) : 0,
      amountWithTaxes: adminRow ? parseInt(adminRow.amount_tax_inc) : 0,
    },
    standards: {
      quantityOrdered: standardField ? parseInt(standardField.value) : 0,
      amountWithTaxes: standardRow ? parseInt(standardRow.amount_tax_inc) : 0,
    },
    outsiders: {
      quantityOrdered: outsiderField ? parseInt(outsiderField.value) : 0,
      amountWithTaxes: outsiderRow ? parseInt(outsiderRow.amount_tax_inc) : 0,
    },
    storage: {
      quantityOrdered: storageField ? parseInt(storageField.value) : 0,
      amountWithTaxes: storageRow ? parseInt(storageRow.amount_tax_inc) : 0,
    },
  };
}
