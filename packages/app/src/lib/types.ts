import { format, parse, parseISO } from "date-fns";
import { formatDateDisplay } from "$lib/date";

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
  createdAt: string;
  updatedAt: string;
};

export type CategoryEdit = { id: string | null; name: string };

export type CategoryDisplay = Category & {
  createdAtCorrected: string;
  updatedAtCorrected: string;
};

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

export function createCategoryEmpty(): CategoryEdit {
  return {
    id: null,
    name: "",
  };
}

export function createCategoryDisplay(category: Category): CategoryDisplay {
  const createdAtParsed = parse(
    `${category.createdAt}Z`,
    "yyyy-MM-dd HH:mm:ssX",
    new Date(),
  );
  const createdAtCorrected = formatDateDisplay(
    createdAtParsed.getTime() / 1_000,
  );
  const updatedAtParsed = parse(
    `${category.updatedAt}Z`,
    "yyyy-MM-dd HH:mm:ssX",
    new Date(),
  );
  const updatedAtCorrected = formatDateDisplay(
    updatedAtParsed.getTime() / 1_000,
  );
  return {
    ...category,
    createdAtCorrected,
    updatedAtCorrected,
  };
}
