import { invoke } from "@tauri-apps/api/core";
import type { iloaderAPI } from "./client";
import { openUrl } from "@tauri-apps/plugin-opener";

export const tauriClient: iloaderAPI = {
  async init() { },

  async connectIdevice(): Promise<void> {
    return invoke("connect_idevice");
  },

  async readLockdown(): Promise<string> {
    return invoke("read_lockdown");
  },

  async openUrl(url: string): Promise<void> {
    return openUrl(url);
  }
};
