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

const translationPrefix = 'clientArea.dashboard.step';

const OrderStepsTranslations: Record<OrderStep, OrderStepTranslations> = {
  [OrderStep.Received]: {
    tag: `${translationPrefix}.requestSent.tag`,
    title: `${translationPrefix}.requestSent.title`,
    description: `${translationPrefix}.requestSent.description`,
  },
  [OrderStep.Processing]: {
    tag: `${translationPrefix}.processing.tag`,
    title: `${translationPrefix}.processing.title`,
    description: `${translationPrefix}.processing.description`,
  },
  [OrderStep.Confirmed]: {
    tag: `${translationPrefix}.confirmed.tag`,
    title: `${translationPrefix}.confirmed.title`,
    description: `${translationPrefix}.confirmed.description`,
  },
  [OrderStep.InvoiceToBePaid]: {
    tag: `${translationPrefix}.invoiceToBePaid.tag`,
    title: `${translationPrefix}.invoiceToBePaid.title`,
    description: `${translationPrefix}.invoiceToBePaid.description`,
  },
  [OrderStep.Available]: {
    tag: `${translationPrefix}.organizationAvailable.tag`,
    title: `${translationPrefix}.organizationAvailable.title`,
    description: `${translationPrefix}.organizationAvailable.description`,
  },
  [OrderStep.Standby]: {
    tag: `${translationPrefix}.standby.tag`,
    title: `${translationPrefix}.standby.title`,
    description: `${translationPrefix}.standby.description`,
  },
  [OrderStep.Cancelled]: {
    tag: `${translationPrefix}.cancel.tag`,
    title: `${translationPrefix}.cancel.title`,
    description: `${translationPrefix}.cancel.description`,
  },
  [OrderStep.Unknown]: {
    tag: `${translationPrefix}.error.tag`,
    title: `${translationPrefix}.error.title`,
    description: `${translationPrefix}.error.description`,
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
