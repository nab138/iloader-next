import { Button } from "./components/ui/button";
import { client } from "./main";
import Header from "./parts/Header";

function PlatformGate({ children }: { children: React.ReactNode }) {
  if ("__TAURI_INTERNALS__" in window) {
    return children;
  }
  let windows = navigator.userAgent.includes("Windows");
  let webusb = "usb" in navigator;
  if (!webusb || windows) {
    return (
      <>
        <Header />
        <main>
          <div className="flex flex-col items-center justify-center gap-4 p-4 text-center mt-10 max-w-[70%] mx-auto">
            <h1 className="text-2xl font-bold">
              {windows
                ? "Windows is not supported on web"
                : "WebUSB Not Supported"}
            </h1>
            <p>
              {windows
                ? "Due to limitations in web technologies, Windows is not supported on the web version of iloader."
                : "It seems that your browser does not support WebUSB."}
            </p>
            <p>
              {windows
                ? "Please use the desktop version for Windows."
                : "Your can switch to a compatible browser such as Chrome, Edge, or Opera, or use the desktop version of iloader."}
            </p>
            <Button
              onClick={() =>
                client.openUrl(
                  "https://github.com/nab138/iloader-next/releases",
                )
              }
            >
              Download desktop app
            </Button>
          </div>
        </main>
      </>
    );
  }
  return children;
}

export default PlatformGate;
