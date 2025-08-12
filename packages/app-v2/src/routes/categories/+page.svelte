<script lang="ts">
import EditIcon from "virtual:icons/mynaui/edit-solid";
import PlusIcon from "virtual:icons/mynaui/plus-solid";
import TrashIcon from "virtual:icons/mynaui/trash-solid";

type Category = {
  id: string;
  name: string;
  dateCreated: string;
  dateModified: string;
};

const records: Category[] = [
  {
    id: "",
    name: "Others",
    dateCreated: "2024-07-31 17:55",
    dateModified: "2024-07-31 17:55",
  },
  {
    id: "",
    name: "Cats",
    dateCreated: "2024-07-31 17:55",
    dateModified: "2024-07-31 17:55",
  },
  {
    id: "",
    name: "Transportation",
    dateCreated: "2024-07-31 17:55",
    dateModified: "2024-07-31 17:55",
  },
  {
    id: "",
    name: "Food",
    dateCreated: "2024-07-31 17:55",
    dateModified: "2024-07-31 17:55",
  },
];
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
                <td>{record.dateModified}</td>
                <td>{record.dateCreated}</td>
            </tr>
        {/each}
        </tbody>
        <tfoot>
        <tr>
            <td class="text-center" colspan="4">
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