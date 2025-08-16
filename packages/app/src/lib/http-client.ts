import type {
  Category,
  CategoryNoId,
  Stats,
  Transaction,
  TransactionEdit,
} from "./types";

export type HttpClient = {
  fetchStats({
    fromTimestamp,
    toTimestamp,
  }: {
    fromTimestamp?: number;
    toTimestamp?: number;
  }): Promise<{
    data: Stats;
  }>;
  fetchTransactions({
    page,
    limit,
    fromTimestamp,
    toTimestamp,
  }: {
    fromTimestamp?: number;
    toTimestamp?: number;
    page: number;
    limit: number;
  }): Promise<{
    data: Transaction[];
    total: number;
  }>;
  createTransaction({
    transaction,
    transactionType,
  }: {
    transaction: Transaction & { dateString: string };
    transactionType: "income" | "expense";
  }): Promise<void>;
  updateTransaction({
    transaction,
  }: {
    transaction: TransactionEdit;
  }): Promise<void>;
  fetchCategories(): Promise<{
    data: Category[];
  }>;
  createCategory({ name }: CategoryNoId): Promise<void>;
  updateCategory({ id, name }: Category): Promise<void>;
  deleteCategory({ id }: { id: string }): Promise<void>;
};

export function createHttpClient(baseUrl: string): HttpClient {
  return {
    async fetchStats({ fromTimestamp, toTimestamp }) {
      const url = new URL("/api/v1/stats", baseUrl);
      if (fromTimestamp) {
        url.searchParams.set("from-timestamp", fromTimestamp.toFixed());
      }
      if (toTimestamp) {
        url.searchParams.set("to-timestamp", toTimestamp.toFixed());
      }
      const resp = await fetch(url);
      const respJson = (await resp.json()) as { data: Stats };
      return respJson;
    },
    async fetchTransactions({ fromTimestamp, toTimestamp, page, limit }) {
      const url = new URL("/api/v1/transactions", baseUrl);
      if (fromTimestamp) {
        url.searchParams.set("from-timestamp", fromTimestamp.toFixed());
      }
      if (toTimestamp) {
        url.searchParams.set("to-timestamp", toTimestamp.toFixed());
      }
      url.searchParams.set("page", page.toString());
      url.searchParams.set("limit", limit.toString());
      const resp = await fetch(url);
      const respJson = (await resp.json()) as {
        data: Transaction[];
        total: number;
      };
      return respJson;
    },
    async createTransaction({ transaction, transactionType }) {
      const url = new URL("/api/v1/transactions", baseUrl);
      if (transactionType === "expense") {
        transaction.amount = -transaction.amount;
      }
      transaction.dateTimestamp = Math.round(
        new Date(transaction.dateString).getTime() / 1_000,
      );
      const payload = JSON.stringify(transaction);

      const resp = await fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: payload,
      });
      if (!resp.ok) {
        throw new Error(
          `Error happened creating transaction; status: ${resp.status}`,
        );
      }
    },
    async updateTransaction({ transaction }) {
      const url = new URL("/api/v1/transactions", baseUrl);
      if (transaction.type === "expense") {
        transaction.amount = -transaction.amount;
      }
      transaction.dateTimestamp = Math.round(
        new Date(transaction.dateString).getTime() / 1_000,
      );
      const payload = JSON.stringify(transaction);

      const resp = await fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },
        body: payload,
      });
      if (!resp.ok) {
        throw new Error(
          `Error happened updating transaction; status: ${resp.status}`,
        );
      }
    },

    async fetchCategories() {
      const url = new URL("/api/v1/categories", baseUrl);
      const resp = await fetch(url);
      const respJson = (await resp.json()) as { data: Category[] };
      return respJson;
    },
    async createCategory() {
      throw new Error("unimplemented");
    },
    async updateCategory() {
      throw new Error("unimplemented");
    },
    async deleteCategory() {
      throw new Error("unimplemented");
    },
  };
}
