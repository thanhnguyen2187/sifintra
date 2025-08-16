export type Stats = {
  totalIncomeVND: number;
  totalExpenseVND: number;
  currentBalanceVND: number;
  chartData: { label: string; value: number }[];
};

export type Transaction = {
  date_timestamp: number;
  description: string;
  amount: number;
  categoryId: string;
};

export type Category = {
  id: string;
  name: string;
};

export type CategoryNoId = Exclude<Category, "id">;
