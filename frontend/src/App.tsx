import { client } from "./main";
import { Button } from "@/components/ui/button";
import Header from "./parts/Header";
import { toast } from "sonner";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { Input } from "./components/ui/input";
import { Field, FieldGroup, FieldLabel, FieldSet } from "./components/ui/field";
import Devices from "./parts/Devices";

function App() {
  return (
    <>
      <Header />
      <main className="flex gap-5 flex-col m-2 mt-3">
        <Devices />
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
