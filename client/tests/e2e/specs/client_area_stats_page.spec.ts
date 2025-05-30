// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_ORGANIZATION_DATA_SLICE, MockBms, clientAreaNavigateTo, expect, msTest } from '@tests/e2e/helpers';

[
  {
    overload: { dataSize: 0, metadataSize: 0 },
    totalData: '0 B',
    freePercentage: '0%',
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free * 0.74, metadataSize: 0 },
    totalData: '74.0 GB',
    freePercentage: '74%',
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 1.7, metadataSize: 0 },
    totalData: '270 GB',
    freePercentage: '100%',
    payingData: '170 GB',
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 2.51, metadataSize: 0 },
    totalData: '351 GB',
    freePercentage: '100%',
    payingData: '251 GB',
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 3.654, metadataSize: 0 },
    totalData: '465 GB',
    freePercentage: '100%',
    payingData: '365 GB',
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 4, metadataSize: 0 },
    totalData: '500 GB',
    freePercentage: '100%',
    payingData: '400 GB',
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 30.1, metadataSize: 0 },
    totalData: '3.03 TB',
    freePercentage: '100%',
    payingData: '2.93 TB',
  },
].forEach(({ overload, totalData, freePercentage, payingData }) => {
  msTest(`Test stats progress bars Data(${totalData})`, async ({ clientArea }) => {
    await MockBms.mockOrganizationStats(clientArea, overload);
    await clientAreaNavigateTo(clientArea, 'Statistics');

    await expect(clientArea.locator('.storage').locator('.storage-detail-data__total')).toHaveText(totalData);
    await expect(clientArea.locator('.storage').locator('.circle__amount')).toHaveText(freePercentage);
    await expect(clientArea.locator('.extra').locator('.usage-data-caption__title')).toHaveText(payingData || '0 B');
  });
});
