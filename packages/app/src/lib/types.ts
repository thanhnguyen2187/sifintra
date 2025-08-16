import { format } from "date-fns";

export type Stats = {
  totalIncomeVND: number;
  totalExpenseVND: number;
  currentBalanceVND: number;
  chartData: { label: string; value: number }[];
};

export type Transaction = {
  id: string | null;
  dateTimestamp: number;
  description: string;
  amount: number;
  categoryId: string | null;
};

export type TransactionEdit = Transaction & {
  dateString: string;
  type: "income" | "expense";
};

export type Category = {
  id: string;
  name: string;
};

export type CategoryNoId = Exclude<Category, "id">;

export function createTransactionEmpty(): TransactionEdit {
  return {
    id: null,
    dateTimestamp: 0,
    description: "",
    amount: 10000,
    categoryId: null,
    dateString: "",
    type: "expense",
  };
}

export function createTransactionEdit(
  transaction: Transaction,
): TransactionEdit {
  return {
    ...transaction,
    dateString: format(
      new Date(transaction.dateTimestamp * 1_000),
      "yyyy-MM-dd'T'HH:mm",
    ),
    type: transaction.amount > 0 ? "income" : "expense",
    amount: Math.abs(transaction.amount),
  };
}
