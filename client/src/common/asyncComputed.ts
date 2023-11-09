// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Greatly inspired from https://github.com/vueuse/vueuse

import { computed, isRef, ref, Ref, shallowRef, watchEffect } from 'vue';

export type AsyncComputedOnCancel = (cancelCallback: any) => void;

export interface AsyncComputedOptions {
  lazy?: boolean;
  evaluating?: Ref<boolean>;
  shallow?: boolean;
  onError?: (e: unknown) => void;
}

export function asyncComputed<T>(
  evaluationCallback: (onCancel: AsyncComputedOnCancel) => T | Promise<T>,
  initialState?: T,
  optionsOrRef?: Ref<boolean> | AsyncComputedOptions,
): Ref<T> {
  let options: AsyncComputedOptions;

  if (isRef(optionsOrRef)) {
    options = {
      evaluating: optionsOrRef,
    };
  } else {
    options = optionsOrRef || {};
  }

  const { lazy = false, evaluating = undefined, shallow = true } = options;

  const started = ref(!lazy);
  const current = (shallow ? shallowRef(initialState) : ref(initialState)) as Ref<T>;
  let counter = 0;

  watchEffect(async (onInvalidate) => {
    if (!started.value) return;

    counter++;
    const counterAtBeginning = counter;
    let hasFinished = false;

    // Defer initial setting of `evaluating` ref
    // to avoid having it as a dependency
    if (evaluating) {
      Promise.resolve().then(() => {
        evaluating.value = true;
      });
    }

    try {
      const result = await evaluationCallback((cancelCallback) => {
        onInvalidate(() => {
          if (evaluating) evaluating.value = false;

          if (!hasFinished) cancelCallback();
        });
      });

      if (counterAtBeginning === counter) current.value = result;
    } catch (e) {
      console.log(e);
    } finally {
      if (evaluating && counterAtBeginning === counter) evaluating.value = false;

      hasFinished = true;
    }
  });

  if (lazy) {
    return computed(() => {
      started.value = true;
      return current.value;
    });
  } else {
    return current;
  }
}
