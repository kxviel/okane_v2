import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { invoke } from "@tauri-apps/api/core";

type Props = {
  isOpen: boolean;
  refetch: () => Promise<void>;
  hideModal: () => void;
};

export function AddCategoryModal({ isOpen, refetch, hideModal }: Props) {
  const [form, setForm] = useState({
    category_name: "",
    category_desc: "",
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log(form);
    await invoke("add_category", {
      category: { ...form },
    })
      .then(refetch)
      .finally(hideModal);
  };

  return (
    <Dialog open={isOpen} onOpenChange={hideModal}>
      <form>
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>Add Category</DialogTitle>
            <DialogDescription>
              Fill in the details of your category and save.
            </DialogDescription>
          </DialogHeader>
          <div className="">
            <div className="">
              <Label htmlFor="category_name">Category Title</Label>
              <Input
                id="category_name"
                name="category_name"
                value={form.category_name}
                onChange={handleChange}
              />
            </div>
            <div className="">
              <Label htmlFor="category_desc">Category Desc</Label>
              <Input
                id="category_desc"
                name="category_desc"
                value={form.category_desc}
                onChange={handleChange}
              />
            </div>
          </div>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline" type="button">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit" onClick={handleSubmit}>
              Save Category
            </Button>
          </DialogFooter>
        </DialogContent>
      </form>
    </Dialog>
  );
}
