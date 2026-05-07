export type DeviceInfo = {
  udid: string;
  name: string;
  connection_type: "USB" | "Network" | "Unknown" | "WebUSB";
  version: string;
}

export type GetDevicesResponse = {
  devices: DeviceInfo[];
  selected: number;
}

export interface iloaderAPI {
  init(): Promise<void>;
  openUrl(url: string): Promise<void>;
  getDevices(): Promise<GetDevicesResponse>;
  readLockdown(): Promise<string>;
  login(email: string, password: string): Promise<void>;
}

let clientInstance: iloaderAPI | null = null;

export async function getClient(): Promise<iloaderAPI> {
  if (clientInstance) return clientInstance;

  if ("__TAURI_INTERNALS__" in window) {
    const { tauriClient } = await import("./tauriClient");
    clientInstance = tauriClient;
  } else {
    const { wasmClient } = await import("./wasmClient");
    clientInstance = wasmClient;
  }

  return clientInstance;
}
