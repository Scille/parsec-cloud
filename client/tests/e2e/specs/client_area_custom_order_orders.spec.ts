// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, MockBms, msTest } from '@tests/e2e/helpers';

msTest('Test initial status', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.topbar').locator('.topbar-left-text__title');
  await expect(title).toHaveText('Orders');
  await MockBms.mockListOrganizations(clientAreaCustomOrder);
  const container = clientAreaCustomOrder.locator('.client-page-orders');
  await expect(container.locator('.orders-new-title__text')).toHaveText('Do you need to create a new organization?');
  await expect(container.locator('.orders-new-title__button')).toHaveText('Request a new organization');
  await expect(container.locator('.no-orders')).toBeHidden();

  const orders = container.locator('.order-progress');
  await expect(orders).toHaveCount(2);
  await expect(orders.locator('.order-header').locator('.order-header__title')).toHaveText([
    'Order n°ORD-YY-00002 Available',
    'Order n°ORD-YY-00001 Confirmed',
  ]);
  const order1 = orders.nth(1).locator('.order-content');
  const order2 = orders.nth(0).locator('.order-content');
  await expect(order1).toBeHidden();
  await expect(order2).toBeHidden();

  // First order, no organization attached
  await orders.nth(1).click();
  await expect(order1).toBeVisible();
  await expect(order1.locator('.details-list-item').nth(0).locator('.details-list-item__title')).toHaveText('Amount of users');
  await expect(order1.locator('.details-list-item').nth(0).locator('.details-list-item__data')).toHaveText('From 100 to 300 people');
  await expect(order1.locator('.details-list-item').nth(1).locator('.details-list-item__title')).toHaveText('Amount of data');
  await expect(order1.locator('.details-list-item').nth(1).locator('.details-list-item__data')).toHaveText('From 500 GB to 1 TB');

  // Second order with an organization
  await orders.nth(0).click();
  await expect(order2).toBeVisible();
  const items = order2.locator('.details-list-item');
  await expect(items.nth(0).locator('.details-list-item__title')).toHaveText('Amount of users');
  await expect(items.nth(0).locator('.details-list-item__data')).toHaveText(['32 administrators', '50 members', '100 externals']);
  await expect(items.nth(1).locator('.details-list-item__title')).toHaveText('Amount of data');
  await expect(items.nth(1).locator('.details-list-item__data')).toHaveText('2 B');
  await expect(items.nth(2).locator('.details-list-item__title')).toHaveText('Starting date');
  await expect(items.nth(2).locator('.details-list-item__data')).toHaveText('Jan 3, 2024');
  await expect(items.nth(3).locator('.details-list-item__title')).toHaveText('Ending date');
  await expect(items.nth(3).locator('.details-list-item__data')).toHaveText('Feb 3, 2024');
});

msTest.describe(() => {
  msTest.use({
    clientAreaInitialParams: {
      mocks: {
        mockCustomOrderRequest: { overload: { noRequest: true } },
      },
    },
  });

  msTest('Initial status with no orders', async ({ clientAreaCustomOrder }) => {
    const title = clientAreaCustomOrder.locator('.topbar').locator('.topbar-left-text__title');
    await expect(title).toHaveText('Orders');

    const container = clientAreaCustomOrder.locator('.client-page-orders');

    await expect(container.locator('.no-orders')).toBeVisible();
    await expect(container.locator('.no-orders')).toHaveText('You do not have any pending orders.');
  });
});

msTest.describe(() => {
  msTest.use({
    clientAreaInitialParams: {
      mocks: {
        mockCustomOrderRequest: { options: { GET: { timeout: true } } },
      },
    },
  });

  msTest('Get custom orders error', async ({ clientAreaCustomOrder }) => {
    const title = clientAreaCustomOrder.locator('.topbar').locator('.topbar-left-text__title');
    await expect(title).toHaveText('Orders');

    const container = clientAreaCustomOrder.locator('.client-page-orders');

    await expect(container.locator('.no-orders')).toBeVisible();
    await expect(container.locator('.no-orders')).toHaveText('Failed to retrieve the information');
  });
});

msTest('Order new org', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockListOrganizations(clientAreaCustomOrder);
  const title = clientAreaCustomOrder.locator('.topbar').locator('.topbar-left-text__title');
  await expect(title).toHaveText('Orders');
  const container = clientAreaCustomOrder.locator('.client-page-orders');
  await expect(container.locator('.orders-new-title__button')).toHaveText('Request a new organization');
  const modal = clientAreaCustomOrder.locator('.new-order-modal');
  await expect(modal).toBeHidden();
  await container.locator('.orders-new-title__button').click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Request a new on-premise contract');
  const okButton = modal.locator('#next-button');
  await expect(okButton).toHaveText('Send the request');
  await expect(okButton).toBeTrulyDisabled();
  await modal.locator('textarea').fill('Request info');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(modal).toBeHidden();
  await expect(clientAreaCustomOrder).toShowToast('Your request has been sent.', 'Success');
});

msTest('Order new org fail', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockListOrganizations(clientAreaCustomOrder);
  await MockBms.mockCustomOrderRequest(clientAreaCustomOrder, undefined, { POST: { timeout: true } });
  const title = clientAreaCustomOrder.locator('.topbar').locator('.topbar-left-text__title');
  await expect(title).toHaveText('Orders');
  const container = clientAreaCustomOrder.locator('.client-page-orders');
  await expect(container.locator('.orders-new-title__button')).toHaveText('Request a new organization');
  const modal = clientAreaCustomOrder.locator('.new-order-modal');
  await expect(modal).toBeHidden();
  await container.locator('.orders-new-title__button').click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Request a new on-premise contract');
  const okButton = modal.locator('#next-button');
  await expect(okButton).toHaveText('Send the request');
  await expect(okButton).toBeTrulyDisabled();
  await modal.locator('textarea').fill('Request info');
  await expect(okButton).toBeTrulyEnabled();
  const error = modal.locator('.ms-error');
  await expect(error).toBeHidden();
  await okButton.click();
  await expect(modal).toBeVisible();
  await expect(error).toBeVisible();
  await expect(error).toHaveText('An error occurred (Failed to contact the server).');
});
