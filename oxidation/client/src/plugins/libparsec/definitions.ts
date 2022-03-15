export interface LibParsecPlugin {
  submitJob(options: {cmd: string, payload: string}): Promise<{ value: string }>;
}
