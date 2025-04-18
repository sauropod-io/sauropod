import { useEffect, useState } from "react";

import RunButton from "@/components/buttons/RunButton";
import SaveButton from "@/components/buttons/SaveButton";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Input } from "@/components/ui/input";
import { SidebarTrigger } from "@/components/ui/sidebar";

import DeleteButton from "./buttons/DeleteButton";

interface EditorHeaderProps {
  name: string;
  onNameChange: (name: string) => void;
  children?: React.ReactNode;
  onRun: () => void;
  onSave: () => void;
  onDelete: () => void;
  disabled: boolean;
}

/** Header used for editor interfaces. */
export function EditorHeader({
  name,
  onRun,
  onSave,
  onNameChange,
  children,
  onDelete,
  disabled,
}: EditorHeaderProps) {
  const [isDeleteAlertOpen, setIsDeleteAlertOpen] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);

  useEffect(() => {
    let timer: NodeJS.Timeout;
    if (saveSuccess) {
      timer = setTimeout(() => {
        setSaveSuccess(false);
      }, 750);
    }
    return () => {
      if (timer) clearTimeout(timer);
    };
  }, [saveSuccess]);

  const handleSave = () => {
    onSave();
    setSaveSuccess(true);
  };

  return (
    <div
      className="flex flex-row items-center justify-between space-y-0 px-4 py-2 shadow-sm"
      style={{ boxShadow: "rgba(0, 0, 0, 0.75) 0px 3px 6px -7px" }}
    >
      <SidebarTrigger className="mr-2" />
      <Input
        value={name}
        onChange={(e) => onNameChange(e.target.value)}
        placeholder="Name"
        className="bg-background text-xl mr-2"
      />
      <div className="flex space-x-2">
        <RunButton onClick={onRun} disabled={disabled} />
        <SaveButton onClick={handleSave} showSuccess={saveSuccess} />

        <AlertDialog
          open={isDeleteAlertOpen}
          onOpenChange={setIsDeleteAlertOpen}
        >
          <AlertDialogTrigger asChild>
            <DeleteButton
              disabled={disabled}
              onClick={() => setIsDeleteAlertOpen(true)}
            />
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Are you sure?</AlertDialogTitle>
              <AlertDialogDescription>
                Are you sure you want to delete{name ? ` ${name}` : ""}?
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction onClick={onDelete}>Delete</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
        {children}
      </div>
    </div>
  );
}
