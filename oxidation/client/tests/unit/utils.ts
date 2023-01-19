import { VueWrapper } from '@vue/test-utils';

export function clearEmitted(wrapper: VueWrapper): void {
  for (const events of Object.values(wrapper.emitted())) events.length = 0;
}
