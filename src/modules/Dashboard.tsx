import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { SidebarTrigger } from "@/components/ui/sidebar";
import {
  ArrowDown,
  ArrowUp,
  DollarSign,
  TrendingDown,
  TrendingUp,
} from "lucide-react";
import { AddExpenseModal } from "./AddExpense.modal";
import { useState } from "react";

export default function Dashboard() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const handleOpenModal = () => {
    setIsModalOpen(true);
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
              <BreadcrumbPage>Dashboard</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>

        <Button className="ml-auto" onClick={handleOpenModal}>
          Add Expense
        </Button>
      </header>

      <div className="flex flex-1 flex-col gap-4 p-4">
        {/* Chart Section */}
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          <div className="aspect-video rounded-xl bg-gray-200 flex items-center justify-center">
            <span className="text-gray-500 font-medium">Revenue Chart</span>
          </div>
          <div className="aspect-video rounded-xl bg-gray-200 flex items-center justify-center">
            <span className="text-gray-500 font-medium">Expenses Chart</span>
          </div>
          <div className="aspect-video rounded-xl bg-gray-200 flex items-center justify-center md:col-span-2 lg:col-span-1">
            <span className="text-gray-500 font-medium">Savings Chart</span>
          </div>
        </div>

        {/* Budget Data Section */}
        <div className="space-y-4">
          <h2 className="text-2xl font-bold tracking-tight">Budget Overview</h2>

          {/* Budget Cards */}
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Total Budget
                </CardTitle>
                <DollarSign className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">$5,420.00</div>
                <p className="text-xs text-muted-foreground">
                  +12% from last month
                </p>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Spent This Month
                </CardTitle>
                <TrendingUp className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">$3,240.00</div>
                <p className="text-xs text-muted-foreground">
                  +8% from last month
                </p>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Remaining Budget
                </CardTitle>
                <TrendingDown className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">$2,180.00</div>
                <p className="text-xs text-muted-foreground">
                  40% of total budget
                </p>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Savings Goal
                </CardTitle>
                <ArrowUp className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">$1,850.00</div>
                <p className="text-xs text-muted-foreground">
                  74% of $2,500 goal
                </p>
              </CardContent>
            </Card>
          </div>

          {/* Budget Categories */}
          <div className="grid gap-4 md:grid-cols-2">
            <Card>
              <CardHeader>
                <CardTitle>Budget Categories</CardTitle>
                <CardDescription>
                  Your spending breakdown by category
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <div className="w-3 h-3 bg-blue-500 rounded-full"></div>
                    <span className="text-sm">Housing</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-medium">$1,200.00</div>
                    <div className="text-xs text-muted-foreground">
                      37% of budget
                    </div>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                    <span className="text-sm">Food & Dining</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-medium">$680.00</div>
                    <div className="text-xs text-muted-foreground">
                      21% of budget
                    </div>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
                    <span className="text-sm">Transportation</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-medium">$420.00</div>
                    <div className="text-xs text-muted-foreground">
                      13% of budget
                    </div>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <div className="w-3 h-3 bg-purple-500 rounded-full"></div>
                    <span className="text-sm">Entertainment</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-medium">$320.00</div>
                    <div className="text-xs text-muted-foreground">
                      10% of budget
                    </div>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <div className="w-3 h-3 bg-red-500 rounded-full"></div>
                    <span className="text-sm">Other</span>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-medium">$620.00</div>
                    <div className="text-xs text-muted-foreground">
                      19% of budget
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>Recent Transactions</CardTitle>
                <CardDescription>Your latest spending activity</CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-sm font-medium">Grocery Store</div>
                    <div className="text-xs text-muted-foreground">
                      Food & Dining • Today
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <ArrowDown className="h-4 w-4 text-red-500" />
                    <span className="text-sm font-medium">$87.50</span>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-sm font-medium">Gas Station</div>
                    <div className="text-xs text-muted-foreground">
                      Transportation • Yesterday
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <ArrowDown className="h-4 w-4 text-red-500" />
                    <span className="text-sm font-medium">$45.20</span>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-sm font-medium">Salary Deposit</div>
                    <div className="text-xs text-muted-foreground">
                      Income • 2 days ago
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <ArrowUp className="h-4 w-4 text-green-500" />
                    <span className="text-sm font-medium">$2,500.00</span>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-sm font-medium">
                      Netflix Subscription
                    </div>
                    <div className="text-xs text-muted-foreground">
                      Entertainment • 3 days ago
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <ArrowDown className="h-4 w-4 text-red-500" />
                    <span className="text-sm font-medium">$15.99</span>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-sm font-medium">Coffee Shop</div>
                    <div className="text-xs text-muted-foreground">
                      Food & Dining • 4 days ago
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <ArrowDown className="h-4 w-4 text-red-500" />
                    <span className="text-sm font-medium">$12.75</span>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>

      <AddExpenseModal
        isOpen={isModalOpen}
        hideModal={() => setIsModalOpen(false)}
      />
    </>
  );
}
