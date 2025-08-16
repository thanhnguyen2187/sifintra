<script lang="ts">
import SaveIcon from "virtual:icons/mynaui/save-solid";
import { httpClient } from "$lib/default";
import type { CategoryEdit } from "$lib/types";

let modal: HTMLDialogElement;
let record: CategoryEdit = $state({
  id: null,
  name: "",
});
let {
  onSuccess,
  onFailure,
}: {
  onSuccess: () => void;
  onFailure: () => void;
} = $props();

export function setRecord(value: CategoryEdit) {
  record = value;
}

export function show() {
  modal.showModal();
}

export function hide() {
  modal.close();
}

export function submit() {
  if (record.id === null) {
    httpClient
      .createCategory(record)
      .then(onSuccess)
      .catch((err) => {
        console.error(err);
        onFailure();
      });
  } else {
    httpClient
      .updateCategory(record)
      .then(onSuccess)
      .catch((err) => {
        console.error(err);
        onFailure();
      });
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
            <label class="fieldset-label" for="date">Name</label>
            <input
                class="input"
                type="text"
                id="name"
                bind:value={record.name}
            />
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