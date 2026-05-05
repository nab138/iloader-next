import { defineConfig } from "vite";
import path from "path";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  plugins: [
    {
      name: "watch-wasm",
      enforce: "post",
      handleHotUpdate({ file, server }) {
        if (file.endsWith(".wasm") || file.includes("/wasm/")) {
          server.ws.send({ type: "full-reload", path: "*" });
        }
      },
    },
  ],
  resolve: {
    alias: {
      "iloader-wasm": path.resolve(__dirname, "./wasm"),
    },
  },
  server: {
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,

    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    target:
      process.env.TAURI_ENV_PLATFORM == "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
  optimizeDeps: {
    exclude: ["./wasm/iloader.js"],
  },
});
