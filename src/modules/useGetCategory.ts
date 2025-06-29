import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type Category = {
  id: number;
  category_name: string;
  category_desc: string;
  badge_color: string;
  created_at: string;
  updated_at: string;
};

type Props = {
  page: number;
  search: string;
};

export const useGetCategory = ({ page, search }: Props) => {
  const [isLoading, setIsLoading] = useState(false);
  const [categories, setCategories] = useState<Category[]>([]);

  const fetchCategories = async () => {
    setIsLoading(true);

    try {
      const data = await invoke("get_category", { params: { page, search } });
      console.log(data);
      setCategories(data as Category[]);
    } catch (error) {
      console.log(error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchCategories();
  }, [page, search]);

  return {
    isLoading,
    categories,
    refetch: fetchCategories,
  };
};
