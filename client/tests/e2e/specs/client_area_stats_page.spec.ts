// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_ORGANIZATION_DATA_SLICE, MockBms, expect, msTest } from '@tests/e2e/helpers';

[
  {
    overload: { dataSize: 0, metadataSize: 0 },
    dataText: '0 B',
    bar1: { amount: '0 B', percentage: '0%' },
    bar2: { visible: false },
    bar3: { visible: false },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free * 0.74, metadataSize: 0 },
    dataText: '148 GB',
    bar1: { amount: '148 GB', percentage: '74%' },
    bar2: { visible: false },
    bar3: { visible: false },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 0.7, metadataSize: 0 },
    dataText: '270 GB',
    bar1: { amount: '200 GB', percentage: '100%' },
    bar2: { visible: true, amount: '70.0 GB', percentage: '70%' },
    bar3: { visible: false },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 1.51, metadataSize: 0 },
    dataText: '351 GB',
    bar1: { amount: '200 GB', percentage: '100%' },
    bar2: { visible: true, amount: '100 GB', multiplier: 'x1', percentage: '100%' },
    bar3: { visible: true, amount: '51.0 GB', percentage: '51%' },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 2.654, metadataSize: 0 },
    dataText: '465 GB',
    bar1: { amount: '200 GB', percentage: '100%' },
    bar2: { visible: true, amount: '100 GB', multiplier: 'x2', percentage: '100%' },
    bar3: { visible: true, amount: '65.4 GB', percentage: '65%' },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 3, metadataSize: 0 },
    dataText: '500 GB',
    bar1: { amount: '200 GB', percentage: '100%' },
    bar2: { visible: true, amount: '100 GB', multiplier: 'x3', percentage: '100%' },
    bar3: { visible: true, amount: '0 B', percentage: '0%' },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 29.1, metadataSize: 0 },
    dataText: '3.03 TB',
    bar1: { amount: '200 GB', percentage: '100%' },
    bar2: { visible: true, amount: '100 GB', multiplier: 'x29', percentage: '100%' },
    bar3: { visible: true, amount: '10.0 GB', percentage: '10%' },
  },
].forEach(({ overload, dataText, bar1, bar2, bar3 }) => {
  msTest(`Test stats progress bars Data(${dataText})`, async ({ clientArea }) => {
    await MockBms.mockOrganizationStats(clientArea, overload);

    await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();

    await expect(clientArea.locator('.storage').locator('.storage-data-global__total')).toHaveText(dataText);
    await expect(clientArea.locator('#firstBar').locator('.consumption-number__amount')).toHaveText(bar1.amount);
    await expect(clientArea.locator('#firstBar').locator('.consumption-number__percentage')).toHaveText(bar1.percentage);

    if (bar2.visible) {
      await expect(clientArea.locator('#secondBar')).toBeVisible();
      if (bar2.amount) {
        if (bar2.multiplier) {
          await expect(clientArea.locator('#secondBar').locator('.number-multiplier')).toHaveText(`<span>${bar2.multiplier}</span>`);
        }
        await expect(clientArea.locator('#secondBar').locator('.consumption-number__amount')).toHaveText(bar2.amount);
        await expect(clientArea.locator('#secondBar').locator('.consumption-number__percentage')).toHaveText(bar2.percentage);
      }
    } else {
      await expect(clientArea.locator('#secondBar')).toBeHidden();
    }

    if (bar3.visible) {
      await expect(clientArea.locator('#thirdBar')).toBeVisible();
      if (bar3.amount) {
        await expect(clientArea.locator('#thirdBar').locator('.consumption-number__amount')).toHaveText(bar3.amount);
        await expect(clientArea.locator('#thirdBar').locator('.consumption-number__percentage')).toHaveText(bar3.percentage);
      }
    } else {
      await expect(clientArea.locator('#thirdBar')).toBeHidden();
    }
  });
});
