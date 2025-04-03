import { useState } from "react";

import { Schemas } from "@sauropod-io/client";

import api from "@/api";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";

interface ModelSelectProps {
  value?: string;
  onValueChange?: (value: string) => void;
}

function ModelSelect({ value, onValueChange }: ModelSelectProps) {
  const { data, isLoading } = api.useQuery("get", "/api/models");
  if (isLoading) return <Skeleton className="w-[180px] h-9" />;
  if (!data) return <p>No models available</p>;

  return (
    <Select value={value} onValueChange={onValueChange}>
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder="Model" />
      </SelectTrigger>
      <SelectContent>
        {data?.map((model) => (
          <SelectItem value={model.name}>{model.name}</SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

export default function Settings() {
  const [selectedModels, setSelectedModels] = useState<{
    [strength: Schemas["ModelStrength"]]: string;
  }>({
    Strong: "",
    Weak: "",
    Vision: "",
  });

  const handleSave = () => {
    console.log("TODO");
  };

  return (
    <main className="container mx-auto p-6 max-w-lg">
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>Models</CardTitle>
          <CardDescription>
            Select which LLM to use for running tasks
          </CardDescription>
        </CardHeader>
        <CardContent>
          {Object.entries(selectedModels).map(([key, value]) => (
            <div className="space-x-1.5">
              <Label className="font-bold py-2" htmlFor={key}>
                {key}
              </Label>
              <p className="text-muted-foreground text-sm"></p>
              <ModelSelect
                value={value}
                onValueChange={(value) => {
                  setSelectedModels((prev) => ({
                    ...prev,
                    [key]: value,
                  }));
                }}
              />
            </div>
          ))}
        </CardContent>
      </Card>

      <Card>
        <CardFooter>
          <Button onClick={handleSave}>Save Settings</Button>
        </CardFooter>
      </Card>
    </main>
  );
}
