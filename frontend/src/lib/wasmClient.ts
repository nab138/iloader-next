import initWasm, { read_lockdown, connect_idevice, login } from "iloader-wasm";
import type { DeviceInfo, iloaderAPI } from "./client";

export const wasmClient: iloaderAPI = {
  async init() {
    await initWasm();
  },

  async getDevices(): Promise<DeviceInfo[]> {
    throw new Error("Not implemented");
  },

  async readLockdown(): Promise<string> {
    return read_lockdown();
  },

  async openUrl(url: string): Promise<void> {
    window.open(url, "_blank");
  },

  async login(email: string, password: string): Promise<void> {
    return login(email, password);
  },
};
