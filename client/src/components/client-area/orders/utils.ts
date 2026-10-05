// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { CustomOrderRequestStatus, CustomOrderStatus } from '@/services/bms';
import { Translatable } from 'megashark-lib';

export enum OrderStep {
  Received = 'received',
  Processing = 'processing',
  Confirmed = 'confirmed',
  InvoiceToBePaid = 'invoiceToBePaid',
  Available = 'available',
  Standby = 'standby',
  Cancelled = 'cancelled',
  Unknown = 'unknown',
}

export interface OrderStepTranslations {
  tag: Translatable;
  title: Translatable;
  description: Translatable;
}

const OrderStepsTranslations: Record<OrderStep, OrderStepTranslations> = {
  [OrderStep.Received]: {
    tag: 'clientArea.dashboard.step.requestSent.tag',
    title: 'clientArea.dashboard.step.requestSent.title',
    description: 'clientArea.dashboard.step.requestSent.description',
  },
  [OrderStep.Processing]: {
    tag: 'clientArea.dashboard.step.processing.tag',
    title: 'clientArea.dashboard.step.processing.title',
    description: 'clientArea.dashboard.step.processing.description',
  },
  [OrderStep.Confirmed]: {
    tag: 'clientArea.dashboard.step.confirmed.tag',
    title: 'clientArea.dashboard.step.confirmed.title',
    description: 'clientArea.dashboard.step.confirmed.description',
  },
  [OrderStep.InvoiceToBePaid]: {
    tag: 'clientArea.dashboard.step.invoiceToBePaid.tag',
    title: 'clientArea.dashboard.step.invoiceToBePaid.title',
    description: 'clientArea.dashboard.step.invoiceToBePaid.description',
  },
  [OrderStep.Available]: {
    tag: 'clientArea.dashboard.step.organizationAvailable.tag',
    title: 'clientArea.dashboard.step.organizationAvailable.title',
    description: 'clientArea.dashboard.step.organizationAvailable.description',
  },
  [OrderStep.Standby]: {
    tag: 'clientArea.dashboard.step.standby.tag',
    title: 'clientArea.dashboard.step.standby.title',
    description: 'clientArea.dashboard.step.standby.description',
  },
  [OrderStep.Cancelled]: {
    tag: 'clientArea.dashboard.step.cancel.tag',
    title: 'clientArea.dashboard.step.cancel.title',
    description: 'clientArea.dashboard.step.cancel.description',
  },
  [OrderStep.Unknown]: {
    tag: 'clientArea.dashboard.step.error.tag',
    title: 'clientArea.dashboard.step.error.title',
    description: 'clientArea.dashboard.step.error.description',
  },
};

export function getOrderStep(
  customOrderRequestStatus: CustomOrderRequestStatus | undefined,
  customOrderStatus?: CustomOrderStatus | undefined,
): OrderStep {
  switch (customOrderRequestStatus) {
    case CustomOrderRequestStatus.Received:
      return OrderStep.Received;
    case CustomOrderRequestStatus.Processing:
      return OrderStep.Processing;
    case CustomOrderRequestStatus.Standby:
      return OrderStep.Standby;
    case CustomOrderRequestStatus.Cancelled:
      return OrderStep.Cancelled;
    case CustomOrderRequestStatus.Finished:
      switch (customOrderStatus) {
        case CustomOrderStatus.NothingLinked:
        case CustomOrderStatus.EstimateLinked:
          return OrderStep.Confirmed;
        case CustomOrderStatus.InvoiceToBePaid:
          return OrderStep.InvoiceToBePaid;
        case CustomOrderStatus.InvoicePaid:
          return OrderStep.Available;
        default:
          return OrderStep.Confirmed;
      }
      break;
    default:
      return OrderStep.Unknown;
  }
  return OrderStep.Unknown;
}

export function getOrderStepTranslations(orderStep: OrderStep): OrderStepTranslations {
  return OrderStepsTranslations[orderStep];
}
