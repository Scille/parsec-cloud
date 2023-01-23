import { mount, VueWrapper } from '@vue/test-utils';
import PasswordInput from '@/components/PasswordInput.vue';
import { clearEmitted } from './utils';
import {
  eyeOutline,
  eyeOffOutline
} from 'ionicons/icons';

describe('PasswordInput.vue', () => {
  const wrapper = mount(PasswordInput, {
    props: {
      label: 'TestLabel'
    }
  });
  let ionInput: VueWrapper;
  let ionButton: VueWrapper;
  let ionIcon: VueWrapper;

  beforeAll(() => {
    ionInput = wrapper.findComponent('ion-input') as VueWrapper;
    ionButton = wrapper.findComponent('ion-button') as VueWrapper;
    ionIcon = wrapper.findComponent('ion-icon') as VueWrapper;
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

  it('should toggle password visibility button icon and password input type on password visibility button click', async () => {
    // check initial state with passwordVisible = false
    expect(ionInput.props('type')).toEqual('password');
    expect(ionIcon.props('icon')).toEqual(eyeOutline);
    // check state when button should toggle passwordVisible to true
    await ionButton.trigger('click');
    expect(ionInput.props('type')).toEqual('text');
    expect(ionIcon.props('icon')).toEqual(eyeOffOutline);
  });

  it('should restore password visibility button icon and password input type on password visibility button second click', async () => {
    // check current state when previous click is still in memory
    expect(ionInput.props('type')).toEqual('text');
    expect(ionIcon.props('icon')).toEqual(eyeOffOutline);
    // check state when button should toggle passwordVisible back to false
    await ionButton.trigger('click');
    expect(ionInput.props('type')).toEqual('password');
    expect(ionIcon.props('icon')).toEqual(eyeOutline);
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
