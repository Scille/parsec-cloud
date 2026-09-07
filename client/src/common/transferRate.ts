// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

interface Sample {
  bytes: number;
  timestamp: number;
}

const MAX_SAMPLES = 5;

class TransferRateCalculator {
  private samples: Sample[] = [];

  constructor() {}

  update(bytes: number): void {
    this.samples.push({ bytes: bytes, timestamp: Date.now() });
    if (this.samples.length > MAX_SAMPLES) {
      this.samples.shift();
    }
  }

  getRate(): number {
    if (this.samples.length < 2) {
      return 0;
    }

    let totalRate = 0;
    let count = 0;

    for (let i = 1; i < this.samples.length; i++) {
      const prev = this.samples[i - 1];
      const curr = this.samples[i];
      const dtMs = curr.timestamp - prev.timestamp;
      if (dtMs <= 0) {
        continue;
      }
      totalRate += Math.abs(curr.bytes - prev.bytes) / (dtMs / 1000);
      count++;
    }

    return count > 0 ? totalRate / count : 0;
  }

  getEta(remainingBytes: number): number {
    const rate = this.getRate();
    if (rate <= 0) {
      return Infinity;
    }
    return remainingBytes / rate;
  }

  clear(): void {
    this.samples = [];
  }
}

export { TransferRateCalculator };
