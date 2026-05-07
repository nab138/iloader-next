import { useState } from "react";
import "./App.css";
import { client } from "./main";
import { Button } from "@/components/ui/button";
import Header from "./parts/Header";
import { toast } from "sonner";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { Input } from "./components/ui/input";
import { Field, FieldGroup, FieldLabel, FieldSet } from "./components/ui/field";

function App() {
  const [connected, setConnected] = useState<boolean>(false);
  const [lockdown, setLockdown] = useState<string>("");
  return (
    <>
      <Header />
      <main className="flex gap-5 flex-col m-2 mt-3">
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
              <Button
                onClick={() => {
                  client
                    .connectIdevice()
                    .then(() => {
                      setConnected(true);
                      toast.success("Connected to idevice");
                    })
                    .catch((err) => {
                      setConnected(false);
                      toast.error(err);
                    });
                }}
              >
                Connect
              </Button>
              <Button
                variant="outline"
                onClick={() => {
                  client
                    .readLockdown()
                    .then((l) => {
                      setLockdown(l);
                      toast.success("Lockdown data read successfully");
                    })
                    .catch((err) => {
                      toast.error(err);
                    });
                }}
              >
                Read Lockdown
              </Button>
            </div>
          </CardContent>
          <CardFooter>
            <pre style={{ backgroundColor: "-moz-initial" }}>
              {lockdown ? `${lockdown}` : "No lockdown data"}
            </pre>
          </CardFooter>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Apple ID</CardTitle>
            <CardDescription>
              Login to your Apple account. Your credentials will only be sent to
              Apple.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form
              onSubmit={(e) => {
                e.preventDefault();
                const formData = new FormData(e.currentTarget);
                const email = formData.get("account-email") as string;
                const password = formData.get("account-password") as string;
                toast.promise(client.login(email, password), {
                  loading: "Logging in...",
                  success: "Logged in successfully!",
                  error: (e) => e,
                });
              }}
            >
              <FieldGroup>
                <FieldSet>
                  <FieldGroup className="gap-4">
                    <Field>
                      <FieldLabel htmlFor="account-email">
                        Apple ID Email
                      </FieldLabel>
                      <Input
                        id="account-email"
                        placeholder="example@icloud.com"
                        type="email"
                        required
                      />
                    </Field>
                    <Field>
                      <FieldLabel htmlFor="account-password">
                        Password
                      </FieldLabel>
                      <Input
                        id="account-password"
                        placeholder="Apple ID password..."
                        type="password"
                        required
                      />
                    </Field>
                  </FieldGroup>
                </FieldSet>

                <Field orientation="horizontal">
                  <Button type="submit">Login</Button>
                </Field>
              </FieldGroup>
            </form>
          </CardContent>
        </Card>
      </main>
    </>
  );
}

export default App;
