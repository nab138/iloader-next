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
import Devices from "./parts/Devices";
import { useState } from "react";

function App() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  return (
    <>
      <Header />
      <main className="flex gap-5 m-2 mt-3 flex-wrap">
        <Card className="w-full md:w-[300px] lg:w-[500px]">
          <CardHeader>
            <CardTitle>Apple ID</CardTitle>
            <CardDescription>
              Login to your Apple account. Your credentials will only be sent to
              Apple.
            </CardDescription>
          </CardHeader>
          <form
            onSubmit={(e) => {
              e.preventDefault();
              toast.promise(
                client.login(email, password, () => Promise.resolve("123456")),
                {
                  loading: "Logging in...",
                  success: "Logged in successfully!",
                  error: (e) => e,
                },
              );
            }}
          >
            <CardContent>
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
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                      />
                    </Field>
                    <Field className="mb-5">
                      <FieldLabel htmlFor="account-password">
                        Password
                      </FieldLabel>
                      <Input
                        id="account-password"
                        placeholder="Apple ID password..."
                        type="password"
                        required
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                      />
                    </Field>
                  </FieldGroup>
                </FieldSet>
              </FieldGroup>
            </CardContent>
            <CardFooter>
              <Field>
                <Button type="submit">Login</Button>
              </Field>
            </CardFooter>
          </form>
        </Card>

        <Devices />
      </main>
    </>
  );
}

export default App;
