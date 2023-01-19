import { mount, VueWrapper } from '@vue/test-utils';
import PasswordInput from '@/components/PasswordInput.vue';
import { clearEmitted } from './utils';

describe('PasswordInput.vue', () => {
  const wrapper = mount(PasswordInput, {
    props: {
      label: 'TestLabel'
    }
  });
  let ionInput: VueWrapper;

  beforeAll(() => {
    ionInput = wrapper.findComponent('ion-input') as VueWrapper;
  });

  beforeEach(() => {
    jest.clearAllMocks();
    clearEmitted(ionInput);
    clearEmitted(wrapper);
    ionInput.setValue('');
  });

  it('renders password input', () => {
    expect(wrapper.find('ion-label').text()).toBe('TestLabel');
  });

  it('should emit "change" event with expected value on password input change', () => {
    ionInput.vm.$emit('ionChange', {detail: {value: 'expected_value'}});
    expect(wrapper.emitted('change')).toEqual([['expected_value']]);
  });

  it('should not emit "enter" event when password is empty and on password input enter keyup', () => {
    const spyOnEnterPress = jest.spyOn(wrapper.vm, 'onEnterPress');
    ionInput.trigger('keyup.enter');
    expect(spyOnEnterPress).toHaveBeenCalledTimes(1);
    expect(wrapper.emitted('enter')).toBeUndefined();
  });

  it('should emit "enter" event when password is not empty and on password input enter keyup', async () => {
    const spyOnEnterPress = jest.spyOn(wrapper.vm, 'onEnterPress');
    await ionInput.setValue('password');
    ionInput.trigger('keyup.enter');
    expect(spyOnEnterPress).toHaveBeenCalledTimes(1);
    expect(wrapper.emitted('enter')).toHaveLength(1);
  });
});
