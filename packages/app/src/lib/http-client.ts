import type { Category, CategoryNoId, Stats, Transaction } from "./types";

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
  fetchTransactions(): Promise<{
    data: Transaction[];
    total: number;
  }>;
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
    async fetchTransactions() {
      throw new Error("unimplemented");
    },
    async fetchCategories() {
      throw new Error("unimplemented");
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
