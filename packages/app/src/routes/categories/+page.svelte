<script lang="ts">
import EditIcon from "virtual:icons/mynaui/edit-solid";
import PlusIcon from "virtual:icons/mynaui/plus-solid";
import TrashIcon from "virtual:icons/mynaui/trash-solid";
import { onMount } from "svelte";
import { httpClient } from "$lib/default";
import type { Category } from "$lib/types";

let records: Category[] = $state([]);

async function populateRecords() {
  const resp = await httpClient.fetchCategories();
  records = resp.data;
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
                    <button class="btn">
                        <EditIcon />
                    </button>
                    <button class="btn">
                        <TrashIcon />
                    </button>
                </td>
                <td>{record.updatedAt}</td>
                <td>{record.createdAt}</td>
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
                <button class="btn">
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