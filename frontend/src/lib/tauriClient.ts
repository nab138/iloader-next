import { invoke } from "@tauri-apps/api/core";
import type { DeviceInfo, iloaderAPI } from "./client";
import { openUrl } from "@tauri-apps/plugin-opener";
import type { AppError } from "./error";
import { toast } from "sonner";

export const tauriClient: iloaderAPI = {
  async init() {},

  async getDevices(): Promise<DeviceInfo[]> {
    const devices =
      await invoke<Array<{ Ok: DeviceInfo } | { Err: AppError }>>(
        "get_devices",
      );

    const validDevices: DeviceInfo[] = [];
    devices.forEach((result, index) => {
      if ("Ok" in result) {
        validDevices.push(result.Ok);
      } else if ("Err" in result) {
        toast.error(`Error loading device ${index}: ` + result.Err);
      }
    });

    return validDevices;
  },

  async readLockdown(): Promise<string> {
    return invoke("read_lockdown");
  },

  async openUrl(url: string): Promise<void> {
    return openUrl(url);
  },

  async login(email: string, password: string): Promise<void> {
    return invoke("login", { email, password });
  },
};
