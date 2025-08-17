<script lang="ts">
/** biome-ignore-all lint/style/useImportType: false positive */
import EditIcon from "virtual:icons/mynaui/edit-solid";
import PlusIcon from "virtual:icons/mynaui/plus-solid";
import TrashIcon from "virtual:icons/mynaui/trash-solid";
import { onMount } from "svelte";
import Toaster from "$lib/components/Toaster.svelte";
import { httpClient } from "$lib/default";
import {
  type Category,
  type CategoryDisplay,
  createCategoryEmpty,
} from "$lib/types";
import { createCategoryDisplay } from "$lib/types.js";
import EditModal from "./EditModal.svelte";

let modal: EditModal;
let toaster: Toaster;

let records: CategoryDisplay[] = $state([]);

async function populateRecords() {
  const resp = await httpClient.fetchCategories();
  records = resp.data.map(createCategoryDisplay);
}

function handleAdd() {
  modal.setRecord(createCategoryEmpty());
  modal.show();
}

function handleEdit(record: Category) {
  modal.setRecord({ ...record });
  modal.show();
}

function handleDelete(id: string) {
  if (confirm("Are you sure? This cannot be reverted.")) {
    httpClient
      .deleteCategory({ id })
      .then(() => {
        toaster.setMessageSuccess("Deleted successfully.");
        populateRecords();
      })
      .catch((err) => {
        console.error(err);
        toaster.setMessageError("Error happened deleting.");
      });
  }
}

onMount(() => {
  populateRecords().then();
});
</script>

<div class="flex flex-col gap-4">
    <span class="font-bold text-xl">Categories</span>
    <table class="table">
        <thead>
        <tr>
            <th>Name</th>
            <th>Actions</th>
            <th>Date Modified</th>
            <th>Date Created</th>
        </tr>
        </thead>
        <tbody>
        {#each records as record}
            <tr>
                <td>{record.name}</td>
                <td>
                    <button
                        class="btn"
                        onclick={() => handleEdit(record)}
                    >
                        <EditIcon />
                    </button>
                    <button
                        class="btn"
                        onclick={() => handleDelete(record.id)}
                    >
                        <TrashIcon />
                    </button>
                </td>
                <td>{record.updatedAtCorrected}</td>
                <td>{record.createdAtCorrected}</td>
            </tr>
        {:else}
            <tr>
                <td colspan="4">No data yet.</td>
            </tr>
        {/each}
        </tbody>
        <tfoot>
        <tr>
            <td class="text-right" colspan="4">
                <button
                    class="btn"
                    onclick={handleAdd}
                >
                    <PlusIcon />
                    Add
                </button>
            </td>
        </tr>
        </tfoot>
    </table>

    <div>
        <a class="underline" href="/">Back</a>
    </div>
</div>

<EditModal
    bind:this={modal}
    onSuccess={() => {
        populateRecords();
        toaster.setMessageSuccess("Succeeded!");
        modal.hide();
    }}
    onFailure={() => {
        toaster.setMessageError("Error happened. Please try again later.");
    }}
/>
<Toaster
    bind:this={toaster}
/>