import type { Category, CategoryNoId, Stats, Transaction } from "./types";

export type HttpClient = {
  fetchStats(): Promise<{
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
    async fetchStats() {
      const url = new URL("/api/v1/stats", baseUrl);
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
