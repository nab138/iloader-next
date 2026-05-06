# iloader-next

A combined web and desktop app for sideloading iOS apps. The next generation of iloader.

## Architecture

To make maintainability easier (though it may actually make it harder...) this repo is a single codebase that serves both the web and desktop versions of the app.

Layout:

```
- frontend
  - src # React frontend code
  - src-tauri # Tauri desktop app code, exposes tauri commands for iloader-core
- rust
  - iloader-core # Core rust code powering both apps
  - server # WIP proxy server for the rust app
  - wasm # WASM bindings for iloader-core
```

### Development

For your convenience, a `dev` script is available to handle compiling the wasm, running the Tauri desktop app, and starting the web server. It will watch for changes and automatically rebuild as needed. It's very janky though
```
bun dev
```
