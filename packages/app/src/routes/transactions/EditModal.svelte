<script lang="ts">
import SaveIcon from "virtual:icons/mynaui/save-solid";
import { httpClient } from "$lib/default";
import type { Transaction, TransactionEdit } from "$lib/types";

let modal: HTMLDialogElement;
let record: TransactionEdit = $state({
  id: null,
  amount: 10_000,
  type: "expense",
  categoryId: null,
  dateTimestamp: 0,
  dateString: "",
  description: "",
});

export function setRecord(value: TransactionEdit) {
  record = value;
}

export function show() {
  modal.showModal();
}

export function submit() {
  if (record.id === null) {
    httpClient
      .createTransaction({
        transaction: {
          ...record,
        },
        transactionType: record.type,
      })
      .catch(console.error);
  } else {
    httpClient
      .updateTransaction({
        transaction: {
          ...record,
        },
      })
      .catch(console.error);
  }
}
</script>

<dialog
    id="modal"
    class="modal"
    bind:this={modal}
>
    <div class="modal-box w-max">
        <fieldset class="fieldset">
            <label class="fieldset-label" for="date">Date</label>
            <input
                class="input"
                type="datetime-local"
                id="date"
                bind:value={record.dateString}
            />
        </fieldset>
        <fieldset class="fieldset">
            <label class="fieldset-label" for="description">Description</label>
            <input
                class="input"
                id="description"
                bind:value={record.description}
            />
        </fieldset>
        <fieldset class="fieldset">
            <label class="fieldset-label" for="amount">Amount</label>
            <input
                class="input"
                type="number"
                id="amount"
                bind:value={record.amount}
            />
        </fieldset>
        <fieldset class="fieldset">
            <label class="fieldset-label" for="amount">Type</label>
            <select
                class="select"
                bind:value={record.type}
            >
                <option value="income">Income</option>
                <option value="expense">Expense</option>
            </select>
        </fieldset>
        <div class="flex mt-4">
            <button
                class="btn"
                onclick={submit}
            >
                <SaveIcon />
                Save
            </button>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>