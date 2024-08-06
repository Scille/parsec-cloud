// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { MockBms } from '@tests/pw/helpers/bms';
import { DEFAULT_ORGANIZATION_DATA_SLICE } from '@tests/pw/helpers/data';
import { msTest } from '@tests/pw/helpers/fixtures';

[
  {
    overload: { dataSize: 0, metadataSize: 0 },
    dataText: '0 B',
    bar1: { text: '0 B 0%' },
    bar2: { visible: false },
    bar3: { visible: false },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free * 0.74, metadataSize: 0 },
    dataText: '148 GB',
    bar1: { text: '148 GB 74%' },
    bar2: { visible: false },
    bar3: { visible: false },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 0.7, metadataSize: 0 },
    dataText: '270 GB',
    bar1: { text: '200 GB 100%' },
    bar2: { visible: true, text: '70.0 GB 70%' },
    bar3: { visible: false },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 1.51, metadataSize: 0 },
    dataText: '351 GB',
    bar1: { text: '200 GB 100%' },
    bar2: { visible: true, text: '100 GB x1 100%' },
    bar3: { visible: true, text: '51.0 GB 51%' },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 2.654, metadataSize: 0 },
    dataText: '465 GB',
    bar1: { text: '200 GB 100%' },
    bar2: { visible: true, text: '100 GB x2 100%' },
    bar3: { visible: true, text: '65.4 GB 65%' },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 3, metadataSize: 0 },
    dataText: '500 GB',
    bar1: { text: '200 GB 100%' },
    bar2: { visible: true, text: '100 GB x3 100%' },
    bar3: { visible: true, text: '0 B 0%' },
  },
  {
    overload: { dataSize: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 29.1, metadataSize: 0 },
    dataText: '3.03 TB',
    bar1: { text: '200 GB 100%' },
    bar2: { visible: true, text: '100 GB x29 100%' },
    bar3: { visible: true, text: '10.0 GB 10%' },
  },
].forEach(({ overload, dataText, bar1, bar2, bar3 }) => {
  msTest(`Test stats progress bars Data(${dataText})`, async ({ clientArea }) => {
    await MockBms.mockOrganizationStats(clientArea, overload);

    await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();

    await expect(clientArea.locator('.bottom-part-data').locator('h2')).toHaveText(dataText);
    await expect(clientArea.locator('#firstBar')).toHaveText(bar1.text);

    if (bar2.visible) {
      await expect(clientArea.locator('#secondBar')).toBeVisible();
      if (bar2.text) {
        await expect(clientArea.locator('#secondBar')).toHaveText(bar2.text);
      }
    } else {
      await expect(clientArea.locator('#secondBar')).toBeHidden();
    }

    if (bar3.visible) {
      await expect(clientArea.locator('#thirdBar')).toBeVisible();
      if (bar3.text) {
        await expect(clientArea.locator('#thirdBar')).toHaveText(bar3.text);
      }
    } else {
      await expect(clientArea.locator('#thirdBar')).toBeHidden();
    }
  });
});
