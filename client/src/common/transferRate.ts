// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

interface Sample {
  remainingBytes: number;
  timestamp: number;
}

// Period for which we computed the value
const RATE_WINDOW_MS = 20 * 1000;
// Period before giving a first estimate, to let the speed stabilize
const MIN_OBSERVATION_MS = 3 * 1000;

class TransferRateCalculator {
  private samples: Sample[] = [];

  update(bytesLeft: number, filesLeft: number, now: number = Date.now()): void {
    if (bytesLeft === 0 && filesLeft === 0) {
      this.clear();
      return;
    }
    this.samples.push({ remainingBytes: bytesLeft, timestamp: now });
    // Drop the samples outside the window
    while (this.samples.length > 2 && this.samples[1].timestamp <= now - RATE_WINDOW_MS) {
      this.samples.shift();
    }
  }

  private getRate(now: number): number | undefined {
    if (this.samples.length < 2) {
      return undefined;
    }
    const elapsedMs = now - this.samples[0].timestamp;
    if (elapsedMs < MIN_OBSERVATION_MS) {
      return undefined;
    }
    let progress = 0;
    for (let i = 1; i < this.samples.length; i++) {
      progress += Math.max(0, this.samples[i - 1].remainingBytes - this.samples[i].remainingBytes);
    }
    // Dividing by the time elapsed until now to decreases the rate over time if upload stalls.
    return progress / (elapsedMs / 1000);
  }

  // Estimated time left in seconds, `Infinity` if it cannot be estimated
  getEta(now: number = Date.now()): number {
    const rate = this.getRate(now);
    if (rate === undefined || rate <= 0) {
      return Infinity;
    }
    const remaining = this.samples[this.samples.length - 1].remainingBytes;
    return remaining / rate;
  }

  clear(): void {
    this.samples = [];
  }
}

export { TransferRateCalculator };
