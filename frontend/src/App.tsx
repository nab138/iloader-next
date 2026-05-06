import { useState } from "react";
import "./App.css";
import { client } from "./main";
import { Button } from "@/components/ui/button";
import Header from "./parts/Header";
import { toast } from "sonner"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "./components/ui/card";

function App() {
  const [connected, setConnected] = useState<boolean>(false);
  const [lockdown, setLockdown] = useState<string>("");
  return (
    <>
      <Header />
      <main>
        <Card>
          <CardHeader>
            <CardTitle>iDevice</CardTitle>
            <CardDescription>Manage your iDevice</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="mb-2">
              {connected ? "Connected to idevice" : "Not connected"}
            </div>
            <div className="flex gap-2">
              <Button onClick={() => {
                client.connectIdevice().then(() => {
                  setConnected(true);
                  toast.success("Connected to idevice");
                }).catch((err) => {
                  toast.error(err);
                });
              }}>Connect</Button>
              <Button variant="outline" onClick={() => {
                client.readLockdown().then((l) => {
                  setLockdown(l);
                  toast.success("Lockdown data read successfully");
                }).catch((err) => {
                  toast.error(err);
                });
              }}>Read Lockdown</Button>
            </div>
          </CardContent>
          <CardFooter>
            <pre style={{ backgroundColor: "-moz-initial" }}>
              {lockdown ? `${lockdown}` : "No lockdown data"}
            </pre>
          </CardFooter>
        </Card>
      </main>
    </>
  );
}

export default App;
