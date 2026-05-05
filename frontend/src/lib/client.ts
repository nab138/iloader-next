export interface iloaderAPI {
  init(): Promise<void>;
  transform(input: string): Promise<string>;
}
let clientInstance: iloaderAPI | null = null;

export async function getClient(): Promise<iloaderAPI> {
  if (clientInstance) return clientInstance;

  if ("__TAURI__" in window) {
    const { tauriClient } = await import("./tauriClient");
    clientInstance = tauriClient;
  } else {
    const { wasmClient } = await import("./wasmClient");
    clientInstance = wasmClient;
  }

  return clientInstance;
}
