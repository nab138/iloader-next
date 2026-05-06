import { invoke } from "@tauri-apps/api/core";
import type { iloaderAPI } from "./client";

export const tauriClient: iloaderAPI = {
  async init() {},

  async connectIdevice(): Promise<void> {
    return invoke("connect_idevice");
  },

  async readLockdown(): Promise<string> {
    return invoke("read_lockdown");
  },
};
