<script lang="ts">
import { endOfMonth, startOfMonth, startOfWeek, subMonths } from "date-fns";
import { onMount } from "svelte";
import { httpClient } from "$lib/default";
import type { Category, Transaction } from "$lib/types";

type TransactionDisplay = Transaction & {
  type: "expense" | "income";
  date: string;
};

let records: TransactionDisplay[] = $state([]);
let categories: Category[] = $state([]);

onMount(() => {
  (async () => {
    categories = (await httpClient.fetchCategories()).data;
  })();
});

let fromTimestamp: number | undefined = $state(undefined);
let toTimestamp: number | undefined = $state(undefined);
const limit = 10;
let pageActive = $state(1);

$effect(() => {
  (async () => {
    const response = await httpClient.fetchTransactions({
      fromTimestamp,
      toTimestamp,
      page: pageActive,
      limit,
    });
    records = response.data.map((transaction) => ({
      ...transaction,
      date: new Date(transaction.date_timestamp * 1_000).toLocaleDateString(
        "vi-vn",
      ),
      amount: Math.abs(transaction.amount),
      type: transaction.amount > 0 ? "income" : "expense",
    }));
    total = response.total;
  })();
});

let total = $state(0);
let pageCount = $derived(Math.max(Math.ceil(total / limit), 1));
let pagesDisplay = $derived.by(() => {
  const result = [];
  for (let i = 1; i <= pageCount; i++) {
    result.push(i);
  }
  return result;
});

function handleDateFilterButton(
  buttonType: "this-week" | "this-month" | "last-month" | "all-time",
) {
  const dateCurrent = new Date();
  switch (buttonType) {
    case "this-week": {
      const dateWeekStart = startOfWeek(dateCurrent);
      fromTimestamp = dateWeekStart.getTime() / 1_000;
      toTimestamp = dateCurrent.getTime() / 1_000;
      break;
    }
    case "this-month": {
      const dateMonthStart = startOfMonth(dateCurrent);
      fromTimestamp = dateMonthStart.getTime() / 1_000;
      toTimestamp = dateCurrent.getTime() / 1_000;
      break;
    }
    case "last-month": {
      const date1MonthAgo = subMonths(dateCurrent, 1);
      const dateMonthStart = startOfMonth(date1MonthAgo);
      const dateMonthEnd = endOfMonth(date1MonthAgo);
      fromTimestamp = dateMonthStart.getTime() / 1_000;
      toTimestamp = dateMonthEnd.getTime() / 1_000;
      break;
    }
    case "all-time": {
      fromTimestamp = undefined;
      toTimestamp = undefined;
      break;
    }
  }
}
</script>

<div class="flex flex-col gap-4">
    <span class="font-bold text-xl">Transactions</span>
    <div class="join">
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("this-week")}
        >
            This week
        </button>
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("this-month")}
        >
            This month
        </button>
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("last-month")}
        >
            Last month
        </button>
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("all-time")}
        >
            All time
        </button>
    </div>
    <table class="table">
        <thead>
        <tr>
            <th>Date</th>
            <th>Description</th>
            <th>Amount</th>
            <th>Type</th>
            <th>Category</th>
        </tr>
        </thead>
        <tbody>
        {#each records as record}
            <tr>
                <td>{record.date}</td>
                <td>{record.description}</td>
                <td>{record.amount}</td>
                <td>{record.type}</td>
                <td>
                    <select class="select">
                        <option disabled selected>Pick a category</option>
                        <option>Crimson</option>
                        <option>Amber</option>
                        <option>Velvet</option>
                    </select>
                </td>
            </tr>
        {/each}
        </tbody>
        <tfoot>
        <tr>
            <td colspan="4">
                <div class="join">
                    {#each pagesDisplay as pageDisplay}
                        <button class="join-item btn">{pageDisplay}</button>
                    {/each}
                </div>
            </td>
        </tr>
        </tfoot>
    </table>

    <a class="underline" href="/">Back</a>
</div>