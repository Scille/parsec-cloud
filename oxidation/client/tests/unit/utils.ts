import { VueWrapper } from '@vue/test-utils';

export function clearEmitted(wrapper: VueWrapper): void {
  for (const events of Object.values(wrapper.emitted())) events.length = 0;
}

export async function getSpyOnLastCallResult(spyOn: jest.SpyInstance): Promise<any> {
  return await spyOn.mock.results[0].value;
}
