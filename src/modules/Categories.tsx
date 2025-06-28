import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { SidebarTrigger } from "@/components/ui/sidebar";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useState } from "react";
import { AddCategoryModal } from "./AddCategory.modal";
import { Input } from "@/components/ui/input";
import { useGetCategory } from "./useGetCategory";
import { EllipsisIcon, Loader2 } from "lucide-react";
import { useDeleteCategory } from "./useDeleteCategory";

export default function Categories() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const [page] = useState(1);
  const [search, setSearch] = useState("");

  const { isLoading, categories, refetch } = useGetCategory({ page, search });
  const { deleteCategory } = useDeleteCategory(refetch);

  const handleOpenModal = () => {
    setIsModalOpen(true);
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearch(e.target.value);
  };

  const handleEdit = () => {};

  const handleDelete = (id: number) => {
    deleteCategory(id);
  };

  return (
    <>
      <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
        <SidebarTrigger className="-ml-1" />
        <Separator orientation="vertical" className="mr-2 h-4" />
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem className="hidden md:block">
              <BreadcrumbLink href="#">Finance</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator className="hidden md:block" />
            <BreadcrumbItem>
              <BreadcrumbPage>Categories</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>

        <Input
          id="title"
          name="title"
          value={search}
          onChange={handleChange}
          className="ml-auto w-64"
          placeholder="Search categories..."
        />

        <Button onClick={handleOpenModal}>Add Category</Button>
      </header>

      <div className="p-4">
        {isLoading ? (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-6 w-6 animate-spin" />
            <span className="ml-2 text-sm text-muted-foreground">
              Loading categories...
            </span>
          </div>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>#</TableHead>
                <TableHead>Category</TableHead>
                <TableHead>Description</TableHead>
                <TableHead>Created At</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {categories.map((category, index) => (
                <TableRow key={category.id}>
                  <TableCell className="font-medium">{index + 1}</TableCell>
                  <TableCell>{category.category_name}</TableCell>
                  <TableCell>{category.category_desc}</TableCell>
                  <TableCell>{category.created_at}</TableCell>
                  <TableCell className="text-right">
                    <DropdownMenu>
                      <DropdownMenuTrigger asChild>
                        <Button variant="outline">
                          <EllipsisIcon />
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent>
                        <DropdownMenuItem onClick={handleEdit}>
                          Edit
                        </DropdownMenuItem>
                        <DropdownMenuItem
                          className="text-red-300"
                          onClick={() => handleDelete(category.id)}
                        >
                          Delete
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </div>

      {isModalOpen && (
        <AddCategoryModal
          isOpen={isModalOpen}
          refetch={refetch}
          hideModal={() => setIsModalOpen(false)}
        />
      )}
    </>
  );
}
