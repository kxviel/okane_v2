import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

export const useDeleteCategory = (refetch: () => Promise<void>) => {
  const [isLoading, setIsLoading] = useState(false);

  const deleteCategory = async (id: number) => {
    setIsLoading(true);

    try {
      await invoke("delete_category", { id });
    } catch (error) {
      console.log(error);
    } finally {
      refetch();
      setIsLoading(false);
    }
  };

  return {
    isLoading,
    deleteCategory,
  };
};
