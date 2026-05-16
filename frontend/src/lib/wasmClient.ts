import initWasm, { read_lockdown, get_devices, login } from "iloader-wasm";
import type { DeviceInfo, iloaderAPI } from "./client";

export const wasmClient: iloaderAPI = {
  async init() {
    await initWasm();
  },

  async getDevices(): Promise<DeviceInfo[]> {
    return get_devices();
  },

  async readLockdown(): Promise<string> {
    return read_lockdown();
  },

  async openUrl(url: string): Promise<void> {
    window.open(url, "_blank");
  },

  async login(
    email: string,
    password: string,
    get2FA: () => Promise<string>,
  ): Promise<void> {
    try {
      await login(email, password, get2FA);
    } catch (error) {
      console.error("Login failed:", error);
      throw error;
    }
  },
};
