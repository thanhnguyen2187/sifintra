<script lang="ts">
import EditIcon from "virtual:icons/mynaui/edit-solid";
import PlusIcon from "virtual:icons/mynaui/plus-solid";
import TrashIcon from "virtual:icons/mynaui/trash-solid";
import { endOfMonth, startOfMonth, startOfWeek, subMonths } from "date-fns";
import { onMount } from "svelte";
// biome-ignore lint/style/useImportType: false positive
import Toaster from "$lib/components/Toaster.svelte";
import { formatDateDb, formatDateDisplay } from "$lib/date";
import { httpClient } from "$lib/default";
import {
  type Category,
  createTransactionEdit,
  createTransactionEmpty,
  type Transaction,
} from "$lib/types";
// biome-ignore lint/style/useImportType: false positive
import EditModal from "./EditModal.svelte";

type TransactionDisplay = Transaction & {
  type: "expense" | "income";
  date: string;
};

let records: TransactionDisplay[] = $state([]);
let categories: Category[] = $state([]);

let editModal: EditModal;
let toaster: Toaster;

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
  populateTransactions({
    fromTimestamp,
    toTimestamp,
    page: pageActive,
    limit,
  });
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

async function populateTransactions({
  fromTimestamp,
  toTimestamp,
  page,
  limit,
}: {
  fromTimestamp: number | undefined;
  toTimestamp: number | undefined;
  page: number;
  limit: number;
}) {
  const response = await httpClient.fetchTransactions({
    fromTimestamp,
    toTimestamp,
    page,
    limit,
  });
  records = response.data.map((transaction) => ({
    ...transaction,
    date: formatDateDisplay(transaction.dateTimestamp),
    amount: Math.abs(transaction.amount),
    type: transaction.amount > 0 ? "income" : "expense",
  }));
  total = response.total;
}

function handleDateFilterButton(
  buttonType: "this-week" | "this-month" | "last-month" | "all-time",
) {
  const dateCurrent = new Date();
  switch (buttonType) {
    case "this-week": {
      const dateWeekStart = startOfWeek(dateCurrent, { weekStartsOn: 1 });
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

function handlePageActiveChange(value: number) {
  pageActive = value;
}

function handleDelete(id: string) {
  if (confirm("Are you sure? This cannot be reverted!")) {
    httpClient
      .deleteTransaction({ id })
      .then(() => {
        toaster.setMessageSuccess("Deleted!");
        populateTransactions({
          fromTimestamp,
          toTimestamp,
          page: pageActive,
          limit,
        });
      })
      .catch((err) => {
        console.error(err);
        toaster.setMessageError("Error happened deleting.");
      });
  }
}

function handleCategoryChange(e: Event, record: TransactionDisplay) {
  const target = e.target as HTMLSelectElement;
  const categoryId = target.value ? target.value : null;
  httpClient
    .updateTransaction({
      transaction: {
        ...record,
        dateString: formatDateDb(record.dateTimestamp),
        categoryId,
      },
    })
    .then(() => {
      toaster.setMessageSuccess("Categorized transaction!");
    })
    .catch((err) => {
      console.error(err);
      toaster.setMessageError("Error happened categorizing.");
    });
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
            <th>Actions</th>
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
                    <select
                        class="select"
                        bind:value={record.categoryId}
                        onchange={(e) => handleCategoryChange(e, record)}
                    >
                        <option value={null}>Uncategorized</option>
                        {#each categories as category}
                            <option value={category.id}>{category.name}</option>
                        {/each}
                    </select>
                </td>
                <td>
                    <div class="flex gap-2">
                        <button
                            class="btn btn-square"
                            onclick={() => {
                                const recordEdit = createTransactionEdit(record);
                                editModal.setRecord(recordEdit);
                                editModal.show();
                            }}
                        >
                            <EditIcon />
                        </button>
                        <button
                            class="btn btn-square"
                            onclick={() => handleDelete(record.id ?? "")}
                        >
                            <TrashIcon />
                        </button>
                    </div>
                </td>
            </tr>
        {:else}
            <tr>
                <td colspan="6">
                    No data yet.
                </td>
            </tr>
        {/each}
        </tbody>
        <tfoot>
        <tr>
            <td colspan="6">
                <div class="flex justify-between">
                    <div class="join">
                        {#each pagesDisplay as pageDisplay}
                            <button
                                class="join-item btn"
                                onclick={() => handlePageActiveChange(pageDisplay)}
                            >
                                {pageDisplay}
                            </button>
                        {/each}
                    </div>
                    <button
                        class="btn"
                        onclick={() => {
                            editModal.setRecord(createTransactionEmpty());
                            editModal.show();
                        }}
                    >
                        <PlusIcon />
                        Add
                    </button>
                </div>
            </td>
        </tr>
        </tfoot>
    </table>

    <a class="underline" href="/">Back</a>
</div>

<EditModal
    bind:this={editModal}
    onSuccess={() => {
        toaster.setMessageSuccess("Succeeded!");
        populateTransactions({
            fromTimestamp,
            toTimestamp,
            page: pageActive,
            limit,
        });
        editModal.hide();
    }}
    onFailure={() => {
        toaster.setMessageError("Error happened!");
    }}
/>
<Toaster bind:this={toaster} />