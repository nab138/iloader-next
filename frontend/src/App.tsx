import { useState } from "react";
import "./App.css";
import { client } from "./main";

function App() {
  const [connected, setConnected] = useState<boolean>(false);
  const [failed, setFailed] = useState<string>("");
  const [lockdown, setLockdown] = useState<string>("");
  return (
    <>
      <div>iloader web!</div>
      <button
        onClick={() => {
          client
            .connectIdevice()
            .then(() => {
              setConnected(true);
              setFailed("");
            })
            .catch((e) => {
              console.log(e);
              setConnected(false);
              setFailed(e);
            });
        }}
      >
        Connect iDevice
      </button>
      <button
        onClick={() => {
          if (!connected) {
            alert("Not connected");
            return;
          }
          client
            .readLockdown()
            .then((result) => {
              setLockdown(result);
            })
            .catch((e) => {
              setFailed(e.message);
            });
        }}
      >
        Read lockdown
      </button>
      <p>{connected ? "Connected" : "Not connected"}</p>
      <pre>
        {failed != "" && <span style={{ color: "red" }}>{failed}</span>}
        {lockdown && <span>{lockdown}</span>}
      </pre>
    </>
  );
}

export default App;
