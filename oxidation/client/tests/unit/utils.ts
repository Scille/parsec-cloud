import { VueWrapper } from '@vue/test-utils';

export function clearEmitted(wrapper: VueWrapper): void {
  for (const events of Object.values(wrapper.emitted())) events.length = 0;
}

export async function getSpyOnLastCallResult(spyOn: jest.SpyInstance): Promise<any> {
  return await spyOn.mock.results[0].value;
}

export class MockDate extends Date {
  constructor(arg: any) {
      super(arg || 1);
  }
}

export const mockDateValue = '1970-01-01T00:00:00.001Z';
