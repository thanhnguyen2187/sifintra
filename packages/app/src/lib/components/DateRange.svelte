<script lang="ts">
let {
  initFromDate = undefined,
  initToDate = undefined,
  onFromDateChange,
  onToDateChange,
}: {
  initFromDate?: string;
  initToDate?: string;
  onFromDateChange: (value: string) => void;
  onToDateChange: (value: string) => void;
} = $props();

function handleFromDateChange(e: Event) {
  const target = e.target as HTMLInputElement;
  onFromDateChange(target.value);
}

function handleToDateChange(e: Event) {
  const target = e.target as HTMLInputElement;
  onToDateChange(target.value);
}
</script>

<div class="flex flex-col sm:flex-row gap-4">
    <label>
        From Date
        <input
            name="from-date"
            class="input w-40"
            type="date"
            onchange={handleFromDateChange}
            value={initFromDate}
        />
    </label>
    <label>
        To Date
        <input
            name="to-date"
            class="input w-40"
            type="date"
            onchange={handleToDateChange}
            value={initToDate}
        />
    </label>
</div>

