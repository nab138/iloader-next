import initWasm, { transform } from "iloader-wasm";
import type { iloaderAPI } from "./client";

export const wasmClient: iloaderAPI = {
  async init() {
    await initWasm();
  },

  async transform(input: string): Promise<string> {
    return transform(input);
  },
};
