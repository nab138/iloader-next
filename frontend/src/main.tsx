import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { getClient } from "./lib/client.ts";
import { Toaster } from "@/components/ui/sonner"

export const client = await getClient();
await client.init();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <>
      <App />
      <Toaster expand />
    </>
  </StrictMode>,
);
