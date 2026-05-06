import { useState } from "react";
import "./App.css";
import { client } from "./main";
import logo from "./assets/iloader.svg";
import { Button } from "@/components/ui/button";

function App() {
  const [connected, setConnected] = useState<boolean>(false);
  const [failed, setFailed] = useState<string>("");
  const [lockdown, setLockdown] = useState<string>("");
  return (
    <>
      <header>
        <div className="title-block">
          <img src={logo} alt="iloader" />
          <div>
            <h1>iloader</h1>
            <span className="subtitle">Sideloading Companion</span>
          </div>
        </div>
      </header>
      <main>
        <Button variant="outline" onClick={() => {
          client.connectIdevice().then(() => {
            setConnected(true);
            setFailed("");
          }).catch((err) => {
            setFailed(err);
          });
        }}>Connect</Button>
        <Button variant="outline" onClick={() => {
          client.readLockdown().then((l) => {
            setLockdown(l);
          }).catch((err) => {
            setFailed(err);
          });
        }}>Read Lockdown</Button>
        <div>
          {connected ? "Connected to idevice" : "Not connected"}
        </div>
        <div style={{ color: "red" }}>
          {failed ? `Error: ${failed}` : ""}
        </div>
        <pre style={{ backgroundColor: "-moz-initial" }}>
          {lockdown ? `${lockdown}` : "No lockdown data"}
        </pre>
      </main>
    </>
  );
}

export default App;
