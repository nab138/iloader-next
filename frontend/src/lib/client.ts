export interface iloaderAPI {
  init(): Promise<void>;
  openUrl(url: string): Promise<void>;
  connectIdevice(): Promise<void>;
  readLockdown(): Promise<string>;
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
