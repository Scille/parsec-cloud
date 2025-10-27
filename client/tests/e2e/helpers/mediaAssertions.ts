// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect as baseExpect, Locator } from '@playwright/test';

interface AssertReturnType {
  message: () => string;
  pass: boolean;
}

export const expectMedia = baseExpect.extend({
  async toBePaused(locator: Locator): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveJSProperty('paused', true);
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
      return {
        message: () => 'Media is not paused',
        pass: false,
      };
    }
  },

  async toBePlaying(locator: Locator): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveJSProperty('paused', false);
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
      return {
        message: () => 'Media is paused',
        pass: false,
      };
    }
  },

  async toHaveDuration(locator: Locator, duration: number): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveJSProperty('duration', duration);
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
      const actualDuration = await locator.evaluate((el) => {
        return (el as HTMLMediaElement).duration;
      });
      return {
        message: () => `Expected media to have a duration of '${duration}' but has '${actualDuration}'`,
        pass: false,
      };
    }
  },

  async toBeMuted(locator: Locator): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveJSProperty('muted', true);
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
      return {
        message: () => 'Media is not muted',
        pass: false,
      };
    }
  },

  async toLoop(locator: Locator): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveJSProperty('loop', true);
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
      return {
        message: () => 'Media does not loop',
        pass: false,
      };
    }
  },

  async toHaveCurrentTime(locator: Locator, currentTime: number): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveJSProperty('currentTime', currentTime);
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
      const actualTime = await locator.evaluate((el) => {
        return (el as HTMLMediaElement).currentTime;
      });
      return {
        message: () => `Expected media to have a current time of '${currentTime}' but has '${actualTime}'`,
        pass: false,
      };
    }
  },

  async toHaveVolume(locator: Locator, volume: number): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveJSProperty('volume', volume);
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
      const actualVolume = await locator.evaluate((el) => {
        return (el as HTMLMediaElement).volume;
      });
      return {
        message: () => `Expected media to have a volume of '${volume}' but has '${actualVolume}'`,
        pass: false,
      };
    }
  },
});
