import initWasm, { read_lockdown, connect_idevice } from "iloader-wasm";
import type { iloaderAPI } from "./client";

export const wasmClient: iloaderAPI = {
  async init() {
    await initWasm();
  },

  async connectIdevice(): Promise<void> {
    return connect_idevice();
  },

  async readLockdown(): Promise<string> {
    return read_lockdown();
  },
};
