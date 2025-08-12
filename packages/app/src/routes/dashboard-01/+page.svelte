<script lang="ts">
import ChartAreaInteractive from "$lib/components/chart-area-interactive.svelte";
import DataTable from "$lib/components/data-table.svelte";
import PieChart from "$lib/components/pie-chart.svelte";
import SectionCards from "$lib/components/section-cards.svelte";
import SiteHeader from "$lib/components/site-header.svelte";
import TransactionTable from "$lib/components/transaction-table.svelte";
import data from "./data.js";
</script>

<SiteHeader />
<div class="flex flex-1 flex-col">
    <div class="@container/main flex flex-1 flex-col gap-2">
        <div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
            <SectionCards />
            <div
                class="grid grid-cols-1 md:grid-cols-[1fr_1fr] lg:grid-cols-[24em_1fr] gap-2 md:gap-4 px-4 lg:px-6"
            >
                <PieChart />
                <TransactionTable />
            </div>
        </div>
    </div>
</div>



