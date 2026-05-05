import { invoke } from "@tauri-apps/api/core";
import type { iloaderAPI } from "./client";

export const tauriClient: iloaderAPI = {
  async init() {},

  async transform(input: string): Promise<string> {
    return invoke("transform", { input });
  },
};
