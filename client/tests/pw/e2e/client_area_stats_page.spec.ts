// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { MockBms } from '@tests/pw/helpers/bms';
import { DEFAULT_ORGANIZATION_DATA_SLICE } from '@tests/pw/helpers/data';
import { msTest } from '@tests/pw/helpers/fixtures';

[
  {
    data: 0,
    bar1: { progress: 0 },
    bar2: { visible: false },
    bar3: { visible: false },
  },
  {
    data: DEFAULT_ORGANIZATION_DATA_SLICE.free * 0.74,
    bar1: { progress: 74 },
    bar2: { visible: false },
    bar3: { visible: false },
  },
  {
    data: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 0.7,
    bar1: { progress: 100 },
    bar2: { visible: true, progress: 70, count: 0 },
    bar3: { visible: false },
  },
  {
    data: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 1.51,
    bar1: { progress: 100 },
    bar2: { visible: true, progress: 100, count: 1 },
    bar3: { visible: true, progress: 51 },
  },
  {
    data: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 2.654,
    bar1: { progress: 100 },
    bar2: { visible: true, progress: 100, count: 2 },
    bar3: { visible: true, progress: 65 },
  },
  {
    data: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 3,
    bar1: { progress: 100 },
    bar2: { visible: true, progress: 100, count: 3 },
    bar3: { visible: true, progress: 0 },
  },
  {
    data: DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying * 29.1,
    bar1: { progress: 100 },
    bar2: { visible: true, progress: 100, count: 29 },
    bar3: { visible: true, progress: 10 },
  },
].forEach(({ data, bar1, bar2, bar3 }) => {
  msTest(
    `Test stats progress bars Data(${data / (DEFAULT_ORGANIZATION_DATA_SLICE.free + DEFAULT_ORGANIZATION_DATA_SLICE.paying)})`,
    async ({ clientArea }) => {
      await MockBms.mockOrganizationStats(clientArea, data);

      await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();

      await expect(clientArea.locator('#firstBar')).toContainText(`${bar1.progress}%`);

      if (bar2.visible) {
        await expect(clientArea.locator('#secondBar')).toBeVisible();
        await expect(clientArea.locator('#secondBar')).toContainText(`${bar2.progress}%`);
        if (bar2.count === undefined || bar2.count === 0) {
          await expect(clientArea.locator('#secondBar')).not.toContainText('x');
        } else {
          await expect(clientArea.locator('#secondBar')).toContainText(`x${bar2.count}`);
        }
      }

      if (bar3.visible) {
        await expect(clientArea.locator('#thirdBar')).toBeVisible();
        await expect(clientArea.locator('#thirdBar')).toContainText(`${bar3.progress}%`);
      }
    },
  );
});
