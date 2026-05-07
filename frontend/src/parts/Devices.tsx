import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import type { DeviceInfo } from "@/lib/client";
import { client } from "@/main";
import { CheckIcon } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";

function Devices() {
    const [devices, setDevices] = useState<DeviceInfo[]>([]);
    const [selectedDevice, setSelectedDevice] = useState<DeviceInfo | null>(null);

    return <Card>
        <CardHeader>
            <CardTitle>iDevice</CardTitle>
            <CardDescription>Choose a device</CardDescription>
        </CardHeader>
        <CardContent>
            <div className="flex flex-col gap-2">
                {devices.length === 0 && <div className="text-base">No devices found</div>}
                {devices.map((device) => (
                    <Button
                        key={device.udid}
                        variant={device.udid === selectedDevice?.udid ? "default" : "outline"}
                        className="h-auto w-full justify-start p-3"
                        onClick={() => setSelectedDevice(device)}
                    >
                        {device.udid === selectedDevice?.udid && <CheckIcon className="mr-3 h-5 w-5" />}
                        <div className="flex flex-col items-start">
                            <div className="text-base">{device.name}</div>
                            <div className="text-xs font-normal opacity-70">{device.udid}</div>
                        </div>

                        <div className="ml-auto flex flex-col items-end text-sm font-normal opacity-70">
                            <div>{device.version}</div>
                            <div>{device.connection_type}</div>
                        </div>
                    </Button>
                ))}
            </div>
        </CardContent>
        <CardFooter>
            <Button onClick={async () => {
                try {
                    const response = await client.getDevices();
                    setDevices(response.devices);
                    let len = response.devices.length;
                    if (len === 0) {
                        toast.warning("No devices found");
                    } else {
                        toast.success("Found " + len + " device" + (len > 1 ? "s" : ""));
                    }
                } catch (e) {
                    toast.error("Failed to get devices: " + e);
                }
            }}>Refresh</Button>
        </CardFooter>
    </Card>
}
export default Devices;