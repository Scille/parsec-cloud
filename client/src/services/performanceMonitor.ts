// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

interface PerformanceStats {
  callCount: number;
  totalTime: number;
  averageTime: number;
  minTime: number;
  maxTime: number;
}

interface MonitoredFunction {
  name: string;
  start: number;
}

export class PerformanceMonitor {
  private stats: Map<string, Omit<PerformanceStats, 'averageTime'>> = new Map();
  private monitoredFunctions: Array<MonitoredFunction> = [];

  start(name: string): void {
    if (this.monitoredFunctions.find((fn) => fn.name.localeCompare(name) === 0)) {
      console.warn(`Function to start already being monitored: ${name}`);
      return;
    }
    const start = performance.now();
    this.monitoredFunctions.push({ name: name, start: start });
  }

  stop(name: string): void {
    const activeFunction = this.monitoredFunctions.find((fn) => fn.name.localeCompare(name) === 0);
    if (!activeFunction) {
      console.warn(`Function to stop not found in monitored functions: ${name}`);
      return;
    }
    const stop = performance.now();
    this.record(name, stop - activeFunction.start);
    this.monitoredFunctions.splice(this.monitoredFunctions.indexOf(activeFunction), 1);
  }

  private record(name: string, elapsed: number): void {
    const existing = this.stats.get(name);
    if (!existing) {
      this.stats.set(name, {
        callCount: 1,
        totalTime: elapsed,
        minTime: elapsed,
        maxTime: elapsed,
      });
    } else {
      existing.callCount++;
      existing.totalTime += elapsed;
      existing.minTime = Math.min(existing.minTime, elapsed);
      existing.maxTime = Math.max(existing.maxTime, elapsed);
    }
  }

  getStats(name: string): PerformanceStats | null {
    const s = this.stats.get(name);
    if (!s) return null;
    return {
      ...s,
      averageTime: s.totalTime / s.callCount,
    };
  }

  getAllStats(): Record<string, PerformanceStats> {
    const result: Record<string, PerformanceStats> = {};
    for (const [name, s] of this.stats.entries()) {
      result[name] = { ...s, averageTime: s.totalTime / s.callCount };
    }
    return result;
  }

  reset(name?: string): void {
    if (name) {
      this.stats.delete(name);
      const index = this.monitoredFunctions.findIndex((fn) => fn.name.localeCompare(name) === 0);
      this.monitoredFunctions.splice(index, 1);
    } else {
      this.stats.clear();
      this.monitoredFunctions = [];
    }
  }

  printReport(): void {
    const all = this.getAllStats();
    console.table(
      Object.fromEntries(
        Object.entries(all).map(([name, s]) => [
          name,
          {
            Calls: s.callCount,
            'Avg (ms)': Number(s.averageTime.toFixed(0)),
            'Min (ms)': Number(s.minTime.toFixed(0)),
            'Max (ms)': Number(s.maxTime.toFixed(0)),
            'Total (ms)': Number(s.totalTime.toFixed(0)),
          },
        ]),
      ),
    );
  }
}
