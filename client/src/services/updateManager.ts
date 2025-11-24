// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { UpdateAvailabilityData } from '@/services/eventDistributor';
import { ref, Ref } from 'vue';

export const updateAvailability: Ref<UpdateAvailabilityData | null> = ref(null);
export const updateDismissed: Ref<boolean> = ref(false);

export function dismissUpdate(): void {
  updateDismissed.value = true;
}

export function resetUpdateDismissal(): void {
  updateDismissed.value = false;
}
